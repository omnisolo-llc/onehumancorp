from pathlib import Path
import unittest
ROOT = Path(__file__).resolve().parents[2]

class CatalogEditContract(unittest.TestCase):
    def test_product_edit_route_is_mounted(self):
        source = (ROOT / 'src/server/api/catalog.rs').read_text()
        self.assertIn('"/product/{id}"', source)
        self.assertIn('put(handle_update_product)', source)
    def test_product_creation_returns_persisted_identity(self):
        source = (ROOT / 'src/server/api/catalog.rs').read_text()
        response = source.split('pub struct CreateProductResponse {',1)[1].split('}',1)[0]
        self.assertIn('pub product_id: String', response)

if __name__ == '__main__': unittest.main()
