"""Compile the actual server_auth library/tests in a bounded isolated workspace."""
import hashlib
import json
from pathlib import Path
import tomllib
import os

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
BUILD = HERE / '.build'
BUILD.mkdir(exist_ok=True)
# sqlx::migrate! in the actual auth test helper is manifest-relative.
migrations = HERE / 'migrations'
if migrations.is_symlink():
    assert migrations.resolve() == ROOT / 'src/server/migrations'
elif migrations.exists():
    raise SystemExit('Unexpected registration harness migration path')
else:
    migrations.symlink_to(os.path.relpath(ROOT / 'src/server/migrations', HERE), target_is_directory=True)
source = ROOT / 'src/server/auth/Cargo.toml'
manifest = tomllib.loads(source.read_text())
workspace = tomllib.loads((ROOT / 'Cargo.toml').read_text())['workspace']['dependencies']


def value(item):
    if isinstance(item, dict):
        return '{ ' + ', '.join(f'{key} = {value(val)}' for key, val in item.items()) + ' }'
    return json.dumps(item)


lines = ['[package]', 'name = "ohc-registration-proof-contract"', 'version = "0.1.0"',
         'edition = "2024"', '[workspace]', '[lib]', 'name = "server_auth"',
         'path = "../../../src/server/auth/mod.rs"']
for section in ['dependencies', 'dev-dependencies']:
    lines.append(f'[{section}]')
    for name, original in manifest.get(section, {}).items():
        spec = dict(original) if isinstance(original, dict) else original
        if isinstance(spec, dict) and spec.pop('workspace', False):
            base = workspace[name]
            base = dict(base) if isinstance(base, dict) else {'version': base}
            for key, item in spec.items():
                if key == 'features':
                    base[key] = sorted(set(base.get(key, []) + item))
                else:
                    base[key] = item
            spec = base
        if isinstance(spec, dict) and 'path' in spec:
            dependency = (source.parent / spec['path']).resolve()
            spec['path'] = os.path.relpath(dependency, BUILD)
        lines.append(f'{name} = {value(spec)}')
lines += ['[[test]]', 'name = "portable_migration"', 'path = "../migration_test.rs"']
lines += ['[profile.dev]', 'debug = 0', 'incremental = false']
(BUILD / 'Cargo.toml').write_text('\n'.join(lines) + '\n')
inputs = [ROOT / 'Cargo.toml', ROOT / 'Cargo.lock', ROOT / 'src/server/persistence/migration.rs',
          ROOT / 'src/server/persistence/commands.rs', ROOT / 'src/server/lib.rs',
          ROOT / 'src/ui/next/src/lib/auth/authLimits.json',
          HERE / 'prepare.py', HERE / 'run.sh', HERE / 'README.md', HERE / 'migration_test.rs']
inputs += [p for p in (ROOT / 'src/server/persistence').rglob('*') if p.is_file() and p.suffix in {'.rs', '.sql'}]
for name in ['auth', 'common', 'config', 'oidc', 'omnisolo', 'telemetry']:
    inputs += [p for p in (ROOT / 'src/server' / name).rglob('*')
               if p.is_file() and (p.suffix == '.rs' or p.name == 'Cargo.toml')]
inputs += list((ROOT / 'src/proto').glob('*.proto'))
inputs += list((ROOT / 'src/server/migrations').glob('*.sql'))
(HERE / 'source-manifest.json').write_text(json.dumps({
    str(p.relative_to(ROOT)): hashlib.sha256(p.read_bytes()).hexdigest()
    for p in sorted(set(inputs))
}, indent=2) + '\n')
print('Prepared the actual server_auth library and tests without a replacement transaction')
