---
name: verify-foundations
description: Verify the Foundations publication (https://foundations.mistystep.io) as its readers, mostly agents, use it. Reads both documents in Chromium at phone and desktop width in light and dark, checks exact Markdown sources, that every cited article and section anchor (`/#12`, `/way-of-working#prove`) resolves, revision/history links through the private-repo access gate, native 404 recovery, and the deploy readback. Use before calling a Foundations change verified, after a deploy, or when a cited Foundations link looks broken.
---

# Verify Foundations

Foundations is a static, public, two-document site: the Constitution at `/` and the Way of working at `/way-of-working`, plus their exact Markdown at `/index.md` and `/way-of-working.md`. Its readers are mostly agents that fetch a page or a cited anchor. It has no sign-in. The only access gate is GitHub: each footer's revision and History links point into the private `misty-step/foundations` repository.

One helper drives everything from a shell. Every check prints one JSON line (`check`, `target`, `ok`, details) to stdout and appends it to `OUT/checks.jsonl`. It exits 1 if any check failed. If the helper itself aborts, it prints a failed `helper` line naming the command and line, then the summary.

```sh
B=.agents/skills/verify-foundations/bin/verify-foundations   # also .claude/skills/verify-foundations/...
OUT=~/.cache/tmp/verify-foundations/run-$(date -u +%Y%m%dT%H%M%SZ)   # one evidence dir per run
$B <command> [--base URL] [--out DIR] [PATH...]
```

`--base` defaults to `https://foundations.mistystep.io`; `--out` defaults to `~/.cache/tmp/verify-foundations/<UTC stamp>/`. Read [features/README.md](features/README.md) for what each command proves.

## Prerequisites

Requires `bash`, `curl`, `git`, `jq`, `gh`, `uv`, `cargo`, `python3`, `setsid` and `pgrep` (Rust is pinned by `rust-toolchain.toml`). `build` and `serve` also need the README's pinned Node 22.23.2. They run it through `mise` when the default `node` differs. `browse` runs Playwright 1.64.0 through `uv` and installs Chromium on first use. `doctor` (live or local), `access`, and reading the served revision's source need native `gh` and git logins that can read the private `misty-step/foundations`. No other credentials are needed, and the helper never deploys.

## Launch

- **Live (default).** There is nothing to launch: the target is production, read-only.
- **Local candidate.** Serve this checkout's freshly built `dist/` in local workerd, the same server the publication tests use:

  ```sh
  $B serve --out "$OUT"            # prints the URL; also written to $OUT/serve.url
  $B all --base "$(cat "$OUT/serve.url")" --out "$OUT"
  $B stop --out "$OUT"
  ```

  It is ready when `serve` reports `ok: true`, which means `GET /` returned 200. Each `serve` binds its own free port, so runs can sit side by side.

## Doctor

Run `$B doctor` first, and again whenever anything looks off. It checks:

- the tools are present;
- `GET /` returns 200 with the Foundations title;
- the footer names a source revision;
- live: that revision equals `master`, with the latest master Publication run's status and URL; local: it equals `HEAD`;
- `gh` can read the private repository.

A live revision behind `master` means the deploy is pending or failed. Open the reported run before driving anything else.

## Drive

| Command | Proves |
| --- | --- |
| `doctor` | Instance worth driving (above). |
| `routes` | Markdown routes return the source bytes at the served revision as `text/markdown; charset=utf-8`. HTML pages carry the nav, revision, Markdown and History links and no script. Ten removed or unknown routes return 404 with no redirect, then `/` recovers. |
| `anchors [PATH...]` | Every numbered article has `/#n` on the item that starts with article *n*'s title, and every bold-led section and subheading has `/way-of-working#<slug>`, all derived from the served source. An unreadable source fails. Every Foundations link with a fragment, and every "Foundations … article N", in PATHs resolves. PATHs default to this repo, `~/.claude/AGENTS.md`, `~/.codex/AGENTS.md` and `~/.agents/skills`. If legacy `~/.claude/CLAUDE.md` still exists, scan its citations too: it hides AGENTS.md until migrated. This is a read-only diagnostic, not a CLAUDE.md installation or native AGENTS-loading proof. |
| `browse` | Real Chromium with page JavaScript off reads both pages at 390px and 1440px in light and dark, with no horizontal overflow and a distinct dark background. It clicks the nav links and the footer Markdown link, opens every anchor on a phone and checks the target is scrolled into view, and gets a 404 for an unknown path, then `/`. Screenshots are saved. |
| `access` | Anonymous revision link status (404 is expected: private repo). As an org reader through `gh`: the revision exists, is on `master`, and each History link has commits. |
| `readback` | Builds the served revision in a throwaway worktree and runs the publication's own `scripts/verify-served.sh` against `--base`: byte-for-byte HTML and Markdown match, removed routes 404. |
| `build` | `npm ci && npm run check` in this checkout: fmt, Clippy, build, and the Rust tests including local workerd. Heavy; run it on a machine with the pinned toolchains. |
| `all` | `doctor routes anchors browse`, plus `access readback` when `--base` is live. |

To test the deploy of a merged change: wait for the master Publication run to finish, run `$B doctor` until "live is master" passes, then run `$B all`.

## Evidence

Everything a run produces stays in `--out`:

- `checks.jsonl`: every check;
- `screens/*.png`: each page at each width and scheme, the footer, the `#12` and `#prove` anchors, and the 404;
- `browse.jsonl` and `browse.log`;
- `readback.log`; `build.log` and `serve.log` when those commands ran.

Keep `--out` outside the checkout. Publish retained evidence externally and link it from the PR with the checked revision, commands and observed result. Put concise summary counts and any failing lines in the PR body or a PR comment. Open the screenshots before publishing them; a green JSON line is not a look at the page.

Proof standards:

- Drive the real routes, not `dist/` files.
- Compare against the served revision's source, never the working tree.
- A local candidate proves the artifact, not the release. Only a live run after deploy proves the release.
- Evidence holds no secrets. `anchors` prints citation paths with `~`, not your home directory.

## Cleanup

- `$B stop --out "$OUT"` kills only the process group `serve` started, after checking it is still that wrangler server, then confirms the port stops answering. `serve` refuses to start a second server into the same `--out`. Never kill wrangler or workerd by name: other lanes run their own on this host.
- `readback` removes its throwaway worktree itself, even when interrupted.
- Cleanup never touches `--out`: evidence stays.

## Helpers

- `bin/verify-foundations`: the CLI above.
- `bin/browse.py`: the Playwright driver behind `browse`. Run it through the CLI, which pins Playwright. It exits 3 when it finished with failed checks; any other nonzero exit is a crash, which the CLI reports as a failed `browse` check.

Keep this skill and [features/](features/README.md) honest with `/maintain-verification-skill`.
