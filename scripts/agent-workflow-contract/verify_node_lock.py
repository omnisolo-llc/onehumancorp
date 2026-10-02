"""Require exactly the repository's pinned TypeScript transpiler, no extra packages."""
from pathlib import Path
import json
root=Path(__file__).resolve().parents[2]
here=Path(__file__).resolve().parent
expected=json.loads((root/'src/ui/next/package-lock.json').read_text())['packages']['node_modules/typescript']
actual=json.loads((here/'package-lock.json').read_text())['packages']
assert set(actual)=={'','node_modules/typescript'}
for field in ['version','resolved','integrity']:
    assert actual['node_modules/typescript'][field]==expected[field], field
manifest=json.loads((here/'package.json').read_text())
assert manifest['dependencies']=={'typescript':expected['version']}
assert actual['']['dependencies']==manifest['dependencies']
print('Workflow witness matches the repository TypeScript lock')
