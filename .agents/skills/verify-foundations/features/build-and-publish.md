# Build and publish

A maintainer changes `index.md` or `way-of-working.md` (or the renderer), the checks prove the built artifact in local workerd, a merge to `master` deploys those same bytes to Cloudflare, and the deployed site reads back identical to the verified artifact.

## Sub-features

- `build-check`: `npm run check` covers fmt, Clippy, the Rust/pulldown-cmark build, and the publication tests including local workerd.
- `serve-local`: the built `dist/` runs in local workerd for a full candidate journey.
- `deploy`: the master `Publication` workflow deploys the verified artifact without rebuilding.
- `readback`: the live site matches a fresh build of the served revision, byte for byte.

## How to get to it (user POV)

- Maintainer: open a PR; the `Publication` workflow's `verify` job runs; merging to `master` runs `deploy`.
- Reader: the next page load shows the new revision in the footer.

## Driving it with verify-foundations

Preconditions: on a Linux build machine, the README's pinned Node (via `mise`) and Rust.

- **Checks.** Run `$B build --out "$OUT"`. `ok: true` with the `test result` lines; `build.log` keeps the full output.
- **Candidate journey.** Run `$B serve --out "$OUT"`, then `$B all --base "$(cat "$OUT/serve.url")" --out "$OUT"`, then `$B stop --out "$OUT"`. Every line is `ok: true`; this is "artifact verified", not "live".
- **Deploy.** After merge, `gh run list -R misty-step/foundations --workflow Publication --branch master --limit 1` shows the run; `$B doctor` reports `live is master` once it deploys.
- **Readback.** Run `$B readback --out "$OUT"`, or `$B all` against live, which includes it. `readback.log` ends with `Canonical reference documents and revision links match the verified artifact; removed routes return 404.`

## Gotchas

- `readback` builds in a throwaway worktree with its own `target/`. Sharing this checkout's `target/` lets cargo reuse a binary built from other sources; if `dist/` ever looks stale, run `cargo clean -p foundations`.
- `serve` and the Rust tests both start wrangler/workerd. `stop` kills only the process group `serve` recorded. Other lanes' wrangler processes on the same machine are not yours.
- Rollback is the README's `wrangler rollback` through the authorized CI path, not this helper.
