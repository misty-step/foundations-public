# ADR-0001: Publish one foundations reference

Status: Accepted, 2026-10-15; placement and hosting approved by the platform owner (internal task reference omitted).

## Decision

Use the private `misty-step/foundations` repository, published at `foundations.mistystep.io` on Cloudflare, because fleet-wide coordination needs a deployed reference and the existing wiki is archived and unreachable from the chief.

`index.md` (Constitution) and `way-of-working.md` are the editable sources, replacing the original single page. A pinned Markdown renderer produces static HTML and unchanged Markdown publications. GitHub Actions verifies and deploys the same artifact to Cloudflare after merges to `master`; no custom request runtime or synchronization job. The permanent URLs are independent of the hosting product. Product repositories retain local code, specs, technical decisions, and run commands. Trackers retain coordination state.

The task's operator comments, including the SPEC and continuous-deployment additions, take precedence over historical catalog and harness guidance. No legacy adoption status is imported as verified compliance. Historical catalog pins remain recoverable in Git history and the task archive. Live pinned safety consumers remain until their owners prove a replacement.

## Static-prose stack exception

Readers and machines need complete reference documents without executing code. Rust renders at build time and Cloudflare serves static bytes; adding a WebAssembly client would add code and payload without a user benefit. This justified exception to the interactive Rust/WASM default preserves the Constitution’s quality requirements. `SPEC.md` owns observable behavior; this record owns architecture and stack rationale.

## Rejected homes

- The archived wiki's tailnet service resolves only from one internal workstation, not from the deploy host. Reopening that repository would revive a larger retired system for one document.
- The harness's existing `FOUNDATIONS.md` differs from the current operator instructions. Keeping it normative would create a second authority tied to one agent adapter.
- A new checker, skill, per-repo catalog, or copied brief would reintroduce the maintenance problem.
- GitHub Pages conflicts with the owner's Cloudflare-only hosting rule. The initially public two-commit history is rewritten without client-organization names before making the repo private; old caches cannot be recalled.

## Verification

The retirement and reachability reports record the observed checks. Independent review precedes publication. A newly dispatched worker must discover the page from the tiny shared pointer and retrieve it without an operator supplying its contents.
