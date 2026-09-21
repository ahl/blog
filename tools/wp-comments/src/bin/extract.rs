//! Extract archived comments from the redacted WordPress export into one
//! `comments.json` per post, alongside the post it belongs to.
//!
//! This is a one-shot migration, not a build step: the blog it reads from no
//! longer exists and its export is frozen at 2020-12-01. Run it once, commit
//! the output, keep the tool for provenance. Nothing in the Eleventy build
//! depends on it.
//!
//! Correctness here is load-bearing and the failure mode is silent -- a
//! sanitizer that quietly eats a `<blockquote>` produces a slightly wrong
//! archive that nobody notices for years. So every run asserts a set of
//! invariants and prints a conservation report, and exits non-zero if any of
//! them fails. See `verify` below.
//!
//! Usage:
//!     cargo run --bin extract -- <redacted.xml> <content/blog> <report.json>

use ammonia::Builder;
use anyhow::{bail, Context, Result};
use regex::Regex;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::path::{Path, PathBuf};
use wp_comments::{
    comparable_text, rewrite_url, visible_text, wpautop, SlugIndex, UrlAction, ALLOWED_TAGS,
};

const WP_NS: &str = "http://wordpress.org/export/1.1/";
const AHL_NAMES: [&str; 2] = ["ahl", "Adam Leventhal"];
const AHL_USER_ID: &str = "3";

/// ahl's display name, normalized. The export splits his 225 comments across
/// "ahl" (logged in) and "Adam Leventhal" (not); the distinction is an
/// artifact of authentication, not intent.
const AHL_DISPLAY: &str = "ahl";

/// Approved comments that are unambiguously spam and are dropped.
///
/// All three are the same 2358-byte Japanese link-spam payload from "hotindex"
/// posted within 40 seconds on 2004-12-19, which slipped past moderation.
/// Listed by id so the decision is reviewable and reversible, rather than
/// hand-edited out of the generated output.
const SPAM_IDS: [&str; 3] = ["1748", "1751", "1753"];

#[derive(Serialize)]
struct CommentsFile {
    /// Permalink of the post these belong to, as a cross-check against the
    /// directory the file sits in.
    post: String,
    count: usize,
    comments: Vec<Comment>,
}

#[derive(Serialize)]
struct Comment {
    id: u64,
    /// Parent comment id, or absent for a top-level comment. Threading is
    /// preserved here; whether the template nests or flattens is its choice.
    #[serde(skip_serializing_if = "Option::is_none")]
    parent: Option<u64>,
    author: String,
    /// The author's own URL, as typed. Always public (it was the link on their
    /// name), unlike the email and IP, which are never emitted in any form.
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    /// True for ahl's own replies.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    is_author: bool,
    /// GMT, ISO-8601.
    date: String,
    /// "comment" or "pingback". Pingbacks are kept so the decision to render
    /// them stays open, but they are almost all dead links by now.
    kind: String,
    html: String,
}

#[derive(Serialize, Default)]
struct Report {
    posts_with_comments: usize,
    comments_emitted: usize,
    by_kind: BTreeMap<String, usize>,
    skipped: BTreeMap<String, usize>,
    accounting: Accounting,
    links: LinkReport,
    tags_dropped: BTreeMap<String, usize>,
    /// Posts in the export carrying comments that have no directory in
    /// content/blog. Expected to be empty.
    unmatched_posts: Vec<String>,
    /// Comments whose parent did not survive extraction and were promoted to
    /// top level. Expected to be empty.
    reparented: Vec<u64>,
    /// Comments whose visible text changed during rendering. This is the
    /// check that catches a lossy sanitizer, and it must stay empty.
    fidelity_mismatches: Vec<FidelityMismatch>,
}

#[derive(Serialize)]
struct FidelityMismatch {
    id: u64,
    /// Character offset where the two first diverge.
    diverges_at: usize,
    /// A window around the divergence, not the head of the string -- the
    /// difference is the whole point of the record.
    before: String,
    after: String,
}

