"""Source wiring complements the database and real-client loopback tests."""
from pathlib import Path
import re
import unittest
ROOT=Path(__file__).resolve().parents[2]

class Wiring(unittest.TestCase):
    def test_existing_quote_routes_remain_under_strict_bearer_and_tenant_layers(self):
        main=(ROOT/'src/server/lib.rs').read_text()
        position=main.index('.nest("/api/v1/quotes", api::quotes::router().with_state(db.pool.clone()))')
        layer=main.index('protected_bearer_auth_middleware,',position)
        self.assertIn('::server_utils::tenant_middleware::tenant_middleware',main[position:layer])
        function=main[main.index('async fn protected_bearer_auth_middleware('):main.index('fn legacy_db_compatibility_layer(')]
        self.assertIn('::server_auth::strict_bearer_auth_middleware',function)
        bypass=(ROOT/'src/server/utils/tenant_middleware.rs').read_text().split('pub fn is_auth_bypass_path',1)[1].split('pub async fn',1)[0]
        paths=re.findall(r'"(/[^\"]*)"',bypass)
        self.assertEqual(set(paths),{'/api/v1/auth/login','/health','/healthz','/readyz','/metrics'})

    def test_route_and_alternate_approve_keep_the_authenticated_claims_extractor(self):
        source=(ROOT/'src/server/api/quotes.rs').read_text()
        for name in ['get_quote','accept_quote','update_quote','approve_quote']:
            signature=source.split('async fn '+name+'(',1)[1].split(') ->',1)[0]
            self.assertIn('Extension(claims): Extension<::server_common::Claims>',signature,name)
        self.assertIn('.route("/{id}/accept", post(accept_quote))',source)
        self.assertIn('.route("/{id}/approve", axum::routing::patch(approve_quote))',source)

    def test_acceptance_uses_the_existing_idempotent_checkout_transport(self):
        source=(ROOT/'src/server/api/quote_acceptance.rs').read_text()
        self.assertIn('create_checkout_session_idempotent',source)
        self.assertIn('operation_id: operation',source)
        client=(ROOT/'src/server/integrations/stripe/safe_checkout.rs').read_text()
        self.assertIn('.header("Idempotency-Key", operation_id)',client)
        self.assertIn('.redirect(reqwest::redirect::Policy::none())',client)

if __name__=='__main__':unittest.main()
