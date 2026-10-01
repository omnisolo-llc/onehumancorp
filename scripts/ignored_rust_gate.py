#!/usr/bin/env python3
"""Execute explicit local-only Rust cases omitted by the default run."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import signal
import subprocess
import sys
from urllib.parse import urlsplit

SDK_REVISION = '85c54682a209ca7c3fc8b1ab2e820b6724dc3028'
# No wildcard or live-provider selector is accepted here. Original test names
# remain intact; each independently launched test must execute exactly once.
CASES = [
    ('omnisolo', '--lib', None, 'api::sync::tests::redis_disconnect_closes_websocket_for_reconnection', 'redis'),
    ('omnisolo', '--test', 'harness_middleware_interop', 'mysql_harness_middleware_migration_is_idempotent', 'mysql'),
    ('omnisolo', '--test', 'harness_middleware_interop', 'postgres_harness_middleware_migration_is_idempotent', 'postgres'),
    ('omnisolo_builtin_agent', '--lib', None, 'mesh::transport::tests::test_pg_notify_durable_delivery_and_reconnect', 'postgres'),
    ('server_harness', '--test', 'opencode_http', 'launches_real_pinned_server_with_bounded_health_and_session_lifecycle', 'opencode'),
    ('server_harness', '--test', 'opencode_http', 'real_pinned_server_executes_a_prompt_through_openai_responses', 'opencode'),
    ('server_harness', '--test', 'opencode_http', 'explicit_shutdown_reaps_managed_process_before_removing_isolated_home', 'opencode'),
    ('server_harness', '--test', 'openharness_sdk', 'real_pinned_sdk_prompts_with_private_tmp_home_and_read_only_workspace', 'sdk'),
 ]
CASES += [('omnisolo', '--lib', None, 'services::campaign::service::tests::' + name, 'campaign') for name in [
    'test_create_draft_campaign', 'test_add_asset_to_campaign', 'test_launch_campaign_requires_asset',
    'test_complete_campaign_flow', 'test_launch_campaign_requires_third_party_activation_dispatch', 'test_tenant_isolation',
]]
RESULT = re.compile(r'test result: (ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored; (\d+) measured; (\d+) filtered out;')
ANSI = re.compile(r'\x1b\[[0-9;]*m')


def command(case):
    package, target, binary, name, _ = case
    return ['cargo', 'test', '--locked', '-p', package, target, *([binary] if binary else []), name,
            '--', '--ignored', '--exact', '--show-output', '--test-threads=1']


def validate_result(name, log, returncode):
    clean = ANSI.sub('', log)
    results = RESULT.findall(clean)
    if returncode != 0 or len(results) != 1:
        raise ValueError('The exact test did not produce one successful execution summary')
    status, passed, failed, ignored, measured, _filtered = results[0]
    if (status, passed, failed, ignored, measured) != ('ok', '1', '0', '0', '0'):
        raise ValueError('Empty, failed, ignored, or extra executions cannot pass this exact selector')
    if not re.search(r'(?m)^test ' + re.escape(name) + r' \.\.\..*\bok\s*$', clean):
        raise ValueError('The expected named test has no successful execution record')


def validate_service_url(value, scheme, database=None):
    try:
        parsed = urlsplit(value)
        port = parsed.port
    except ValueError as error:
        raise ValueError('Invalid isolated service URL') from error
    if parsed.scheme != scheme or parsed.hostname not in {'127.0.0.1', 'localhost', '::1'} or not port:
        raise ValueError('Only an explicit loopback service port may be used')
    if parsed.query or parsed.fragment or (database and parsed.path != '/' + database):
        raise ValueError('The service must be the dedicated disposable test database')
    return value


def child_environment(source):
    # Keep build/tool discovery without inheriting provider credentials or live
    # opt-ins. Adapter children additionally apply their own isolated environment.
    allowed = {'PATH', 'HOME', 'TMPDIR', 'TEMP', 'TMP', 'CARGO_HOME', 'RUSTUP_HOME',
               'CARGO_TARGET_DIR', 'CARGO_BUILD_JOBS', 'CARGO_INCREMENTAL', 'RUSTFLAGS',
               'RUSTDOCFLAGS', 'CARGO_PROFILE_DEV_DEBUG', 'CARGO_PROFILE_TEST_DEBUG',
               'CARGO_NET_OFFLINE', 'LD_LIBRARY_PATH', 'SSL_CERT_FILE', 'SSL_CERT_DIR',
               'LANG', 'LC_ALL', 'CI'}
    result = {key: value for key, value in source.items() if key in allowed}
    result['CARGO_TERM_COLOR'] = 'never'
    result['RUST_TEST_THREADS'] = '1'
    result['OMNISOLO_STANDALONE_MODE'] = 'false'
    return result


def prerequisites(root, source):
    if source.get('OHC_IGNORED_SERVICE_ISOLATION') != '1':
        raise ValueError('Owned disposable PostgreSQL/MySQL/Redis services must be explicitly declared')
    pg = validate_service_url(source.get('OHC_IGNORED_POSTGRES_URL', ''), 'postgres', 'ohc_ignored_postgres')
    mysql = validate_service_url(source.get('OHC_IGNORED_MYSQL_URL', ''), 'mysql', 'ohc_ignored_mysql')
    redis = validate_service_url(source.get('OHC_IGNORED_REDIS_URL', ''), 'redis')
    sdk = Path(source.get('OPENHARNESS_PINNED_SDK_SOURCE', '')).resolve()
    expected = root / 'target/ignored-prerequisites/openharness'
    if sdk != expected.resolve() or not (sdk / 'src/harness/__init__.py').is_file():
        raise ValueError('The existing pinned SDK source must be checked out at the isolated test path')
    revision = subprocess.check_output(['git', '-C', str(sdk), 'rev-parse', 'HEAD'], text=True).strip()
    if revision != SDK_REVISION or subprocess.check_output(['git', '-C', str(sdk), 'status', '--porcelain'], text=True).strip():
        raise ValueError('Pinned SDK checkout identity or clean-source verification failed')
    binary = root / '.github/test-tools/node_modules/.bin/opencode'
    version = subprocess.check_output([str(binary), '--version'], text=True, env=child_environment(source)).strip()
    if version != '1.18.15':
        raise ValueError('The pinned OpenCode 1.18.15 executable is required')
    env = child_environment(source)
    env['PATH'] = str(binary.parent) + os.pathsep + env.get('PATH', '')
    # A false standalone flag alone still falls back to standalone when the
    # application has no database source, disabling Redis before subscription.
    env['OMNISOLO_DATABASE_URL'] = pg
    env.update(OHC_TEST_PG_URL=pg, OHC_CAMPAIGN_TEST_DATABASE_URL=pg, OMNISOLO_HARNESS_POSTGRES_URL=pg,
               OMNISOLO_HARNESS_MYSQL_URL=mysql, OHC_TEST_REDIS_URL=redis,
               REDIS_URL=redis, OPENHARNESS_PINNED_SDK_SOURCE=str(sdk))
    return env


def execute_case(argv, root, env, log_path, timeout=180):
    if os.name != 'posix':
        raise ValueError('This isolated CI runner requires POSIX process groups')
    with log_path.open('w') as output:
        process = subprocess.Popen(argv, cwd=root, env=env, text=True,
                                   stdout=output, stderr=subprocess.STDOUT,
                                   start_new_session=True)
        code = 1
        try:
            code = process.wait(timeout=timeout)
        except subprocess.TimeoutExpired:
            code = 124
            output.write(f'\nExact test timed out after {timeout} seconds; stopping its owned process group\n')
            output.flush()
        finally:
            try:
                os.killpg(process.pid, 0)
            except ProcessLookupError:
                pass
            else:
                if code == 0:
                    code = 125
                    output.write('\nOwned descendants survived the test; execution is not clean\n')
                    output.flush()
                try:
                    os.killpg(process.pid, signal.SIGTERM)
                except ProcessLookupError:
                    pass
                try:
                    process.wait(timeout=1)
                except subprocess.TimeoutExpired:
                    pass
                try:
                    os.killpg(process.pid, signal.SIGKILL)
                except ProcessLookupError:
                    pass
                process.wait(timeout=3)
    return code, log_path.read_text()


def run(root, evidence):
    evidence.mkdir(parents=True, exist_ok=True)
    receipt = {'status': 'failed', 'required_cases': len(CASES), 'cases': [],
               'live_provider_cases': 'not selected or executed'}
    try:
        tracked = subprocess.check_output(['git', 'ls-files', '-z'], cwd=root).decode().split('\0')
        inputs = {name: hashlib.sha256((root / name).read_bytes()).hexdigest() for name in tracked
                  if name and (name.endswith(('.rs', '.sql')) or name.endswith(('Cargo.toml', 'Cargo.lock')) or name in {'scripts/ignored_rust_gate.py', '.github/workflows/ci.yml'})}
        (evidence / 'source-manifest.json').write_text(json.dumps(inputs, sort_keys=True, indent=2) + '\n')
        env = prerequisites(root, os.environ)
        for case in CASES:
            name = case[3]
            code, output = execute_case(command(case), root, env,
                                        evidence / (name.replace('::', '_') + '.log'))
            print(output, end='', flush=True)
            item = {'name': name, 'exit_code': code, 'status': 'failed'}
            receipt['cases'].append(item)
            try:
                validate_result(name, output, code)
                item['status'] = 'passed'
            except ValueError as error:
                item['error'] = str(error)
            (evidence / (name.replace('::', '_') + '.json')).write_text(json.dumps(item, indent=2) + '\n')
        for name, expected in inputs.items():
            if hashlib.sha256((root / name).read_bytes()).hexdigest() != expected:
                raise ValueError('Rust execution inputs changed during the gate')
        if subprocess.check_output(['git', '-C', env['OPENHARNESS_PINNED_SDK_SOURCE'], 'diff', '--name-only'], text=True).strip():
            raise ValueError('Pinned SDK source changed during the gate')
        (evidence / 'source-manifest.json').write_text(json.dumps(inputs, sort_keys=True, indent=2) + '\n')
        if all(item['status'] == 'passed' for item in receipt['cases']) and len(receipt['cases']) == len(CASES):
            receipt['status'] = 'passed'
    except (OSError, ValueError, subprocess.SubprocessError) as error:
        receipt['error'] = str(error)
        print(str(error), file=sys.stderr)
    finally:
        (evidence / 'result.json').write_text(json.dumps(receipt, indent=2) + '\n')
    return 0 if receipt['status'] == 'passed' else 1


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('action', choices=['list', 'run'])
    parser.add_argument('--evidence', type=Path, default=Path('target/ignored-rust-results'))
    args = parser.parse_args()
    if args.action == 'list':
        print(json.dumps([{'name': case[3], 'prerequisite': case[4], 'command': command(case)} for case in CASES], indent=2))
        return 0
    return run(Path(__file__).resolve().parents[1], args.evidence.resolve())


if __name__ == '__main__':
    sys.exit(main())
