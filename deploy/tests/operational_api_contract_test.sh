#!/usr/bin/env bash
set -euo pipefail

for script in "$@"; do
  grep -Fq '/api/v1/' "$script" || {
    echo "$script must call only versioned OmniSolo APIs" >&2
    exit 1
  }
  if grep -E '/api/(dev|agents)/' "$script" | grep -Fvq '/api/v1/'; then
    echo "$script contains an unversioned OmniSolo API call" >&2
    exit 1
  fi
  for literal in \
    OMNISOLO_ACCESS_TOKEN \
    OMNISOLO_ACCESS_TOKEN_FILE \
    'Authorization: Bearer ${ACCESS_TOKEN}' \
    '--connect-timeout' \
    '--max-time' \
    'jq -n' \
    '--data-binary'; do
    grep -Fq -- "$literal" "$script" || {
      echo "$script is missing required secure API client behavior: $literal" >&2
      exit 1
    }
  done
  if grep -Eq 'export \$\(.*\.env|--data[[:space:]]+"?\$' "$script"; then
    echo "$script must not evaluate .env files or put JSON payloads in argv" >&2
    exit 1
  fi
done
