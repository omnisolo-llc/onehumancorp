#!/usr/bin/env bash
set -euo pipefail

migration_dir="${1:-$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)}"

normalized_sql="$({ cat "${migration_dir}"/*.sql; } | tr '\n' ' ' | tr -s '[:space:]' ' ')"

if ! grep -Eiq 'ALTER TABLE inbox_messages ADD COLUMN IF NOT EXISTS sender_id[[:space:]]+TEXT' <<<"${normalized_sql}"; then
  echo "inbox_messages.sender_id is queried by the desktop inbox API but is absent from the production SQLx migrations" >&2
  exit 1
fi
