"""Compile the complete, unchanged production VectorRepository implementation."""
import hashlib
import json
from pathlib import Path
import re

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
source = ROOT / 'src/agents/builtin/memory_store.rs'
text = source.read_text()
# The remainder of this file implements unrelated filesystem/provider memory.
# Keep every VectorRepository method; only omit the unrelated async-trait import.
repository = text[:text.index('#[async_trait]\npub trait OmniSoloMemory')]
repository = repository.removeprefix('use async_trait::async_trait;\n')
generated = repository + '\n#[cfg(test)] #[path="test.rs"] mod tests;\n'
(HERE / 'generated.rs').write_text(generated)
db = ROOT / 'src/server/db.rs'
active = re.search(r'static POSTGRES_MIGRATOR[^;]+sqlx::migrate!\("([^"]+)"\)', db.read_text())
assert active and (ROOT / active[1]).resolve() == ROOT / 'src/server/migrations'
active_path = ROOT / 'src/server/migrations/002_missing_tables.sql'
legacy_path = ROOT / 'src/server/db/migrations/039_a_consolidated_memory.sql'
for name, path in [('active', active_path), ('legacy', legacy_path)]:
    ddl = re.search(r'CREATE TABLE IF NOT EXISTS consolidated_memory \(.*?\n\);', path.read_text(), re.S)
    assert ddl, f'Missing actual {name} consolidated_memory migration'
    (HERE / f'{name}.sql').write_text(ddl.group() + '\n')
paths = [source, db, active_path, legacy_path, ROOT / 'src/agents/builtin/Cargo.toml', ROOT / 'Cargo.toml', ROOT / 'Cargo.lock',
         ROOT / '.github/workflows/ci.yml', ROOT / 'scripts/focused_ci_gate.py',
         ROOT / 'scripts/test_focused_ci_gate.py', ROOT / 'scripts/widget-chat-contract/database_guard.py']
paths += [p for p in HERE.iterdir() if p.is_file() and p.name not in
          {'generated.rs', 'active.sql', 'legacy.sql', 'source-manifest.json', 'Cargo.lock'}]
manifest = {str(p.relative_to(ROOT)): hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(paths)}
manifest['generated.rs'] = hashlib.sha256(generated.encode()).hexdigest()
(HERE / 'source-manifest.json').write_text(json.dumps(manifest, indent=2) + '\n')
print('Imported the complete production VectorRepository and actual active/legacy table migrations')
