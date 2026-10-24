# Foundations publication

## What it does

Foundations publishes the Constitution and Way of working as readable web pages and machine-readable source at one permanent address.

## What it enables

Misty Step's humans and agents can build, verify, release and recover projects against one living standard instead of contradictory copies of shared rules.

This publication does not define a second engineering policy.

Architecture and the static-prose stack exception are recorded in [ADR-0001](docs/adr/0001-canonical-foundations.md); this SPEC owns observable publication behavior.

## US-001 Read

- An engineer opens `/` or `/way-of-working` without credentials and reads the complete Constitution or Way of working, respectively, without executing JavaScript. Each page links to both documents and retains the Foundations site title.
- Guidance can cite any article or section by a stable link: `/#<n>` opens Constitution article *n*, and `/way-of-working#<section>` opens a bold-led section or subheading by its slugged name, such as `/way-of-working#prove`.
- Text is readable on phone and desktop without horizontal overflow. Both documents follow the system light/dark preference.
- A machine fetches `/index.md` or `/way-of-working.md` and receives exactly the corresponding editable source bytes. A reader can identify the deployed source revision and open each document's source history.
- The site publishes only these two documents and their Markdown sources. Revision and history links remain in each document's footer, not a separate publication route. Unknown paths, including `/research`, all former research assets and `/revision.txt`, return a native 404 with no redirect or fallback document.
- Publication checks reject any extra file or directory in the deployable artifact; the only non-document file is Cloudflare's non-served `_headers` configuration.

## US-002 Update and recover

- The only editable reference documents are `index.md` and `way-of-working.md`. A reviewed source change builds a static publication; generated content is never edited independently.
- A pull request builds and verifies without deployment credentials. Before promotion, the built publication serves both documents with their source revision links, exact Markdown bytes, and a real not-found response followed by a successful home request over isolated local HTTP. A merge to the default branch publishes the very artifact that passed verification, with no parallel production promotions.
- Publication verification fails on a missing document link, missing reference text, exposed private/client identifiers, or a mismatch between either source and its machine-readable publication.
- If publication verification fails, production is unchanged. After a successful deployment, both Markdown documents and both HTML pages (including their revision links) must all match the verified artifact exactly in one attempt, and former research routes must return 404 without redirects. Readback allows nine attempts, ten seconds apart, with each request limited to fifteen seconds; persistent mismatches, HTTP errors, and transport failures fail publication. An operator can restore the previous Cloudflare deployment using the native rollback command.
