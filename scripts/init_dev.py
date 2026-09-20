#!/usr/bin/env python3
"""Initialize host development tools. No application services, keys, or releases.

Supported full-test hosts: Debian/Ubuntu (including WSL2), and macOS with Homebrew.
Run --plan to inspect the steps or --check for a read-only readiness check.
"""
from __future__ import annotations
import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import re
import shlex
import shutil
import subprocess
import sys
import tempfile
from urllib.request import urlopen

ROOT = Path(__file__).resolve().parents[1]
TOOLS = ROOT / 'target/dev-tools'
NPM_TREES = ('.', 'src/ui/next', 'src/cli', '.github/test-tools')
MAX_DOWNLOAD = 160 * 1024 * 1024


def load_pins(root: Path) -> dict:
    node = (root / '.node-version').read_text().strip()
    match = re.search(r'^channel\s*=\s*"([0-9]+\.[0-9]+\.[0-9]+)"', (root/'rust-toolchain.toml').read_text(), re.M)
    distribution = json.loads((root/'scripts/node-distributions.json').read_text())
    if not re.fullmatch(r'\d+\.\d+\.\d+', node) or not match or distribution['version'] != node:
        raise ValueError('Toolchain pins and verified Node digests must agree')
    return {'node': node, 'rust': match[1], 'digests': distribution['digests']}


def system_packages(system: str, distro: str) -> list[str]:
    if system == 'darwin':
        return ['pkg-config', 'openssl@3', 'python@3.12', 'make', 'bash', 'coreutils', 'libpq']
    if system != 'linux' or distro not in ('ubuntu', 'debian', 'linuxmint', 'pop'):
        raise ValueError('Automatic setup supports Debian/Ubuntu/WSL2 and macOS. Use the documented native Windows release prerequisites, or WSL2 for the full POSIX test harness.')
    return ['build-essential', 'curl', 'ca-certificates', 'git', 'pkg-config',
            'python3', 'python3-venv', 'python3-pip', 'python3-yaml', 'libssl-dev',
            'libwebkit2gtk-4.1-dev', 'libayatana-appindicator3-dev', 'librsvg2-dev',
            'patchelf', 'file', 'unzip', 'zip', 'xz-utils', 'postgresql-client', 'xvfb']


def execute(argv: list[str], *, capture=False, timeout=900, env=None) -> str:
    if not capture:
        print('+ ' + shlex.join(map(str, argv)), flush=True)
    result = subprocess.run(argv, cwd=ROOT, env=env, text=True, capture_output=capture, timeout=timeout)
    if result.returncode:
        # Avoid echoing environment values or provider credentials in captured diagnostics.
        raise RuntimeError(f'Command failed (exit {result.returncode}): {shlex.join(map(str, argv))}')
    return (result.stdout or '').strip()


def verify_download(path: Path, expected: str) -> None:
    if not re.fullmatch(r'[a-f0-9]{64}', expected) or not path.is_file() or not 0 < path.stat().st_size <= MAX_DOWNLOAD:
        raise ValueError('Invalid download or expected SHA-256')
    h = hashlib.sha256()
    with path.open('rb') as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b''): h.update(chunk)
    if h.hexdigest() != expected:
        raise ValueError('Download checksum mismatch; refusing installation')


def download(url: str, path: Path, expected: str) -> None:
    if not url.startswith(('https://nodejs.org/dist/', 'https://static.rust-lang.org/rustup/')):
        raise ValueError('Only official toolchain downloads are accepted')
    with urlopen(url, timeout=120) as response, path.open('xb') as target:
        if response.geturl() != url:
            raise ValueError('Unexpected toolchain download redirect')
        total = 0
        while chunk := response.read(1024 * 1024):
            total += len(chunk)
            if total > MAX_DOWNLOAD: raise ValueError('Toolchain download exceeds its size limit')
            target.write(chunk)
    verify_download(path, expected)


def dependency_fingerprint(tree: Path, node: str) -> str:
    h = hashlib.sha256(json.dumps([node, platform.system(), platform.machine()]).encode())
    for name in ('package.json', 'package-lock.json'):
        h.update(name.encode()); h.update((tree/name).read_bytes())
    return h.hexdigest()


def dependencies_current(tree: Path, node: str) -> bool:
    stamp = tree / 'node_modules/.ohc-init-sha256'
    return stamp.is_file() and stamp.read_text().strip() == dependency_fingerprint(tree, node)


def record_dependencies(tree: Path, node: str) -> None:
    (tree/'node_modules/.ohc-init-sha256').write_text(dependency_fingerprint(tree, node) + '\n')


