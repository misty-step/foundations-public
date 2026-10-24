# Read the documents

A reader opens the Constitution at `/` or the Way of working at `/way-of-working` without credentials or JavaScript, reads the complete document on a phone or desktop in their system's light or dark scheme, and moves between the two documents.

## Sub-features

- `read-page`: each page returns 200 with title `Foundations — Misty Step` and its `h1` (`Constitution`, `Way of working`).
- `read-fit`: no horizontal overflow at 390px or 1440px.
- `read-scheme`: the page follows the system light or dark preference.
- `read-nav`: the `Documents` navigation links both pages; clicking them moves between them.
- `read-footer`: the footer's `Markdown` link opens the page's source.

## How to get to it (user POV)

- Open `https://foundations.mistystep.io/` or `/way-of-working` in a browser.
- Follow `Constitution` or `Way of working` in the `Documents` navigation at the top of either page.
- Follow `Markdown` in the footer.

## Driving it with verify-foundations

Preconditions: `$B doctor` passes against the target.

- **Read each page.** Run `$B browse --out "$OUT"`. Eight `read` lines, one per page × `phone|desktop` × `light|dark`, show `ok: true`, status 200, the expected `h1`, `nav_links: true` and `scroll_width <= client_width`.
- **Read changed policy.** For prose changes under [US-001](../../../../SPEC.md#us-001-read), read each changed article or section in the served page and follow its links. Confirm the intended rule is present on the candidate and absent or wrong on the base revision. A page title and matching source bytes alone do not prove the wording.
- **Scheme.** The same run prints four `color-scheme` lines with the light and dark page backgrounds (set in `src/style.html`); they must differ.
- **Navigate.** `navigate` lines show `Constitution -> Way of working` and back by clicking, and the footer `Markdown` link returning `# Constitution` as `text/markdown`.
- **Proof.** Open `screens/constitution-phone-light.png`, `way-of-working-phone-dark.png` and both desktop shots. Text is readable, nothing is clipped, and dark really is dark.

## Gotchas

- JavaScript is disabled in the browser context on purpose (SPEC: readable without executing code). Playwright's `evaluate` still works, so the measurements are real.
- `#12` screenshots sit at the page bottom: the last article cannot scroll to the top edge, which is still in view.
- The Content-Security-Policy forbids scripts; a page that needs one is a regression, and `routes` also asserts no `<script`.
