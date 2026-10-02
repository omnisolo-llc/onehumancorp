"""Compile byte-exact production handlers; isolate only unused state dependencies."""
import hashlib
import json
from pathlib import Path
import re

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
SOURCE = ROOT / "src/server/api/growth.rs"
source = SOURCE.read_text()
names = [
    ("handle_post_purchase_embed", "PostPurchaseEmbedQuery"),
    ("handle_customer_referral_embed", "CustomerReferralEmbedQuery"),
    ("handle_viral_goal_tracker", "ViralGoalTrackerQuery"),
    ("handle_birthday_club_embed", "BirthdayClubEmbedQuery"),
    ("handle_viral_widget_embed", "ViralWidgetEmbedQuery"),
]
items, queries, slices = [], [], {}
for function, struct in names:
    # Top-level Rust closing braces are unindented. Embedded format-string braces
    # are doubled, so they cannot match this complete-item delimiter.
    for name, pattern in [
        (struct, rf"^#\[derive\([^\n]+\)\]\npub struct {struct} \{{.*?^\}}\n"),
        (function, rf"^(?:pub )?async fn {function}\(.*?^\}}\n"),
    ]:
        matches = list(re.finditer(pattern, source, re.M | re.S))
        if len(matches) != 1:
            raise RuntimeError(f"Expected exactly one complete production item: {name}")
        item = matches[0].group()
        items.append(item)
        slices[name] = hashlib.sha256(item.encode()).hexdigest()
        if name == function and function != "handle_viral_widget_embed":
            sql = re.findall(r'"(SELECT plan_tier FROM tenants WHERE [^"\n]+)"', item)
            if len(sql) != 1:
                raise RuntimeError(f"Expected one production tenant lookup in {name}")
            queries.extend(sql)

escape = re.search(r"^fn escape_html\(.*?^\}\n", source, re.M | re.S).group()
items.append(escape)
slices["escape_html"] = hashlib.sha256(escape.encode()).hexdigest()

migration = ROOT / "src/server/migrations/008_data_model_architecture.sql"
db = ROOT / "src/server/db.rs"
referral_migration = ROOT / "src/server/migrations/002_missing_tables.sql"
pg_ddl = re.search(r"CREATE TABLE IF NOT EXISTS tenants \(.*?\n\);", migration.read_text(), re.S).group()
# The first definition is the standalone SQLite branch, before the MySQL branch.
sqlite_ddl = re.search(r"CREATE TABLE IF NOT EXISTS tenants \(.*?\n\s*\);", db.read_text(), re.S).group()
referral_ddl = re.search(r"CREATE TABLE IF NOT EXISTS referrals \(.*?\n\);", referral_migration.read_text(), re.S).group()
head = '''#![allow(dead_code)]
use axum::{Extension, response::IntoResponse};
use serde::Deserialize;
use sqlx::PgPool;
// These handlers only read pool. No database, query or rendering behavior is mocked.
#[derive(Clone)]
pub struct GrowthState { pool: PgPool }
'''
generated = head + "\n".join(items)
generated += f"\nconst LOOKUPS: [&str; 4] = {json.dumps(queries)};\n"
generated += f"const PG_TENANTS_DDL: &str = {json.dumps(pg_ddl)};\n"
generated += f"const PG_REFERRALS_DDL: &str = {json.dumps(referral_ddl)};\n"
generated += f"const SQLITE_TENANTS_DDL: &str = {json.dumps(sqlite_ddl)};\n"
generated += '#[cfg(test)]\n#[path = "test.rs"]\nmod tests;\n'
(HERE / "generated.rs").write_text(generated)
inputs = [SOURCE, migration, db, referral_migration, ROOT / "Cargo.lock", HERE / "prepare.py", HERE / "test.rs", HERE / "Cargo.toml", HERE / "check_referral_html.py", HERE / "check_referral_urls.py", HERE / "check_birthday_capture.py", HERE / "birthday_capture_runner.cjs"]
manifest = {
    "inputs": {str(p.relative_to(ROOT)): hashlib.sha256(p.read_bytes()).hexdigest() for p in inputs},
    "exact_production_items": slices,
    "generated_sha256": hashlib.sha256(generated.encode()).hexdigest(),
}
(HERE / "source-manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
print("Prepared five complete production handlers and canonical PostgreSQL/SQLite schemas")