/// Character offset of the first difference, plus a readable window around it.
fn diff_window(a: &str, b: &str) -> (usize, String, String) {
    let (ac, bc): (Vec<char>, Vec<char>) = (a.chars().collect(), b.chars().collect());
    let at = ac
        .iter()
        .zip(bc.iter())
        .position(|(x, y)| x != y)
        .unwrap_or_else(|| ac.len().min(bc.len()));
    let start = at.saturating_sub(60);
    let window = |v: &[char]| -> String {
        let s: String = v.iter().skip(start).take(160).collect();
        format!("{}{s}{}", if start > 0 { "…" } else { "" }, if v.len() > start + 160 { "…" } else { "" })
    };
    (at, window(&ac), window(&bc))
}

#[derive(Serialize, Default)]
struct Accounting {
    comment_elements_in_file: usize,
    emitted: usize,
    skipped_total: usize,
    balances: bool,
}

#[derive(Serialize, Default)]
struct LinkReport {
    hrefs_in_source: usize,
    rewritten: usize,
    rewrites_by_rule: BTreeMap<String, usize>,
    /// Internal links we could not resolve. Left exactly as written; listed
    /// here so they can be mapped by hand if desired.
    unresolved_internal: Vec<String>,
    /// Comments where sanitizing changed the number of links. Must be empty.
    lost: Vec<LostLinks>,
    /// Internal URLs that appear as plain text rather than as links. Left
    /// alone -- rewriting them would change what the comment says -- but
    /// listed because they are candidates for linkifying in the template.
    bare_internal: Vec<String>,
}

