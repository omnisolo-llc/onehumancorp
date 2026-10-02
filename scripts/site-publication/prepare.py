from pathlib import Path
import json, hashlib
HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[1]
source='pub mod builder {\n'
for name in ['db','publication_store','publication_render','publication_worker','publication_public','publication_http','publication_json']:
    source += '#[path='+json.dumps(str(ROOT/'src/server/builder'/f'{name}.rs'))+'] pub mod '+name+';\n'
source += '}\n#[cfg(test)] #[path="test.rs"] mod tests;\n#[cfg(test)] #[path="render_tests.rs"] mod render_tests;\n#[cfg(test)] #[path="json_tests.rs"] mod json_tests;\n'
(HERE/'generated.rs').write_text(source)
inputs=[ROOT/'Cargo.lock',ROOT/'Cargo.toml',ROOT/'src/server/builder/db.rs',ROOT/'src/server/builder/publication_store.rs',ROOT/'src/server/migrations/001_initial.sql',ROOT/'src/server/migrations/009_builder.sql',ROOT/'src/server/migrations/1019_site_publication_receipts.sql']
inputs += [ROOT/'src/server/lib.rs', ROOT/'src/server/builder/mod.rs', ROOT/'scripts/focused_ci_gate.py', ROOT/'scripts/test_focused_ci_gate.py', ROOT/'.github/workflows/ci.yml']
inputs += [ROOT/'src/ui/next/package.json', ROOT/'src/ui/next/package-lock.json']
inputs += list((ROOT/'src/proto').glob('*.proto'))
inputs += [ROOT/'src/server/builder/publication_render.rs', ROOT/'src/server/builder/publication_worker.rs', ROOT/'src/server/builder/publication_public.rs', ROOT/'src/server/builder/publication_http.rs', ROOT/'src/server/builder/publication_json.rs']
inputs += [p for p in HERE.iterdir() if p.name in ['Cargo.toml','prepare.py','test.rs','render_tests.rs','worker_tests.rs','public_tests.rs','http_tests.rs','json_tests.rs','raw-number-vectors.json','jcs-proof.cjs','test_mount.py','package.json','package-lock.json','verify_node_lock.py','test_node_lock.py','database_guard.py','test_database_guard.py','run.sh','verify_lock.py','README.md']]
for name in ['common','config','auth','oidc','omnisolo','telemetry']:
    inputs += [p for p in (ROOT/'src/server'/name).rglob('*') if p.is_file() and (p.suffix=='.rs' or p.name=='Cargo.toml')]
(HERE/'source-manifest.json').write_text(json.dumps({str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(set(inputs))},indent=2)+'\n')