def check_docker(probe) -> None:
    context = os.environ.get('DOCKER_CONTEXT')
    if context:
        info = json.loads(probe(['docker', 'context', 'inspect', context]))
        endpoint = info[0]['Endpoints']['docker']['Host']
    elif os.environ.get('DOCKER_HOST'):
        endpoint = os.environ['DOCKER_HOST']
    else:
        info = json.loads(probe(['docker', 'context', 'inspect']))
        endpoint = info[0]['Endpoints']['docker']['Host']
    if not endpoint.startswith(('unix://', 'npipe://')):
        raise ValueError('Use a local Docker context for test databases; a remote Docker endpoint is not accepted')
    probe(['docker', 'info', '--format', '{{.ServerVersion}}'])
    probe(['docker', 'compose', 'version'])
    probe(['docker', 'buildx', 'version'])


def permission(question: str, yes: bool) -> None:
    if not yes and (not sys.stdin.isatty() or input(question + ' [y/N] ').strip().lower() != 'y'):
        raise RuntimeError('System installation was not approved. Use --yes for unattended setup or --no-system after installing the listed prerequisites')


def install_system(system: str, distro: str, yes: bool) -> None:
    packages = system_packages(system, distro)
    if system == 'darwin':
        if not shutil.which('brew'):
            raise RuntimeError('Install Homebrew first, then rerun make init; its installer is not executed implicitly')
        if subprocess.run(['xcode-select', '-p'], capture_output=True).returncode:
            raise RuntimeError('Install Apple command line tools with xcode-select --install, complete its dialog, then rerun make init')
        missing = [p for p in packages if subprocess.run(['brew', 'list', '--versions', p], capture_output=True).returncode]
        if missing:
            permission('Install Homebrew prerequisites: ' + ', '.join(missing) + '?', yes)
            execute(['brew', 'install', *missing])
    else:
        missing = []
        for name in packages:
            result = subprocess.run(['dpkg-query', '-W', '-f=${Status}', name], capture_output=True, text=True)
            if result.returncode or result.stdout != 'install ok installed': missing.append(name)
        if missing:
            permission('Install system prerequisites with sudo: ' + ', '.join(missing) + '?', yes)
            execute(['sudo', 'apt-get', 'update'])
            execute(['sudo', 'apt-get', 'install', '-y', '--no-install-recommends', *missing])


def link_tool(source: Path, destination: Path) -> None:
    if not source.is_file(): raise RuntimeError(f'Missing tool: {source}')
    if destination.is_symlink(): destination.unlink()
    elif destination.exists(): raise RuntimeError(f'Refusing to replace non-symlink developer tool: {destination}')
    destination.symlink_to(source)


def install_node(pins: dict, system: str, arch: str) -> None:
    node = shutil.which('node')
    if node and execute([node, '-p', 'process.versions.node'], capture=True) == pins['node']:
        node_path = Path(node).resolve()
        npm_path = Path(shutil.which('npm') or '')
        npx_path = npm_path.resolve().parent/'npx-cli.js'
        if not npm_path.is_file() or not npx_path.is_file(): node = None
    else: node = None
    if not node:
        key = f'{system}-{arch}'
        sha = pins['digests'].get(key)
        if not sha: raise ValueError('Unsupported native Node architecture')
        name = f"node-v{pins['node']}-{key}"
        destination = TOOLS/name
        # Only trust unpacked local tools after a completed verified installation.
        if not (destination/'.verified-sha256').is_file() or (destination/'.verified-sha256').read_text().strip() != sha:
            if destination.exists(): raise RuntimeError('Incomplete local Node installation; inspect and remove only ' + str(destination))
            with tempfile.TemporaryDirectory(prefix='node-install-', dir=TOOLS) as tmp:
                temporary = Path(tmp); archive = temporary/'node.tar.gz'; unpacked = temporary/'unpacked'; unpacked.mkdir()
                download(f"https://nodejs.org/dist/v{pins['node']}/{name}.tar.gz", archive, sha)
                execute(['tar', '-xzf', str(archive), '--strip-components=1', '-C', str(unpacked)])
                (unpacked/'.verified-sha256').write_text(sha + '\n')
                unpacked.rename(destination)
        node_path = destination/'bin/node'; npm_path = destination/'bin/npm'
        npx_path = destination/'bin/npx'
    link_tool(node_path, TOOLS/'bin/node')
    link_tool(npm_path.resolve(), TOOLS/'bin/npm')
    link_tool(npx_path.resolve(), TOOLS/'bin/npx')
    if execute([str(TOOLS/'bin/node'), '-p', 'process.versions.node'], capture=True) != pins['node']:
        raise RuntimeError('Installed Node version does not match .node-version')


