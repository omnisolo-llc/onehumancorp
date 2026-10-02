"""Static wiring guards complement (and do not replace) the focused HTTP tests."""
from pathlib import Path
import re
import unittest

ROOT = Path(__file__).resolve().parents[2]

class SourceContracts(unittest.TestCase):
    def test_billing_path_matches_the_actual_public_mount(self):
        main = (ROOT / "src/server/lib.rs").read_text()
        middleware = (ROOT / "src/server/api/billing_webhook.rs").read_text()
        self.assertRegex(main, r'\.route\(\s*"/api/v1/webhooks/stripe",\s*axum::routing::post\(api::billing_webhook::stripe_webhook_handler\)')
        self.assertIn('let is_stripe = req.uri().path() == "/api/v1/webhooks/stripe";', middleware)
        router = main[main.index("let webhook_router ="):main.index("let meta_webhook_state =")]
        self.assertIn("api::billing_webhook::webhook_security_middleware", router)
        self.assertIn(".merge(webhook_router)", main)
        self.assertLess(middleware.index("stripe_webhook_security::verify_request"), middleware.index('let redis_key = format!("webhook:idempotency:'))

    def test_ledger_guard_is_on_the_existing_nested_webhook_only(self):
        main = (ROOT / "src/server/lib.rs").read_text()
        ledger = (ROOT / "src/server/api/payment_ledger.rs").read_text()
        self.assertRegex(main, r'"/api/v1/payments/ledger",\s*api::payment_ledger::router\(\)')
        self.assertRegex(ledger, r'"/webhook",\s*post\(stripe_webhook\)\.route_layer\(axum::middleware::from_fn_with_state\(\s*super::stripe_webhook_security::Endpoint::Ledger,\s*super::stripe_webhook_security::require_verified_stripe,')
        self.assertEqual(ledger.count("require_verified_stripe"), 1)

    def test_shared_verifier_modules_are_exported(self):
        self.assertIn("pub mod stripe_webhook_security;", (ROOT / "src/server/api/mod.rs").read_text())
        self.assertIn("pub mod webhook_signature;", (ROOT / "src/server/integrations/stripe/mod.rs").read_text())
        self.assertIn("pub use ::server_integrations_stripe as stripe;", (ROOT / "src/server/integrations/mod.rs").read_text())

    def test_issuing_remains_unmounted_outside_the_active_route_scope(self):
        issuing = ROOT / "src/server/integrations/stripe/issuing.rs"
        client = ROOT / "src/server/integrations/stripe/client.rs"
        callers = []
        for source in (ROOT / "src/server").rglob("*.rs"):
            if source not in (issuing, client) and re.search(r'handle_issuing_webhook\s*\(', source.read_text()):
                callers.append(str(source.relative_to(ROOT)))
        self.assertEqual(callers, [], "a new issuing caller requires its own authority and safety review")

if __name__ == "__main__":
    unittest.main()
