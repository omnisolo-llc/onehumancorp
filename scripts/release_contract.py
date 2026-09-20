#!/usr/bin/env python3
"""One release identity, locked version stamping, and complete asset assembly.

prepare changes manifests only in disposable release checkouts. No command in
this script creates tags, publishes, reads signing keys, or upgrades dependencies.
"""
from __future__ import annotations
import argparse
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tomllib
from urllib.parse import quote
from urllib.error import HTTPError
from urllib.request import Request, urlopen

ROOT = Path(__file__).resolve().parents[1]
TARGETS = (
    'x86_64-unknown-linux-gnu', 'aarch64-unknown-linux-gnu',
    'x86_64-pc-windows-gnu', 'x86_64-pc-windows-msvc', 'aarch64-pc-windows-msvc',
    'x86_64-apple-darwin', 'aarch64-apple-darwin',
)
PRODUCT_CRATES = {
    'Cargo.toml': 'omnisolo',
    'src/agents/builtin/Cargo.toml': 'omnisolo_builtin_agent',
    'src/server/harness_worker/Cargo.toml': 'omnisolo_harness_worker',
    'src/ui/tauri/Cargo.toml': 'app',
}
NODE_PACKAGES = ('package.json', 'src/ui/next/package.json', 'src/cli/package.json')
SEMVER = re.compile(r'(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?(?:\+([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?\Z')


def validate_version(version: str) -> re.Match:
    match = SEMVER.fullmatch(version)
    if not match or len(version) > 100 or any(item.isdigit() and len(item) > 1 and item[0] == '0'
                        for item in (match[4] or '').split('.')):
        raise ValueError('Release version must be canonical SemVer')
    major, minor, patch = map(int, match.group(1, 2, 3))
    if major > 255 or minor > 255 or patch > 65535:
        raise ValueError('Release version exceeds the Windows MSI version range')
    return match


def resolve_identity(*, ref: str, event: str, run_number: int, attempt: int,
                     revision: str, repository: str, base_version: str,
                     publish_requested: bool = False) -> dict:
    if event not in ('push', 'workflow_dispatch', 'schedule'):
        raise ValueError('Unsupported release event')
    if not re.fullmatch(r'[0-9a-f]{40}', revision):
        raise ValueError('An exact checked-out commit is required')
    if not re.fullmatch(r'[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+', repository):
        raise ValueError('Invalid GitHub repository')
    if not 1 <= run_number <= 2100000000 or not 1 <= attempt <= 65535:
        raise ValueError('Invalid release run/build number')
    tagged = ref.startswith('refs/tags/')
    if tagged:
        if not ref.startswith('refs/tags/v'):
            raise ValueError('Release tags must start with v')
        version = ref.removeprefix('refs/tags/v')
    else:
        if event == 'push' or not ref.startswith('refs/heads/'):
            raise ValueError('A branch release must be a manual or nightly run')
        base = validate_version(base_version)
        version = '.'.join(base.group(1, 2, 3)) + f'-nightly.{run_number}.{attempt}'
    parsed = validate_version(version)
    prerelease = parsed[4] is not None
    if prerelease and run_number > 65535:
        raise ValueError('Prerelease build number exceeds the MSI fourth-component limit')
    return {
        'schema_version': 1, 'version': version, 'tag': 'v' + version,
        'image_tag': 'v' + version.replace('+', '_'),
        'revision': revision, 'repository': repository,
        'run_number': run_number, 'attempt': attempt,
        'prerelease': prerelease,
        'publish': (event == 'push' and tagged) or event == 'schedule' or publish_requested,
        'wix_version': '.'.join(parsed.group(1, 2, 3)) + f'.{run_number if prerelease else 0}',
        'android_version_code': run_number,
    }


def check_identity(identity: dict) -> None:
    version = identity.get('version', '')
    parsed = validate_version(version)
    if (identity.get('schema_version') != 1 or identity.get('tag') != 'v' + version
            or not re.fullmatch(r'[0-9a-f]{40}', identity.get('revision', ''))
            or not re.fullmatch(r'[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+', identity.get('repository', ''))
            or identity.get('prerelease') != (parsed[4] is not None)):
        raise ValueError('Invalid or conflicting release identity')
    number = identity.get('android_version_code')
    if type(number) is not int or not 1 <= number <= 2100000000:
        raise ValueError('Invalid Android version code')
    run = identity.get('run_number')
    attempt = identity.get('attempt')
    if (type(run) is not int or run != number or type(attempt) is not int
            or not 1 <= attempt <= 65535 or type(identity.get('publish')) is not bool):
        raise ValueError('Invalid release run identity')
    fourth = run if identity['prerelease'] else 0
    if fourth > 65535 or identity.get('wix_version') != '.'.join(parsed.group(1, 2, 3)) + f'.{fourth}':
        raise ValueError('Conflicting MSI version')
    if identity.get('image_tag') != identity['tag'].replace('+', '_'):
        raise ValueError('Conflicting container tag')


