"""Compile whole production builder database/jobs modules with explicit effect boundaries."""
from pathlib import Path
import hashlib
import json

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
def module(path, name):
    return f'#[path={json.dumps(str(ROOT / path))}] pub mod {name};\n'

source = '#![allow(dead_code)]\n'
source += module('scripts/builder-publication/boundaries.rs', 'boundaries')
source += 'pub use boundaries::minimax;\npub mod builder {\n'
source += 'pub use crate::boundaries::edge;\n'
source += module('src/server/builder/db.rs', 'db')
source += 'pub mod jobs {\ninclude!(' + json.dumps(str(ROOT / 'src/server/builder/jobs.rs')) + ');\n'
source += '#[cfg(test)]' + module('scripts/builder-publication/test.rs', 'publication_tests') + '}\n}\n'
(HERE / 'generated.rs').write_text(source)
inputs = [ROOT / p for p in ['Cargo.lock', 'Cargo.toml', '.github/workflows/ci.yml', 'scripts/focused_ci_gate.py', 'scripts/test_focused_ci_gate.py', 'src/server/builder/jobs.rs', 'src/server/builder/db.rs', 'src/server/builder/edge.rs', 'src/server/migrations/009_builder.sql', 'src/server/migrations/229_quotes_and_builder_parity.sql']]
inputs += list(HERE.glob('*.py')) + [HERE / p for p in ['Cargo.toml', 'boundaries.rs', 'test.rs', 'run.sh', 'README.md']]
for part in ['common', 'config']:
    inputs += list((ROOT / 'src/server' / part).glob('*.rs')) + [ROOT / 'src/server' / part / 'Cargo.toml']
(HERE / 'source-manifest.json').write_text(json.dumps({str(p.relative_to(ROOT)): hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(set(inputs))}, indent=2) + '\n')
