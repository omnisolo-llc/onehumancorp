"""Source-bound regression guard, supplementary to PostgreSQL execution."""
from pathlib import Path
import unittest
ROOT = Path(__file__).resolve().parents[2]

class MountedSyncContract(unittest.TestCase):
    def test_mounted_events_use_real_durable_business_handler(self):
        source=(ROOT/'src/server/api/offline_sync.rs').read_text()
        start=source.index('pub async fn sync_events_handler(')
        end=source.index('\n#[cfg(test)]', start)
        mounted=source[start:end]
        self.assertNotIn('test_sync_entities', mounted)
        self.assertIn('durable_sync::sync_events(', mounted)
    def test_intents_require_signed_bearer_auth(self):
        source=(ROOT/'src/server/api/offline_sync.rs').read_text()
        handler=source[source.index('pub async fn operation_intents_handler('):]
        self.assertNotIn('parse_spiffe_id',handler)
        self.assertIn('validate_token_and_get_tenant',handler)
    def test_terminal_has_exact_request_identity_and_atomic_processing(self):
        helper=ROOT/'src/server/api/terminal_offline_sync.rs'
        self.assertTrue(helper.exists(),'The durable terminal helper was not preserved')
        source=helper.read_text()
        self.assertIn('request_identity',source)
        self.assertIn('commit().await',source)

if __name__=='__main__': unittest.main()
