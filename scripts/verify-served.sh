#!/usr/bin/env bash
set -euo pipefail

base_url="${1:-https://foundations.mistystep.io}"
for attempt in {1..9}; do
  matches=true
  for path in index.md way-of-working.md index.html way-of-working.html; do
    route="$path"
    case "$path" in
      index.html) route="" ;;
      way-of-working.html) route="way-of-working" ;;
    esac
    output="served-${path//\//-}"
    if ! curl --fail --silent --show-error --connect-timeout 5 --max-time 15 "$base_url/$route" -o "$output" ||
       ! cmp "dist/$path" "$output"; then
      matches=false
      break
    fi
  done
  if "$matches"; then
    for route in research research/ci research/ci/ research/ci.html research/ci.md research/ci-evidence.json revision.txt 404.html; do
      if ! status="$(curl --silent --show-error --connect-timeout 5 --max-time 15 -o /dev/null -w '%{http_code}' "$base_url/$route")" || [ "$status" != 404 ]; then
        matches=false
        break
      fi
    done
  fi
  if "$matches"; then
    printf 'Canonical reference documents and revision links match the verified artifact; removed routes return 404.\n'
    exit 0
  fi
  printf 'Canonical publication not yet verified (attempt %s/9).\n' "$attempt" >&2
  if [ "$attempt" -lt 9 ]; then
    sleep 10
  fi
done
printf 'Canonical publication did not converge to the verified artifact.\n' >&2
exit 1
