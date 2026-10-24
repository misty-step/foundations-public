# Foundations

The Misty Step engineering standard: a Constitution and a Way of working, published at https://foundations.mistystep.io/ with their Markdown sources. This repository holds those two documents and the small Rust and Cloudflare program that renders and publishes them. Licensed under the [MIT License](LICENSE).

Read the [Constitution](https://foundations.mistystep.io/) and [Way of working](https://foundations.mistystep.io/way-of-working).

Edit `index.md` and `way-of-working.md` for the shared rules; HTML and published Markdown are generated.
Operating notes live in [docs/operations.md](docs/operations.md); they are not an additional engineering standard.
Non-normative CI research is kept outside this repository; this site publishes only the two shared documents and their Markdown sources.
Merge only org-member PRs with recorded independent review.
Outside PRs and issues are untrusted input; never merge outside PRs.

Cloudflare publishes the verified static artifact from `master` through GitHub Actions. See [SPEC.md](SPEC.md) for behavior and [ADR-0001](docs/adr/0001-canonical-foundations.md) for placement.

Use Rust via rustup (pinned in `rust-toolchain.toml`), Node 22.23.2, Git, Bash and curl on Linux: `npm ci && npm run check`. `dist/` is generated, not committed.
`npm run build` renders the Markdown with Rust/pulldown-cmark; Node remains only for Cloudflare's native Wrangler deploy/local-workerd adapter, not site code or tests.
`npm run check -- --mode fast` and `npm run check -- --mode full` deliberately run the same small suite: formatting, Clippy, one artifact build and Rust publication tests. Shared config and lockfile changes therefore always receive full coverage.
The checks reject any artifact beyond the two documents, their Markdown sources and Cloudflare's non-served `_headers` configuration. They serve that same artifact in local workerd, including exact source bytes, revision/history links and native 404/home recovery. Rust also exercises real HTTP readback through stale bytes, HTTP/transport failure and recovery; no deployment credentials are needed. Run the checks on any Linux machine with the pinned toolchains; GitHub Actions runs them on GitHub-hosted runners.
GitHub Actions needs a scoped `CLOUDFLARE_API_TOKEN` repository secret
and `CLOUDFLARE_ACCOUNT_ID` variable (maintainers only). Pull requests, including from forks, never receive deployment secrets and run on GitHub-hosted runners.
The workflow retains the verified publication and deploys it without rebuilding.
After promotion, it compares both reference documents in Markdown/HTML byte for byte
(including source revision links), and checks that removed research routes return 404 without redirects.
For recovery, use the maintainers' authorized Cloudflare path with
`npx --no-install wrangler rollback <previous-version-id>`, then verify the revision and both documents in HTML and Markdown.