def replace_version(section: str, version: str) -> str:
    updated, count = re.subn(r'^version\s*=\s*"[^"\n]+"', 'version = ' + json.dumps(version),
                            section, flags=re.M)
    if count != 1:
        raise ValueError('Expected exactly one package version')
    return updated


def prepare(root: Path, identity: dict) -> None:
    check_identity(identity)
    version = identity['version']
    changes = {}
    old_versions = {}
    for filename, expected_name in PRODUCT_CRATES.items():
        path = root / filename
        source = path.read_text(encoding='utf-8')
        package = tomllib.loads(source)['package']
        if package['name'] != expected_name:
            raise ValueError(f'Unexpected release package: {filename}')
        old_versions[expected_name] = package['version']
        sections = re.split(r'(?m)(?=^\[)', source)
        matches = [i for i, part in enumerate(sections) if part.startswith('[package]\n')]
        if len(matches) != 1:
            raise ValueError(f'Expected one package table in {filename}')
        index = matches[0]
        sections[index] = replace_version(sections[index], version)
        changes[path] = ''.join(sections)
    lock_path = root / 'Cargo.lock'
    sections = re.split(r'(?m)(?=^\[\[package\]\])', lock_path.read_text(encoding='utf-8'))
    seen = set()
    for i, section in enumerate(sections):
        if not section.startswith('[[package]]'):
            continue
        package = tomllib.loads(section)['package'][0]
        name = package['name']
        if name in old_versions and 'source' not in package:
            if name in seen or package['version'] != old_versions[name]:
                raise ValueError('Inconsistent local package version in Cargo.lock')
            seen.add(name)
            sections[i] = replace_version(section, version)
    if seen != set(old_versions):
        raise ValueError('Cargo.lock is missing a shipped local package')
    lock = ''.join(sections)
    for name, old in old_versions.items():
        lock = lock.replace(json.dumps(f'{name} {old}'), json.dumps(f'{name} {version}'))
    tomllib.loads(lock)
    changes[lock_path] = lock
    for filename in NODE_PACKAGES:
        path = root / filename
        package = json.loads(path.read_text(encoding='utf-8'))
        package['version'] = version
        changes[path] = json.dumps(package, indent=2) + '\n'
        path = path.with_name('package-lock.json')
        lockfile = json.loads(path.read_text(encoding='utf-8'))
        lockfile['version'] = version
        lockfile['packages']['']['version'] = version
        changes[path] = json.dumps(lockfile, indent=2) + '\n'
    path = root / 'src/ui/tauri/tauri.conf.json'
    config = json.loads(path.read_text(encoding='utf-8'))
    config['version'] = version
    bundle = config.setdefault('bundle', {})
    bundle.setdefault('windows', {}).setdefault('wix', {})['version'] = identity['wix_version']
    bundle.setdefault('android', {})['versionCode'] = identity['android_version_code']
    bundle['android']['autoIncrementVersionCode'] = False
    updater = config.setdefault('plugins', {}).setdefault('updater', {})
    updater['endpoints'] = [f"https://github.com/{identity['repository']}/releases/latest/download/latest.json"]
    changes[path] = json.dumps(config, indent=2) + '\n'
    # Validate every input before writing. External dependency resolutions and
    # checksums remain unchanged; subsequent Cargo invocations remain --locked.
    for path, content in changes.items():
        path.write_text(content, encoding='utf-8')
    target = root / 'target/release-identity.json'
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text(json.dumps(identity, indent=2) + '\n', encoding='utf-8')


def archive_name(target: str, identity: dict) -> str:
    if target not in TARGETS:
        raise ValueError('Unsupported native release target')
    return f"omnisolo-{identity['tag']}-{target}" + ('.zip' if 'windows' in target else '.tar.gz')


def expected_assets(identity: dict) -> dict[str, list[str]]:
    check_identity(identity)
    tag = identity['tag']
    result = {f'backend-{target}': [archive_name(target, identity)] for target in TARGETS}
    for arch in ('aarch64', 'x86_64'):
        stem = f'OmniSolo-{tag}-macOS-{arch}'
        result['desktop-macos-' + arch] = [stem + suffix for suffix in ('.dmg', '.zip', '.tar.gz', '.tar.gz.sig')]
    for arch in ('x86_64', 'arm64'):
        stem = f'OmniSolo-{tag}-Linux-{arch}'
        result['desktop-linux-' + arch] = [stem + suffix for suffix in ('.AppImage', '.AppImage.sig', '.deb', '.rpm')]
    result['desktop-windows-x86_64'] = [f'OmniSolo-{tag}-Windows{suffix}'
                                      for suffix in ('.msi', '.msi.sig', '-Portable.zip')]
    result['mobile-android'] = [f'OmniSolo-{tag}-Android-universal{suffix}' for suffix in ('.apk', '.aab')]
    result['server-web-image'] = [f'omnisolo-server-web-{tag}-linux-amd64.tar']
    return result


