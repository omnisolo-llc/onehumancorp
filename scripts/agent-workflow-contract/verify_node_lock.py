"""Require the repository's pinned transpiler and session codec dependencies."""
from pathlib import Path
import json
root=Path(__file__).resolve().parents[2]
here=Path(__file__).resolve().parent
canonical=json.loads((root/'src/ui/next/package-lock.json').read_text())['packages']
required=('jose','typescript')
expected={name:canonical[f'node_modules/{name}'] for name in required}
actual=json.loads((here/'package-lock.json').read_text())['packages']
assert set(actual)=={'',*(f'node_modules/{name}' for name in required)}
for name, package in expected.items():
    assert not actual[f'node_modules/{name}'].get('dev',False), (name,'runtime dependency marked dev-only')
    for field in ['version','resolved','integrity']:
        assert actual[f'node_modules/{name}'][field]==package[field], (name,field)
manifest=json.loads((here/'package.json').read_text())
assert manifest['dependencies']=={name:package['version'] for name,package in expected.items()}
assert actual['']['dependencies']==manifest['dependencies']
print('Workflow witness matches the repository jose and TypeScript locks')
