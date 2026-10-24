# Foundations verification map

The maintained source for verifying what Foundations' readers do. Read this index, then the matching feature file. [SPEC.md](../../../../SPEC.md) owns intended behavior; these files say how to prove it.

## Baseline preconditions

- `B=.agents/skills/verify-foundations/bin/verify-foundations`, run from the repository root.
- `$B doctor` passes against the target (`--base`, live by default).
- For a local candidate: `$B serve --out "$OUT"`, then pass `--base "$(cat "$OUT/serve.url")"`.
- Never write to production. Every check is a read; deploys happen only through the repository's Publication workflow.

## Driving conventions

- Use one `--out` per run so every check lands in one `checks.jsonl`.
- Browser steps use accessible names: the `Documents` navigation, links `Constitution`, `Way of working`, `Markdown` and `History`, and `main h1`.
- Compare served bytes with the served revision's source (`git show <rev>:index.md`), never the working tree.

## Proof and skip reporting

- A feature is proven when its checks pass in `checks.jsonl` and the named screenshots, opened and inspected, show it.
- Report a check that could not run (no `gh` login, no Chromium) as unverified with the reason. Never count it as a pass.
- Keep the two claims separate: "artifact verified" is a local candidate run; "live behavior proven" is a live run after the deploy.

## Features

- [Read the documents](./read-documents.md): both pages, in a browser, on phone and desktop, in light and dark, with navigation.
- [Machine-readable sources](./machine-sources.md): `/index.md` and `/way-of-working.md` are the exact source bytes.
- [Cited anchors](./cited-anchors.md): `/#12`, `/way-of-working#prove`, and every Foundations link guidance cites resolve.
- [Revision and history](./revision-and-history.md): the GitHub access gate behind the footer links.
- [Not found and recovery](./not-found.md): unknown and removed routes 404 natively, then home loads.
- [Build and publish](./build-and-publish.md): checks, local serving, the deploy and its readback.
