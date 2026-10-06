#!/usr/bin/env bash
set -euo pipefail

source_root="${TEST_SRCDIR:-}/${TEST_WORKSPACE:-_main}/src/server"
if [[ ! -f "$source_root/lib.rs" ]]; then
  source_root="$(git rev-parse --show-toplevel)/src/server"
fi

server_source="$source_root/lib.rs"
db_source="$source_root/db.rs"
catalog_source="$source_root/api/catalog.rs"
collective_source="$source_root/services/collective/service.rs"
collective_http_source="$source_root/api/collective.rs"
docs_source="$source_root/api/docs.rs"
payment_source="$source_root/api/payment_ledger.rs"
pos_source="$source_root/api/pos.rs"
inventory_source="$source_root/api/pos_inventory.rs"
local_seo_source="$source_root/api/local_seo.rs"

[[ -f "$server_source" ]] || { echo "server source is unavailable: $server_source" >&2; exit 1; }
[[ -f "$db_source" ]] || { echo "database source is unavailable: $db_source" >&2; exit 1; }
[[ -f "$catalog_source" ]] || { echo "catalog source is unavailable: $catalog_source" >&2; exit 1; }
[[ -f "$collective_source" ]] || { echo "collective source is unavailable: $collective_source" >&2; exit 1; }
[[ -f "$collective_http_source" ]] || { echo "collective HTTP source is unavailable: $collective_http_source" >&2; exit 1; }
[[ -f "$docs_source" ]] || { echo "docs source is unavailable: $docs_source" >&2; exit 1; }
[[ -f "$payment_source" ]] || { echo "payment source is unavailable: $payment_source" >&2; exit 1; }
[[ -f "$pos_source" ]] || { echo "POS source is unavailable: $pos_source" >&2; exit 1; }
[[ -f "$inventory_source" ]] || { echo "inventory persistence source is unavailable: $inventory_source" >&2; exit 1; }
[[ -f "$local_seo_source" ]] || { echo "local SEO source is unavailable: $local_seo_source" >&2; exit 1; }

python3 - "$server_source" "$db_source" "$catalog_source" "$collective_source" "$collective_http_source" "$docs_source" "$payment_source" "$pos_source" "$local_seo_source" "$inventory_source" <<'PY'
import pathlib
import re
import sys

server = pathlib.Path(sys.argv[1]).read_text().rsplit("\n#[cfg(test)]\nmod tests {", 1)[0]
db = pathlib.Path(sys.argv[2]).read_text()
catalog = pathlib.Path(sys.argv[3]).read_text()
collective = pathlib.Path(sys.argv[4]).read_text()
collective_http = pathlib.Path(sys.argv[5]).read_text()
docs = pathlib.Path(sys.argv[6]).read_text()
payment = pathlib.Path(sys.argv[7]).read_text()
pos = pathlib.Path(sys.argv[8]).read_text()
local_seo = pathlib.Path(sys.argv[9]).read_text()
inventory = pathlib.Path(sys.argv[10]).read_text()

if "legacy_db_compatibility_layer" not in server:
    raise SystemExit("legacy database routes do not use a shared compatibility layer")

if "Extension(std::sync::Arc::new(db.clone()))" in server:
    raise SystemExit("legacy database layer still wraps an Arc<DB> in a nested Arc")

for route in ("/api/v1/help", "/api/v1/tooltips", "/api/v1/videos"):
    if route not in server:
        raise SystemExit(f"legacy route is missing from the server router: {route}")

if "/api/v1/mesh/v2/collective" not in server:
    raise SystemExit("neighborhood collective UI route is missing from the server router")

if "get_mysql_pool_if_exists" not in docs:
    raise SystemExit("legacy documentation handlers do not declare a real MySQL/HeatWave path")
if "WHERE tenant_id = ?" not in docs:
    raise SystemExit("legacy documentation handlers do not use parameterized MySQL tenant queries")
if "CREATE TABLE IF NOT EXISTS help_articles" not in db or "CREATE TABLE IF NOT EXISTS walkthrough_steps" not in db:
    raise SystemExit("HeatWave startup schema does not initialize documentation tables")
if "CREATE TABLE IF NOT EXISTS ledger_accounts" not in db or "CREATE TABLE IF NOT EXISTS ledger_entries" not in db:
    raise SystemExit("HeatWave startup schema does not initialize payment ledger tables")
if "INSERT IGNORE INTO ohc_collective_member" not in collective_http:
    raise SystemExit("collective HTTP handlers do not declare a real MySQL persistence path")
if "get_mysql_pool_if_exists" not in payment or "WHERE tenant_id = ?" not in payment:
    raise SystemExit("payment ledger read handlers do not declare a real MySQL persistence path")
if not re.search(r'"/api/v1/ui/inventory",\s*axum::routing::get\(api::pos::get_inventory_handler\)', server):
    raise SystemExit("inventory UI route is not mounted to its real read handler")
inventory_handler = pos.split("pub async fn get_inventory_handler(", 1)[-1].split("#[cfg(test)]", 1)[0]
if not re.search(r'let Some\(tenant\) = pos_tenant\(claims\.as_ref\(\)\)', inventory_handler) or not re.search(
    r'if let Some\(pool\) = crate::db::get_mysql_pool_if_exists\(\)\s*\{\s*inventory::read_mysql\(&pool,\s*&tenant\)\.await', inventory_handler
):
    raise SystemExit("inventory UI handler does not dispatch its signed tenant to the real HeatWave reader")
mysql_inventory_reader = inventory.split("pub async fn read_mysql(", 1)[-1].split("pub async fn apply_mysql(", 1)[0]
if not re.search(
    r'sqlx::query\("SELECT [^"]* FROM products p WHERE p\.tenant_id=\? ORDER BY p\.id"\)\s*\.bind\(tenant\)\s*\.fetch_all\(pool\)\.await\?', mysql_inventory_reader
):
    raise SystemExit("inventory HeatWave reader does not execute its parameterized tenant-owned product query")
if "/api/v1/ledger/accounts" not in server or "pub async fn get_accounts" not in payment:
    raise SystemExit("dashboard ledger account alias is not wired to the real ledger handler")
if "/api/v1/ledger/entries" not in server or "pub async fn get_entries" not in payment:
    raise SystemExit("dashboard ledger entry alias is not wired to the real ledger handler")
if "/api/v1/user/usage" not in server or "pub async fn get_user_usage" not in payment:
    raise SystemExit("dashboard usage alias is not wired to persisted usage data")
if "CREATE TABLE IF NOT EXISTS application_settings" not in db or "CREATE TABLE IF NOT EXISTS tenant_ai_budgets" not in db:
    raise SystemExit("HeatWave startup schema does not initialize assistant settings and usage tables")
if "CREATE TABLE IF NOT EXISTS seo_discovery_reports" not in db or "pub async fn get_discovery_report" not in local_seo or "get_mysql_pool_if_exists" not in local_seo:
    raise SystemExit("Local SEO discovery reports do not declare a real HeatWave persistence path")
if "OMNISOLO_DATABASE_URL" not in db:
    raise SystemExit("canonical OmniSolo database URL alias is missing")

for fabricated in ("carlos_repairs", "fatima_food_cart"):
    if fabricated in collective:
        raise SystemExit(f"collective discovery still contains fabricated tenant data: {fabricated}")

for fabricated in ("Generated Offering", "AI description", '"10.00"'):
    if fabricated in catalog:
        raise SystemExit(f"catalog generation still contains fabricated fallback data: {fabricated}")

if not re.search(r"MINIMAX_API_KEY", catalog):
    raise SystemExit("catalog generation does not declare its real provider dependency")
PY
