"""Bind the executed build's complete source and selected bytes to current input."""
import hashlib
import json
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]
LANE = ROOT/'scripts/redis-reconnect'


def snapshot():
    paths = json.loads((LANE/'source-inputs.json').read_text())
    paths += [str(path.relative_to(ROOT)) for path in sorted((ROOT/'src/proto').glob('*.proto'))]
    return {path: hashlib.sha256((ROOT/path).read_bytes()).hexdigest() for path in paths}


def verify(records=None):
    current = snapshot()
    if records is None:
        records = json.loads((LANE/'source-manifest.json').read_text())
    inputs = {record['source']: record['source_sha256'] for record in records if 'kind' not in record}
    if inputs != current:
        raise ValueError('Build source proof does not match the complete current inputs')
    selections = [record for record in records if 'kind' in record]
    expected = {('struct', 'Job', None), ('trait', 'TaskQueue', None),
                ('struct', 'RedisTaskQueue', None), ('impl', 'RedisTaskQueue', None),
                ('impl', 'RedisTaskQueue', 'TaskQueue')}
    if len(selections) != 5 or {(r['kind'], r['name'], r['trait']) for r in selections} != expected:
        raise ValueError('Build proof must contain all five exact queue selections')
    for record in selections:
        source = (ROOT/record['source']).read_bytes()
        if record['source'] != 'src/server/queue.rs' or record['source_sha256'] != current[record['source']]:
            raise ValueError('Selected queue source changed')
        if not 0 <= record['start'] < record['end'] <= len(source):
            raise ValueError('Invalid selection byte range')
        if hashlib.sha256(source[record['start']:record['end']]).hexdigest() != record['selection_sha256']:
            raise ValueError('Selected queue bytes changed')
    print(f'Verified {len(inputs)} source files and {len(selections)} exact queue selections')


def restore(output):
    # Cargo reports build-script outputs even when it reuses an unchanged build.
    # Identify this exact manifest through metadata; never search a shared target
    # directory for an arbitrary source-manifest.json from another checkout.
    metadata = json.loads(subprocess.check_output([
        'cargo', 'metadata', '--locked', '--offline', '--no-deps', '--format-version', '1',
        '--manifest-path', str(LANE/'Cargo.toml'),
    ], text=True))
    packages = [p['id'] for p in metadata['packages'] if Path(p['manifest_path']).resolve() == LANE/'Cargo.toml']
    if len(packages) != 1:
        raise ValueError('Cargo metadata did not identify one exact Redis manifest')
    artifacts = []
    for line in Path(output).read_text().splitlines():
        try:
            message = json.loads(line)
        except ValueError:
            continue  # Rust test output follows Cargo's compiler messages.
        if (isinstance(message, dict) and message.get('reason') == 'build-script-executed'
                and message.get('package_id') == packages[0]):
            artifacts.append(Path(message['out_dir'])/'source-manifest.json')
    if len(artifacts) != 1:
        raise ValueError('Cargo execution did not identify one exact build source proof')
    manifest = artifacts[0].read_bytes()
    verify(json.loads(manifest))
    (LANE/'source-manifest.json').write_bytes(manifest)


if __name__ == '__main__':
    if sys.argv[1:] == ['snapshot']:
        print(json.dumps(snapshot(), sort_keys=True))
    elif sys.argv[1:] == ['verify']:
        verify()
    elif len(sys.argv) == 3 and sys.argv[1] == 'restore':
        restore(sys.argv[2])
    else:
        raise SystemExit('Expected snapshot, verify, or restore <cargo-output>')
