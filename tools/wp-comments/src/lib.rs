//! Shared helpers for the WordPress export migration tools.
//!
//! The source of truth is a WXR (WordPress eXtended RSS) file exported from
//! dtrace.org/blogs/ahl on 2020-12-01. That file contains personal data --
//! commenter email addresses and IP addresses -- and is deliberately NOT
//! committed to this repository. See `bin/redact.rs`.

use regex::Regex;
use std::sync::OnceLock;

/// Partially redact an email address: `trevoro@gmail.com` -> `t..@g..`
///
/// This is deliberately *not* reversible and deliberately *not* linkable: many
/// distinct addresses collapse onto the same output, so the redacted file
/// cannot be used to tell whether two comments came from the same person. The
/// redacted value exists only so a human reading the WXR can eyeball it; no
/// tool in this crate emits an email address, redacted or otherwise, into the
/// generated site data.
pub fn redact_email(email: &str) -> String {
    let email = email.trim();
    if email.is_empty() {
        return String::new();
    }
    let first_char = |s: &str| s.chars().next().map(|c| c.to_lowercase().to_string());

    match email.split_once('@') {
        Some((local, domain)) => {
            let l = first_char(local).unwrap_or_default();
            let d = first_char(domain).unwrap_or_default();
            format!("{l}..@{d}..")
        }
        // Malformed address with no `@`. Redact it to an opaque marker rather
        // than passing it through: we cannot reason about what it contains.
        None => "..".to_string(),
    }
}

/// Matches anything email-shaped. Used as a tripwire against redacted output,
/// so it errs toward over-matching.
pub fn email_shaped() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}").unwrap())
}

/// Matches a dotted-quad IPv4 address or anything colon-separated enough to be
/// an IPv6 address. Also a tripwire, also errs toward over-matching.
pub fn ip_shaped() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"\b(?:\d{1,3}\.){3}\d{1,3}\b|\b(?:[0-9a-fA-F]{1,4}:){2,7}[0-9a-fA-F]{1,4}\b")
            .unwrap()
    })
}

/// Tags permitted in a rendered comment body.
///
/// Derived from the corpus, not from a generic default: across all 1073
/// comments the only tags ever used are `a` (329), `i` (46), `b` (22),
/// `blockquote` (16), `strong` (2) and one stray `<poolname>` that is plainly
/// someone typing a ZFS example rather than markup. `p`, `br`, `code`, `pre`,
/// `ul`, `ol` and `li` are here because `wpautop` introduces the first two and
/// the rest are cheap to allow.
pub const ALLOWED_TAGS: &[&str] = &[
    "a", "b", "blockquote", "br", "code", "em", "i", "li", "ol", "p", "pre", "strong", "ul",
];

/// Block-level tags that `wpautop` must not wrap in a paragraph.
const BLOCK_TAGS: &[&str] = &["blockquote", "pre", "ul", "ol", "p"];

/// Reimplementation of WordPress's `wpautop`: blank lines become paragraph
/// breaks, remaining single newlines become `<br>`.
///
/// Comment bodies in the export are stored exactly as typed, with significant
/// newlines and no paragraph markup, because WordPress applied this at render
/// time. Without it every comment collapses into one run-on block.
pub fn wpautop(input: &str) -> String {
    let normalized = input.replace("\r\n", "\n").replace('\r', "\n");
    let mut out = String::new();

    for chunk in normalized.split("\n\n") {
        let chunk = chunk.trim();
        if chunk.is_empty() {
            continue;
        }
        let starts_with_block = BLOCK_TAGS.iter().any(|t| {
            let open = format!("<{t}");
            chunk.to_ascii_lowercase().starts_with(&open)
        });
        let with_breaks = chunk.replace('\n', "<br>\n");
        if starts_with_block {
            out.push_str(&with_breaks);
        } else {
            out.push_str("<p>");
            out.push_str(&with_breaks);
            out.push_str("</p>");
        }
        out.push('\n');
    }
    out
}

/// Maps a bare WordPress slug to the permalink the post lives at today.
pub type SlugIndex = std::collections::BTreeMap<String, String>;

/// What to do with a URL found in a comment body.
#[derive(Debug, PartialEq, Eq)]
pub enum UrlAction {
    /// Leave untouched — external, and not ours to fix.
    Keep,
    /// Rewrite to this value.
    Rewrite(String),
    /// Points at this blog but cannot be mechanically resolved. Left
    /// untouched and reported, never guessed at.
    UnresolvedInternal,
}

