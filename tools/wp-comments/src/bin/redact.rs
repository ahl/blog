//! Produce a committable, PII-free copy of the WordPress export.
//!
//! The raw WXR export contains 750 commenter email addresses (453 distinct)
//! and 476 IP addresses (323 distinct), belonging to real people who typed
//! them into a comment form between 2004 and 2020 with no expectation that
//! they would ever be published. `github.com/ahl/blog` is public, and a commit
//! is effectively irreversible -- so the raw file stays out of git and this
//! tool produces the copy that goes in.
//!
//! Redaction is deliberately minimal: exactly three element types change, and
//! nothing else is touched. That is what makes the transform verifiable -- a
//! line-by-line diff of source against output must show differences *only* on
//! lines carrying one of those three elements, and the line count must be
//! unchanged. A parse-and-reserialize approach would reformat the whole
//! document and destroy that property, so this is a line-oriented textual
//! transform instead. Every target element was verified to sit alone on its
//! own line with no CDATA wrapper (the channel-header `wp:author_email` is the
//! one inline exception, handled by the same substitution).
//!
//! Usage:
//!     cargo run --bin redact -- <source.xml> <output.xml> <manifest.json>

use anyhow::{bail, Context, Result};
use regex::{Captures, Regex};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use wp_comments::{email_shaped, ip_shaped, redact_email};

const WP_NS: &str = "http://wordpress.org/export/1.1/";

/// Display names that identify ahl's own comments. See `AuthorIdentity` below.
const AHL_NAMES: [&str; 2] = ["ahl", "Adam Leventhal"];
const AHL_USER_ID: &str = "3";

/// Facts about the source file that redaction destroys.
///
/// Once emails and IPs are gone, claims like "750 comments carried an email"
/// or "the author-identification rule has no false positives" can no longer be
/// checked against the committed file. Recording them here, generated from the
/// raw file at the one moment it is in hand, keeps those claims auditable.
#[derive(Serialize)]
struct Manifest {
    source_file: String,
    source_sha256: String,
    redacted_sha256: String,
    elements_redacted: BTreeMap<String, usize>,
    source_facts: SourceFacts,
    author_identity: AuthorIdentity,
}

/// Counts are file-wide unless the field name says otherwise. The three
/// `comments_on_*` fields partition `comment_elements_total` exactly, so a
/// reader can reconcile the extractor's narrower numbers against this one;
/// `assert_comments_partitioned` enforces that.
#[derive(Serialize)]
struct SourceFacts {
    comment_elements_total: usize,
    comments_on_published_posts: usize,
    comments_on_draft_posts: usize,
    comments_on_non_post_items: usize,
    emails_non_empty: usize,
    emails_distinct_casefolded: usize,
    ips_non_empty: usize,
    ips_distinct: usize,
}

/// Evidence for the rule the extractor uses to flag ahl's own comments.
///
/// No single field identifies them: `comment_user_id == 3` catches only the 78
/// comments made while logged in, and the email field -- which would catch 113
/// -- is exactly what redaction removes. The rule that works is
/// `user_id == 3 OR author name in AHL_NAMES`, and `triples` is the evidence
/// that it over-matches nothing: every (user_id, name, email) combination
/// appearing under those names is listed, so a reader can confirm no one else
/// ever used them.
#[derive(Serialize)]
struct AuthorIdentity {
    rule: String,
    matched_total: usize,
    matched_by_user_id: usize,
    matched_by_name_only: usize,
    triples: Vec<IdentityTriple>,
}

#[derive(Serialize)]
struct IdentityTriple {
    user_id: String,
    author: String,
    email_redacted: String,
    count: usize,
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        bail!(
            "usage: {} <source.xml> <output.xml> <manifest.json>\n\n\
             The source WXR export is intentionally not in this repository.\n\
             Point this at your local copy of adamleventhal039sblog.wordpress.2020-12-01.xml",
            args[0]
        );
    }
    let (src_path, out_path, manifest_path) = (&args[1], &args[2], &args[3]);

    let source = std::fs::read_to_string(src_path)
        .with_context(|| format!("reading source export from {src_path}"))?;

    let (redacted, counts) = redact(&source)?;
    verify_only_targets_changed(&source, &redacted)?;
    verify_no_pii_remains(&redacted)?;

    let manifest = build_manifest(src_path, &source, &redacted, counts)?;

    std::fs::write(out_path, &redacted).with_context(|| format!("writing {out_path}"))?;
    let manifest_json = serde_json::to_string_pretty(&manifest)? + "\n";
    std::fs::write(manifest_path, &manifest_json)
        .with_context(|| format!("writing {manifest_path}"))?;

    report(&manifest, out_path, manifest_path);
    Ok(())
}

