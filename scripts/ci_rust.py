#!/usr/bin/env python3
"""Compile once; verify Cargo/nextest selection and exact-source shard evidence.

This does not replace doctests or the canonical local `make test` command.
Only the standard libtest targets validated against Cargo enter the archive.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import re
import subprocess
import sys
import xml.etree.ElementTree as ET

NEXTEST_VERSION = '0.9.145'
ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / 'target/ci-rust'


def digest(path):
    with Path(path).open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()


def key(binary, name):
    return json.dumps([binary, name], separators=(',', ':'), ensure_ascii=False)


def unique(values, label):
    if not isinstance(values, list) or any(not isinstance(v, str) or not v for v in values):
        raise ValueError(f'invalid {label}')
    if len(set(values)) != len(values):
        raise ValueError(f'duplicate {label}')
    return set(values)


def inventory(listing, *, partition=False):
    active, ignored, count = [], [], 0
    suites = listing.get('rust-suites')
    if not isinstance(suites, dict) or not suites:
        raise ValueError('missing Rust suites')
    for binary, suite in suites.items():
        if suite.get('status') != 'listed' or suite.get('binary-id') != binary:
            raise ValueError(f'incomplete binary listing: {binary}')
        cases = suite.get('testcases')
        if not isinstance(cases, dict):
            raise ValueError('missing testcases')
        for name, case in cases.items():
            count += 1
            if not isinstance(case.get('ignored'), bool):
                raise ValueError('missing ignore status')
            match = case.get('filter-match', {}).get('status')
            if match != 'matches':
                if partition and match == 'mismatch':
                    continue
                raise ValueError(f'filtered test in complete inventory: {binary}::{name}')
            (ignored if case['ignored'] else active).append(key(binary, name))
    if count != listing.get('test-count'):
        raise ValueError('inconsistent nextest count')
    unique(active + ignored, 'test IDs')
    if not partition and not active:
        raise ValueError('zero executable tests')
    return sorted(active), sorted(ignored)


def libtest_names(text):
    names = []
    for line in text.splitlines():
        if line.endswith(': test'):
            names.append(line[:-6])
        elif line.endswith(': benchmark'):
            raise ValueError('benchmark requires a retained native benchmark lane')
        elif line.strip() and not re.fullmatch(r'\d+ tests?, \d+ benchmarks?', line):
            raise ValueError('unsupported/custom libtest listing')
    counts = re.findall(r'^(\d+) tests?, (\d+) benchmarks?$', text, re.M)
    if len(counts) != 1 or int(counts[0][0]) != len(names) or int(counts[0][1]):
        raise ValueError('incomplete libtest listing')
    return unique(names, 'libtest names')


def junit_results(text):
    if '<!DOCTYPE' in text.upper() or '<!ENTITY' in text.upper():
        raise ValueError('XML declarations are not permitted')
    root = ET.fromstring(text)
    if root.tag != 'testsuites':
        raise ValueError('expected testsuites root')
    results = {}
    for suite in root:
        if suite.tag != 'testsuite' or not suite.get('name'):
            raise ValueError('missing test suite identity')
        for case in suite.findall('testcase'):
            if not case.get('name'):
                raise ValueError('missing test identity')
            ident = key(suite.attrib['name'], case.attrib['name'])
            if ident in results:
                raise ValueError('duplicate test result')
            failures = {'failure', 'error', 'skipped', 'flakyFailure', 'flakyError', 'rerunFailure', 'rerunError'}
            results[ident] = 'failed' if any(child.tag in failures for child in case) else 'passed'
    return results


def verify_reports(manifest, reports, identity, total):
    if manifest.get('identity') != identity or not 1 <= total <= 60:
        raise ValueError('wrong qualification identity or shard count')
    expected = unique(manifest['tests'], 'expected tests')
    ignored = unique(manifest['ignored'], 'ignored tests')
    if not expected or expected & ignored or len(reports) != total:
        raise ValueError('missing shards or invalid complete selection')
    seen, indexes = set(), set()
    for report in reports:
        if report.get('identity') != identity or report.get('total') != total or report.get('complete') is not True:
            raise ValueError('incomplete or wrong-source shard')
        index = report.get('index')
        if not isinstance(index, int) or not 1 <= index <= total or index in indexes:
            raise ValueError('invalid or duplicate shard index')
        indexes.add(index)
        selected = unique(report['selected'], 'selected tests')
        results = report.get('results', {})
        if set(results) != selected or not selected <= expected or seen & selected:
            raise ValueError('missing, extra or duplicate results')
        if any(value != 'passed' for value in results.values()):
            raise ValueError('failed, skipped or nonterminal test')
        seen |= selected
    if seen != expected:
        raise ValueError(f'incomplete execution: missing {len(expected - seen)} tests')
    return len(seen)


def check_archive(manifest, archive, identity):
    if manifest.get('identity') != identity or manifest.get('archive_sha256') != digest(archive):
        raise ValueError('archive is corrupt or belongs to another source/run/attempt/platform')


def capture(args, **kwargs):
    return subprocess.check_output(args, cwd=ROOT, text=True, **kwargs)


def identity():
    run, attempt = os.environ.get('GITHUB_RUN_ID', ''), os.environ.get('GITHUB_RUN_ATTEMPT', '')
    sha = capture(['git', 'rev-parse', 'HEAD']).strip()
    if not re.fullmatch(r'[a-f0-9]{40}', sha) or not run.isdigit() or not attempt.isdigit() or int(attempt) < 1:
        raise ValueError('hosted source/run identity is required')
    return {'sha': sha, 'run': run, 'attempt': attempt,
            'platform': f'{platform.system()}-{platform.machine()}', 'nextest': NEXTEST_VERSION}


def check_tool():
    version = capture(['cargo', 'nextest', '--version']).strip().split()
    if version[:2] != ['cargo-nextest', NEXTEST_VERSION]:
        raise ValueError('unexpected nextest executable version')


def write_json(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_suffix('.tmp')
    temporary.write_text(json.dumps(value, indent=2) + '\n')
    temporary.replace(path)


def build():
    check_tool()
    source = identity()
    OUT.mkdir(parents=True, exist_ok=True)
    args = ['--locked', '--workspace', '--exclude', 'app']
    subprocess.run(['cargo', 'nextest', 'archive', *args, '--profile', 'ci',
                    '--archive-file', str(OUT / 'tests.tar.zst')], cwd=ROOT, check=True)
    listing = json.loads(capture(['cargo', 'nextest', 'list', *args, '--message-format', 'json']))
    active, ignored = inventory(listing)
    # Cargo determines the original executable set; source-file regexes cannot
    # discover cfg-dependent or generated tests. Reusing fingerprints avoids a
    # second compilation of already-built targets.
    cargo_text = capture(['cargo', 'test', *args, '--no-run', '--message-format=json'])
    artifacts = [json.loads(line) for line in cargo_text.splitlines() if line.startswith('{')]
    cargo_binaries = {str(Path(item['executable']).resolve()) for item in artifacts
                      if item.get('reason') == 'compiler-artifact' and item.get('profile', {}).get('test') and item.get('executable')}
    suites = listing['rust-suites']
    if cargo_binaries != {str(Path(suite['binary-path']).resolve()) for suite in suites.values()}:
        raise ValueError('Cargo and nextest executable sets differ')
    for suite in suites.values():
        binary = suite['binary-path']
        actual = libtest_names(subprocess.check_output([binary, '--list', '--format', 'terse'], cwd=suite['cwd'], text=True))
        skipped = libtest_names(subprocess.check_output([binary, '--list', '--ignored', '--format', 'terse'], cwd=suite['cwd'], text=True))
        if actual != set(suite['testcases']) or skipped != {name for name, test in suite['testcases'].items() if test['ignored']}:
            raise ValueError(f'Cargo and nextest test/ignore inventories differ: {suite["binary-id"]}')
    manifest = {'schema': 1, 'identity': source, 'tests': active, 'ignored': ignored,
                'archive_sha256': digest(OUT / 'tests.tar.zst'),
                'rustc': capture(['rustc', '-vV']).strip(),
                'cargo_lock_sha256': digest(ROOT / 'Cargo.lock')}
    write_json(OUT / 'manifest.json', manifest)
    print(f'Cargo/nextest equivalence: {len(cargo_binaries)} binaries, {len(active)} executable tests, {len(ignored)} pre-existing ignored tests', flush=True)


def run_shard(index, total):
    if not 1 <= index <= total <= 60:
        raise ValueError('invalid shard index/count')
    check_tool()
    current = identity()
    manifest = json.loads((OUT / 'manifest.json').read_text())
    check_archive(manifest, OUT / 'tests.tar.zst', current)
    if manifest['cargo_lock_sha256'] != digest(ROOT / 'Cargo.lock'):
        raise ValueError('Cargo lock does not match archive')
    selection = ['--archive-file', str(OUT / 'tests.tar.zst'), '--workspace-remap', str(ROOT), '--partition', f'hash:{index}/{total}']
    listed = json.loads(capture(['cargo', 'nextest', 'list', *selection, '--message-format', 'json']))
    selected, _ = inventory(listed, partition=True)
    if not selected:
        raise ValueError('empty execution partition')
    # An absolute JUnit path avoids depending on archive extraction locations.
    config = (ROOT / '.config/nextest.toml').read_text()
    junit = OUT / f'junit-{index}.xml'
    config = config.replace('path = "junit.xml"', 'path = ' + json.dumps(str(junit)))
    config_path = OUT / f'nextest-{index}.toml'
    config_path.write_text(config)
    report = {'identity': current, 'index': index, 'total': total, 'selected': selected, 'results': {}, 'complete': False}
    report_path = OUT / f'result-{index}.json'
    write_json(report_path, report)
    completed = subprocess.run(['cargo', 'nextest', 'run', *selection, '--config-file', str(config_path), '--profile', 'ci',
                                '--retries', '0', '--test-threads', '1', '--no-fail-fast'], cwd=ROOT)
    if junit.exists():
        report['results'] = junit_results(junit.read_text())
        report['complete'] = completed.returncode == 0 and set(report['results']) == set(selected)
    write_json(report_path, report)
    if not report['complete'] or any(value != 'passed' for value in report['results'].values()):
        raise ValueError('Rust shard failed or produced incomplete results')
    print(f'Shard {index}/{total}: {len(selected)} tests passed')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest='command', required=True)
    sub.add_parser('build')
    run = sub.add_parser('run'); run.add_argument('--index', type=int, required=True); run.add_argument('--total', type=int, required=True)
    verify = sub.add_parser('verify'); verify.add_argument('--total', type=int, required=True)
    args = parser.parse_args()
    if args.command == 'build':
        build()
    elif args.command == 'run':
        run_shard(args.index, args.total)
    else:
        manifest = json.loads((OUT / 'manifest.json').read_text())
        reports = [json.loads(path.read_text()) for path in OUT.glob('result-*.json')]
        count = verify_reports(manifest, reports, identity(), args.total)
        print(f'Complete exact-source Rust coverage: {count} passed; {len(manifest["ignored"])} pre-existing ignored')


if __name__ == '__main__':
    try:
        main()
    except (ValueError, KeyError, OSError, subprocess.CalledProcessError, ET.ParseError) as error:
        print(f'Rust qualification failed: {error}', file=sys.stderr)
        sys.exit(1)
