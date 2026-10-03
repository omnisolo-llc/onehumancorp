"""Reject focused-harness dependency drift from the repository lockfile."""
from pathlib import Path
import tomllib
here=Path(__file__).resolve().parent
root=here.parents[1]
repository=tomllib.loads((root/'Cargo.lock').read_text())
harness=tomllib.loads((here/'Cargo.lock').read_text())
allowed={(p['name'],p['version'],p.get('source'),p.get('checksum')) for p in repository['package'] if p.get('source')}
extra=[(p['name'],p['version']) for p in harness['package'] if p.get('source') and (p['name'],p['version'],p.get('source'),p.get('checksum')) not in allowed]
if extra: raise SystemExit(f'Harness dependency drift: {extra}')
print('Focused harness registry dependencies match repository Cargo.lock')
