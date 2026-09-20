#!/usr/bin/env python3
"""Package only the just-built Cargo runtime and required migrations."""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path
import tomllib
import shutil
import tarfile
import tempfile
import zipfile

ROOT = Path(__file__).resolve().parents[1]
from release_contract import TARGETS, archive_name, environment_identity

def package(target: str, archive: str | None = None) -> Path:
    identity = environment_identity()
    if tomllib.loads((ROOT / 'Cargo.toml').read_text())['package']['version'] != identity['version']:
        raise ValueError('Product manifests must be stamped before release compilation')
    archive = archive or archive_name(target, identity)
    if target not in TARGETS:
        raise ValueError('Unsupported native release target')
    suffix = '.zip' if 'windows' in target else '.tar.gz'
    if archive != archive_name(target, identity):
        raise ValueError('Archive name must identify its release version and Cargo target')
    binary_suffix = '.exe' if 'windows' in target else ''
    source = ROOT / 'target' / target / 'release'
    destination = ROOT / 'dist' / archive
    destination.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix='ohc-release-') as temporary:
        stage = Path(temporary) / 'omnisolo'
        (stage / 'bin').mkdir(parents=True)
        for name in ('server', 'omnisolo-builtin-agent', 'omnisolo-harness-worker'):
            binary = source / f'{name}{binary_suffix}'
            if binary.is_symlink() or not binary.is_file() or binary.stat().st_size == 0:
                raise FileNotFoundError(f'Cargo output is missing: {binary}')
            shutil.copy2(binary, stage / 'bin' / binary.name)
        for name in ('src/server/migrations', 'src/server/db/migrations'):
            shutil.copytree(ROOT / name, stage / name)
        (stage / 'README.txt').write_text(
            'OmniSolo native headless runtime\n\n'
            'Set the documented database, authentication and provider settings before starting bin/server.\n'
            'The web frontend is a separate Node service; use the desktop installer or native web image.\n'
            'No customer credentials, environment files or provider subscriptions are included.\n'
            'See README.md and docs/development/native-build.md in the source repository.\n', encoding='utf-8')
        files = {p.relative_to(stage).as_posix(): hashlib.sha256(p.read_bytes()).hexdigest()
                 for p in sorted(stage.rglob('*')) if p.is_file()}
        manifest = {'schemaVersion': 2, 'version': identity['version'], 'tag': identity['tag'], 'target': target,
                    'revision': identity['revision'],
                    'files': files}
        (stage / 'manifest.json').write_text(json.dumps(manifest, indent=2) + '\n', encoding='utf-8')
        if suffix == '.zip':
            with zipfile.ZipFile(destination, 'w', zipfile.ZIP_DEFLATED) as output:
                for p in sorted(stage.rglob('*')):
                    if p.is_file(): output.write(p, p.relative_to(stage.parent))
        else:
            with tarfile.open(destination, 'w:gz') as output:
                output.add(stage, arcname='omnisolo')
    if not destination.is_file() or destination.stat().st_size == 0:
        raise RuntimeError('Release packaging produced no artifact')
    return destination

if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--target', required=True, choices=sorted(TARGETS))
    parser.add_argument('--archive')
    arguments = parser.parse_args()
    print(package(arguments.target, arguments.archive))