/// Rewrite links that point at content we still control.
///
/// Internal links become root-relative (no hostname), and Bryan's old
/// `dtrace.org/blogs/bmc` posts now live at `bcantrill.dtrace.org`. Everything
/// else is left exactly as the commenter typed it: most of these hosts died a
/// decade ago and inventing replacements would be fabrication.
pub fn rewrite_url(url: &str, index: &SlugIndex) -> UrlAction {
    let rest = match url
        .strip_prefix("http://")
        .or_else(|| url.strip_prefix("https://"))
    {
        Some(r) => r,
        None => return UrlAction::Keep,
    };
    let (host, path) = match rest.split_once('/') {
        Some((h, p)) => (h.to_ascii_lowercase(), format!("/{p}")),
        None => (rest.to_ascii_lowercase(), "/".to_string()),
    };

    // The blog's current home: strip the hostname, keep the path.
    if host == "ahl.dtrace.org" {
        return UrlAction::Rewrite(path);
    }
    if host != "dtrace.org" {
        return UrlAction::Keep;
    }
    // A URL someone pasted already ellipsized. Unrecoverable wherever it
    // points, so this must be checked before the per-blog branches below.
    if path.contains("...") {
        return UrlAction::UnresolvedInternal;
    }
    if let Some(p) = path.strip_prefix("/blogs/bmc/") {
        return UrlAction::Rewrite(format!("https://bcantrill.dtrace.org/{p}"));
    }
    let Some(p) = path.strip_prefix("/blogs/ahl/") else {
        return UrlAction::Keep;
    };

    // Uploaded files from the old install; nothing serves them now.
    if p.starts_with("files/") {
        return UrlAction::UnresolvedInternal;
    }
    // Modern dated permalink: /YYYY/MM/DD/slug/ — already the right shape.
    let segments: Vec<&str> = p.trim_end_matches('/').split('/').collect();
    let dated = segments.len() >= 4
        && segments[0].len() == 4
        && segments[0].chars().all(|c| c.is_ascii_digit())
        && segments[1].len() == 2
        && segments[2].len() == 2;
    if dated {
        return UrlAction::Rewrite(format!("/{}", p.trim_start_matches('/')));
    }
    // Old Roller-era flat slug with no date. Resolvable only if the slug still
    // exists; these mostly changed during the migration, so usually it does not.
    let slug = segments.first().copied().unwrap_or("");
    match index.get(slug) {
        Some(permalink) => UrlAction::Rewrite(permalink.clone()),
        None => UrlAction::UnresolvedInternal,
    }
}

/// Extract visible text from HTML by brute force, for verification only.
///
/// This is deliberately a *different* implementation from the `html5ever`
/// parser that `ammonia` uses to sanitize. If the fidelity check re-parsed the
/// output with the same machinery that produced it, a shared bug would cancel
/// itself out and the check would pass on mangled data. A crude tag-stripper
/// is wrong in different ways, which is the whole point.
pub fn visible_text(html: &str) -> String {
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| Regex::new(r"<[^>]*>").unwrap());
    let without_tags = re.replace_all(html, " ");
    let decoded = html_escape::decode_html_entities(&without_tags);
    decoded.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// `visible_text` with all whitespace removed, for equality comparison.
