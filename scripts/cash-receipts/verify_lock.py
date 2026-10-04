from pathlib import Path
import tomllib
root=Path(__file__).resolve().parents[2]
workspace=tomllib.loads((root/'Cargo.lock').read_text())['package']
focused=tomllib.loads((Path(__file__).resolve().parent/'Cargo.lock').read_text())['package']
keys={(p['name'],p['version'],p.get('source'),p.get('checksum')) for p in workspace}
for package in focused:
    if package['name']=='ohc-cash-receipt-regressions':continue
    identity=(package['name'],package['version'],package.get('source'),package.get('checksum'))
    if identity not in keys:raise SystemExit(f'Focused dependency differs from workspace: {identity[:2]}')
print('Cash receipt dependencies match root Cargo.lock')