def digest(path: Path) -> str:
    with path.open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()


def record_payload(directory: Path, group: str, identity: dict) -> None:
    expected = expected_assets(identity)
    if group not in expected:
        raise ValueError('Unexpected release artifact group')
    rows = {}
    for name in expected[group]:
        path = directory / name
        if path.is_symlink() or not path.is_file() or path.stat().st_size == 0:
            raise ValueError(f'Missing or empty release artifact: {name}')
        rows[name] = {'sha256': digest(path), 'bytes': path.stat().st_size}
    value = {**identity, 'group': group, 'files': rows}
    (directory / f'release-payload-{group}.json').write_text(json.dumps(value, indent=2) + '\n', encoding='utf-8')


def assemble(incoming: Path, output: Path, identity: dict) -> None:
    expected = expected_assets(identity)
    manifests = sorted(incoming.rglob('release-payload-*.json'))
    seen = set()
    payloads = {}
    verified_paths = set()
    for path in manifests:
        if path.is_symlink():
            raise ValueError('Symlink release manifest rejected')
        value = json.loads(path.read_text(encoding='utf-8'))
        group = value.get('group')
        if group not in expected or group in seen:
            raise ValueError('Unknown or duplicated artifact group')
        if any(value.get(key) != item for key, item in identity.items()):
            raise ValueError('Release artifact has a different version, source, or run identity')
        if set(value.get('files', {})) != set(expected[group]):
            raise ValueError('Incomplete release artifact group')
        seen.add(group)
        verified_paths.add(path)
        for name, description in value['files'].items():
            binary = path.parent / name
            if (binary.is_symlink() or not binary.is_file() or binary.stat().st_size == 0
                    or binary.stat().st_size != description['bytes'] or digest(binary) != description['sha256']):
                raise ValueError(f'Release checksum/size mismatch: {name}')
            if name in payloads:
                raise ValueError('Release filename collision')
            payloads[name] = (binary, description)
            verified_paths.add(binary)
    if seen != set(expected):
        raise ValueError('Missing release groups: ' + ', '.join(sorted(set(expected) - seen)))
    extras = [p for p in incoming.rglob('*') if (p.is_file() or p.is_symlink()) and p not in verified_paths]
    if extras:
        raise ValueError('Unreviewed files in release artifact input')
    if output.exists() and any(output.iterdir()):
        raise ValueError('Release output must be empty; never merge stale assets')
    output.mkdir(parents=True, exist_ok=True)
    for name, (source, _) in payloads.items():
        shutil.copy2(source, output / name)
    tag = identity['tag']
    updater_files = {
        'darwin-aarch64': f'OmniSolo-{tag}-macOS-aarch64.tar.gz',
        'darwin-x86_64': f'OmniSolo-{tag}-macOS-x86_64.tar.gz',
        'windows-x86_64': f'OmniSolo-{tag}-Windows.msi',
        'linux-x86_64': f'OmniSolo-{tag}-Linux-x86_64.AppImage',
        'linux-aarch64': f'OmniSolo-{tag}-Linux-arm64.AppImage',
    }
    platforms = {}
    for platform, name in updater_files.items():
        signature = (output / (name + '.sig')).read_text(encoding='utf-8').strip()
        if not signature:
            raise ValueError('Updater signature is empty')
        platforms[platform] = {'signature': signature,
            'url': f"https://github.com/{identity['repository']}/releases/download/{quote(tag, safe='')}/{quote(name, safe='')}"}
    latest = {'version': identity['version'], 'notes': f'Release {tag}',
        'pub_date': datetime.now(timezone.utc).isoformat().replace('+00:00', 'Z'), 'platforms': platforms}
    (output / 'latest.json').write_text(json.dumps(latest, indent=2) + '\n', encoding='utf-8')
    manifest = {**identity, 'assets': {name: row for name, (_, row) in sorted(payloads.items())}}
    (output / 'release-manifest.json').write_text(json.dumps(manifest, indent=2) + '\n', encoding='utf-8')
    checksums = ''.join(f'{digest(path)}  {path.name}\n' for path in sorted(output.iterdir()) if path.is_file())
    (output / 'checksums.txt').write_text(checksums, encoding='utf-8')


