#!/usr/bin/env bash
set -euo pipefail

migration_dir="${1:-$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)}"

duplicate_versions="$(find "${migration_dir}" -maxdepth 1 -type f -name '*.sql' -printf '%f\n' \
  | sed -E 's/^([0-9]+)_.*/\1/' \
  | sort \
  | uniq -d)"
if [[ -n "${duplicate_versions}" ]]; then
  echo "SQLx migration versions must be unique; duplicates: ${duplicate_versions}" >&2
  exit 1
fi

if matches="$(grep -RIn --include='*.sql' '^-- +goose Down' "${migration_dir}" || true)" && \
   [[ -n "${matches}" ]]; then
  echo "SQLx executes Goose down sections as forward SQL; remove these rollback sections:" >&2
  printf '%s\n' "${matches}" >&2
  exit 1
fi

help_articles_migration="${migration_dir}/1004_help_articles.sql"
if [[ ! -f "${help_articles_migration}" ]]; then
  echo "Active SQLx migrations must own the help_articles schema: ${help_articles_migration} is missing." >&2
  exit 1
fi

for required_pattern in \
  'CREATE TABLE IF NOT EXISTS help_articles' \
  'tenant_id TEXT NOT NULL' \
  'CREATE INDEX IF NOT EXISTS idx_help_articles_tenant_id' \
  'ALTER TABLE help_articles ENABLE ROW LEVEL SECURITY' \
  'ALTER TABLE help_articles FORCE ROW LEVEL SECURITY' \
  'CREATE POLICY tenant_isolation_help_articles ON help_articles'; do
  if ! grep -Fq "${required_pattern}" "${help_articles_migration}"; then
    echo "help_articles migration is missing required schema contract: ${required_pattern}" >&2
    exit 1
  fi
done

for required_pattern in \
  'CREATE TABLE IF NOT EXISTS video_tutorials' \
  'PRIMARY KEY (tenant_id, id)' \
  'ALTER TABLE video_tutorials FORCE ROW LEVEL SECURITY' \
  'CREATE POLICY tenant_isolation_video_tutorials ON video_tutorials' \
  'CREATE TABLE IF NOT EXISTS tooltips' \
  'ALTER TABLE tooltips FORCE ROW LEVEL SECURITY' \
  'CREATE TABLE IF NOT EXISTS walkthrough_steps' \
  'ALTER TABLE walkthrough_steps FORCE ROW LEVEL SECURITY'; do
  if ! grep -Fq "${required_pattern}" "${help_articles_migration}"; then
    echo "documentation migration is missing required schema contract: ${required_pattern}" >&2
    exit 1
  fi
done
