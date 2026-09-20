#!/usr/bin/env python3
"""Apply OHC filenames to exact tauri-action outputs; never search stale bundles."""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import zipfile
from release_contract import check_identity, environment_identity, expected_assets, record_payload

TARGETS = {
    ('linux', 'x86_64'): 'x86_64-unknown-linux-gnu',
    ('linux', 'arm64'): 'aarch64-unknown-linux-gnu',
    ('windows', 'x86_64'): 'x86_64-pc-windows-msvc',
    ('macos', 'aarch64'): 'aarch64-apple-darwin',
    ('macos', 'x86_64'): 'x86_64-apple-darwin',
}


def contained(path: Path, parent: Path) -> Path:
    if path.is_symlink() or not path.resolve().is_relative_to(parent.resolve()):
        raise ValueError('Artifact must be a non-symlink inside the current build directory')
    return path.resolve()


def collect(root: Path, os_name: str, arch: str, artifact_paths: list[str], app_version: str,
            identity: dict, *, target: str = '', macos_signed: bool = False,
            run=subprocess.run, notary_keychain: str | None = None) -> None:
    check_identity(identity)
    key = (os_name, arch)
    if key not in TARGETS or (target and target != TARGETS[key]):
        raise ValueError('Unsupported or mislabeled desktop target')
    if app_version != identity['version']:
        raise ValueError('Tauri reported a different product version')
    if not isinstance(artifact_paths, list) or not artifact_paths or len(artifact_paths) > 64:
        raise ValueError('Tauri must report a bounded nonempty artifact list')
    root = root.resolve()
    build = root / 'target' / target / 'release'
    paths = []
    for raw in artifact_paths:
        if not isinstance(raw, str) or not raw or '\0' in raw:
            raise ValueError('Invalid Tauri artifact path')
        path = Path(raw)
        if not path.is_absolute(): path = root / path
        path = contained(path, build)
        if not path.exists() or (path.is_file() and path.stat().st_size == 0):
            raise ValueError('Tauri artifact is absent or empty')
        if path in paths: raise ValueError('Duplicate Tauri artifact path')
        paths.append(path)

    def one(suffix: str, directory: bool = False) -> Path:
        matches = [p for p in paths if p.name.endswith(suffix) and p.is_dir() == directory]
        if len(matches) != 1:
            raise ValueError(f'Expected exactly one reported {suffix} artifact, found {len(matches)}')
        return matches[0]

    group = f'desktop-{os_name}-{arch}'
    tag = identity['tag']
    copies = {}
    app = None
    exe = None
    runtime_files = []
    if os_name == 'linux':
        stem = f'OmniSolo-{tag}-Linux-{arch}'
        for suffix in ('.AppImage', '.AppImage.sig', '.deb', '.rpm'):
            copies[stem + suffix] = one(suffix)
        if copies[stem + '.AppImage.sig'] != Path(str(copies[stem + '.AppImage']) + '.sig'):
            raise ValueError('Updater signature is not paired with the reported AppImage')
    elif os_name == 'macos':
        stem = f'OmniSolo-{tag}-macOS-{arch}'
        app = one('.app', directory=True)
        for suffix in ('.dmg', '.tar.gz', '.tar.gz.sig'):
            copies[stem + suffix] = one(suffix)
        if copies[stem + '.tar.gz'] != Path(str(app) + '.tar.gz') or copies[stem + '.tar.gz.sig'] != Path(str(app) + '.tar.gz.sig'):
            raise ValueError('Updater archive/signature do not belong to the reported application')
        if macos_signed:
            # Tauri notarizes/staples the application before creating its updater
            # archive. Never mutate that app/tar/signature here. The separately
            # signed DMG still needs its own notarization and stapling, before
            # calculating release checksums or recording build provenance.
            dmg = copies[stem + '.dmg']
            for command in (
                ['codesign', '--verify', '--deep', '--strict', '--verbose=2', str(app)],
                ['spctl', '-a', '-t', 'exec', '-vv', str(app)],
                ['xcrun', 'stapler', 'validate', str(app)],
                ['codesign', '--verify', '--verbose=2', str(dmg)],
            ): run(command, check=True)
            submit = ['xcrun', 'notarytool', 'submit', str(dmg),
                      '--keychain-profile', 'omnisolo-release', '--wait', '--output-format', 'json']
            if notary_keychain: submit.extend(['--keychain', notary_keychain])
            result = run(submit, check=True, capture_output=True, text=True, timeout=1800)
            if json.loads(result.stdout).get('status') != 'Accepted':
                raise ValueError('Apple did not accept DMG notarization; nothing will be collected')
            run(['xcrun', 'stapler', 'staple', str(dmg)], check=True)
            run(['xcrun', 'stapler', 'validate', str(dmg)], check=True)
            run(['spctl', '-a', '-t', 'open', '--context', 'context:primary-signature', '-vv', str(dmg)], check=True)
    else:
        stem = f'OmniSolo-{tag}-Windows'
        copies[stem + '.msi'] = one('.msi')
        copies[stem + '.msi.sig'] = one('.msi.sig')
        if copies[stem + '.msi.sig'] != Path(str(copies[stem + '.msi']) + '.sig'):
            raise ValueError('Updater signature is not paired with the reported MSI')
        exe = contained(build / 'app.exe', build)
        runtime = root / 'src/ui/tauri/native-resources'
        for relative in ('bin/node.exe', 'runtime-manifest.json', 'node-distribution.json', 'NODE-LICENSE'):
            path = contained(runtime / relative, runtime)
            if not path.is_file() or not path.stat().st_size: raise ValueError('Portable runtime is incomplete')
        if not exe.is_file() or not exe.stat().st_size: raise ValueError('Portable executable is missing')
        for path in runtime.rglob('*'):
            contained(path, runtime)
            if path.is_file(): runtime_files.append((path, 'native-runtime/' + path.relative_to(runtime).as_posix()))
        if not any(name.startswith('native-runtime/web/') for _, name in runtime_files):
            raise ValueError('Portable web service is missing')

    destination = root / 'dist'
    if destination.exists() and any(destination.iterdir()):
        raise ValueError('Desktop output must be empty; do not combine stale builds')
    destination.mkdir(parents=True, exist_ok=True)
    for name, path in copies.items(): shutil.copy2(path, destination / name)
    if app:
        run(['ditto', '-c', '-k', '--sequesterRsrc', '--keepParent', str(app), str(destination / (stem + '.zip'))], check=True)
    if exe:
        with zipfile.ZipFile(destination / (stem + '-Portable.zip'), 'w', zipfile.ZIP_DEFLATED) as archive:
            archive.write(exe, 'OmniSolo.exe')
            for path, name in sorted(runtime_files): archive.write(path, name)
            archive.writestr('portable.ini', '# OmniSolo portable build marker\nportable=true\n')
    if {p.name for p in destination.iterdir()} != set(expected_assets(identity)[group]):
        raise ValueError('Collected desktop packages do not match the complete release contract')
    record_payload(destination, group, identity)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--os', required=True, choices=('linux', 'macos', 'windows'))
    parser.add_argument('--arch', required=True)
    options = parser.parse_args()
    try:
        collect(Path(__file__).resolve().parents[1], options.os, options.arch,
            json.loads(os.environ['TAURI_ARTIFACT_PATHS']), os.environ['TAURI_APP_VERSION'], environment_identity(),
            target=os.environ.get('TAURI_BUILD_TARGET', ''), macos_signed=os.environ.get('MACOS_SIGNING_ENABLED') == 'true',
            notary_keychain=os.environ.get('OMNISOLO_NOTARY_KEYCHAIN'))
        return 0
    except (OSError, ValueError, KeyError, TypeError, subprocess.SubprocessError) as error:
        print(f'Desktop artifact verification failed: {error}', file=sys.stderr)
        return 1


if __name__ == '__main__':
    raise SystemExit(main())
