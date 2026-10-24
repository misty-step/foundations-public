# Cited anchors

Guidance across the fleet cites Foundations by article and section: "Foundations article 12", `https://foundations.mistystep.io/#12`, `https://foundations.mistystep.io/way-of-working#prove`. A reader following such a link lands on that article or section.

## Sub-features

- `anchor-article`: every numbered Constitution article *n* has `id="n"` on its own item, so `/#n` opens that article.
- `anchor-section`: every bold-led Way of working section and every subheading has a slug id, such as `/way-of-working#milei-check` or `#ci-deploy-credentials`.
- `anchor-scroll`: opening the anchor in a browser scrolls its target into view on a phone.
- `anchor-citation`: every Foundations fragment link, and every "Foundations … article N", in the guidance an agent reads points at an anchor that exists.

## How to get to it (user POV)

- Follow a cited link such as `https://foundations.mistystep.io/#12` from `~/.claude/AGENTS.md`, a skill, or a repository's docs.
- Type `/#<n>` after the site address to jump to article *n*.

## Driving it with verify-foundations

Preconditions: `$B doctor` passes.

- **Every anchor exists.** Run `$B anchors --out "$OUT"`. One `anchors` line per article and section derived from the served source, each `ok: true`.
- **Citations resolve.** The same run prints a `citation` line per cited link, with `~/path:line`. Scan more guidance by passing paths: `$B anchors --out "$OUT" ~/development/misty-step/overmind ~/development/misty-step/harness`.
- **Lands in view.** `$B browse --out "$OUT"` prints one `anchor` line per id on both pages, at 390px, with the target's `top` inside the viewport. Open `screens/anchor-constitution-12.png` and `screens/anchor-way-of-working-prove.png`: article 12 and the **Prove.** section are on screen.

## Gotchas

- Anchors are rendered by `src/main.rs` from document structure. `/#n` sits on the item that starts with article *n*'s title; a second top-level numbered list in the Constitution that would reuse an article number fails the build rather than steal `/#n`. A list written directly above article 1 with only a blank line between merges into the article list in Markdown; the build cannot tell, but the publication test and `anchors` both fail because `/#1` no longer starts with article 1's title. Numbered lists in the Way of working are ordinary lists without ids. Renumbering articles moves `/#n`. Renaming a bold lead changes its slug, and every citation of the old slug breaks: run `anchors` over the fleet's guidance after such an edit.
- Older docs cite retired sections (`#own-the-work`, `#merge-fast-release-verified-candidates`, `#audit-checklist`). Those sections no longer exist, so the right fix is in the citing repository, not a new anchor here.
- A failing `citation` in someone else's repository is a bug for its owner: report it with the `~/path:line` from the check.
- The scan matches absolute Foundations links, with or without `https://`, in files or directories. Relative links are not scanned. Only the two HTML pages have anchors: `/index` and `/index.html` count as `/` (they redirect and keep the fragment), and a fragment on `/index.md` or any other path fails.
