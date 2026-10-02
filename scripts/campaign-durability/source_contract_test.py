"""Guard real native discovery, explicit database opt-in, and original coverage."""
from pathlib import Path
import re
import unittest
ROOT = Path(__file__).resolve().parents[2]

class CampaignDiscovery(unittest.TestCase):
    def test_real_inline_services_discovers_campaign_only_in_tests(self):
        source = (ROOT / 'src/server/lib.rs').read_text().split('pub mod services {', 1)[1].split('\n}', 1)[0]
        self.assertRegex(source, r'#\[cfg\(test\)\]\s+pub mod campaign;')

    def test_all_six_original_postgres_tests_are_retained(self):
        source = (ROOT / 'src/server/services/campaign/service.rs').read_text()
        names = re.findall(r'#\[ignore[^\n]*\]\s+async fn (\w+)', source)
        self.assertEqual(set(names), {'test_create_draft_campaign', 'test_add_asset_to_campaign', 'test_launch_campaign_requires_asset', 'test_complete_campaign_flow', 'test_launch_campaign_requires_third_party_activation_dispatch', 'test_tenant_isolation'})
        self.assertEqual(len(names), 6)

    def test_database_fixture_has_no_ambient_or_default_database(self):
        fixture = ROOT / 'src/server/services/campaign/test_database.rs'
        source = fixture.read_text() if fixture.exists() else (ROOT / 'src/server/services/campaign/service.rs').read_text()
        self.assertIn('OHC_CAMPAIGN_TEST_DATABASE_URL', source)
        self.assertNotIn('var("DATABASE_URL")', source)
        self.assertNotIn('postgres://', source)
        self.assertIn('NOSUPERUSER NOBYPASSRLS', source)
        self.assertIn('FORCE ROW LEVEL SECURITY', source)
        self.assertIn('session_user', source)

if __name__ == '__main__': unittest.main()
