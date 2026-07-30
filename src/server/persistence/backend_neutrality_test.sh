#!/usr/bin/env bash
set -euo pipefail

source_root="${OHC_SERVER_SOURCE_ROOT:-${TEST_SRCDIR:-}/${TEST_WORKSPACE:-_main}/src/server}"
if [[ ! -d "$source_root" ]]; then
  source_root="$(git rev-parse --show-toplevel)/src/server"
fi

violations="$({
  rg --follow -n --glob '*.rs' \
    'sqlx::(query|query_as|query_scalar)|\b(PgPool|MySqlPool|SqlitePool|DbStore|GLOBAL_(PG|MYSQL|SQLITE)?_?POOL)\b' \
    "$source_root" \
    -g '!persistence/**' \
    -g '!**/*_test.rs' || true
} | sort)"

if [[ -n "$violations" ]]; then
  violation_count="$(printf '%s\n' "$violations" | wc -l | tr -d ' ')"
  printf '%s\n' "$violations" | sed -n '1,200p' >&2
  printf 'database-driver access must stay in src/server/persistence (%s violations; first 200 shown)\n' "$violation_count" >&2
  exit 1
fi
