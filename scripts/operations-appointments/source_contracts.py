from pathlib import Path
import unittest
ROOT=Path(__file__).resolve().parents[2]
class AppointmentWiring(unittest.TestCase):
    def test_real_parent_mount_uses_the_protected_read_router(self):
        parent=(ROOT/'src/server/api/field_ops.rs').read_text()
        self.assertIn('pub mod appointments;',parent)
        self.assertIn('let reads = appointments::router(pool.clone(), auth_store);',parent)
        self.assertIn('.merge(reads)',parent)
        self.assertIn('crate::api::field_ops::router(db.pool.clone(), mesh_transport.clone(), http_auth_store.clone())',(ROOT/'src/server/lib.rs').read_text())
        child=(ROOT/'src/server/api/field_ops/appointments.rs').read_text()
        self.assertIn('server_auth::strict_bearer_auth_middleware',child)
        self.assertIn('"/appointments"',child)
    def test_maintained_next_proxy_preserves_the_authenticated_read_boundary(self):
        source=(ROOT/'src/ui/next/src/app/api/v1/field-ops/appointments/route.ts').read_text()
        self.assertIn('export async function GET(request: Request)',source)
        self.assertIn('proxyBackendRequest(request, "/api/v1/field-ops/appointments"',source)
        self.assertIn('suppressRequestBody: true',source)
if __name__=='__main__':unittest.main()
