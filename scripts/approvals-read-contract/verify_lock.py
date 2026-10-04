"""The focused approvals read gate must use only the workspace's locked dependency graph."""
from pathlib import Path
import tomllib

here = Path(__file__).resolve().parent
root = here.parents[1]
workspace = tomllib.loads((root / 'Cargo.lock').read_text())['package']
focused = tomllib.loads((here / 'Cargo.lock').read_text())['package']
keys = {(p['name'], p['version'], p.get('source'), p.get('checksum')) for p in workspace}
for package in focused:
    if package['name'] == 'ohc-approvals-read-contract':
        continue
    identity = (package['name'], package['version'], package.get('source'), package.get('checksum'))
    if identity not in keys:
        raise SystemExit(f'focused dependency differs from workspace: {identity[:2]}')
print('Focused approvals read dependencies match the root Cargo.lock')