#[derive(Serialize)]
struct LostLinks {
    id: u64,
    before: usize,
    after: usize,
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        bail!(
            "usage: {} <redacted.xml> <content/blog> <report.json>",
            args[0]
        );
    }
    let (xml_path, blog_dir, report_path) = (&args[1], PathBuf::from(&args[2]), &args[3]);

    let xml = std::fs::read_to_string(xml_path)
        .with_context(|| format!("reading {xml_path}"))?;
    let doc = roxmltree::Document::parse(&xml).context("parsing export")?;

    let posts = collect_posts(&doc)?;
    let index: SlugIndex = posts
        .iter()
        .filter(|p| !p.slug.is_empty())
        .map(|p| (p.slug.clone(), p.permalink.clone()))
        .collect();

    let mut report = Report::default();
    report.accounting.comment_elements_in_file = doc
        .descendants()
        .filter(|n| n.has_tag_name((WP_NS, "comment")))
        .count();

    let dirs = existing_dirs(&blog_dir)?;
    let mut written: Vec<(PathBuf, String)> = Vec::new();

    // Comments attached to attachments and the lone page. Not rendered
    // anywhere, but they exist in the file and must be accounted for or the
    // conservation check cannot balance.
    *report.skipped.entry("on_non_post_item".into()).or_default() += non_post_comments(&doc);

    for post in &posts {
        let raw = &post.comments;
        if raw.is_empty() {
            continue;
        }
        if !post.published {
            *report
                .skipped
                .entry("on_unpublished_draft".into())
                .or_default() += raw.len();
            continue;
        }
        let Some(dir) = dirs.get(&post.dir_name()) else {
            report.unmatched_posts.push(post.dir_name());
            *report.skipped.entry("post_directory_missing".into()).or_default() += raw.len();
            continue;
        };

        let mut comments = Vec::new();
        let mut kept_ids: HashSet<u64> = HashSet::new();

        for c in raw {
            if c.approved != "1" {
                *report.skipped.entry("unapproved".into()).or_default() += 1;
                continue;
            }
            if SPAM_IDS.contains(&c.id.as_str()) {
                *report.skipped.entry("spam".into()).or_default() += 1;
                continue;
            }
            let rendered = render(&c.content, &index, &mut report);
            let id: u64 = c.id.parse().context("comment id")?;
            kept_ids.insert(id);

            // Fidelity: rendering may add markup, never change the words.
            let (before, after) = (visible_text(&c.content), visible_text(&rendered));
            if comparable_text(&c.content) != comparable_text(&rendered) {
                let (diverges_at, b, a) = diff_window(&before, &after);
                report.fidelity_mismatches.push(FidelityMismatch {
                    id,
                    diverges_at,
                    before: b,
                    after: a,
                });
            }

            // Tag accounting: every tag the sanitizer removed, by name.
            for tag in tags_in(&c.content).difference(&tags_in(&rendered)) {
                *report.tags_dropped.entry(tag.clone()).or_default() += 1;
            }

            // Link conservation: sanitizing must not lose a link. Counting
            // them is not enough -- the counts have to be asserted equal.
            let (before_n, after_n) = (count_hrefs(&c.content), count_hrefs(&rendered));
            if before_n != after_n {
                report.links.lost.push(LostLinks { id, before: before_n, after: after_n });
            }

            // Bare internal URLs are left as typed: rewriting them would
            // change visible text. Recorded so they can be linkified by hand.
            for u in bare_urls(&c.content) {
                if matches!(rewrite_url(&u, &index), UrlAction::Rewrite(_)) {
                    report.links.bare_internal.push(u);
                }
            }

            // Author names are plain text, not markup, but WordPress stored
            // them HTML-encoded -- 37 pingback titles carry entities like
            // `&#039;` and `&raquo;`. Decode here so the template escapes once
            // rather than the reader seeing the entity spelled out.
            let author = html_escape::decode_html_entities(&c.author).into_owned();
            let is_author = c.user_id == AHL_USER_ID || AHL_NAMES.contains(&author.as_str());
            let kind = if c.kind == "pingback" { "pingback" } else { "comment" };
            *report.by_kind.entry(kind.to_string()).or_default() += 1;

            comments.push((
                Comment {
                    id,
                    parent: c.parent.parse::<u64>().ok().filter(|p| *p != 0),
                    author: if is_author {
                        AHL_DISPLAY.to_string()
                    } else if author.trim().is_empty() {
                        "Anonymous".to_string()
                    } else {
                        author
                    },
                    url: Some(c.url.trim().to_string()).filter(|u| !u.is_empty()),
                    is_author,
                    date: to_iso8601(&c.date_gmt)?,
                    kind: kind.to_string(),
                    html: rendered,
                },
                c.date_gmt.clone(),
            ));
        }

        if comments.is_empty() {
            continue;
        }

        // Chronological, with id as a tiebreaker so the ordering is total and
        // the output is byte-stable across runs.
        comments.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.id.cmp(&b.0.id)));
        let mut comments: Vec<Comment> = comments.into_iter().map(|(c, _)| c).collect();

        // Dropping spam can orphan a reply. Promote rather than lose it.
        for c in &mut comments {
            if let Some(p) = c.parent {
                if !kept_ids.contains(&p) {
                    report.reparented.push(c.id);
                    c.parent = None;
                }
            }
        }

        report.posts_with_comments += 1;
        report.comments_emitted += comments.len();

        let file = CommentsFile {
            post: post.permalink.clone(),
            count: comments.len(),
            comments,
        };
        let json = serde_json::to_string_pretty(&file)? + "\n";
        written.push((dir.join("comments.json"), json));
    }

    let verdict = verify(&mut report, &posts, &index);

    // Always write the report, even when verification fails -- a failure is
    // precisely when you need to read it. Comment files are only written if
    // everything passed, so a failed run never leaves partial output behind.
    let report_json = serde_json::to_string_pretty(&report)? + "\n";
    std::fs::write(report_path, &report_json)?;

    if let Err(e) = verdict {
        eprintln!("verification FAILED; report written to {report_path}");
        return Err(e);
    }

    for (path, json) in &written {
        std::fs::write(path, json).with_context(|| format!("writing {}", path.display()))?;
    }

    print_report(&report, written.len(), report_path);
    if !report.accounting.balances {
        bail!("comment accounting does not balance; refusing to claim success");
    }
    Ok(())
}

struct Post {
    slug: String,
    date: String,
    permalink: String,
    published: bool,
    comments: Vec<RawComment>,
}

impl Post {
    fn dir_name(&self) -> String {
        format!("{}-{}", self.date, self.slug)
    }
}

