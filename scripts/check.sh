#!/usr/bin/env bash
set -euo pipefail
case "${*:-}" in
  ''|'--mode fast'|'--mode full') ;;
  *) printf 'Usage: npm run check -- [--mode fast|--mode full]\n' >&2; exit 2 ;;
esac
# The complete publication suite is small: fast and full deliberately exercise the same artifact.
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
npm run build
npm test
