"""Compile the complete actual chat module and embed the actual SQLx source tree."""
from pathlib import Path
import hashlib,json,re
HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[1]
db=(ROOT/'src/server/db.rs').read_text()
match=re.search(r'static POSTGRES_MIGRATOR[^;]+sqlx::migrate!\("([^"]+)"\)',db)
if not match:raise SystemExit('The actual SQLx migration source is missing')
source=(ROOT/match[1]).resolve()
if source!=ROOT/'src/server/migrations':raise SystemExit('Unexpected active migration source')
if 'cargo:rerun-if-changed=src/server/migrations' not in (ROOT/'src/server/build.rs').read_text():raise SystemExit('Build does not watch the active source')
generated=f'''#[allow(dead_code)]
#[path={json.dumps(str(ROOT/'src/server/services/chat/mod.rs'))}]mod chat;
#[cfg(test)]
static MIGRATIONS:sqlx::migrate::Migrator=sqlx::migrate!("../../src/server/migrations");
#[tokio::test]
async fn actual_chat_migration_is_embedded_with_unchanged_sql_and_checksums(){{
 let disk=sqlx::migrate::Migrator::new(std::path::Path::new({json.dumps(str(source))})).await.unwrap();
 let actual:Vec<_>=MIGRATIONS.iter().collect();let expected:Vec<_>=disk.iter().collect();
 assert_eq!(actual.len(),expected.len());assert!(actual.iter().any(|migration|migration.version==1009));
 for (a,b) in actual.iter().zip(expected){{assert_eq!(a.version,b.version);assert_eq!(a.sql,b.sql);assert_eq!(a.checksum,b.checksum);}}
}}
'''
(HERE/'generated.rs').write_text(generated)
paths=['Cargo.lock','src/server/db.rs','src/server/build.rs','scripts/focused_ci_gate.py','scripts/test_focused_ci_gate.py','.github/workflows/ci.yml']
paths += [str(p.relative_to(ROOT)) for p in (ROOT/'src/server/services/chat').glob('*.rs')]
paths += [str(p.relative_to(ROOT)) for p in source.glob('*.sql')]
paths += [str(p.relative_to(ROOT)) for p in HERE.iterdir() if p.is_file() and p.name not in ['generated.rs','Cargo.lock','source-manifest.json']]
(HERE/'source-manifest.json').write_text(json.dumps({'inputs':{p:hashlib.sha256((ROOT/p).read_bytes()).hexdigest() for p in sorted(paths)},'generated_sha256':hashlib.sha256(generated.encode()).hexdigest(),'active_migration_source':str(source.relative_to(ROOT))},indent=2)+'\n')
print('Prepared actual complete chat module and active embedded migration inventory')