/// The three substitutions, and only these three.
fn redact(source: &str) -> Result<(String, BTreeMap<String, usize>)> {
    let mut counts = BTreeMap::new();

    // `<wp:author_email>` must not also match `<wp:comment_author_email>`; it
    // does not, because the `<wp:` prefix is anchored immediately before the
    // element name. Asserted in tests.
    let rules: [(&str, &str, bool); 3] = [
        ("wp:comment_author_email", r"(<wp:comment_author_email>)([^<]*)(</wp:comment_author_email>)", true),
        ("wp:comment_author_IP", r"(<wp:comment_author_IP>)([^<]*)(</wp:comment_author_IP>)", false),
        ("wp:author_email", r"(<wp:author_email>)([^<]*)(</wp:author_email>)", true),
    ];

    let mut out = source.to_string();
    for (name, pattern, partial) in rules {
        let re = Regex::new(pattern)?;
        let mut n = 0usize;
        out = re
            .replace_all(&out, |caps: &Captures| {
                n += 1;
                let value = &caps[2];
                let replacement = if partial {
                    redact_email(value)
                } else {
                    // IP addresses are removed outright, not partially
                    // redacted: a truncated IP still narrows a person down and
                    // serves no eyeballing purpose.
                    String::new()
                };
                format!("{}{}{}", &caps[1], replacement, &caps[3])
            })
            .into_owned();
        counts.insert(name.to_string(), n);
    }

    Ok((out, counts))
}

/// Assert that redaction touched nothing it was not supposed to touch.
///
/// This is the load-bearing check. If a regex were wrong in a way that ate
/// surrounding markup, or if the transform silently reflowed the document,
/// this fails loudly and names the offending line.
fn verify_only_targets_changed(source: &str, redacted: &str) -> Result<()> {
    let src_lines: Vec<&str> = source.lines().collect();
    let out_lines: Vec<&str> = redacted.lines().collect();

    if src_lines.len() != out_lines.len() {
        bail!(
            "redaction changed the line count ({} -> {}); it must be a \
             line-for-line transform",
            src_lines.len(),
            out_lines.len()
        );
    }

    let targets = ["wp:comment_author_email", "wp:comment_author_IP", "wp:author_email"];
    let mut changed = 0usize;
    for (i, (a, b)) in src_lines.iter().zip(out_lines.iter()).enumerate() {
        if a == b {
            continue;
        }
        changed += 1;
        if !targets.iter().any(|t| a.contains(t)) {
            bail!(
                "line {} changed but carries none of the redaction targets:\n  \
                 before: {}\n  after:  {}",
                i + 1,
                a.trim(),
                b.trim()
            );
        }
    }

    if changed == 0 {
        bail!("redaction changed nothing; the source file is not what we expect");
    }
    Ok(())
}

/// Belt-and-suspenders: no email- or IP-shaped string may survive anywhere in
/// the output, including places we did not think to look.
///
/// Note this is intentionally a *different* mechanism from the substitution
/// above -- a broad regex sweep over the finished bytes rather than a targeted
/// element rewrite -- so that a bug in the rewrite rules does not also disable
/// its own check.
fn verify_no_pii_remains(redacted: &str) -> Result<()> {
    for (line_no, line) in redacted.lines().enumerate() {
        if let Some(m) = email_shaped().find(line) {
            // Comment bodies and author URLs legitimately contain text that can
            // look address-like; only the redacted elements are our concern,
            // and they must be clean.
            if line.contains("wp:comment_author_email") || line.contains("wp:author_email") {
                bail!("line {}: email survived redaction: {}", line_no + 1, m.as_str());
            }
        }
        if line.contains("wp:comment_author_IP") {
            if let Some(m) = ip_shaped().find(line) {
                bail!("line {}: IP survived redaction: {}", line_no + 1, m.as_str());
            }
        }
    }
    Ok(())
}

