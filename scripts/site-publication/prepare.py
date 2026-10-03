from pathlib import Path
import json, hashlib, re
HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[1]
source='pub mod builder {\n'
for name in ['db','edge','publication_store','publication_render','publication_worker','publication_public','publication_http','publication_json']:
    source += '#[path='+json.dumps(str(ROOT/'src/server/builder'/f'{name}.rs'))+'] pub mod '+name+';\n'
source += '}\n#[cfg(test)] #[path="test.rs"] mod tests;\n#[cfg(test)] #[path="render_tests.rs"] mod render_tests;\n#[cfg(test)] #[path="json_tests.rs"] mod json_tests;\n'
# Compile the real global bearer/tenant middleware and derive publication ordering
# from main, so a child-router-only test cannot miss a surrounding auth layer.
main = (ROOT/'src/server/lib.rs').read_text().split('\n#[cfg(test)]\nmod tests {', 1)[0]
bearer = re.search(r'(?ms)^async fn protected_bearer_auth_middleware\(.*?^}\n', main).group()
tenant = (ROOT/'src/server/utils/tenant_middleware.rs').read_text().split('#[cfg(test)]\nmod tests {', 1)[0]
source += 'extern crate self as server_utils;\npub mod tenant_middleware {\n' + tenant + '\n}\n#[cfg(test)]\n' + bearer
mount = re.search(r'\.merge\(crate::builder::publication_http::router\(\s*db.pool.clone\(\),\s*http_auth_store.clone\(\),\s*\)\)', main)
assert mount, 'Actual application publication mount is required'
layer_start = main.index('.route_layer(axum::middleware::from_fn(::server_utils::tenant_middleware::tenant_middleware))', main.index('let app = axum::Router::new()'))
layer_end = main.index('.with_state(mesh_transport)', layer_start)
# Tier middleware between these layers changes only protected/autodream paths.
# It is intentionally not a replacement implementation in this focused graph.
assert 'protected_bearer_auth_middleware,' in main[layer_start:layer_end]
assert '::server_utils::tier_middleware::tier_middleware,' in main[layer_start:layer_end]
publication = mount.group().replace('db.pool.clone()', 'pool.clone()').replace('http_auth_store.clone()', 'auth.clone()')
protected = '.route_layer(axum::middleware::from_fn(tenant_middleware::tenant_middleware)).route_layer(axum::middleware::from_fn_with_state(auth.clone(), protected_bearer_auth_middleware))'
chain = publication + protected if mount.start() < layer_start else protected + publication
# Existing health endpoint is an actual auth bypass; it supplies a route for the
# outer protected composition without inventing a successful business handler.
source += '#[cfg(test)] fn application_publication_routes(pool: sqlx::PgPool, auth: std::sync::Arc<server_auth::Store>) -> axum::Router { axum::Router::new().route("/health", axum::routing::get(|| async { axum::http::StatusCode::OK }))' + chain + ' }\n'

# Include the complete mutable storefront and shared response cache. Auth tests
# exercise the same mounted router rather than a replacement handler.
cache = (ROOT/'src/server/utils/cache.rs').read_text().split('#[cfg(test)]',1)[0]
source += 'pub mod cache {\n'+cache+'\n}\npub mod utils {pub use crate::cache;\n'
source += '#[path='+json.dumps(str(ROOT/'src/server/utils/edge_caching_middleware.rs'))+'] pub mod edge_caching_middleware; }\n'
source += 'pub mod api { #[path='+json.dumps(str(ROOT/'src/server/api/storefront_delivery.rs'))+'] pub mod storefront_delivery; }\n'
storefront_mount = next(line.strip() for line in main.splitlines() if '.nest("/api/v1/storefront",' in line)
storefront_mount = storefront_mount.replace('db.pool.clone()', 'pool.clone()').replace('http_auth_store.clone()', 'auth.clone()')
source += '#[cfg(test)] fn application_storefront_routes(pool:sqlx::PgPool,auth:std::sync::Arc<server_auth::Store>)->axum::Router { axum::Router::new()'+storefront_mount+'.route_layer(axum::middleware::from_fn_with_state(auth,protected_bearer_auth_middleware)) }\n'

(HERE/'generated.rs').write_text(source)
inputs=[ROOT/'Cargo.lock',ROOT/'Cargo.toml',ROOT/'src/server/builder/db.rs',ROOT/'src/server/builder/publication_store.rs',ROOT/'src/server/migrations/001_initial.sql',ROOT/'src/server/migrations/009_builder.sql',ROOT/'src/server/migrations/1019_site_publication_receipts.sql']
inputs += [ROOT/'src/server/utils/tenant_middleware.rs', ROOT/'src/server/utils/tier_middleware.rs', ROOT/'src/server/lib.rs', ROOT/'src/server/builder/mod.rs', ROOT/'scripts/focused_ci_gate.py', ROOT/'scripts/test_focused_ci_gate.py', ROOT/'.github/workflows/ci.yml']
inputs += [ROOT/'src/server/api/storefront_delivery.rs', ROOT/'src/server/builder/edge.rs', ROOT/'src/server/utils/cache.rs', ROOT/'src/server/utils/edge_caching_middleware.rs', HERE/'storefront_tests.rs']
inputs += [ROOT/'src/ui/next/package.json', ROOT/'src/ui/next/package-lock.json']
inputs += list((ROOT/'src/proto').glob('*.proto'))
inputs += [ROOT/'src/server/builder/publication_render.rs', ROOT/'src/server/builder/publication_worker.rs', ROOT/'src/server/builder/publication_public.rs', ROOT/'src/server/builder/publication_http.rs', ROOT/'src/server/builder/publication_json.rs']
inputs += [p for p in HERE.iterdir() if p.name in ['Cargo.toml','Cargo.lock','fetch.sh','test_fetch.py','prepare.py','test.rs','render_tests.rs','worker_tests.rs','public_tests.rs','http_tests.rs','json_tests.rs','raw-number-vectors.json','jcs-proof.cjs','test_mount.py','package.json','package-lock.json','verify_node_lock.py','test_node_lock.py','database_guard.py','test_database_guard.py','deny_http_egress.py','test_deny_http_egress.py','run.sh','verify_lock.py','README.md']]
for name in ['common','config','auth','oidc','omnisolo','telemetry']:
    inputs += [p for p in (ROOT/'src/server'/name).rglob('*') if p.is_file() and (p.suffix=='.rs' or p.name=='Cargo.toml')]
(HERE/'source-manifest.json').write_text(json.dumps({str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(set(inputs))},indent=2)+'\n')
