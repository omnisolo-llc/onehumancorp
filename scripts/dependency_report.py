#!/usr/bin/env python3
"""Manual, source-bound dependency reporting. Findings are not a release gate."""
import argparse
from collections import Counter
from datetime import datetime, timedelta, timezone
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import time

HERE = Path(__file__).resolve().parent
POLICY = HERE / 'dependency-audit/inventory.json'
TOOL_LOCK = HERE / 'dependency-audit/requirements.lock'


def sha256(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def fingerprint(paths):
    return {str(path): sha256(path) for path in sorted(set(paths))}


def git(root, *args):
    return subprocess.check_output(['git', '-C', str(root), *args], text=True).strip()


def tracked(root):
    return git(root, 'ls-files', '-z').split('\0')


def validate_inventory(root, policy):
    paths = set(tracked(root))
    expected = {str(Path(tree) / 'package-lock.json') for tree in policy['npm']}
    expected |= {'Cargo.lock', *policy['inactive_cargo_locks']}
    expected |= {f'{tree}/Cargo.lock' for tree in policy['cargo_focused']}
    expected |= {item['lock'] for item in policy['python']}
    actual = {path for path in paths if Path(path).name in {
        'Cargo.lock', 'package-lock.json', 'requirements.lock', 'requirements_lock.txt'}}
    actual -= set(policy.get('audit_tool_locks', []))
    if actual != expected or any(not (root / path).is_file() for path in expected):
        raise ValueError(f'lock inventory mismatch: extra={sorted(actual - expected)}, '
                         f'missing={sorted(expected - actual)}')
    for path in policy['inactive_cargo_locks']:
        if (root / path).with_name('Cargo.toml').exists():
            raise ValueError(f'inactive lock gained an adjacent manifest: {path}')
    return sorted(expected)


def source_inputs(root):
    # Include graph declarations and the build/install consumers that define scope.
    return [root / path for path in tracked(root) if path and (
        Path(path).name in {'Cargo.toml', 'Cargo.lock', 'package.json', 'package-lock.json',
                           '.node-version', 'rust-toolchain.toml', 'Makefile', 'LICENSE'}
        or 'requirements' in Path(path).name
        or 'Dockerfile' in Path(path).name
        or path.startswith(('.github/workflows/', '.cargo/')))]


def normalized_name(name):
    if not isinstance(name, str) or not name:
        raise ValueError('package name must be a nonempty string')
    return re.sub(r'[-_.]+', '-', name).lower()


def declared_python(audit_python, lock):
    # Reuse the pinned requirements parser; do not implement pip syntax here.
    command = [str(audit_python), '-c',
               'import json,sys; from pip_requirements_parser import RequirementsFile; '
               'print(json.dumps(RequirementsFile.from_file(sys.argv[1], include_nested=False).to_dict()))',
               str(lock)]
    parsed = json.loads(subprocess.check_output(command, text=True, timeout=30))
    if parsed['invalid_lines'] or parsed['options']:
        raise ValueError('Python lock contains invalid lines/options or nested input; review inventory scope')
    rows = {}
    for item in parsed['requirements']:
        specifier = item['specifier']
        if (not item['is_pinned'] or len(specifier) != 1 or not specifier[0].startswith('==')
                or '*' in specifier[0]
                or any(item[key] for key in ('is_editable', 'link', 'marker', 'is_constraint',
                                            'extras', 'install_options', 'global_options', 'invalid_options'))):
            raise ValueError('Python reporting requires the inventoried unconditional exact package pins')
        name = normalized_name(item['name'])
        if name in rows:
            raise ValueError('duplicate Python lock package')
        rows[name] = specifier[0][2:]
    if not rows:
        raise ValueError('Python lock has no declared package rows')
    return rows


def classify(kind, code, stdout, stderr, expected_python=None):
    if code not in (0, 1):
        raise ValueError(f'scanner terminated with unexpected exit {code}')
    if kind == 'npm':
        data = json.loads(stdout)
        if not isinstance(data, dict) or data.get('error') or data.get('auditReportVersion') != 2:
            raise ValueError('npm did not produce a complete v2 audit report')
        total = data['metadata']['vulnerabilities']['total']
        if type(total) is not int or total < 0 or not isinstance(data['vulnerabilities'], dict):
            raise ValueError('invalid npm vulnerability summary')
        if total != len(data['vulnerabilities']):
            raise ValueError('inconsistent npm vulnerability count')
        summary = {'affected_nodes': total, 'severity': data['metadata']['vulnerabilities']}
    elif kind == 'python':
        data = json.loads(stdout)
        if not isinstance(data, dict) or not expected_python:
            raise ValueError('Python report must bind a declared package/version set')
        dependencies = data['dependencies']
        if not isinstance(dependencies, list) or not dependencies:
            raise ValueError('pip-audit did not return an audited dependency set')
        observed = {}
        for item in dependencies:
            if (not isinstance(item, dict) or item.get('skip_reason') or not item.get('name')
                    or not isinstance(item.get('version'), str) or not item['version']
                    or not isinstance(item.get('vulns'), list)
                    or any(not isinstance(vuln, dict) or not isinstance(vuln.get('id'), str)
                           or not vuln['id'] for vuln in item['vulns'])):
                raise ValueError('pip-audit skipped a package or returned incomplete findings')
            name = normalized_name(item['name'])
            if name in observed:
                raise ValueError('pip-audit returned duplicate package rows')
            observed[name] = item['version']
        if observed != expected_python:
            raise ValueError('pip-audit package/version set differs from the declared lock')
        total = sum(len(item['vulns']) for item in dependencies)
        summary = {'packages': len(dependencies), 'advisory_records': total,
                   'affected_packages': sum(bool(item['vulns']) for item in dependencies)}
    elif kind == 'cargo':
        records = [json.loads(line) for line in (stdout + '\n' + stderr).splitlines() if line.strip()]
        if any(not isinstance(item, dict) or not isinstance(item.get('fields'), dict) for item in records):
            raise ValueError('cargo-deny protocol requires objects with fields')
        summaries = [item['fields'] for item in records if item.get('type') == 'summary']
        if len(summaries) != 1 or set(summaries[0]) != {'advisories', 'sources'}:
            raise ValueError('cargo-deny completion summary is missing or invalid')
        diagnostics = [item['fields'] for item in records if item.get('type') == 'diagnostic']
        for item in records:
            if item.get('type') not in {'summary', 'diagnostic', 'log'}:
                raise ValueError('unknown cargo-deny protocol record')
            if item.get('type') == 'log' and str(item.get('fields', {}).get('level', '')).lower() in {'error', 'fatal'}:
                raise ValueError('cargo-deny reported an operational error')
        counts = summaries[0]
        for scope in counts.values():
            if not isinstance(scope, dict) or any(type(scope.get(key)) is not int or scope[key] < 0
                   for key in ('errors', 'warnings', 'notes', 'helps')):
                raise ValueError('invalid cargo-deny summary counts')
        errors = sum(scope['errors'] for scope in counts.values())
        severities = {'error': 'errors', 'warning': 'warnings', 'note': 'notes', 'help': 'helps'}
        if any(item.get('severity') not in severities for item in diagnostics):
            raise ValueError('invalid cargo-deny diagnostic severity')
        for severity, key in severities.items():
            if sum(scope[key] for scope in counts.values()) != sum(item.get('severity') == severity for item in diagnostics):
                raise ValueError('cargo-deny diagnostic/summary mismatch')
        total = len(diagnostics)
        summary = {'checks': counts, 'diagnostics': total,
                   'categories': dict(Counter(item.get('code', 'unknown') for item in diagnostics)),
                   'advisory_ids': sorted({item['advisory']['id'] for item in diagnostics if item.get('advisory')})}
        if (code == 1) != bool(errors):
            raise ValueError('cargo-deny exit and completion summary disagree')
        return ('findings' if total else 'clean'), summary
    else:
        raise ValueError(f'unknown scanner kind: {kind}')
    if (code == 1) != bool(total):
        raise ValueError('scanner exit and findings disagree')
    return ('findings' if total else 'clean'), summary


def scan(kind, command, cwd, prefix, inputs, timeout, *, expected_python=None):
    started = time.monotonic()
    result = {'kind': kind, 'command': list(command), 'cwd': str(cwd), 'state': 'operational_error'}
    stdout = stderr = ''
    try:
        inputs = list(inputs)
        if kind == 'cargo' and '--config' in command:
            config = Path(command[command.index('--config') + 1])
            inputs.append(config)
            result['configuration'] = {'path': str(config), 'sha256': sha256(config)}
        environment = os.environ.copy()
        if kind == 'npm':
            graph_keys = {'node_env', 'npm_config_omit', 'npm_config_include',
                          'npm_config_production', 'npm_config_only'}
            for key in list(environment):
                if key.lower() in graph_keys:
                    del environment[key]
            result['graph_environment'] = 'NODE_ENV and npm omit/include/production/only environment settings removed; explicit CLI scope'
        before = fingerprint(inputs)
        completed = subprocess.run(command, cwd=cwd, capture_output=True, text=True, timeout=timeout, env=environment)
        stdout, stderr = completed.stdout, completed.stderr
        result['exit_code'] = completed.returncode
        if fingerprint(inputs) != before:
            raise ValueError('source inputs changed during scan')
        result['state'], result['summary'] = classify(kind, completed.returncode, stdout, stderr, expected_python)
        if expected_python is not None:
            result['declared_python_packages'] = expected_python
    except (OSError, ValueError, KeyError, TypeError, AttributeError, IndexError, subprocess.SubprocessError) as error:
        if isinstance(error, subprocess.TimeoutExpired):
            stdout = (error.stdout or b'').decode(errors='replace')
            stderr = (error.stderr or b'').decode(errors='replace')
        result['error'] = str(error)
    result['seconds'] = round(time.monotonic() - started, 3)
    for channel, content in [('stdout', stdout), ('stderr', stderr)]:
        path = Path(str(prefix) + '.' + channel)
        path.write_text(content)
        result[channel] = {'path': str(path), 'sha256': sha256(path)}
    return result


def validate_db_time(timestamp, now):
    age = now - datetime.fromisoformat(timestamp)
    if age < timedelta(0) or age > timedelta(days=7):
        raise ValueError('RustSec database timestamp is future-dated or older than seven days')


def db_receipt(cache):
    repos = sorted(path.parent for path in cache.glob('*/.git'))
    if len(repos) != 1:
        raise ValueError('expected exactly one RustSec repository in the dedicated DB cache')
    repo = repos[0]
    if git(repo, 'remote', 'get-url', 'origin') not in {
            'https://github.com/RustSec/advisory-db', 'https://github.com/RustSec/advisory-db.git'}:
        raise ValueError('unexpected RustSec database origin')
    if git(repo, 'status', '--porcelain'):
        raise ValueError('RustSec database has local changes')
    stamp = git(repo, 'show', '-s', '--format=%cI', 'HEAD')
    validate_db_time(stamp, datetime.now(timezone.utc))
    return {'commit': git(repo, 'rev-parse', 'HEAD'), 'committed_at': stamp}


def verify_tools(args, policy):
    evidence = {}
    if 'npm' in args.ecosystem:
        version = subprocess.check_output(['node', '--version'], text=True).strip()
        if version != 'v' + (args.root / '.node-version').read_text().strip():
            raise ValueError('Node version does not match .node-version')
        evidence['node'] = version
        evidence['npm'] = subprocess.check_output(['npm', '--version'], text=True).strip()
    if 'cargo' in args.ecosystem:
        if not args.db_cache:
            raise ValueError('--db-cache must name a dedicated, refreshed RustSec cache')
        if not args.cargo_deny or sha256(args.cargo_deny) != policy['cargo_deny']['binary_sha256']:
            raise ValueError('cargo-deny binary checksum does not match the verified 0.20.2 artifact')
        version = subprocess.check_output([str(args.cargo_deny), '--version'], text=True).strip()
        if version != 'cargo-deny 0.20.2':
            raise ValueError('unexpected cargo-deny version')
        evidence['cargo_deny'] = {'version': version, 'sha256': sha256(args.cargo_deny)}
        evidence['rustsec_before'] = db_receipt(args.db_cache)
    if 'python' in args.ecosystem:
        if not args.audit_python:
            raise ValueError('--audit-python must point to the isolated hash-locked tool venv')
        normalize = lambda value: re.sub(r'[-_.]+', '-', value).lower()
        expected = {normalize(line.split('==')[0]): line.split('==')[1].split()[0]
                    for line in TOOL_LOCK.read_text().splitlines() if line and not line.startswith('#')}
        command = [str(args.audit_python), '-c', 'import importlib.metadata as m,json; '
                   'print(json.dumps({d.metadata["Name"]:d.version for d in m.distributions()}))']
        actual = {normalize(name): version for name, version in json.loads(
            subprocess.check_output(command, text=True)).items()}
        if actual != expected:
            raise ValueError('audit tool environment differs from the complete pinned tool lock')
        subprocess.run([str(args.audit_python), '-m', 'pip', 'check'], check=True, capture_output=True)
        evidence['python_tools'] = {'versions': actual, 'requirements_sha256': sha256(TOOL_LOCK),
                                    'integrity_boundary': 'wheel hashes checked by documented installation command'}
    return evidence


def plans(args, policy):
    if 'npm' in args.ecosystem:
        for tree in policy['npm']:
            for mode in ('production', 'all'):
                command = ['npm', 'audit', '--package-lock-only', '--json', '--include=optional', '--include=peer']
                if mode == 'production':
                    command.append('--omit=dev')
                else:
                    command.append('--include=dev')
                yield 'npm', f'npm-{tree}-{mode}', command, args.root / tree
    if 'python' in args.ecosystem:
        for item in policy['python']:
            command = [str(args.audit_python), '-m', 'pip_audit', '--disable-pip', '--no-deps',
                       '--strict', '--progress-spinner', 'off', '--timeout', '20', '-f', 'json',
                       '--cache-dir', str(args.output / 'pip-audit-cache'),
                       '-r', str(args.root / item['lock'])]
            if item['hashed']:
                command.append('--require-hashes')
            yield 'python', 'python-' + item['lock'], command, args.root
    if 'cargo' in args.ecosystem:
        graphs = [('root', 'Cargo.toml', policy['desktop_targets'], True),
                  ('tauri-android', 'src/ui/tauri/Cargo.toml', policy['android_targets'], False)]
        graphs += [(tree, tree + '/Cargo.toml', ['x86_64-unknown-linux-gnu'], False)
                   for tree in policy['cargo_focused']]
        for name, manifest, targets, workspace in graphs:
            config = args.output / (re.sub(r'[^a-zA-Z0-9-]', '_', name) + '.toml')
            config.write_text('[graph]\nall-features = false\ntargets = ['
                              + ', '.join('{ triple = ' + json.dumps(target) + ' }' for target in targets)
                              + ']\n[advisories]\ndb-path = ' + json.dumps(str(args.db_cache))
                              + '\ndb-urls = ["https://github.com/RustSec/advisory-db"]\n'
                              'yanked = "deny"\nunmaintained = "all"\nunsound = "all"\n'
                              'maximum-db-staleness = "P7D"\n[sources]\nunknown-registry = "deny"\n'
                              'unknown-git = "deny"\nallow-registry = ["https://github.com/rust-lang/crates.io-index"]\n')
            command = [str(args.cargo_deny), '--manifest-path', str(args.root / manifest),
                       '--config', str(config), '--locked', '--format', 'json']
            if workspace:
                command.append('--workspace')
            # Refresh the database explicitly before this command. A stable DB
            # commit must cover every graph in this report.
            command += ['--offline', 'check', 'advisories', 'sources']
            yield 'cargo', 'cargo-' + name, command, args.root


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root', type=Path, default=HERE.parent)
    parser.add_argument('--output', type=Path, required=True, help='new directory for immutable raw evidence')
    parser.add_argument('--ecosystem', choices=['npm', 'python', 'cargo'], action='append')
    parser.add_argument('--inventory-only', action='store_true')
    parser.add_argument('--audit-python', type=Path)
    parser.add_argument('--cargo-deny', type=Path)
    parser.add_argument('--db-cache', type=Path)
    parser.add_argument('--timeout', type=float, default=180)
    args = parser.parse_args()
    args.root = args.root.resolve()
    args.output = args.output.resolve()
    args.output.mkdir(parents=True, exist_ok=False)
    args.ecosystem = args.ecosystem or ['npm', 'python', 'cargo']
    policy = json.loads(POLICY.read_text())
    report = {'schema': 1, 'purpose': 'manual reporting; not release clearance',
              'started_at': datetime.now(timezone.utc).isoformat(), 'results': [], 'errors': [],
              'selected_ecosystems': args.ecosystem, 'image_boundaries': policy['image_boundaries'],
              'inactive_cargo_locks': policy['inactive_cargo_locks']}
    try:
        report['locks'] = validate_inventory(args.root, policy)
        report['commit'] = git(args.root, 'rev-parse', 'HEAD')
        report['git_status'] = git(args.root, 'status', '--porcelain')
        inputs = source_inputs(args.root) + [Path(__file__).resolve(), POLICY, TOOL_LOCK]
        report['sources'] = before = fingerprint(inputs)
        report['source_map_sha256'] = hashlib.sha256(json.dumps(before, sort_keys=True).encode()).hexdigest()
        if not args.inventory_only:
            report['tools'] = verify_tools(args, policy)
            for index, (kind, name, command, cwd) in enumerate(plans(args, policy)):
                expected_python = declared_python(args.audit_python, command[command.index('-r') + 1]) if kind == 'python' else None
                result = scan(kind, command, cwd, args.output / f'{index:02d}', inputs, args.timeout,
                              expected_python=expected_python)
                report['results'].append({'boundary': name, **result})
                print(f'{name}: {result["state"]}', flush=True)
            if 'cargo' in args.ecosystem:
                after_db = db_receipt(args.db_cache)
                if after_db != report['tools']['rustsec_before']:
                    raise ValueError('RustSec database changed during report')
        validate_inventory(args.root, policy)
        if fingerprint(source_inputs(args.root) + [Path(__file__).resolve(), POLICY, TOOL_LOCK]) != before:
            raise ValueError('source graph changed during report')
    except (OSError, ValueError, KeyError, TypeError, AttributeError, IndexError, subprocess.SubprocessError) as error:
        report['errors'].append(str(error))
    failed = bool(report['errors']) or any(r['state'] == 'operational_error' for r in report['results'])
    report['state'] = ('operational_error' if failed else 'inventory_only' if args.inventory_only
                       else 'findings' if any(r['state'] == 'findings' for r in report['results']) else 'clean')
    report['completed_at'] = datetime.now(timezone.utc).isoformat()
    (args.output / 'report.json').write_text(json.dumps(report, indent=2) + '\n')
    print(f'{report["state"]}: {args.output / "report.json"}')
    return 2 if failed else 0


if __name__ == '__main__':
    raise SystemExit(main())