///
/// Substituting a space for each tag means a tag present on one side but not
/// the other shifts the spacing, even though no words changed. That happens
/// legitimately: one 2005 comment contains a stray unmatched `</a>`, which
/// html5ever discards during sanitization, and the crude stripper used here
/// turns into a space. Whitespace is not content, so the comparison ignores it
/// -- while a sanitizer that actually ate a word still fails, because the
/// characters would be missing.
pub fn comparable_text(html: &str) -> String {
    visible_text(html).chars().filter(|c| !c.is_whitespace()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redacts_ordinary_addresses() {
        assert_eq!(redact_email("trevoro@gmail.com"), "t..@g..");
        assert_eq!(redact_email("davidtehhw@gmail.com"), "d..@g..");
    }

    #[test]
    fn lowercases_so_case_does_not_leak() {
        assert_eq!(redact_email("Adam.Leventhal@Sun.COM"), "a..@s..");
    }

    #[test]
    fn empty_stays_empty() {
        assert_eq!(redact_email(""), "");
        assert_eq!(redact_email("   "), "");
    }

    #[test]
    fn malformed_becomes_opaque() {
        assert_eq!(redact_email("not-an-address"), "..");
    }

    /// The whole point of the scheme: distinct people collapse together.
    #[test]
    fn is_not_linkable() {
        assert_eq!(redact_email("alice@gmail.com"), redact_email("adam@gmail.com"));
    }

    /// Redacted output must never trip the tripwires we check the file with.
    #[test]
    fn output_is_not_email_shaped() {
        for input in ["trevoro@gmail.com", "Adam.Leventhal@Sun.COM", "x@y.co.uk"] {
            let out = redact_email(input);
            assert!(!email_shaped().is_match(&out), "{out} still looks like an email");
        }
    }

    #[test]
    fn ip_tripwire_matches_real_ips() {
        assert!(ip_shaped().is_match("209.139.199.122"));
        assert!(ip_shaped().is_match("220.255.1.80"));
        assert!(!ip_shaped().is_match(""));
    }

    #[test]
    fn wpautop_makes_paragraphs_and_breaks() {
        let out = wpautop("first para\nsame para\n\nsecond para");
        assert!(out.contains("<p>first para<br>\nsame para</p>"));
        assert!(out.contains("<p>second para</p>"));
    }

    #[test]
    fn wpautop_does_not_wrap_block_elements() {
        let out = wpautop("<blockquote>quoted</blockquote>\n\nafter");
        assert!(out.starts_with("<blockquote>quoted</blockquote>"));
        assert!(!out.contains("<p><blockquote>"));
    }

    #[test]
    fn wpautop_drops_nothing() {
        let input = "alpha\nbravo\n\ncharlie";
        assert_eq!(visible_text(&wpautop(input)), "alpha bravo charlie");
    }

    fn index() -> SlugIndex {
        let mut m = SlugIndex::new();
        m.insert("leaving_oracle".into(), "/2010/08/18/leaving_oracle/".into());
        m
    }

    #[test]
    fn dated_internal_links_become_root_relative() {
        assert_eq!(
            rewrite_url("http://dtrace.org/blogs/ahl/2016/06/19/apfs-part1/", &index()),
            UrlAction::Rewrite("/2016/06/19/apfs-part1/".into())
        );
        assert_eq!(
            rewrite_url("https://ahl.dtrace.org/2016/06/19/apfs-part1/", &index()),
            UrlAction::Rewrite("/2016/06/19/apfs-part1/".into())
        );
    }

    #[test]
    fn bmc_links_move_to_his_current_home() {
        assert_eq!(
            rewrite_url("http://dtrace.org/blogs/bmc/2010/08/bye-bye/", &index()),
            UrlAction::Rewrite("https://bcantrill.dtrace.org/2010/08/bye-bye/".into())
        );
    }

    #[test]
    fn known_flat_slug_resolves() {
        assert_eq!(
            rewrite_url("http://dtrace.org/blogs/ahl/leaving_oracle", &index()),
            UrlAction::Rewrite("/2010/08/18/leaving_oracle/".into())
        );
    }

    /// The migration changed most Roller-era slugs. We report these rather
    /// than guessing which post the commenter meant.
    #[test]
    fn unknown_flat_slug_is_reported_not_guessed() {
        assert_eq!(
            rewrite_url("http://dtrace.org/blogs/ahl/what_if_machine_dtrace_port", &index()),
            UrlAction::UnresolvedInternal
        );
    }

    #[test]
    fn old_uploads_and_truncated_urls_are_reported() {
        assert_eq!(
            rewrite_url("http://dtrace.org/blogs/ahl/files/2012/02/riverwalk_jr.jpg", &index()),
            UrlAction::UnresolvedInternal
        );
        assert_eq!(
            rewrite_url("http://dtrace.org/blogs/ahl/2010/08/...re-of-solaris/", &index()),
            UrlAction::UnresolvedInternal
        );
        // Truncation makes a URL unrecoverable regardless of which blog it
        // points at -- including Bryan's, which would otherwise be rewritten.
        assert_eq!(
            rewrite_url("http://dtrace.org/blogs/bmc/2010/08/...f-opensolaris/", &index()),
            UrlAction::UnresolvedInternal
        );
    }

    #[test]
    fn external_links_are_left_alone() {
        for u in [
            "http://blogs.sun.com/roller/page/bmc?entry=demo_ing_dtrace",
            "https://en.wikipedia.org/wiki/DTrace",
            "mailto:someone@example.com",
        ] {
            assert_eq!(rewrite_url(u, &index()), UrlAction::Keep, "{u}");
        }
    }

    #[test]
    fn visible_text_decodes_entities() {
        assert_eq!(visible_text("<p>a &amp; b</p>"), "a & b");
        // From a real (spam) comment: U+4EBA U+6C14. Numeric entities are the
        // only way non-Latin text survives in this export, so decoding them
        // correctly is what keeps the fidelity check honest.
        assert_eq!(visible_text("&#20154;&#27668;"), "人气");
    }
}
