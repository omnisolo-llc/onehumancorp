"""Source-bound regression guard, supplementary to PostgreSQL execution."""
from pathlib import Path
import json
import re
import subprocess
import sys
import tomllib
import unittest
ROOT = Path(__file__).resolve().parents[2]

class MountedSyncContract(unittest.TestCase):
    def test_generated_handlers_share_one_canonical_offline_authority(self):
        subprocess.run([sys.executable, str(ROOT/'scripts/sync-durability/prepare.py')], check=True)
        generated=(ROOT/'scripts/sync-durability/generated.rs').read_text()
        module=f'#[path={json.dumps(str(ROOT/"src/server/api/terminal_offline_authority.rs"))}] pub mod terminal_offline_authority;'
        self.assertEqual(generated.count(module), 1, 'retain one real authority at crate::api::terminal_offline_authority')

    def test_generated_pos_reader_keeps_exact_inventory_dependencies(self):
        subprocess.run([sys.executable, str(ROOT/'scripts/sync-durability/prepare.py')], check=True)
        generated=(ROOT/'scripts/sync-durability/generated.rs').read_text()
        pos=(ROOT/'src/server/api/pos.rs').read_text()
        query=re.search(r'#\[derive\(serde::Deserialize\)\]\s*pub struct InventoryQuery\s*\{[^}]*\}', pos)
        self.assertIsNotNone(query)
        self.assertIn(query.group(), generated, 'compile the exact production query extractor type')
        module=f'#[path = {json.dumps(str(ROOT/"src/server/api/pos_inventory.rs"))}]\nmod inventory;'
        self.assertIn(module, generated, 'include the complete real inventory module and its tests')
        manifest=json.loads((ROOT/'scripts/sync-durability/source-manifest.json').read_text())
        for path in ['src/server/api/pos_inventory.rs', 'src/server/api/pos_inventory_test.rs', 'src/server/migrations/1041_inventory_adjustment_receipts.sql', 'src/server/api/durable_sync_test_schema.sql']:
            self.assertIn(path, manifest, f'fingerprint transitive inventory input {path}')

    def test_inventory_dependency_tests_retain_owned_database_and_discovery(self):
        runner=(ROOT/'scripts/sync-durability/run.sh').read_text()
        self.assertIn('export OHC_INVENTORY_TEST_DATABASE_URL="$OHC_SYNC_TEST_DATABASE_URL"', runner)
        self.assertIn('-- --include-ignored --test-threads=2', runner)
        manifest=tomllib.loads((ROOT/'scripts/sync-durability/Cargo.toml').read_text())
        self.assertIn('reqwest', manifest['dev-dependencies'])

    def test_mounted_inventory_fixture_supplies_real_reader_columns(self):
        fixture=(ROOT/'scripts/sync-durability/mounted_test.rs').read_text()
        self.assertIn('ALTER TABLE inventory_levels ADD COLUMN id TEXT', fixture)
        self.assertIn('ADD COLUMN committed_count INTEGER NOT NULL DEFAULT 0', fixture)

    def test_mounted_events_use_real_durable_business_handler(self):
        source=(ROOT/'src/server/api/offline_sync.rs').read_text()
        start=source.index('pub async fn sync_events_handler(')
        end=source.index('\n#[cfg(test)]', start)
        mounted=source[start:end]
        self.assertNotIn('test_sync_entities', mounted)
        self.assertIn('durable_sync::sync_authorized_events(', mounted)
    def test_intents_require_signed_bearer_auth(self):
        source=(ROOT/'src/server/api/offline_sync.rs').read_text()
        handler=source[source.index('pub async fn operation_intents_handler('):]
        self.assertNotIn('parse_spiffe_id',handler)
        self.assertIn('validate_token_and_get_tenant',handler)
    def test_pos_read_routes_use_the_real_bearer_boundary(self):
        source=(ROOT/'src/server/lib.rs').read_text()
        start=source.index('"/api/v1/pos"')
        mounted=source[start:source.index('.nest("/api/v1/cart"',start)]
        self.assertIn('api::pos::pos_routes(hub.clone())',mounted)
        self.assertIn('::server_auth::strict_bearer_auth_middleware',mounted)
        routes=(ROOT/'src/server/api/pos.rs').read_text()
        self.assertIn('get(get_orders_handler)',routes)
        self.assertIn('get(get_inventory_handler)',routes)
    def test_terminal_has_exact_request_identity_and_atomic_processing(self):
        helper=ROOT/'src/server/api/terminal_offline_sync.rs'
        self.assertTrue(helper.exists(),'The durable terminal helper was not preserved')
        source=helper.read_text()
        self.assertIn('request_identity',source)
        self.assertIn('commit().await',source)

if __name__=='__main__': unittest.main()