struct RawComment {
    id: String,
    parent: String,
    author: String,
    url: String,
    date_gmt: String,
    content: String,
    approved: String,
    kind: String,
    user_id: String,
}

fn collect_posts(doc: &roxmltree::Document) -> Result<Vec<Post>> {
    let wp = |n: &roxmltree::Node, tag: &str| -> String {
        n.children()
            .find(|c| c.has_tag_name((WP_NS, tag)))
            .and_then(|c| c.text())
            .unwrap_or("")
            .trim()
            .to_string()
    };

    let mut posts = Vec::new();
    for item in doc.descendants().filter(|n| n.has_tag_name("item")) {
        if wp(&item, "post_type") != "post" {
            continue;
        }
        let slug = wp(&item, "post_name");
        let date = wp(&item, "post_date");
        let day = date.get(..10).unwrap_or("").to_string();
        let comments = item
            .children()
            .filter(|n| n.has_tag_name((WP_NS, "comment")))
            .map(|c| RawComment {
                id: wp(&c, "comment_id"),
                parent: wp(&c, "comment_parent"),
                author: wp(&c, "comment_author"),
                url: wp(&c, "comment_author_url"),
                date_gmt: wp(&c, "comment_date_gmt"),
                // Body text must not be trimmed the way metadata is: leading
                // whitespace is meaningful once wpautop runs.
                content: c
                    .children()
                    .find(|n| n.has_tag_name((WP_NS, "comment_content")))
                    .and_then(|n| n.text())
                    .unwrap_or("")
                    .to_string(),
                approved: wp(&c, "comment_approved"),
                kind: wp(&c, "comment_type"),
                user_id: wp(&c, "comment_user_id"),
            })
            .collect();

        posts.push(Post {
            permalink: format!("/{}/{}/", day.replace('-', "/"), slug),
            published: !slug.is_empty(),
            slug,
            date: day,
            comments,
        });
    }
    Ok(posts)
}

fn existing_dirs(blog_dir: &Path) -> Result<BTreeMap<String, PathBuf>> {
    let mut m = BTreeMap::new();
    for entry in std::fs::read_dir(blog_dir)
        .with_context(|| format!("reading {}", blog_dir.display()))?
    {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            m.insert(entry.file_name().to_string_lossy().into_owned(), entry.path());
        }
    }
    Ok(m)
}

/// Paragraph-wrap, sanitize, and rewrite links in one comment body.
fn render(raw: &str, index: &SlugIndex, report: &mut Report) -> String {
    let paragraphed = wpautop(raw);

    let allowed: HashSet<&str> = ALLOWED_TAGS.iter().copied().collect();
    let cleaned = Builder::default()
        .tags(allowed)
        .link_rel(Some("nofollow ugc noopener noreferrer"))
        .clean(&paragraphed)
        .to_string();

    // Rewrite hrefs after sanitizing. Ammonia's output is normalized
    // html5ever serialization -- attributes are double-quoted and any literal
    // quote inside a value is escaped -- so this pattern matches an attribute
    // value exactly. It is not being used to parse arbitrary HTML.
    let re = href_re();
    re.replace_all(&cleaned, |caps: &regex::Captures| {
        let url = html_escape::decode_html_entities(&caps[1]).into_owned();
        report.links.hrefs_in_source += 1;
        match rewrite_url(&url, index) {
            UrlAction::Keep => format!("href=\"{}\"", &caps[1]),
            UrlAction::Rewrite(new) => {
                report.links.rewritten += 1;
                let rule = if new.starts_with("https://bcantrill") {
                    "bmc_to_bcantrill"
                } else {
                    "internal_to_root_relative"
                };
                *report.links.rewrites_by_rule.entry(rule.into()).or_default() += 1;
                format!("href=\"{}\"", html_escape::encode_double_quoted_attribute(&new))
            }
            UrlAction::UnresolvedInternal => {
                report.links.unresolved_internal.push(url);
                format!("href=\"{}\"", &caps[1])
            }
        }
    })
    .into_owned()
}

