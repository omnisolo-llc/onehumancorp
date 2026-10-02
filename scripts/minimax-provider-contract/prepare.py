"""Bind the focused crate to complete production modules and current sources."""
from pathlib import Path
import hashlib
import json

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
(HERE / 'generated.rs').write_text(
    '#[path="../../src/server/minimax.rs"] pub mod minimax;\n'
)
inputs = [ROOT / 'Cargo.toml', ROOT / 'Cargo.lock']
inputs.extend(
    path for path in (ROOT / 'src').rglob('*')
    if path.is_file() and (path.suffix == '.rs' or path.name == 'Cargo.toml')
)
inputs.extend(
    path for path in HERE.iterdir()
    if path.is_file() and path.name not in ['generated.rs', 'source-manifest.json']
)
(HERE / 'source-manifest.json').write_text(json.dumps({
    str(path.relative_to(ROOT)): hashlib.sha256(path.read_bytes()).hexdigest()
    for path in sorted(set(inputs))
}, indent=2) + '\n')