def check_publication(identity: dict, fetch_json) -> None:
    """Read-only preflight: never overwrite releases or move an existing tag."""
    check_identity(identity)
    tag = quote(identity['tag'], safe='')
    if fetch_json(f'releases/tags/{tag}') is not None:
        raise ValueError('A release already exists for this tag; use a new version or explicitly recover the existing draft')
    reference = fetch_json(f'git/ref/tags/{tag}')
    if reference is None:
        return  # A new nightly tag will be created at target_commitish.
    obj = reference.get('object', {})
    for _ in range(8):
        sha = obj.get('sha', '')
        if not re.fullmatch(r'[0-9a-f]{40}', sha):
            raise ValueError('Invalid remote tag object')
        if obj.get('type') == 'commit':
            if sha != identity['revision']:
                raise ValueError('Remote release tag does not match the built commit')
            return
        if obj.get('type') != 'tag':
            raise ValueError('Release tag does not reference a commit')
        annotated = fetch_json(f'git/tags/{sha}')
        if annotated is None:
            raise ValueError('Annotated release tag disappeared')
        obj = annotated.get('object', {})
    raise ValueError('Excessively nested annotated release tag')


def github_reader(identity: dict):
    def fetch(route: str):
        request = Request(f"https://api.github.com/repos/{identity['repository']}/{route}", headers={
            'Accept': 'application/vnd.github+json',
            'Authorization': 'Bearer ' + os.environ['GH_TOKEN'],
            'User-Agent': 'omnisolo-release-contract',
        })
        try:
            with urlopen(request, timeout=30) as response:
                value = json.load(response)
        except HTTPError as error:
            if error.code == 404:
                return None
            raise
        if not isinstance(value, dict):
            raise ValueError('Malformed GitHub release response')
        return value
    return fetch


def environment_identity() -> dict:
    identity = json.loads(os.environ['RELEASE_IDENTITY'])
    check_identity(identity)
    revision = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip()
    if revision != identity['revision']:
        raise ValueError('Release metadata is not for this checkout')
    return identity


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('action', choices=('resolve', 'prepare', 'record', 'assemble', 'archive-name', 'check-publication'))
    parser.add_argument('--group')
    parser.add_argument('--target')
    parser.add_argument('--directory', type=Path, default=Path('dist'))
    parser.add_argument('--incoming', type=Path, default=Path('incoming'))
    args = parser.parse_args()
    try:
        if args.action == 'resolve':
            revision = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip()
            identity = resolve_identity(ref=os.environ['GITHUB_REF'], event=os.environ['GITHUB_EVENT_NAME'],
                run_number=int(os.environ['GITHUB_RUN_NUMBER']), attempt=int(os.environ['GITHUB_RUN_ATTEMPT']),
                revision=revision, repository=os.environ['GITHUB_REPOSITORY'],
                base_version=tomllib.loads((ROOT / 'Cargo.toml').read_text())['package']['version'],
                publish_requested=os.environ.get('PUBLISH_REQUESTED') == 'true')
            if identity['publish'] and not os.environ['GITHUB_REF'].startswith('refs/tags/'):
                if os.environ['GITHUB_REF'] != 'refs/heads/' + os.environ.get('DEFAULT_BRANCH', 'main'):
                    raise ValueError('Only the default branch may publish a non-tagged prerelease')
            with open(os.environ['GITHUB_OUTPUT'], 'a', encoding='utf-8') as output:
                for key in ('version', 'tag', 'image_tag', 'revision', 'prerelease', 'publish'):
                    value = identity[key]
                    output.write(f'{key}={str(value).lower() if isinstance(value, bool) else value}\n')
                output.write('identity=' + json.dumps(identity, separators=(',', ':')) + '\n')
        else:
            identity = environment_identity()
            if args.action == 'prepare':
                prepare(ROOT, identity)
                with open(os.environ['GITHUB_ENV'], 'a', encoding='utf-8') as output:
                    output.write(f"OMNISOLO_RELEASE_VERSION={identity['version']}\nOMNISOLO_RELEASE_TAG={identity['tag']}\n")
                    output.write('RELEASE_IDENTITY=' + json.dumps(identity, separators=(',', ':')) + '\n')
            elif args.action == 'check-publication':
                check_publication(identity, github_reader(identity))
            elif args.action == 'record':
                record_payload(args.directory, args.group, identity)
            elif args.action == 'assemble':
                assemble(args.incoming, args.directory, identity)
            else:
                print(archive_name(args.target, identity))
        return 0
    except (ValueError, KeyError, TypeError, OSError, subprocess.CalledProcessError) as error:
        print(f'Release contract failed: {error}', file=sys.stderr)
        return 1


if __name__ == '__main__':
    raise SystemExit(main())