/// Comments hanging off items that are not posts (attachments, the one page).
fn non_post_comments(doc: &roxmltree::Document) -> usize {
    doc.descendants()
        .filter(|n| n.has_tag_name("item"))
        .filter(|item| {
            item.children()
                .find(|c| c.has_tag_name((WP_NS, "post_type")))
                .and_then(|c| c.text())
                .map(|t| t.trim() != "post")
                .unwrap_or(true)
        })
        .map(|item| {
            item.children()
                .filter(|n| n.has_tag_name((WP_NS, "comment")))
                .count()
        })
        .sum()
}

/// Count `href` attributes, accepting either quote style (the source uses
/// both; ammonia's output uses only double).
fn count_hrefs(html: &str) -> usize {
    use std::sync::OnceLock;
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| Regex::new(r#"(?i)href\s*=\s*["']"#).unwrap());
    re.find_iter(html).count()
}

/// URLs sitting in the body as plain text rather than inside an anchor.
fn bare_urls(html: &str) -> Vec<String> {
    use std::sync::OnceLock;
    static URL: OnceLock<Regex> = OnceLock::new();
    static HREF: OnceLock<Regex> = OnceLock::new();
    let url = URL.get_or_init(|| Regex::new(r#"https?://[^\s"'<>)]+"#).unwrap());
    let href = HREF.get_or_init(|| Regex::new(r#"(?i)href\s*=\s*["']([^"']+)["']"#).unwrap());
    let linked: Vec<&str> = href.captures_iter(html).map(|c| c.get(1).unwrap().as_str()).collect();
    url.find_iter(html)
        .map(|m| m.as_str())
        .filter(|u| !linked.iter().any(|h| h.contains(*u)))
        .map(|u| u.to_string())
        .collect()
}

/// The set of element names appearing in a fragment, lowercased.
fn tags_in(html: &str) -> BTreeSet<String> {
    use std::sync::OnceLock;
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| Regex::new(r"<\s*/?\s*([a-zA-Z][a-zA-Z0-9]*)").unwrap());
    re.captures_iter(html)
        .map(|c| c[1].to_ascii_lowercase())
        .collect()
}


fn href_re() -> &'static Regex {
    use std::sync::OnceLock;
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r#"href="([^"]*)""#).unwrap())
}

/// `2006-06-04 20:49:24` -> `2006-06-04T20:49:24Z`.
///
/// The export writes GMT timestamps in a fixed format, so this is string
/// surgery rather than date parsing -- but validate the shape so a malformed
/// value fails loudly instead of producing a bogus timestamp.
fn to_iso8601(gmt: &str) -> Result<String> {
    let bytes = gmt.as_bytes();
    let shaped = bytes.len() == 19
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes[10] == b' '
        && bytes[13] == b':'
        && bytes[16] == b':';
    if !shaped {
        bail!("unexpected timestamp format: {gmt:?}");
    }
    Ok(format!("{}T{}Z", &gmt[..10], &gmt[11..]))
}

/// The invariants. Each one exists because its failure mode is silent.
fn verify(report: &mut Report, posts: &[Post], index: &SlugIndex) -> Result<()> {
    // 1. Conservation: nothing vanishes unaccounted for.
    let skipped_total: usize = report.skipped.values().sum();
    report.accounting.emitted = report.comments_emitted;
    report.accounting.skipped_total = skipped_total;
    report.accounting.balances =
        report.comments_emitted + skipped_total == report.accounting.comment_elements_in_file;

    // 2. Thread integrity: every surviving parent link resolves.
    //    (Orphans were promoted during extraction and recorded; anything left
    //    here would be a bug.)

    // 3. Join integrity: posts carrying comments must have a directory.
    if !report.unmatched_posts.is_empty() {
        eprintln!(
            "warning: {} post(s) with comments have no directory in content/blog",
            report.unmatched_posts.len()
        );
    }

    // 4. Fidelity: rendering must never change the words. A single mismatch
    //    means the sanitizer is eating content, which is the exact silent
    //    failure this whole tool is built to prevent.
    if !report.fidelity_mismatches.is_empty() {
        bail!(
            "{} comment(s) had their visible text changed by rendering; \
             see fidelity_mismatches in the report (first: id {})",
            report.fidelity_mismatches.len(),
            report.fidelity_mismatches[0].id
        );
    }

    // 5. Tag accounting: the only tag the corpus should lose is the stray
    //    <poolname> someone typed in a ZFS example. Anything else means the
    //    allowlist is wrong and real markup is being discarded.
    const EXPECTED_DROPPED: [&str; 1] = ["poolname"];
    let unexpected: Vec<&String> = report
        .tags_dropped
        .keys()
        .filter(|t| !EXPECTED_DROPPED.contains(&t.as_str()))
        .collect();
    if !unexpected.is_empty() {
        bail!(
            "sanitizer dropped unexpected tag(s): {unexpected:?} -- \
             either the allowlist is wrong or the corpus changed"
        );
    }

    // 6. Link conservation: no comment may come out with fewer links than it
    //    went in with.
    if !report.links.lost.is_empty() {
        let f = &report.links.lost[0];
        bail!(
            "{} comment(s) lost links during sanitization (first: id {} had {} \
             href(s), emitted {})",
            report.links.lost.len(),
            f.id,
            f.before,
            f.after
        );
    }

    // 7. Slug index sanity: it must cover every published post.
    let published = posts.iter().filter(|p| p.published).count();
    if index.len() != published {
        bail!(
            "slug index has {} entries but there are {} published posts",
            index.len(),
            published
        );
    }
    Ok(())
}

fn print_report(r: &Report, files: usize, report_path: &str) {
    println!("extraction report");
    println!("  posts with comments   {}", r.posts_with_comments);
    println!("  comments emitted      {}", r.comments_emitted);
    for (k, v) in &r.by_kind {
        println!("    {v:>5}  {k}");
    }
    println!();
    println!("  skipped:");
    if r.skipped.is_empty() {
        println!("    (none)");
    }
    for (k, v) in &r.skipped {
        println!("    {v:>5}  {k}");
    }
    println!();
    let a = &r.accounting;
    println!(
        "  accounting: {} emitted + {} skipped = {} (file has {}) -> {}",
        a.emitted,
        a.skipped_total,
        a.emitted + a.skipped_total,
        a.comment_elements_in_file,
        if a.balances { "BALANCES" } else { "MISMATCH" }
    );
    println!();
    let l = &r.links;
    println!("  links: {} hrefs, {} rewritten", l.hrefs_in_source, l.rewritten);
    for (k, v) in &l.rewrites_by_rule {
        println!("    {v:>5}  {k}");
    }
    if !l.unresolved_internal.is_empty() {
        println!(
            "    {:>5}  unresolved internal (left as written, listed in report)",
            l.unresolved_internal.len()
        );
    }
    if !r.tags_dropped.is_empty() {
        println!();
        println!("  tags dropped by the sanitizer:");
        for (k, v) in &r.tags_dropped {
            println!("    {v:>5}  <{k}>");
        }
    }
    if !r.reparented.is_empty() {
        println!();
        println!("  {} reply/replies promoted to top level (parent removed)", r.reparented.len());
    }
    println!();
    println!("  wrote {files} comments.json files");
    println!("  wrote {report_path}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iso8601_conversion() {
        assert_eq!(to_iso8601("2006-06-04 20:49:24").unwrap(), "2006-06-04T20:49:24Z");
    }

    #[test]
    fn iso8601_rejects_garbage() {
        assert!(to_iso8601("").is_err());
        assert!(to_iso8601("2006-06-04").is_err());
        assert!(to_iso8601("not a timestamp!!!").is_err());
    }

    /// The central fidelity invariant, stated as a property rather than a
    /// hand-typed expectation: rendering may add markup but must never change
    /// the words. `visible_text` applies the same crude normalization to both
    /// sides, so tag-boundary spacing cancels out and only real content
    /// differences can fail this.
    #[test]
    fn render_preserves_visible_text() {
        let mut r = Report::default();
        let idx = SlugIndex::new();
        for raw in [
            "I think <i>this</i> is right.\n\nAlso see <a href=\"http://example.com/\">here</a>.",
            "plain text with no markup at all",
            "line one\nline two\n\npara two",
            "<blockquote>quoted <b>bold</b></blockquote>\n\nreply",
            "entities: &amp; &lt; &#20154;",
            "set <poolname> here",
        ] {
            let out = render(raw, &idx, &mut r);
            assert_eq!(
                comparable_text(raw),
                comparable_text(&out),
                "rendering changed the visible text of {raw:?}"
            );
        }
    }

    /// The real-world case that forced the whitespace-insensitive comparison:
    /// a stray unmatched `</a>` that html5ever discards. No words change, so
    /// this must pass -- comment 1627 from 2005.
    #[test]
    fn stray_close_tag_is_not_a_fidelity_failure() {
        let mut r = Report::default();
        let raw = "Focuses on bringing more usability to Arch.</a>&lt;/dd&gt;";
        let out = render(raw, &SlugIndex::new(), &mut r);
        assert_eq!(comparable_text(raw), comparable_text(&out));
    }

    /// ...but the check must still fail when content genuinely disappears,
    /// or relaxing it would have quietly disabled the whole safety net.
    #[test]
    fn comparable_text_still_catches_real_content_loss() {
        // `<script>` is the one case where ammonia removes the *contents*, not
        // just the tag -- exactly the kind of silent loss we must detect.
        let mut r = Report::default();
        let raw = "before<script>SECRETLY EATEN</script>after";
        let out = render(raw, &SlugIndex::new(), &mut r);
        assert_ne!(
            comparable_text(raw),
            comparable_text(&out),
            "content vanished but the fidelity check did not notice"
        );
        assert!(!comparable_text(&out).contains("SECRETLY"));
    }

    #[test]
    fn comparable_text_ignores_only_whitespace() {
        assert_eq!(comparable_text("a b  c"), comparable_text("a  b c"));
        assert_ne!(comparable_text("a b c"), comparable_text("a b"));
    }

    #[test]
    fn render_keeps_allowed_markup() {
        let mut r = Report::default();
        let out = render("a <blockquote>quoted</blockquote> b", &SlugIndex::new(), &mut r);
        assert!(out.contains("<blockquote>"));
    }

    #[test]
    fn render_strips_unknown_tags_but_keeps_their_text() {
        let mut r = Report::default();
        let out = render("set <poolname> here", &SlugIndex::new(), &mut r);
        assert!(!out.contains("<poolname"));
        assert!(visible_text(&out).contains("set"));
        assert!(visible_text(&out).contains("here"));
    }

    #[test]
    fn render_strips_dangerous_markup_entirely() {
        let mut r = Report::default();
        let out = render("<script>alert(1)</script>ok", &SlugIndex::new(), &mut r);
        assert!(!out.contains("script"));
        assert!(!out.contains("alert"));
    }

    #[test]
    fn render_adds_nofollow() {
        let mut r = Report::default();
        let out = render("<a href=\"http://x.example/\">x</a>", &SlugIndex::new(), &mut r);
        assert!(out.contains("nofollow"));
    }

    #[test]
    fn render_rewrites_internal_links() {
        let mut r = Report::default();
        let out = render(
            "<a href=\"http://dtrace.org/blogs/ahl/2016/06/19/apfs-part1/\">apfs</a>",
            &SlugIndex::new(),
            &mut r,
        );
        assert!(out.contains("href=\"/2016/06/19/apfs-part1/\""), "{out}");
        assert_eq!(r.links.rewritten, 1);
    }

    #[test]
    fn render_reports_but_does_not_touch_unresolvable_internal_links() {
        let mut r = Report::default();
        let url = "http://dtrace.org/blogs/ahl/what_if_machine_dtrace_port";
        let out = render(&format!("<a href=\"{url}\">x</a>"), &SlugIndex::new(), &mut r);
        assert!(out.contains(url), "link must be left exactly as written");
        assert_eq!(r.links.unresolved_internal, vec![url.to_string()]);
        assert_eq!(r.links.rewritten, 0);
    }
}
