"""Registration guard; actual router/worker behavior is tested by the Rust gate."""
from pathlib import Path
import unittest
ROOT = Path(__file__).resolve().parents[2]
class PublicationMount(unittest.TestCase):
    maxDiff = 1000
    def test_publication_routes_are_merged_into_application(self):
        source = (ROOT / 'src/server/lib.rs').read_text().split('\n#[cfg(test)]\nmod tests {')[0]
        self.assertTrue('.merge(crate::builder::publication_http::router(' in source)
        self.assertIn('pub mod publication_http;', (ROOT / 'src/server/builder/mod.rs').read_text())
        self.assertIn('pub mod publication_json;', (ROOT / 'src/server/builder/mod.rs').read_text())
    def test_publication_worker_is_started_only_with_postgres_background_support(self):
        source = (ROOT / 'src/server/lib.rs').read_text().split('\n#[cfg(test)]\nmod tests {')[0]
        self.assertTrue('if legacy_sqlx_background_enabled && matches!(&db.store, db::DbStore::Postgres) {\n        tokio::spawn(crate::builder::publication_worker::run_publication_worker(' in source)
        self.assertIn('let (_publication_shutdown, publication_shutdown_rx)', source)
if __name__ == '__main__': unittest.main()
