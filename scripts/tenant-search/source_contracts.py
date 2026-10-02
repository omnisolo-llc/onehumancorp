"""Verify the public search path uses the source-bound authenticated router."""
from pathlib import Path
import unittest

ROOT = Path(__file__).resolve().parents[2]

class SearchWiring(unittest.TestCase):
    def test_search_is_compiled_and_mounted_with_real_auth_and_database(self):
        self.assertIn('pub mod search;', (ROOT/'src/server/api/mod.rs').read_text())
        self.assertIn('.merge(api::search::router(db.pool.clone(), http_auth_store.clone()))',
                      (ROOT/'src/server/lib.rs').read_text())
        source = (ROOT/'src/server/api/search.rs').read_text()
        self.assertIn('"/api/v1/search"', source)
        self.assertIn('::server_auth::strict_bearer_auth_middleware', source)
        self.assertNotIn('test-tenant-id', source)

    def test_result_destinations_exist(self):
        for path in ['orders/[id]/page.tsx', 'customer/memory-graph/page.tsx', 'inbox/page.tsx']:
            self.assertTrue((ROOT/'src/ui/next/src/app'/path).is_file(), path)
        bff = (ROOT/'src/ui/next/src/app/api/v1/[...path]/route.ts').read_text()
        self.assertIn('export const GET = proxy;', bff)
        self.assertIn('proxyBackendRequest(request, path', bff)

if __name__ == '__main__':
    unittest.main()
