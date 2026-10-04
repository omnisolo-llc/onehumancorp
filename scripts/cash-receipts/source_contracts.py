"""Cheap source guards; these supplement, never replace, real PG/Redis tests."""
from pathlib import Path
import unittest
ROOT = Path(__file__).resolve().parents[2]
class CashSourceContract(unittest.TestCase):
 def test_provider_is_validated_before_stock_mutation(self):
  source=(ROOT/'src/server/api/billing_api.rs').read_text().split('pub async fn create_checkout_session_handler(',1)[1].split('pub async fn cancel_subscription_handler(',1)[0]
  self.assertIn('require_api_key()',source)
  self.assertLess(source.index('require_api_key()'),source.index('.reserve_inventory('))
 def test_cash_handler_uses_atomic_receipt_boundary(self):
  source=(ROOT/'src/server/api/terminal_api.rs').read_text().split('pub async fn commit_inventory_handler(',1)[1].split('pub async fn create_payment_intent_handler(',1)[0]
  self.assertIn('cash_receipts::commit',source)
  self.assertNotIn('.commit_inventory(',source)
 def test_atomic_stock_helper_leaves_commit_with_caller(self):
  source=(ROOT/'src/server/services/inventory/service.rs').read_text()
  self.assertIn('pub async fn commit_inventory_in_transaction(',source)
 def test_readback_is_mounted(self):
  source=(ROOT/'src/server/api/terminal_api.rs').read_text()
  self.assertIn('"/commit/{operation_id}"',source)
  self.assertIn('axum::routing::get(read_cash_receipt_handler)',source)
if __name__ == '__main__':unittest.main()
