# Not found and recovery

A reader who follows a stale or mistyped link, including the retired research routes and `/revision.txt`, gets a native 404 with no redirect or fallback page, and the site's home still loads afterwards.

## Sub-features

- `not-found-native`: unknown paths return 404 without `Location`.
- `not-found-retired`: `/research`, `/research/ci`, `/research/ci/`, `/research/ci.html`, `/research/ci.md`, `/research/ci-evidence.json`, `/revision.txt`, `/404.html` and `/_headers` all return 404.
- `recover-home`: `/` returns 200 after any 404.

## How to get to it (user POV)

- Open a stale link such as `https://foundations.mistystep.io/research/ci`, or any mistyped path.

## Driving it with verify-foundations

Preconditions: `$B doctor` passes.

- **Retired and unknown routes.** Run `$B routes --out "$OUT"`. Ten 404 lines, each `ok: true` with no `redirects to`, then `/ after 404s` with status 200.
- **In a browser.** `$B browse --out "$OUT"` prints `not-found` (404, not redirected) and `recover` (`/` loads the Constitution). Open `screens/not-found.png`.

## Gotchas

- `wrangler.jsonc` sets `not_found_handling: none`; a change to it shows up here as a 200 fallback page or a redirect.
- `/_headers` is a deploy-time config file and must never be served.
