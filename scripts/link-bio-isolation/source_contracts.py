from pathlib import Path
import unittest
ROOT=Path(__file__).resolve().parents[2]
class Wiring(unittest.TestCase):
    def test_handlers_remain_under_the_existing_protected_growth_route(self):
        growth=(ROOT/'src/server/api/growth.rs').read_text()
        self.assertIn('.route("/link-in-bio", post(handle_post_link_in_bio))',growth)
        self.assertIn('.route("/link-in-bio/{tenant}", get(handle_get_link_in_bio))',growth)
        source=(ROOT/'src/server/lib.rs').read_text()
        mounted_at = source.index('.nest("/api/v1/growth"')
        protected_at = source.index('            protected_bearer_auth_middleware,', mounted_at)
        self.assertLess(mounted_at, protected_at)
        bypass=(ROOT/'src/server/utils/tenant_middleware.rs').read_text().split('pub fn is_auth_bypass_path',1)[1].split('pub async fn tenant_middleware',1)[0]
        self.assertNotIn('/growth',bypass)
if __name__=='__main__':unittest.main()