def install_rust(pins: dict, system: str, arch: str) -> None:
    cargo_home = Path(os.environ.get('CARGO_HOME', str(Path.home()/'.cargo')))
    rustup = shutil.which('rustup') or str(cargo_home/'bin/rustup')
    if not Path(rustup).is_file():
        target = ('aarch64' if arch == 'arm64' else 'x86_64') + ('-apple-darwin' if system == 'darwin' else '-unknown-linux-gnu')
        url = f'https://static.rust-lang.org/rustup/dist/{target}/rustup-init'
        with tempfile.TemporaryDirectory(prefix='rustup-install-', dir=TOOLS) as tmp:
            with urlopen(url + '.sha256', timeout=30) as response:
                expected = response.read(4096).decode().split()[0]
            installer = Path(tmp)/'rustup-init'; download(url, installer, expected); installer.chmod(0o700)
            execute([str(installer), '-y', '--no-modify-path', '--default-toolchain', 'none', '--profile', 'minimal'])
        rustup = str(cargo_home/'bin/rustup')
    execute([rustup, 'toolchain', 'install', pins['rust'], '--profile', 'minimal', '--component', 'rustfmt,clippy'])
    # Link proxies, not toolchain-private binaries, so rust-toolchain.toml applies.
    for name in ('cargo', 'rustc', 'rustfmt', 'cargo-fmt', 'cargo-clippy', 'clippy-driver', 'rustup'):
        link_tool(cargo_home/'bin'/name, TOOLS/'bin'/name)


def check_host(system: str, probe) -> None:
    for name in ('git', 'make', 'cc', 'pkg-config', 'python3'):
        if not shutil.which(name): raise RuntimeError('Missing host prerequisite: ' + name)
    if system == 'linux':
        for library in ('webkit2gtk-4.1', 'openssl', 'ayatana-appindicator3-0.1', 'librsvg-2.0'):
            probe(['pkg-config', '--exists', library])
    elif system == 'darwin': probe(['xcrun', '--find', 'clang'])


def select_python(probe) -> str:
    for name in ('python3.12', 'python3.11', 'python3'):
        executable = shutil.which(name)
        if not executable:
            continue
        try:
            version = json.loads(probe([executable, '-c', 'import json,sys; print(json.dumps(list(sys.version_info[:2])))']))
            if version >= [3, 11]:
                return executable
        except (ValueError, RuntimeError, OSError, subprocess.SubprocessError):
            continue
    raise RuntimeError('Python 3.11+ is required for release/test tooling. Install it with venv support, then rerun make init (Ubuntu 24.04+/Debian 12+ provide it).')


