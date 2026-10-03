"""Reject focused registry identities that differ from the maintained root lock."""
from pathlib import Path
import tomllib
HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[1]
packages=tomllib.loads((ROOT/'Cargo.lock').read_text())['package']
root={(p['name'],p['version']):p for p in packages}
actual=tomllib.loads((HERE/'Cargo.lock').read_text())['package']
for package in actual:
    if 'source' in package:
        expected=root.get((package['name'],package['version']))
        assert expected and all(package.get(field)==expected.get(field) for field in ['source','checksum']), package['name']
print(f'{len(actual)} focused package identities match the root lock')
