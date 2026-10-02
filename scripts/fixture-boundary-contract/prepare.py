"""Import the actual production fixture-path middleware without replacing it."""
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
helper = ROOT / 'src/server/api/fixture_boundary.rs'
source = f'#[path={json.dumps(str(helper))}] pub mod fixture_boundary;\n'
(HERE / 'generated.rs').write_text(source)
inputs = [helper, ROOT / 'src/server/api/agents/approvals.rs', ROOT / 'src/server/lib.rs',
          ROOT / 'src/server/api/growth.rs', ROOT / 'Cargo.lock', HERE / 'Cargo.toml',
          HERE / 'prepare.py', HERE / 'run.sh', HERE / 'verify_lock.py']
(HERE / 'source-manifest.json').write_text(json.dumps({
    str(path.relative_to(ROOT)): hashlib.sha256(path.read_bytes()).hexdigest()
    for path in inputs
}, indent=2) + '\n')
