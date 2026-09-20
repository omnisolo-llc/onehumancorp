#!/usr/bin/env python3
"""Read-only verification of a complete uploaded draft before its public promotion."""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import subprocess
import sys
from release_contract import check_identity, digest, environment_identity, expected_assets


def verify_snapshot(identity: dict, directory: Path, release_id: int, release: dict, assets: list) -> None:
    check_identity(identity)
    if type(release_id) is not int or release_id <= 0 or not isinstance(release, dict):
        raise ValueError('A valid draft release ID is required')
    for key, expected in {'id': release_id, 'draft': True, 'tag_name': identity['tag'],
                          'target_commitish': identity['revision'], 'prerelease': identity['prerelease']}.items():
        if release.get(key) != expected or (isinstance(expected, bool) and type(release.get(key)) is not bool):
            raise ValueError(f'Remote draft has conflicting {key}')
    payload_names = {name for names in expected_assets(identity).values() for name in names}
    names = payload_names | {'latest.json', 'release-manifest.json', 'checksums.txt'}
    if not directory.is_dir() or {p.name for p in directory.iterdir()} != names:
        raise ValueError('Local release is incomplete or includes unexpected files')
    manifest = json.loads((directory / 'release-manifest.json').read_text(encoding='utf-8'))
    if any(manifest.get(key) != value for key, value in identity.items()) or set(manifest.get('assets', {})) != payload_names:
        raise ValueError('Local release manifest identity or asset set differs')
    latest = json.loads((directory / 'latest.json').read_text(encoding='utf-8'))
    if latest.get('version') != identity['version']:
        raise ValueError('Local updater version differs')
    local = {}
    for name in sorted(names):
        path = directory / name
        if path.is_symlink() or not path.is_file() or not path.stat().st_size:
            raise ValueError('Release includes absent, empty or symlink files')
        local[name] = {'size': path.stat().st_size, 'digest': 'sha256:' + digest(path)}
        if name in payload_names:
            row = manifest['assets'][name]
            if row.get('bytes') != local[name]['size'] or row.get('sha256') != local[name]['digest'][7:]:
                raise ValueError('Local asset no longer matches the assembled manifest')
    checksum_text = ''.join(f"{local[name]['digest'][7:]}  {name}\n" for name in sorted(names - {'checksums.txt'}))
    if (directory / 'checksums.txt').read_text(encoding='utf-8') != checksum_text:
        raise ValueError('Local checksum list no longer matches the payload')
    if not isinstance(assets, list) or len(assets) != len(names):
        raise ValueError('Remote asset inventory is incomplete or contains extras')
    seen_names = set()
    seen_ids = set()
    for asset in assets:
        if not isinstance(asset, dict): raise ValueError('Malformed remote asset')
        name, asset_id = asset.get('name'), asset.get('id')
        if name not in names or name in seen_names or type(asset_id) is not int or asset_id <= 0 or asset_id in seen_ids:
            raise ValueError('Unexpected or duplicate remote release asset')
        seen_names.add(name); seen_ids.add(asset_id)
        if (asset.get('state') != 'uploaded' or asset.get('size') != local[name]['size']
                or asset.get('digest') != local[name]['digest']):
            raise ValueError(f'Uploaded asset state, size or SHA-256 does not match: {name}')


def verify_remote(identity: dict, directory: Path, release_id: int, fetch) -> None:
    release = fetch(f'releases/{release_id}')
    assets = []
    # The complete current payload is much smaller than one page. Pagination is
    # still explicit so a future larger matrix cannot silently lose artifacts.
    for page in range(1, 6):
        batch = fetch(f'releases/{release_id}/assets?per_page=100&page={page}')
        if not isinstance(batch, list) or len(batch) > 100: raise ValueError('Malformed asset page')
        assets.extend(batch)
        if len(batch) < 100: break
    else:
        raise ValueError('Release asset pagination exceeded its bound')
    verify_snapshot(identity, directory, release_id, release, assets)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--directory', type=Path, default=Path('dist'))
    parser.add_argument('--release-id', type=int, required=True)
    options = parser.parse_args()
    try:
        identity = environment_identity()
        def fetch(route):
            completed = subprocess.run(['gh', 'api', f"repos/{identity['repository']}/{route}"],
                check=True, capture_output=True, text=True, timeout=45)
            return json.loads(completed.stdout)
        verify_remote(identity, options.directory, options.release_id, fetch)
        with open(os.environ['GITHUB_OUTPUT'], 'a', encoding='utf-8') as output:
            output.write(f'verified_release_id={options.release_id}\n')
        print('Verified complete draft asset inventory, byte sizes, SHA-256 digests and release identity.')
        return 0
    except (OSError, ValueError, KeyError, TypeError, subprocess.SubprocessError) as error:
        # Do not print captured subprocess output or tokens on API failures.
        print(f'Draft release verification failed: {error}', file=sys.stderr)
        return 1


if __name__ == '__main__':
    raise SystemExit(main())
