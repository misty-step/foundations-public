# Machine-readable sources

An agent fetches `/index.md` or `/way-of-working.md` and receives exactly the editable source bytes of the deployed revision, so it can quote and diff the rules without scraping HTML.

## Sub-features

- `source-bytes`: each Markdown route equals `git show <served revision>:<file>` byte for byte.
- `source-type`: served as `text/markdown; charset=utf-8`.
- `source-links`: each HTML page links its own source (`/index.md` or `/way-of-working.md`).

## How to get to it (user POV)

- `curl https://foundations.mistystep.io/index.md` or `/way-of-working.md`.
- The footer `Markdown` link on either page.

## Driving it with verify-foundations

Preconditions: `$B doctor` reports a served revision; `git` can fetch it from `origin`.

- **Fetch and compare.** Run `$B routes --out "$OUT"`. The `/index.md` and `/way-of-working.md` lines show status 200, `text/markdown; charset=utf-8`, and `ok: true` only when the bytes equal the source at the served revision (named in `detail`).
- **Linked from the page.** The `/` and `/way-of-working` lines in the same run assert the footer's `href="/<doc>.md">Markdown</a>`.

## Gotchas

- Compare with the served revision, not your checkout: during a Process-lane content change, `HEAD` and live legitimately differ.
- A fresh clone may lack the served commit; the helper fetches it from `origin`, which needs read access to the private repository.
