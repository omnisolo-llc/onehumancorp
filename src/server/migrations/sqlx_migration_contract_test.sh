#!/usr/bin/env bash
set -euo pipefail

migration_dir="${1:-$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)}"

# SQLx parses an i64 version, so 1 and 001 are the same identity. Do not use
# string sorting or GNU-only find -printf (developers also run this on macOS).
python3 - "${migration_dir}" <<'PY'
from pathlib import Path
import re
import sys

migrations = Path(sys.argv[1])
if not migrations.is_dir():
    raise SystemExit("SQLx migration directory is missing")
versions = {}
for migration in sorted(migrations.glob("*.sql")):
    match = re.fullmatch(r"([0-9]+)_(.+?)(?:\.(up|down))?\.sql", migration.name)
    if not match or int(match[1]) > 9223372036854775807:
        raise SystemExit(f"Invalid SQLx migration identity: {migration.name}")
    version, description, direction = int(match[1]), match[2], match[3] or "simple"
    entries = versions.setdefault(version, {})
    if direction in entries or (entries and (direction == "simple" or "simple" in entries)):
        raise SystemExit(f"SQLx migration versions must be unique; duplicate numeric version {version}: "
                         f"{', '.join(name for _, name in entries.values())}, {migration.name}")
    if entries and any(previous != description for previous, _ in entries.values()):
        raise SystemExit(f"SQLx migration {version} has mismatched up/down descriptions")
    entries[direction] = (description, migration.name)
if not versions:
    raise SystemExit("SQLx migration directory contains no migrations")
PY

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

runtime_schema_migration="${migration_dir}/222_field_ops_and_global_commerce.sql"
for required_pattern in \
  'CREATE TABLE IF NOT EXISTS job_locations' \
  'ALTER TABLE tenants' \
  'ADD COLUMN IF NOT EXISTS base_currency' \
  'ADD COLUMN IF NOT EXISTS enabled_currencies' \
  'ALTER TABLE job_locations FORCE ROW LEVEL SECURITY' \
  'CREATE POLICY tenant_isolation_job_locations'; do
  if ! grep -Fq "${required_pattern}" "${runtime_schema_migration}"; then
    echo "runtime schema migration is missing required contract: ${required_pattern} (${runtime_schema_migration})" >&2
    exit 1
  fi
done

runtime_parity_migration="${migration_dir}/225_runtime_schema_parity.sql"
for required_pattern in \
  'ADD COLUMN IF NOT EXISTS service_id' \
  'ADD COLUMN IF NOT EXISTS resource_id' \
  'ADD COLUMN IF NOT EXISTS tenant_id' \
  'CREATE TRIGGER shared_tasks_sync_owner_ids' \
  'CREATE TABLE IF NOT EXISTS daily_work_items' \
  'CREATE TABLE IF NOT EXISTS unified_triage_actions' \
  'CREATE TABLE IF NOT EXISTS agent_event_subscriptions'; do
  if ! grep -Fq "${required_pattern}" "${runtime_parity_migration}"; then
    echo "runtime parity migration is missing required contract: ${required_pattern} (${runtime_parity_migration})" >&2
    exit 1
  fi
done

initial_schema="${migration_dir}/001_initial.sql"
for required_pattern in \
  'id TEXT PRIMARY KEY' \
  'member_id TEXT NOT NULL REFERENCES users(id)' \
  'user_id TEXT NOT NULL REFERENCES users(id)'; do
  case "${required_pattern}" in
    'id TEXT PRIMARY KEY') schema_file="${initial_schema}" ;;
    'member_id TEXT NOT NULL REFERENCES users(id)') schema_file="${migration_dir}/1006_api_keys_and_usage_logs.sql" ;;
    'user_id TEXT NOT NULL REFERENCES users(id)') schema_file="${migration_dir}/1007_user_usage_logs.sql" ;;
  esac
  if ! grep -Fq "${required_pattern}" "${schema_file}"; then
    echo "PostgreSQL identity schema contract is missing: ${required_pattern} (${schema_file})" >&2
    exit 1
  fi
done
