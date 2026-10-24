# Revision and history

Each page's footer names the source revision it was built from and links that commit and the document's history on GitHub. Those links cross the site's only access gate: `misty-step/foundations` is a private repository, so they open only for readers with access to the `misty-step` organization.

## Sub-features

- `revision-shown`: the footer names a 40-character revision, linked to `github.com/misty-step/foundations/commit/<rev>`.
- `revision-current`: live, that revision is `master` and is reachable from it.
- `history-link`: `commits/master/<doc>.md` lists the document's changes.
- `access-gate`: anonymous readers get GitHub's 404; org readers see the commit and history.

## How to get to it (user POV)

- Scroll to the footer of either page and follow the revision hash or `History`.
- As an agent: `gh api repos/misty-step/foundations/commits/<rev>`, with the native `gh` login.

## Driving it with verify-foundations

Preconditions: a native `gh` login with read access to `misty-step/foundations` (`$B doctor`'s `GitHub reader` line names the account).

- **Revision is current.** Run `$B doctor --out "$OUT"`. `live is master` shows the served revision, `master`, and the latest master Publication run's status and URL.
- **Access gate.** Run `$B access --out "$OUT"`. `anonymous revision link` reports 404, which is expected for a private repository. `revision link as org reader` and `revision is on master` pass through `gh`, and each `History link` names the latest commit touching that document.
- **Proof.** `screens/constitution-footer.png` from `browse` shows the footer with the revision and both links.

## Gotchas

- Anonymous 404 is the designed gate, not a failure. A reader without organization access cannot open history; that is a property of the private repository.
- Never paste a GitHub token into the helper. It uses the native `gh` login only.
- Right after a merge, `live is master` fails until the Publication run deploys. That is pending, not broken; open the run URL in the detail.