def doctor(pins: dict, system: str) -> None:
    # Even rustc --version may otherwise install a missing pinned toolchain.
    # Inspection must fail with a prerequisite message, not download software.
    environment = {**os.environ, 'RUSTUP_AUTO_INSTALL': '0', 'CARGO_NET_OFFLINE': 'true'}
    probe = lambda args: execute(args, capture=True, timeout=60, env=environment)
    check_host(system, probe)
    probe(['python3', '-c', 'import sys,tomllib; assert sys.version_info >= (3, 11), "Python 3.11+ required"'])
    if probe(['node', '-p', 'process.versions.node']) != pins['node']: raise RuntimeError('Node does not match .node-version; rerun make init')
    if not probe(['rustc', '--version']).startswith('rustc ' + pins['rust'] + ' '): raise RuntimeError('Rust version does not match rust-toolchain.toml')
    probe(['cargo', 'fmt', '--version']); probe(['cargo', 'clippy', '--version'])
    probe(['python3', '-c', 'import yaml'])
    for tree in NPM_TREES:
        if not (ROOT/tree/'node_modules').is_dir(): raise RuntimeError('Run npm ci for ' + tree)
    expected = json.loads((ROOT/'.github/test-tools/package.json').read_text())['dependencies']['opencode-ai']
    if probe(['opencode', '--version']) != expected: raise RuntimeError('Pinned OpenCode test executable unavailable')
    check_docker(probe)
    # Actually start/close the pinned browser, not just check that a filename exists.
    probe(['node', '--input-type=module', '-e', "import { chromium } from '@playwright/test'; const browser=await chromium.launch({headless:true}); await browser.close();"])
    print('Ready: pinned Rust/Node, desktop libraries, all dependency trees, OpenCode, Chromium and local Docker/Compose/Buildx. This is setup evidence, not passing application tests.')


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument('--plan', action='store_true', help='Print the setup plan without network, writes or installations')
    mode.add_argument('--check', action='store_true', help='Check readiness without installing anything')
    parser.add_argument('--no-system', action='store_true', help='Do not invoke sudo or Homebrew; system dependencies must already exist')
    parser.add_argument('--yes', action='store_true', help='Approve installation of listed system packages without prompting')
    parser.add_argument('--force', action='store_true', help='Reinstall the locked npm dependency trees even with matching stamps')
    args = parser.parse_args()
    try:
        pins = load_pins(ROOT)
        system = platform.system().lower(); arch = {'x86_64': 'x64', 'amd64': 'x64', 'aarch64': 'arm64', 'arm64': 'arm64'}.get(platform.machine().lower())
        distro = ''
        if system == 'linux' and Path('/etc/os-release').exists():
            distro = next((line[3:].strip('"') for line in Path('/etc/os-release').read_text().splitlines() if line.startswith('ID=')), '')
        if args.plan:
            print(f"make init: Rust {pins['rust']} (rustfmt/clippy), Node {pins['node']} and npm from verified official archives; no global default changes.")
            print('Host packages: ' + ', '.join(system_packages(system, distro)))
            print('Locked npm ci: ' + ', '.join(NPM_TREES) + '; includes OpenCode. Matching successful installs are reused.')
            print('Python yaml in an isolated venv; cargo fetch --locked; Playwright Chromium plus Linux dependencies.')
            print('Check local Docker daemon, Compose and Buildx; install/start Docker separately if absent. No account permissions are changed.')
            print('Release/mobile SDKs and signing credentials remain explicit platform prerequisites; make init does not create application secrets.')
            return 0
        if arch is None: raise ValueError('Unsupported host architecture')
        system_packages(system, distro)
        os.environ['PATH'] = os.pathsep.join([str(TOOLS/'bin'), str(TOOLS/'venv/bin'), str(ROOT/'.github/test-tools/node_modules/.bin'), str(Path(os.environ.get('CARGO_HOME', str(Path.home()/'.cargo')))/'bin'), os.environ.get('PATH', '')])
        if args.check:
            doctor(pins, system); return 0
        if hasattr(os, 'geteuid') and os.geteuid() == 0:
            raise RuntimeError('Run make init as your normal user, not with sudo; only package installation requests elevation')
        if not args.no_system: install_system(system, distro, args.yes)
        if system == 'darwin' and shutil.which('brew'):
            prefix = execute(['brew', '--prefix'], capture=True)
            os.environ['PATH'] = os.pathsep.join([str(Path(prefix)/'opt/python@3.12/libexec/bin'), str(Path(prefix)/'opt/libpq/bin'), str(Path(prefix)/'opt/coreutils/libexec/gnubin'), os.environ['PATH']])
            os.environ.setdefault('OPENSSL_DIR', str(Path(prefix)/'opt/openssl@3'))
        check_host(system, lambda argv: execute(argv, capture=True))
        TOOLS.mkdir(parents=True, exist_ok=True); (TOOLS/'bin').mkdir(exist_ok=True)
        lock = TOOLS/'.initializing'
        try: lock.mkdir()
        except FileExistsError: raise RuntimeError('Another init may be running; inspect target/dev-tools/.initializing before retrying')
        try:
            install_node(pins, system, arch); install_rust(pins, system, arch)
            # Isolate Python packages from OS-managed Python and other projects.
            if not (TOOLS/'venv/bin/python3').exists():
                python = select_python(lambda argv: execute(argv, capture=True))
                execute([python, '-m', 'venv', str(TOOLS/'venv')])
            execute([str(TOOLS/'venv/bin/python3'), '-m', 'pip', 'install', '--disable-pip-version-check', 'PyYAML==6.0.3'])
            for name in NPM_TREES:
                tree = ROOT/name
                if args.force or not dependencies_current(tree, pins['node']):
                    execute(['npm', '--prefix', str(tree), 'ci', '--include=dev', '--prefer-offline', '--no-audit', '--no-fund'])
                    record_dependencies(tree, pins['node'])
                else: print('Reusing verified dependency install: ' + name)
            execute(['cargo', 'fetch', '--locked'])
            browser = ['node', str(ROOT/'node_modules/@playwright/test/cli.js')]
            if system == 'linux' and not args.no_system:
                permission('Allow Playwright to install Chromium OS libraries with sudo?', args.yes)
                execute([*browser, 'install-deps', 'chromium'])
            execute([*browser, 'install', 'chromium'])
            environment = 'export PATH=' + shlex.quote(os.environ['PATH']) + '\n'
            if system == 'darwin' and os.environ.get('OPENSSL_DIR'):
                environment += 'export OPENSSL_DIR=' + shlex.quote(os.environ['OPENSSL_DIR']) + '\n'
            (TOOLS/'env.sh').write_text(environment)
            doctor(pins, system)
        finally: lock.rmdir()
        print('Next: make lint; make test. For direct cargo/npm commands: source target/dev-tools/env.sh')
        return 0
    except (RuntimeError, ValueError, OSError, KeyError, subprocess.SubprocessError) as error:
        print('make init: ' + str(error), file=sys.stderr)
        print('See docs/development/native-build.md for Docker, platform and permission prerequisites.', file=sys.stderr)
        return 1

if __name__ == '__main__': raise SystemExit(main())
