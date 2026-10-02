from pathlib import Path
import tomllib
root = Path(__file__).resolve().parents[2]
reference = tomllib.loads((root / 'Cargo.lock').read_text())['package']
known = {(p['name'], p['version'], p.get('source'), p.get('checksum')) for p in reference}
actual = tomllib.loads((Path(__file__).parent / 'Cargo.lock').read_text())['package']
registry = [p for p in actual if p.get('source')]
unknown = [p for p in registry if (p['name'], p['version'], p.get('source'), p.get('checksum')) not in known]
assert not unknown, f'Harness dependencies differ from the repository lock: {unknown}'
print(f'{len(registry)} registry packages match the repository lock')