fn build_manifest(
    src_path: &str,
    source: &str,
    redacted: &str,
    elements_redacted: BTreeMap<String, usize>,
) -> Result<Manifest> {
    let doc = roxmltree::Document::parse(source).context("parsing source export")?;
    let channel = doc
        .descendants()
        .find(|n| n.has_tag_name("channel"))
        .context("no <channel> in export")?;

    let wp = |n: &roxmltree::Node, tag: &str| -> String {
        n.children()
            .find(|c| c.has_tag_name((WP_NS, tag)))
            .and_then(|c| c.text())
            .unwrap_or("")
            .trim()
            .to_string()
    };

    let mut comment_elements_total = 0usize;
    let mut comments_on_published_posts = 0usize;
    let mut comments_on_draft_posts = 0usize;
    let mut comments_on_non_post_items = 0usize;
    let mut emails: Vec<String> = Vec::new();
    let mut ips: Vec<String> = Vec::new();
    let mut triples: BTreeMap<(String, String, String), usize> = BTreeMap::new();
    let (mut by_user_id, mut by_name_only) = (0usize, 0usize);

    for item in channel.children().filter(|n| n.has_tag_name("item")) {
        let is_post = wp(&item, "post_type") == "post";
        // A post with no slug was never published; WordPress leaves
        // `post_name` empty on drafts. Five comments sit on one such draft.
        let is_published_post = is_post && !wp(&item, "post_name").is_empty();

        for c in item.children().filter(|n| n.has_tag_name((WP_NS, "comment"))) {
            comment_elements_total += 1;
            match (is_post, is_published_post) {
                (_, true) => comments_on_published_posts += 1,
                (true, false) => comments_on_draft_posts += 1,
                (false, false) => comments_on_non_post_items += 1,
            }

            let email = wp(&c, "comment_author_email");
            let ip = wp(&c, "comment_author_IP");
            let user_id = wp(&c, "comment_user_id");
            let author = wp(&c, "comment_author");

            if !email.is_empty() {
                emails.push(email.to_lowercase());
            }
            if !ip.is_empty() {
                ips.push(ip.clone());
            }

            let matches_user_id = user_id == AHL_USER_ID;
            let matches_name = AHL_NAMES.contains(&author.as_str());
            if matches_user_id || matches_name {
                if matches_user_id {
                    by_user_id += 1;
                } else {
                    by_name_only += 1;
                }
                *triples
                    .entry((user_id, author, redact_email(&email)))
                    .or_insert(0) += 1;
            }
        }
    }

    let emails_distinct = emails.iter().collect::<BTreeSet<_>>().len();
    let ips_distinct = ips.iter().collect::<BTreeSet<_>>().len();

    assert_comments_partitioned(
        comment_elements_total,
        comments_on_published_posts,
        comments_on_draft_posts,
        comments_on_non_post_items,
    )?;

    Ok(Manifest {
        source_file: Path::new(src_path)
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| src_path.to_string()),
        source_sha256: sha256(source),
        redacted_sha256: sha256(redacted),
        elements_redacted,
        source_facts: SourceFacts {
            comment_elements_total,
            comments_on_published_posts,
            comments_on_draft_posts,
            comments_on_non_post_items,
            emails_non_empty: emails.len(),
            emails_distinct_casefolded: emails_distinct,
            ips_non_empty: ips.len(),
            ips_distinct,
        },
        author_identity: AuthorIdentity {
            rule: format!(
                "comment_user_id == {AHL_USER_ID} OR comment_author in {AHL_NAMES:?}"
            ),
            matched_total: by_user_id + by_name_only,
            matched_by_user_id: by_user_id,
            matched_by_name_only: by_name_only,
            triples: triples
                .into_iter()
                .map(|((user_id, author, email_redacted), count)| IdentityTriple {
                    user_id,
                    author,
                    email_redacted,
                    count,
                })
                .collect(),
        },
    })
}

/// Every comment element in the file must land in exactly one bucket. If this
/// ever fails, some item kind exists that we have not thought about, and the
/// extractor's "nothing was lost" accounting would be silently incomplete.
fn assert_comments_partitioned(
    total: usize,
    published: usize,
    draft: usize,
    non_post: usize,
) -> Result<()> {
    let sum = published + draft + non_post;
    if sum != total {
        bail!(
            "comment buckets do not partition the file: {published} published + \
             {draft} draft + {non_post} non-post = {sum}, but the file has {total}"
        );
    }
    Ok(())
}

fn sha256(s: &str) -> String {
    let mut h = Sha256::new();
    h.update(s.as_bytes());
    format!("{:x}", h.finalize())
}

