# verify-foundations: first run, 2026-10-20

Runs of `.claude/skills/verify-foundations/bin/verify-foundations` (the same files as `.agents/skills/verify-foundations/`) from commit `f88a980` on a Linux workstation. Each `checks.jsonl` line is one check.

| Run | Target | Result |
| --- | --- | --- |
| [`build/`](build/) | `npm run check` on the candidate | pass: fmt, Clippy, build, 4 publication + 1 readback tests ([test-results.txt](build/test-results.txt)) |
| [`live-base/`](live-base/) | `https://foundations.mistystep.io`, serving `29c2ec2` (master before this change) | 77 checks. Fails only on anchors: 0/25 article and section anchors, 0/6 cited links. Routes 15/15, browser read 8/8, scheme 4/4, navigation 3/3, 404 and recovery, access gate 5/5 and deploy readback ([readback.log](live-base/readback.log)) pass |
| [`candidate/`](candidate/) | the candidate's `dist/` in local workerd | 98/98 pass: each `/#n` sits on article *n*'s item; all 25 anchors scroll into view from the top of the page on a phone; all 6 cited links resolve |

The live failure is the defect this change fixes. Guidance cites "Foundations article 12" and `/#…` anchors, but the live renderer emits no ids. The new publication test fails on the base renderer and passes on the candidate:

```text
# base src/main.rs, rebuilt dist/
thread 'reference_sources_navigation_and_complete_prose' panicked at tests/publication.rs:84:9:
missing anchor <li id="1">
test result: FAILED. 2 passed; 2 failed
# candidate src/main.rs, rebuilt dist/
test result: ok. 4 passed; 0 failed
```

Screenshots, inspected: [Constitution, phone, light](candidate/screens/constitution-phone-light.png), [Way of working, phone, dark](candidate/screens/way-of-working-phone-dark.png), [desktop, dark](candidate/screens/constitution-desktop-dark.png), [`/#12` on a phone](candidate/screens/anchor-constitution-12.png), [`/way-of-working#prove`](candidate/screens/anchor-way-of-working-prove.png), [footer](candidate/screens/constitution-footer.png), [404](candidate/screens/not-found.png).

The candidate run shows "artifact verified"; the base run shows the live site before this change. "Live behavior proven" for the anchors needs a live run after the deploy, which is linked on the PR.
