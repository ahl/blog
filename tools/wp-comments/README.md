# wp-comments

One-shot migration tooling that pulled the comments off the old WordPress blog
(`dtrace.org/blogs/ahl`, 2004–2020) into `content/blog/<post>/comments.json`.

This is **not** part of the Eleventy build. The generated JSON is committed; the
tools are kept for provenance, so the extraction can be audited or redone.

## Why the export isn't in this repo

The WordPress export (`adamleventhal039sblog.wordpress.2020-12-01.xml`) contains
**750 commenter email addresses** (450 distinct) and **476 IP addresses** (323
distinct), belonging to real people who typed them into a comment form with no
expectation they'd ever be published. This repository is public and a commit is
effectively irreversible, so the export lives outside it.

`redact` produces a PII-free copy suitable for sharing. That copy is also
untracked by default — see `.gitignore` — but can be committed if the
reproducibility is worth more than the 560KB.

## Running it

Both tools take explicit paths; nothing is hardcoded.

```sh
# 1. Strip PII from the raw export (needs your local copy)
cargo run --bin redact -- \
    /path/to/adamleventhal039sblog.wordpress.2020-12-01.xml \
    wxr/export.redacted.xml \
    wxr/export.manifest.json

# 2. Regenerate the per-post comment files
cargo run --bin extract -- \
    wxr/export.redacted.xml \
    ../../content/blog \
    wxr/extract.report.json

cargo test
```

Both are deterministic: same input, byte-identical output. Re-running should
leave `git status` clean.

## How correctness is established

The failure mode here is silent — a sanitizer that quietly eats a `<blockquote>`
produces a subtly wrong archive nobody notices for years. So both tools assert
invariants on every run and exit non-zero if any fails:

- **Redaction is minimal and provable.** Exactly three element types change, via
  a line-oriented transform rather than parse-and-reserialize. A line-for-line
  diff must differ *only* on lines carrying a redaction target, with the line
  count unchanged.
- **Conservation.** Every one of the 1077 comment elements is either emitted or
  attributed to a named skip reason, and the buckets must sum exactly.
- **Body fidelity.** For every comment, the visible text before and after
  rendering must match. The checker uses a deliberately *different*
  implementation from the `html5ever` parser that does the sanitizing, so a
  shared bug can't cancel itself out.
- **Tag accounting.** The set of tags dropped corpus-wide must be exactly
  `{poolname}` — one stray tag someone typed in a ZFS example. Anything else
  means the allowlist is wrong.
- **Link conservation.** No comment may emit fewer links than it went in with.
- **Thread integrity.** Every surviving `parent` resolves to an emitted comment.
- **PII tripwire.** No email- or IP-shaped text may survive in a redacted field.

`wxr/export.manifest.json` records the facts redaction destroys — source
checksum, PII counts, and the evidence for the rule that identifies ahl's own
comments — so those claims stay auditable after the evidence is gone.

## Validation against the original site

Invariants prove the transform is self-consistent; they can't prove the WXR was
read correctly in the first place. For that, two posts were diffed against the
Wayback Machine's copy of the original rendered pages:

| post | comment ids | bodies |
| --- | --- | --- |
| `mac-os-x-and-the-missing-probes` | 77/77 | 77/77 |
| `apfs-part5` | 41/41 | 41/41 |

Bodies match once WordPress's *render-time* transforms are accounted for — none
of which are stored in the export: `wptexturize` (straight quotes to curly,
`--` to em dash), emoticons replaced by images, and `make_clickable` rewriting
bare URLs.

## Decisions worth knowing

- **Emails and IPs are never emitted**, in any form. Comments carry only the
  `author` name and the `url` the commenter supplied — the two fields the
  original form presented as public.
- **ahl's own comments** are identified by `comment_user_id == 3 OR author in
  {"ahl", "Adam Leventhal"}`, which matches all 225 with no false positives, and
  are normalized to `ahl`. No single field identifies them: the user id catches
  only 78, and email — which would catch 113 — is redacted.
- **Three spam comments** (ids 1748, 1751, 1753 — one payload from "hotindex",
  posted within 40 seconds in 2004) are dropped via a list in `extract.rs`, so
  the decision is reviewable rather than hand-edited out of the output.
- **Pingbacks are extracted but not rendered.** They're in the JSON with
  `kind: "pingback"`; `_data/comments.js` filters them out. To start showing
  them, change that filter — no regeneration needed.
- **Unresolvable links are left exactly as written**, never guessed at. Nine
  `dtrace.org` links use Roller-era slugs that changed during the migration;
  nine more internal URLs appear as bare text rather than anchors, and
  rewriting those would alter what the comment says. Both sets are listed in
  `wxr/extract.report.json`.