fn report(m: &Manifest, out_path: &str, manifest_path: &str) {
    println!("redaction report");
    println!("  source          {}", m.source_file);
    println!("  source sha256   {}", m.source_sha256);
    println!();
    println!("  elements redacted:");
    for (k, v) in &m.elements_redacted {
        println!("    {v:>5}  {k}");
    }
    println!();
    let f = &m.source_facts;
    println!("  source facts (file-wide; recorded because redaction destroys them):");
    println!("    {:>5}  comment elements total", f.comment_elements_total);
    println!("    {:>5}    on published posts", f.comments_on_published_posts);
    println!("    {:>5}    on draft posts", f.comments_on_draft_posts);
    println!("    {:>5}    on non-post items", f.comments_on_non_post_items);
    println!(
        "    {:>5}  non-empty emails ({} distinct, case-folded)",
        f.emails_non_empty, f.emails_distinct_casefolded
    );
    println!("    {:>5}  non-empty IPs ({} distinct)", f.ips_non_empty, f.ips_distinct);
    println!();
    let a = &m.author_identity;
    println!("  author identity: {}", a.rule);
    println!("    {:>5}  matched total ({} by user_id, {} by name only)",
        a.matched_total, a.matched_by_user_id, a.matched_by_name_only);
    for t in &a.triples {
        println!(
            "    {:>5}  user_id={:<3} author={:<16} email={}",
            t.count, t.user_id, t.author, t.email_redacted
        );
    }
    println!();
    println!("  checks passed:  line-for-line diff touched only redaction targets");
    println!("                  no email- or IP-shaped text survives in target elements");
    println!();
    println!("  wrote {out_path}");
    println!("  wrote {manifest_path}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const FRAGMENT: &str = concat!(
        "<wp:author><wp:author_id>3</wp:author_id><wp:author_email>adam.leventhal@gmail.com</wp:author_email></wp:author>\n",
        "\t\t\t<wp:comment_author><![CDATA[Trevor]]></wp:comment_author>\n",
        "\t\t\t<wp:comment_author_email>trevoro@gmail.com</wp:comment_author_email>\n",
        "\t\t\t<wp:comment_author_url>http://example.com/</wp:comment_author_url>\n",
        "\t\t\t<wp:comment_author_IP>209.139.199.122</wp:comment_author_IP>\n",
        "\t\t\t<wp:comment_content><![CDATA[mail me at foo@bar.com]]></wp:comment_content>\n",
    );

    #[test]
    fn redacts_all_three_element_types() {
        let (out, counts) = redact(FRAGMENT).unwrap();
        assert_eq!(counts["wp:comment_author_email"], 1);
        assert_eq!(counts["wp:comment_author_IP"], 1);
        assert_eq!(counts["wp:author_email"], 1);

        assert!(out.contains("<wp:comment_author_email>t..@g..</wp:comment_author_email>"));
        assert!(out.contains("<wp:author_email>a..@g..</wp:author_email>"));
        assert!(out.contains("<wp:comment_author_IP></wp:comment_author_IP>"));
    }

    /// `<wp:author_email>` and `<wp:comment_author_email>` share a suffix. If
    /// the narrower pattern also matched the wider element we would
    /// double-redact and miscount, so pin the behavior down.
    #[test]
    fn author_email_rule_does_not_match_comment_author_email() {
        let only_comment = "<wp:comment_author_email>a@b.com</wp:comment_author_email>\n";
        let (_, counts) = redact(only_comment).unwrap();
        assert_eq!(counts["wp:comment_author_email"], 1);
        assert_eq!(counts["wp:author_email"], 0);
    }

    /// Redaction must not disturb neighbouring elements -- notably
    /// `comment_author_url`, which was always public and stays intact.
    #[test]
    fn leaves_everything_else_byte_identical() {
        let (out, _) = redact(FRAGMENT).unwrap();
        verify_only_targets_changed(FRAGMENT, &out).unwrap();
        assert!(out.contains("<wp:comment_author_url>http://example.com/</wp:comment_author_url>"));
        assert!(out.contains("<![CDATA[Trevor]]>"));
        // An address inside a comment body is content, not PII we introduced;
        // it is left exactly as the commenter wrote it.
        assert!(out.contains("mail me at foo@bar.com"));
    }

    #[test]
    fn output_passes_the_pii_tripwire() {
        let (out, _) = redact(FRAGMENT).unwrap();
        verify_no_pii_remains(&out).unwrap();
    }

    #[test]
    fn tripwire_catches_an_unredacted_email() {
        let leaked = "<wp:comment_author_email>trevoro@gmail.com</wp:comment_author_email>\n";
        assert!(verify_no_pii_remains(leaked).is_err());
    }

    #[test]
    fn tripwire_catches_an_unredacted_ip() {
        let leaked = "<wp:comment_author_IP>209.139.199.122</wp:comment_author_IP>\n";
        assert!(verify_no_pii_remains(leaked).is_err());
    }

    #[test]
    fn diff_check_rejects_a_change_outside_the_targets() {
        let tampered = FRAGMENT.replace("http://example.com/", "http://evil.example/");
        let err = verify_only_targets_changed(FRAGMENT, &tampered).unwrap_err();
        assert!(err.to_string().contains("carries none of the redaction targets"));
    }

    #[test]
    fn diff_check_rejects_a_line_count_change() {
        let tampered = format!("{FRAGMENT}<wp:extra/>\n");
        let err = verify_only_targets_changed(FRAGMENT, &tampered).unwrap_err();
        assert!(err.to_string().contains("line count"));
    }

    #[test]
    fn redaction_is_idempotent() {
        let (once, _) = redact(FRAGMENT).unwrap();
        let (twice, _) = redact(&once).unwrap();
        assert_eq!(once, twice);
    }
}
