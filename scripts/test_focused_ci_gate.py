import importlib.util
import json
from pathlib import Path
import tempfile
import unittest

SPEC = importlib.util.spec_from_file_location('focused_ci_gate', Path(__file__).with_name('focused_ci_gate.py'))
if SPEC is None or SPEC.loader is None:
    raise RuntimeError('focused gate module is required')
gate = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(gate)


class FocusedGateTests(unittest.TestCase):
    def test_postgres_prefetches_the_entire_locked_graph_before_offline_gates(self):
        import yaml
        root = Path(__file__).resolve().parents[1]
        steps = yaml.safe_load((root / '.github/workflows/ci.yml').read_text())['jobs']['postgres-security']['steps']
        prefetch = next((i for i, step in enumerate(steps)
                         if step.get('run', '').strip() == 'cargo fetch --locked'), None)
        self.assertIsNotNone(prefetch, 'cold-cache offline gates need the full locked workspace graph, including non-host dependencies')
        first_gate = next(i for i, step in enumerate(steps)
                          if 'scripts/sync-durability/run.sh' in step.get('run', ''))
        self.assertLess(prefetch, first_gate)
        self.assertNotIn('--target', steps[prefetch]['run'])
        self.assertNotIn('continue-on-error', steps[prefetch])

    def test_redis_reconnect_requires_owned_fixture_and_complete_inventory(self):
        import yaml
        root = Path(__file__).resolve().parents[1]
        self.assertEqual(gate.GATES.get('redis-reconnect'), (20, None))
        steps = yaml.safe_load((root/'.github/workflows/ci.yml').read_text())['jobs']['postgres-security']['steps']
        fetch = next(i for i, step in enumerate(steps) if step.get('run') == 'bash scripts/redis-reconnect/fetch.sh')
        execute = next(i for i, step in enumerate(steps) if step.get('run') == 'python3 scripts/focused_ci_gate.py redis-reconnect')
        self.assertLess(fetch, execute)
        self.assertEqual(steps[execute].get('env', {}), {'CARGO_TARGET_DIR': 'target'})
        runner = (root/'scripts/redis-reconnect/run.sh').read_text()
        self.assertIn('--locked --offline', runner)
        self.assertIn('verify_source.py verify', runner)
        self.assertIn('verify_lock.py', runner)
        minimum, _ = gate.GATES['redis-reconnect']
        for result in ['19 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out',
                       '20 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out',
                       '20 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out']:
            with self.assertRaises(ValueError):
                gate.validate_results('test result: ok. '+result+';', minimum)

    def test_cash_runner_rejects_unavailable_redis_before_native_execution(self):
        import os
        import shutil
        import subprocess
        root = Path(__file__).resolve().parents[1]
        with tempfile.TemporaryDirectory() as directory:
            sandbox = Path(directory)
            folder = sandbox/'scripts/cash-receipts'
            folder.mkdir(parents=True)
            for name in ['run.sh', 'database_guard.py']:
                shutil.copyfile(root/'scripts/cash-receipts'/name, folder/name)
            shared = sandbox/'scripts/agent-feed-decision-contract'
            shared.mkdir()
            shutil.copyfile(root/'scripts/agent-feed-decision-contract/database_guard.py', shared/'database_guard.py')
            (sandbox/'Cargo.lock').write_text('fixture lock')
            binaries = sandbox/'bin'
            binaries.mkdir()
            capture = sandbox/'native-started'
            (binaries/'cargo').write_text('#!/bin/sh\ntouch "$CASH_NATIVE_CAPTURE"\n')
            (binaries/'redis-cli').write_text('#!/bin/sh\nexit 1\n')
            (binaries/'cargo').chmod(0o755)
            (binaries/'redis-cli').chmod(0o755)
            environment = dict(os.environ, PATH=str(binaries)+os.pathsep+os.environ['PATH'],
                CASH_NATIVE_CAPTURE=str(capture), OHC_CASH_TEST_DATABASE_URL='postgres://fixture@127.0.0.1:5432/ohc_cash_test',
                OHC_CASH_TEST_REDIS_URL='redis://127.0.0.1:56379/0')
            result = subprocess.run(['bash', str(folder/'run.sh')], cwd=sandbox, env=environment, capture_output=True, text=True)
            self.assertNotEqual(result.returncode, 0)
            self.assertFalse(capture.exists(), 'unavailable Redis must stop before metadata or compilation')
            self.assertIn('Owned Redis fixture is unavailable', result.stderr)

    def test_cash_receipts_require_all_cases_cold_fetch_and_owned_services(self):
        import yaml
        root = Path(__file__).resolve().parents[1]
        self.assertIn('cash-receipts', gate.GATES)
        self.assertEqual(gate.GATES['cash-receipts'], (45, 'OHC_CASH_TEST_DATABASE_URL'))
        steps = yaml.safe_load((root/'.github/workflows/ci.yml').read_text())['jobs']['postgres-security']['steps']
        fetch = next(i for i, step in enumerate(steps) if step.get('run') == 'bash scripts/cash-receipts/fetch.sh')
        execute = next(i for i, step in enumerate(steps) if 'focused_ci_gate.py cash-receipts' in step.get('run', ''))
        self.assertLess(fetch, execute)
        self.assertIn('!cancelled()', steps[fetch]['if'])
        self.assertIn('!cancelled()', steps[execute]['if'])
        self.assertEqual(steps[execute]['env']['OHC_CASH_TEST_DATABASE_URL'], 'postgres://postgres:postgres@127.0.0.1:5432/ohc_cash_receipts_test')
        self.assertIn('job.services.widget_redis.ports[6379]', steps[execute]['env']['OHC_CASH_TEST_REDIS_URL'])
        self.assertIn('createdb ', steps[execute]['run'])
        fetch_script = (root/'scripts/cash-receipts/fetch.sh').read_text()
        self.assertLess(fetch_script.index('cp Cargo.lock '), fetch_script.index('cargo metadata '))
        self.assertLess(fetch_script.index('cargo metadata '), fetch_script.index('verify_lock.py'))
        self.assertLess(fetch_script.index('verify_lock.py'), fetch_script.index('cargo fetch --locked'))
        self.assertNotIn('metadata --offline', fetch_script)
        self.assertNotIn('metadata --no-deps', fetch_script)
        runner = (root/'scripts/cash-receipts/run.sh').read_text()
        self.assertIn('database_guard.py', runner)
        self.assertIn('--locked --offline', runner)
        self.assertNotIn(' cash_contract --', runner)
        self.assertNotIn('--ignored', runner)
        manifest = (root/'scripts/cash-receipts/prepare.py').read_text()
        for source in ['.github/workflows/ci.yml', 'scripts/focused_ci_gate.py', 'scripts/test_focused_ci_gate.py', 'scripts/agent-feed-decision-contract/database_guard.py']:
            self.assertIn(source, manifest)

    def test_clock_gate_requires_both_stores_and_locked_fetch(self):
        import yaml
        root = Path(__file__).resolve().parents[1]
        minimum, database = gate.GATES['staff-timecard-contract']
        self.assertGreaterEqual(minimum, 96)
        self.assertEqual(database, 'OHC_CLOCK_TEST_DATABASE_URL')
        steps = yaml.safe_load((root/'.github/workflows/ci.yml').read_text())['jobs']['postgres-security']['steps']
        fetch = next(i for i, step in enumerate(steps) if step.get('run') == 'bash scripts/staff-timecard-contract/fetch.sh')
        execute = next(i for i, step in enumerate(steps) if 'focused_ci_gate.py staff-timecard-contract' in step.get('run', ''))
        self.assertLess(fetch, execute)
        self.assertIn('!cancelled()', steps[fetch]['if'])
        self.assertIn('!cancelled()', steps[execute]['if'])
        self.assertEqual(steps[execute]['env'][database], 'postgres://postgres:postgres@127.0.0.1:5432/ohc_clock_timecard_test')
        self.assertIn('createdb ', steps[execute]['run'])
        folder = root/'scripts/staff-timecard-contract'
        fetch_script = (folder/'fetch.sh').read_text()
        self.assertLess(fetch_script.index('verify_lock.py'), fetch_script.index('cargo fetch --locked'))
        self.assertNotIn('cargo metadata', fetch_script)
        runner = (folder/'run.sh').read_text()
        self.assertIn('--locked --offline', runner)
        self.assertNotIn('--ignored', runner)
        self.assertNotIn('--include-ignored', runner)
        self.assertIn(' -- --test-threads=1', runner)
        self.assertNotIn('cargo metadata', runner)
        self.assertIn('test-staff-timecards:', (root/'Makefile').read_text())
        native_steps = yaml.safe_load((root/'.github/workflows/ci.yml').read_text())['jobs']['native-test']['steps']
        workspace = next(step for step in native_steps if step.get('run') == 'make test-backend')
        self.assertIn(database, workspace['env'])
        earlier = native_steps[:native_steps.index(workspace)]
        self.assertTrue(any('createdb ' in step.get('run', '') and 'ohc_clock_timecard_test' in step['run'] for step in earlier))

    def test_clock_runner_rejects_unavailable_postgres_before_native_execution(self):
        import os
        import shutil
        import subprocess
        root = Path(__file__).resolve().parents[1]
        with tempfile.TemporaryDirectory() as directory:
            sandbox = Path(directory)
            folder = sandbox/'scripts/staff-timecard-contract'
            folder.mkdir(parents=True)
            for name in ['run.sh', 'database_guard.py']:
                shutil.copyfile(root/'scripts/staff-timecard-contract'/name, folder/name)
            shared = sandbox/'scripts/agent-feed-decision-contract'
            shared.mkdir()
            shutil.copyfile(root/'scripts/agent-feed-decision-contract/database_guard.py', shared/'database_guard.py')
            binaries = sandbox/'bin'
            binaries.mkdir()
            capture = sandbox/'native-started'
            (binaries/'cargo').write_text('#!/bin/sh\ntouch "$CLOCK_NATIVE_CAPTURE"\n')
            (binaries/'psql').write_text('#!/bin/sh\nexit 1\n')
            (binaries/'cargo').chmod(0o755)
            (binaries/'psql').chmod(0o755)
            environment = dict(os.environ, PATH=str(binaries)+os.pathsep+os.environ['PATH'],
                CLOCK_NATIVE_CAPTURE=str(capture),
                OHC_CLOCK_TEST_DATABASE_URL='postgres://fixture@127.0.0.1:5432/ohc_clock_test')
            result = subprocess.run(['bash', str(folder/'run.sh')], cwd=sandbox, env=environment,
                                    capture_output=True, text=True)
            self.assertNotEqual(result.returncode, 0)
            self.assertFalse(capture.exists(), 'fixture outage must fail before Cargo can start')
            self.assertIn('Owned PostgreSQL fixture is unavailable', result.stderr)

    def test_clock_lock_rejects_registry_and_local_package_drift(self):
        import runpy
        root = Path(__file__).resolve().parents[1]
        verify = runpy.run_path(str(root/'scripts/staff-timecard-contract/verify_lock.py'))['verify']
        dependency = dict(name='dependency', version='1', source='registry+example', checksum='original')
        local = dict(name='server_auth', version='0.1.0')
        harness = dict(name='ohc-clock-receipt-regressions', version='0.1.0')
        repository = {'package': [dependency, local]}
        verify(repository, {'package': [dependency, local, harness]})
        for changed in [dict(dependency, checksum='changed'), dict(local, version='0.2.0')]:
            with self.subTest(changed=changed), self.assertRaises(ValueError):
                verify(repository, {'package': [changed, harness]})

    def test_clock_generator_binds_whole_runtime_and_actual_schema_fragments(self):
        import runpy
        root = Path(__file__).resolve().parents[1]
        folder = root/'scripts/staff-timecard-contract'
        prepare = runpy.run_path(str(folder/'prepare.py'))['prepare']
        prepare()
        manifest = json.loads((folder/'source-manifest.json').read_text())
        fragments = {item['label']: item for item in manifest['production_fragments']}
        for label in ['whole module: staff_timecards', 'whole module: sync_transaction',
                      'whole module: staff_timecards_test', 'timecard method route', 'receipt recovery method route',
                      'canonical access configuration', 'actual staff parent mount',
                      'actual global protected bearer layer',
                      'actual SQLite ohc_timecard_event bootstrap',
                      'actual SQLite receipt upgrade',
                      'actual PostgreSQL migration: 1035_staff_timecard_receipts.sql']:
            item = fragments[label]
            exact = (root/item['path']).read_bytes()[item['start_byte']:item['end_byte_exclusive']]
            self.assertEqual(exact.decode(), item['literal'])
            import hashlib
            self.assertEqual(hashlib.sha256(exact).hexdigest(), item['sha256'])
        generated = (folder/'generated.rs').read_text()
        self.assertIn('mod staff_timecards;', generated)
        self.assertNotIn('pub async fn sync_timecard_handler(', generated,
                         'the POST implementation must be imported whole, never copied')
        for source in ['.github/workflows/ci.yml', 'scripts/focused_ci_gate.py',
                       'scripts/test_focused_ci_gate.py', 'src/server/db.rs',
                       'src/server/api/staff_timecards_test/fixture.rs']:
            self.assertIn(source, manifest['input_hashes'])

    def test_nats_metadata_fetch_precedes_required_offline_gate(self):
        import yaml
        root = Path(__file__).resolve().parents[1]
        steps = yaml.safe_load((root/'.github/workflows/ci.yml').read_text())['jobs']['postgres-security']['steps']
        fetch = next(i for i, step in enumerate(steps) if step.get('run') == 'bash scripts/nats-metadata-contract/fetch.sh')
        run = next(i for i, step in enumerate(steps) if step.get('run') == 'python3 scripts/focused_ci_gate.py nats-metadata-contract')
        self.assertLess(fetch, run)
        self.assertEqual(gate.GATES['nats-metadata-contract'], (16, None))
        script = (root/'scripts/nats-metadata-contract/fetch.sh').read_text()
        self.assertNotIn('metadata --no-deps', script)
        self.assertLess(script.index('cargo metadata '), script.index('verify_lock.py'))
        self.assertLess(script.index('verify_lock.py'), script.index('cargo fetch --locked'))
        self.assertIn('--locked --offline', (root/'scripts/nats-metadata-contract/run.sh').read_text())

    def test_mesh_startup_fetch_precedes_required_offline_gate(self):
        import yaml
        root = Path(__file__).resolve().parents[1]
        steps = yaml.safe_load((root/'.github/workflows/ci.yml').read_text())['jobs']['postgres-security']['steps']
        fetch = next(i for i, step in enumerate(steps) if step.get('run') == 'bash scripts/mesh-startup-contract/fetch.sh')
        run = next(i for i, step in enumerate(steps) if step.get('run') == 'python3 scripts/focused_ci_gate.py mesh-startup-contract')
        self.assertLess(fetch, run)
        self.assertEqual(gate.GATES['mesh-startup-contract'], (13, None))
        script = (root/'scripts/mesh-startup-contract/fetch.sh').read_text()
        self.assertNotIn('metadata --no-deps', script)
        self.assertLess(script.index('cargo metadata '), script.index('verify_lock.py'))
        self.assertLess(script.index('verify_lock.py'), script.index('cargo fetch --locked'))
        self.assertIn('--locked --offline', (root/'scripts/mesh-startup-contract/run.sh').read_text())

    def test_quote_acceptance_requires_complete_owner_version_inventory(self):
        minimum, database = gate.GATES['quote-acceptance']
        self.assertGreaterEqual(minimum, 36)
        self.assertEqual(database, 'OHC_QUOTE_TEST_DATABASE_URL')
        with self.assertRaises(ValueError):
            gate.validate_results('test result: ok. 35 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;', minimum)

    def test_redis_startup_fetch_precedes_offline_gate(self):
        import yaml
        root = Path(__file__).resolve().parents[1]
        steps = yaml.safe_load((root/'.github/workflows/ci.yml').read_text())['jobs']['postgres-security']['steps']
        fetch = next(i for i, step in enumerate(steps) if step.get('run') == 'bash scripts/redis-startup-contract/fetch.sh')
        gate_step = next(i for i, step in enumerate(steps) if step.get('run') == 'python3 scripts/focused_ci_gate.py redis-startup-contract')
        self.assertLess(fetch, gate_step)
        self.assertEqual(gate.GATES['redis-startup-contract'], (23, None))
        script = (root/'scripts/redis-startup-contract/fetch.sh').read_text()
        self.assertIn('cp Cargo.lock scripts/redis-startup-contract/Cargo.lock', script)
        self.assertNotIn('metadata --no-deps', script)
        self.assertLess(script.index('cargo metadata '), script.index('verify_lock.py'))
        self.assertLess(script.index('verify_lock.py'), script.index('cargo fetch --locked'))
        self.assertIn('--locked --offline', (root/'scripts/redis-startup-contract/run.sh').read_text())

    def test_memory_jsonb_requires_actual_repository_and_pgvector_schema(self):
        import runpy
        root = Path(__file__).resolve().parents[1]
        minimum, database = gate.GATES['memory-jsonb-contract']
        self.assertGreaterEqual(minimum, 27)
        self.assertEqual(database, 'OHC_MEMORY_TEST_DATABASE_URL')
        folder = root/'scripts/memory-jsonb-contract'
        runpy.run_path(str(folder/'prepare.py'))
        source = (root/'src/agents/builtin/memory_store.rs').read_text()
        repository = source[:source.index('#[async_trait]\npub trait OmniSoloMemory')]
        self.assertIn(repository.removeprefix('use async_trait::async_trait;\n'), (folder/'generated.rs').read_text())
        self.assertIn('metadata JSONB', (folder/'active.sql').read_text())
        self.assertIn('metadata TEXT', (folder/'legacy.sql').read_text())
        self.assertIn('python3 scripts/focused_ci_gate.py memory-jsonb-contract', (root/'.github/workflows/ci.yml').read_text())
        self.assertIn('--locked --offline', (folder/'run.sh').read_text())

    def test_approval_runner_isolates_config_and_cleans_only_its_home_on_failure(self):
        import os
        import shutil
        import subprocess
        import sys
        root = Path(__file__).resolve().parents[1]
        with tempfile.TemporaryDirectory() as directory:
            sandbox = Path(directory)
            probe = sandbox/'scripts/approvals-read-contract'
            probe.mkdir(parents=True)
            shutil.copyfile(root/'scripts/approvals-read-contract/run.sh', probe/'run.sh')
            (sandbox/'Cargo.lock').write_text('fixture lock')
            (probe/'verify_lock.py').write_text('')
            (probe/'prepare.py').write_text("from pathlib import Path\nPath(__file__).with_name('source-manifest.json').write_text('{}')\n")
            binaries = sandbox/'bin'
            binaries.mkdir()
            capture = sandbox/'environment.json'
            cargo = binaries/'cargo'
            cargo.write_text(f'#!{sys.executable}\n' + '''import json, os, pathlib, sys
if sys.argv[1] == 'test':
    names = ['USERPROFILE', 'JWT_SECRET', 'JWT_SECRET_FILE', 'OMNISOLO_JWT_SECRET_FILE',
             'OMNISOLO_STANDALONE_MODE', 'OMNISOLO_DATABASE_URL', 'OMNISOLO_DATABASE_URL_FILE',
             'DATABASE_URL_FILE', 'DATABASE_URL', 'REDIS_URL', 'REDIS_URL_FILE',
             'OMNISOLO_REDIS_URL', 'OMNISOLO_REDIS_URL_FILE']
    pathlib.Path(os.environ['APPROVAL_ENV_CAPTURE']).write_text(json.dumps({name: os.environ[name] for name in names if name in os.environ}))
    sys.exit(42)
''')
            cargo.chmod(0o755)
            operator_home = sandbox/'operator-home'
            operator_home.mkdir()
            sentinel = operator_home/'keep'
            sentinel.write_text('operator state')
            scrubbed = ['JWT_SECRET_FILE', 'OMNISOLO_DATABASE_URL_FILE', 'DATABASE_URL_FILE',
                        'DATABASE_URL', 'REDIS_URL', 'REDIS_URL_FILE', 'OMNISOLO_REDIS_URL',
                        'OMNISOLO_REDIS_URL_FILE', 'OMNISOLO_JWT_SECRET_FILE']
            env = dict(os.environ, PATH=str(binaries)+os.pathsep+os.environ['PATH'],
                       USERPROFILE=str(operator_home), APPROVAL_ENV_CAPTURE=str(capture),
                       OHC_APPROVAL_TEST_DATABASE_URL='postgres://fixture@127.0.0.1/ohc_approval_test',
                       JWT_SECRET='ambient-canary', OMNISOLO_STANDALONE_MODE='true',
                       OMNISOLO_DATABASE_URL='postgres://ambient.invalid/operator')
            env.update({name: 'ambient-canary' for name in scrubbed})
            result = subprocess.run(['bash', str(probe/'run.sh')], env=env,
                                    capture_output=True, text=True)
            self.assertEqual(result.returncode, 42, result.stderr)
            observed = json.loads(capture.read_text())
            self.assertEqual(observed['OMNISOLO_DATABASE_URL'], 'sqlite::memory:')
            self.assertEqual(observed['OMNISOLO_STANDALONE_MODE'], 'false')
            self.assertEqual(observed['JWT_SECRET'], 'public-local-approval-regression-signing-key-only')
            self.assertTrue(all(name not in observed for name in scrubbed))
            self.assertNotEqual(observed['USERPROFILE'], str(operator_home))
            self.assertFalse(Path(observed['USERPROFILE']).exists())
            self.assertEqual(sentinel.read_text(), 'operator state')

    def test_full_approval_decisions_are_required_in_native_tests(self):
        import yaml
        root = Path(__file__).resolve().parents[1]
        steps = yaml.safe_load((root/'.github/workflows/ci.yml').read_text())['jobs']['native-test']['steps']
        native = next(step for step in steps if step.get('run') == 'make test-backend')
        self.assertIn('/ohc_approval_test', native['env']['OHC_APPROVAL_TEST_DATABASE_URL'])
        provision = next(step for step in steps if 'createdb' in step.get('run', '') and 'ohc_approval_test' in step.get('run', ''))
        self.assertLess(steps.index(provision), steps.index(native))
        self.assertIn('!cancelled()', provision['if'])
        tests = (root/'src/server/api/agents/approvals_readback_test.rs').read_text()
        self.assertNotIn('ignore =', tests)
        wrapper = (root/'scripts/approvals-read-contract/with-owned-postgres.sh').read_text()
        self.assertIn('api::agents::approvals::readback_tests', wrapper)
        self.assertNotIn('--ignored', wrapper)

    def test_approval_reads_require_real_pg_and_complete_read_inventory(self):
        minimum, database = gate.GATES['approvals-read-contract']
        self.assertGreaterEqual(minimum, 12)
        self.assertEqual(database, 'OHC_APPROVAL_TEST_DATABASE_URL')
        root = Path(__file__).resolve().parents[1]
        runner = (root/'scripts/approvals-read-contract/run.sh').read_text()
        self.assertIn('--locked --offline', runner)
        self.assertNotIn(' read_contract --', runner)
        self.assertIn('python3 scripts/focused_ci_gate.py approvals-read-contract', (root/'.github/workflows/ci.yml').read_text())

    def test_field_boundaries_require_complete_actual_handler_and_owned_database_gate(self):
        root = Path(__file__).resolve().parents[1]
        self.assertIn('field-boundary-contract', gate.GATES)
        minimum, database = gate.GATES['field-boundary-contract']
        self.assertGreaterEqual(minimum, 89)
        self.assertEqual(database, 'OHC_FIELD_TEST_DATABASE_URL')
        self.assertIn('python3 scripts/focused_ci_gate.py field-boundary-contract', (root/'.github/workflows/ci.yml').read_text())
        self.assertTrue((root/'scripts/field-boundary-contract/run.sh').is_file())
        self.assertIn('--locked --offline', (root/'scripts/field-boundary-contract/run.sh').read_text())
        import re, runpy
        runpy.run_path(str(root/'scripts/field-boundary-contract/prepare.py'))
        setup = re.search(r'let field_ops_pool\s*=\s*.*?;', (root/'src/server/lib.rs').read_text(), re.S).group()
        self.assertIn(setup, (root/'scripts/field-boundary-contract/generated.rs').read_text())
        generated = (root/'scripts/field-boundary-contract/generated.rs').read_text()
        sync_setup = re.search(r'let sync_events_state\s*=\s*.*?;', (root/'src/server/lib.rs').read_text(), re.S).group()
        self.assertIn(sync_setup, generated)
        write_setup = re.search(r'let sync_write_state\s*=\s*.*?;', (root/'src/server/lib.rs').read_text(), re.S).group()
        self.assertIn(write_setup, generated)
        self.assertIn('/api/v1/sync/offline', generated)
        self.assertIn('/api/v1/sync/operation-intents', generated)
        self.assertIn('/api/v1/sync/events', generated)
        runner = (root/'scripts/field-boundary-contract/run.sh').read_text()
        self.assertIn('OHC_SYNC_TEST_DATABASE_URL="$OHC_FIELD_TEST_DATABASE_URL"', runner)
        self.assertIn('--include-ignored', runner)
        manifest = json.loads((root/'scripts/field-boundary-contract/source-manifest.json').read_text())
        self.assertIn('src/server/api/durable_appointment_sync.rs', manifest)


    def test_operations_worker_gate_compiles_actual_spawn_and_cache_boundary(self):
        import runpy
        root = Path(__file__).resolve().parents[1]
        minimum, database = gate.GATES['proactive-worker-contract']
        self.assertGreaterEqual(minimum, 11)
        self.assertEqual(database, 'OHC_OPS_PROBE_DB')
        folder = root/'scripts/proactive-worker-contract'
        runpy.run_path(str(folder/'prepare.py'))
        generated = (folder/'generated.rs').read_text()
        self.assertIn(str(root/'src/server/workers/proactive_operations_worker.rs'), generated)
        self.assertIn('pub struct DB {', generated)
        self.assertIn('pub struct HybridCacheInner<T>', generated)
        self.assertIn('pub fn get_agent_feed_cache()', generated)
        self.assertNotIn('struct Mock', generated)
        self.assertIn('worker.start();', (folder/'test.rs').read_text())
        workflow = (root/'.github/workflows/ci.yml').read_text()
        self.assertIn('python3 scripts/focused_ci_gate.py proactive-worker-contract', workflow)
        self.assertIn('bash scripts/proactive-worker-contract/fetch.sh', workflow)

    def test_staff_reads_require_real_owned_database_and_preserved_failure_cases(self):
        minimum, database = gate.GATES['staff-read-contract']
        self.assertGreaterEqual(minimum, 8)
        self.assertEqual(database, 'OHC_STAFF_TEST_DATABASE_URL')
        root = Path(__file__).resolve().parents[1]
        workflow = (root/'.github/workflows/ci.yml').read_text()
        self.assertIn('python3 scripts/focused_ci_gate.py staff-read-contract', workflow)
        self.assertIn('bash scripts/staff-read-contract/fetch.sh', workflow)
        self.assertIn('--locked --offline', (root/'scripts/staff-read-contract/run.sh').read_text())


    def test_builder_generation_requires_real_http_and_owned_storage(self):
        minimum, database = gate.GATES['builder-generation-contract']
        self.assertGreaterEqual(minimum, 90)
        self.assertEqual(database, 'OHC_BUILDER_GENERATION_TEST_DATABASE_URL')
        root = Path(__file__).resolve().parents[1]
        workflow = (root/'.github/workflows/ci.yml').read_text()
        self.assertIn('python3 scripts/focused_ci_gate.py builder-generation-contract', workflow)
        self.assertIn('bash scripts/builder-generation-contract/fetch.sh', workflow)

    def test_builder_generation_preserves_receipt_storage_compile_inputs(self):
        import runpy
        import re
        root = Path(__file__).resolve().parents[1]
        folder = root/'scripts/builder-generation-contract'
        runpy.run_path(str(folder/'prepare.py'))
        generated = (folder/'generated.rs').read_text()
        manifest = json.loads((folder/'source-manifest.json').read_text())
        self.assertIn('pub mod persistence {', generated)
        parent = (root/'src/server/persistence/mod.rs').read_text()
        for name in ['capabilities', 'connection', 'entities', 'migration']:
            declaration = re.search(r'(?m)^(?:pub(?:\([^)]*\))? )?mod '+name+r';$', parent)
            self.assertIsNotNone(declaration)
            self.assertIn(declaration.group(), generated[generated.index('pub mod persistence {'):])
        self.assertRegex(parent, r'(?m)^mod connection;$')
        self.assertNotIn('pub mod connection;', generated)
        self.assertNotIn('pub use crate::{capabilities,connection', generated)
        self.assertIn('pub(crate) use connection::require_sqlite_encryption;', generated)
        self.assertIn('crate::persistence::require_sqlite_encryption(&canonical_connection)', generated)
        required = list((root/'src/server/workflow_execution').rglob('*.rs'))
        required += [p for p in (root/'src/server/persistence').rglob('*') if p.is_file() and p.suffix in {'.rs', '.sql'}]
        for source in required:
            self.assertIn(str(source.relative_to(root)), manifest)

    def test_widget_chat_requires_the_complete_owned_database_gate(self):
        minimum, database = gate.GATES['widget-chat-contract']
        self.assertGreaterEqual(minimum, 32)
        self.assertEqual(database, 'OHC_WIDGET_TEST_DATABASE_URL')
        root = Path(__file__).resolve().parents[1]
        self.assertTrue((root/'scripts/widget-chat-contract/run.sh').is_file())
    def test_recorded_order_milestones_require_real_database_and_all_cases(self):
        minimum, database = gate.GATES['order-milestones']
        self.assertGreaterEqual(minimum, 13)
        self.assertEqual(database, 'OHC_MILESTONE_TEST_DATABASE_URL')
        root = Path(__file__).resolve().parents[1]
        self.assertTrue((root/'scripts/order-milestones/run.sh').is_file())
        self.assertIn('python3 scripts/focused_ci_gate.py order-milestones', (root/'.github/workflows/ci.yml').read_text())
    def test_site_publication_gate_requires_complete_pg_http_and_javascript_proof(self):
        self.assertIn('site-publication', gate.GATES)
        minimum, database = gate.GATES['site-publication']
        self.assertGreaterEqual(minimum, 93)
        self.assertEqual(database, 'OHC_PUBLICATION_TEST_DATABASE_URL')
        root = Path(__file__).resolve().parents[1]
        runner = (root/'scripts/site-publication/run.sh').read_text()
        self.assertIn('node scripts/site-publication/jcs-proof.cjs', runner)
        self.assertIn('src/ui/next/node_modules', runner)
        self.assertIn('python3 scripts/site-publication/test_deny_http_egress.py', runner)
        self.assertIn('python3 scripts/site-publication/deny_http_egress.py -- cargo test --locked --offline', runner)
        witness = (root/'scripts/site-publication/jcs-proof.cjs').read_text()
        self.assertIn("'5.1.0'", witness)
        self.assertIn('process.versions.node', witness)
    def test_agent_receipt_postgres_gate_keeps_real_storage_and_sqlite_inventory(self):
        minimum, database = gate.GATES['agent-receipt-postgres-contract']
        self.assertGreaterEqual(minimum, 97)
        self.assertEqual(database, 'OHC_AGENT_RECEIPT_TEST_DATABASE_URL')
        root = Path(__file__).resolve().parents[1]
        runner = (root/'scripts/agent-receipt-postgres-contract/run.sh').read_text()
        self.assertIn('--locked --offline', runner)
        workflow = (root/'.github/workflows/ci.yml').read_text()
        self.assertIn('bash scripts/agent-receipt-postgres-contract/fetch.sh', workflow)
        self.assertIn('python3 scripts/focused_ci_gate.py agent-receipt-postgres-contract', workflow)

    def test_agent_definition_gate_requires_real_database_and_complete_inventory(self):
        minimum,database=gate.GATES['agent-definition-contract']
        self.assertGreaterEqual(minimum,63)
        self.assertEqual(database,'OHC_AGENT_DEFINITION_TEST_DATABASE_URL')
        self.assertTrue((Path(__file__).resolve().parents[1]/'scripts/agent-definition-contract/run.sh').is_file())

    def test_appointment_reads_require_the_full_real_database_inventory(self):
        minimum,database=gate.GATES['operations-appointments']
        self.assertGreaterEqual(minimum,11)
        self.assertEqual(database,'OHC_APPOINTMENTS_TEST_DATABASE_URL')
        self.assertTrue((Path(__file__).resolve().parents[1]/'scripts/operations-appointments/run.sh').is_file())

    def test_chat_gate_requires_real_database_and_complete_inventory(self):
        minimum, database = gate.GATES['chat-tenant-isolation']
        self.assertGreaterEqual(minimum, 22)
        self.assertEqual(database, 'OHC_CHAT_TEST_DATABASE_URL')
        root = Path(__file__).resolve().parents[1]
        self.assertTrue((root/'scripts/chat-tenant-isolation/run.sh').is_file())
        workflow = (root/'.github/workflows/ci.yml').read_text()
        self.assertIn('python3 scripts/focused_ci_gate.py chat-tenant-isolation', workflow)
        native = workflow.split('  native-test:', 1)[1].split('  native-node:', 1)[0]
        self.assertIn('OHC_CHAT_TEST_DATABASE_URL:', native)
        self.assertIn('ohc_chat_service_test', native)
        self.assertIn('make test-backend', native)

    def test_chat_preparation_keeps_actual_outbox_dependency_modules_and_types(self):
        import re
        import runpy
        root = Path(__file__).resolve().parents[1]
        folder = root/'scripts/chat-tenant-isolation'
        runpy.run_path(str(folder/'prepare.py'))
        generated = (folder/'generated.rs').read_text()
        for module, source in {
            'chat': 'src/server/services/chat/mod.rs',
            'omnichannel_repo': 'src/server/domain/repository/omnichannel_repo.rs',
            'redis_pool': 'src/server/redis_pool.rs',
        }.items():
            self.assertRegex(generated, r'#\[path=' + re.escape(json.dumps(str(root/source))) + r'\]\s*pub mod ' + module + ';')
        self.assertIn('pub mod services { pub use crate::chat; }', generated)
        self.assertIn('pub mod domain { pub mod repository { pub use crate::omnichannel_repo; } }', generated)
        db = (root/'src/server/db.rs').read_text()
        for item in ['enum DbStore', 'struct DB']:
            declaration = re.search(r'#\[derive\(Clone\)\]\npub ' + item + r' \{.*?\n\}', db, re.S)
            self.assertIsNotNone(declaration)
            self.assertIn(declaration.group(), generated)
        standalone = re.search(r'pub fn is_standalone_runtime\(\) -> bool \{.*?\n\}', (root/'src/server/lib.rs').read_text(), re.S)
        self.assertIn(standalone.group(), generated)
        self.assertIn('pub use server_config as config;', generated)
        self.assertNotIn('#[cfg(any())]', generated)
        self.assertNotIn('struct Mock', generated)

    def test_chat_preparation_declares_outbox_dependency_closure(self):
        import tomllib
        root = Path(__file__).resolve().parents[1]
        dependencies = tomllib.loads((root/'scripts/chat-tenant-isolation/Cargo.toml').read_text())['dependencies']
        for name in ['base64', 'tracing', 'redis', 'server_common', 'server_config']:
            self.assertIn(name, dependencies)
        self.assertEqual(dependencies['server_common']['path'], '../../src/server/common')
        self.assertEqual(dependencies['server_config']['path'], '../../src/server/config')
        self.assertTrue({'tokio-comp', 'connection-manager'}.issubset(dependencies['redis']['features']))
        self.assertIn('sqlite', dependencies['sqlx']['features'])

    def test_chat_preparation_fingerprints_imported_dependency_sources(self):
        import hashlib
        import runpy
        root = Path(__file__).resolve().parents[1]
        folder = root/'scripts/chat-tenant-isolation'
        runpy.run_path(str(folder/'prepare.py'))
        manifest = json.loads((folder/'source-manifest.json').read_text())
        paths = [root/p for p in ['Cargo.toml', 'Cargo.lock', 'src/server/lib.rs', 'src/server/db.rs', 'src/server/build.rs',
                                  'src/server/domain/repository/omnichannel_repo.rs', 'src/server/redis_pool.rs']]
        paths += list((root/'src/server/services/chat').rglob('*.rs'))
        paths += list((root/'src/server/migrations').glob('*.sql'))
        paths += [p for name in ['common', 'config'] for p in (root/'src/server'/name).rglob('*')
                  if p.is_file() and (p.suffix == '.rs' or p.name == 'Cargo.toml')]
        for path in paths:
            with self.subTest(path=path):
                self.assertEqual(manifest['inputs'].get(str(path.relative_to(root))), hashlib.sha256(path.read_bytes()).hexdigest())
        self.assertEqual(manifest['generated_sha256'], hashlib.sha256((folder/'generated.rs').read_bytes()).hexdigest())
        self.assertEqual(manifest['active_migration_source'], 'src/server/migrations')

    def test_bootstrap_portable_role_gate_is_mandatory_and_complete(self):
        self.assertIn('bootstrap-portable-roles', gate.GATES)
        minimum, database = gate.GATES['bootstrap-portable-roles']
        self.assertGreaterEqual(minimum, 9)
        self.assertEqual(database, 'OHC_SETUP_TEST_DATABASE_URL')
        self.assertTrue((Path(__file__).resolve().parents[1]/'scripts/bootstrap-portable-roles/run.sh').is_file())

    def test_link_bio_gate_requires_all_private_database_cases(self):
        minimum, database = gate.GATES['link-bio-isolation']
        self.assertGreaterEqual(minimum, 14)
        self.assertEqual(database, 'OHC_BIO_TEST_DATABASE_URL')
        self.assertTrue((Path(__file__).resolve().parents[1]/'scripts/link-bio-isolation/run.sh').is_file())

    def test_tenant_search_gate_runs_all_real_database_cases(self):
        minimum, database = gate.GATES['tenant-search']
        self.assertGreaterEqual(minimum, 13)
        self.assertEqual(database, 'OHC_SEARCH_TEST_DATABASE_URL')
        self.assertTrue((Path(__file__).resolve().parents[1]/'scripts/tenant-search/run.sh').is_file())

    def test_service_creation_gate_keeps_canonical_pg_and_original_sqlite_cases(self):
        minimum, database = gate.GATES['service-creation']
        self.assertGreaterEqual(minimum, 6)
        self.assertEqual(database, 'OHC_SERVICE_TEST_DATABASE_URL')
        runner = Path(__file__).resolve().parents[1]/'scripts/service-creation/run.sh'
        self.assertTrue(runner.is_file())

    def test_agent_workflow_gate_keeps_its_full_offline_inventory(self):
        minimum, database = gate.GATES['agent-workflow-contract']
        self.assertGreaterEqual(minimum, 108)
        root = Path(__file__).resolve().parents[1]
        workflow = (root/'.github/workflows/ci.yml').read_text()
        self.assertLess(workflow.index('Install the pinned workflow proxy witness'), workflow.index('Verify agent workflow tenant isolation and requested tasks offline'))
        self.assertLess(workflow.index('Install the pinned workflow proxy witness'), workflow.index('Verify durable PostgreSQL execution receipts and SQLite lifecycle parity'))
        self.assertIn('verify_node_lock.py', (root/'scripts/agent-workflow-contract/run.sh').read_text())
        self.assertIsNone(database)
        runner = Path(__file__).resolve().parents[1]/'scripts/agent-workflow-contract/run.sh'
        self.assertTrue(runner.is_file())

    def test_agent_workflow_generated_provider_tests_keep_the_real_schema_helper(self):
        import hashlib
        import re
        import runpy
        root = Path(__file__).resolve().parents[1]
        folder = root/'scripts/agent-workflow-contract'
        prepared = runpy.run_path(str(folder/'prepare.py'))
        generated = (folder/'generated.rs').read_text()
        wire = prepared['extract_item'](generated, 'mod', 'llm_wire_contract')
        helper = root/'src/agents/builtin/llm/structured_output_test.rs'
        self.assertRegex(wire, r'#\[cfg\(test\)\]\s*#\[path='
                         + re.escape(json.dumps(str(helper)))
                         + r'\]\s*mod structured_output_test;')
        self.assertNotIn('async fn array_request', wire, 'include the canonical helper, not a copied schema fixture')
        manifest = json.loads((folder/'source-manifest.json').read_text())
        for relative in ['src/agents/builtin/llm/structured_output_test.rs',
                         'src/agents/builtin/llm/mod.rs',
                         'src/agents/builtin/llm/anthropic.rs',
                         'src/agents/builtin/llm/openai.rs',
                         'src/agents/builtin/output_parser.rs']:
            self.assertEqual(manifest.get(relative), hashlib.sha256((root/relative).read_bytes()).hexdigest())

    def test_checkpoint_restore_gate_requires_all_cases_and_owned_database(self):
        minimum, database = gate.GATES['checkpoint-restore-contract']
        self.assertGreaterEqual(minimum, 37)
        self.assertEqual(database, 'OHC_CHECKPOINT_TEST_DATABASE_URL')
        root = Path(__file__).resolve().parents[1]
        workflow = (root/'.github/workflows/ci.yml').read_text()
        self.assertIn('python3 scripts/focused_ci_gate.py checkpoint-restore-contract', workflow)
        self.assertNotIn('run: bash scripts/checkpoint-restore-contract/run.sh', workflow)
        manifest_builder = (root/'scripts/checkpoint-restore-contract/prepare.py').read_text()
        self.assertIn('scripts/focused_ci_gate.py', manifest_builder)
        self.assertIn('scripts/test_focused_ci_gate.py', manifest_builder)

    def test_checkpoint_restore_gate_rejects_partial_ignored_and_filtered_inventory(self):
        minimum, _ = gate.GATES['checkpoint-restore-contract']
        complete = 'test result: ok. 37 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;'
        self.assertEqual(gate.validate_results(complete, minimum), 37)
        for result in [
            'ok. 36 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out',
            'ok. 37 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out',
            'ok. 37 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out',
            'FAILED. 37 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out',
        ]:
            with self.subTest(result=result), self.assertRaises(ValueError):
                gate.validate_results('test result: '+result+';', minimum)

    def test_counts_real_passes_and_permits_empty_doctest_target(self):
        self.assertEqual(gate.validate_results('test result: ok. 25 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1s\ntest result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0s', 25), 25)

    def test_zero_discovery_cannot_pass(self):
        with self.assertRaises(ValueError):
            gate.validate_results('test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0s', 1)

    def test_missing_or_partial_discovery_cannot_pass(self):
        for log in ['', 'test result: ok. 24 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1s']:
            with self.subTest(log=log), self.assertRaises(ValueError):
                gate.validate_results(log, 25)

    def test_ignored_filtered_or_failed_tests_cannot_pass(self):
        for result in ['ok. 25 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out', 'ok. 25 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out', 'FAILED. 25 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out']:
            with self.subTest(result=result), self.assertRaises(ValueError):
                gate.validate_results('test result: '+result+'; finished in 1s', 25)

    def fixture(self, root, output, code=0, manifest=True):
        directory = root / 'scripts/quote-acceptance'
        directory.mkdir(parents=True)
        if manifest:
            (directory / 'source-manifest.json').write_text(json.dumps({'production.rs': 'abc'}))
        (directory / 'run.sh').write_text('#!/usr/bin/env bash\nprintf %s '+repr(output)+'\nexit '+str(code)+'\n')

    def test_real_child_success_retains_log_and_manifest(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            self.fixture(root, 'test result: ok. 36 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1s')
            self.assertEqual(gate.run_gate(root, 'quote-acceptance', root/'evidence'), 0)
            receipt = json.loads((root/'evidence/quote-acceptance/result.json').read_text())
            self.assertEqual(receipt['status'], 'passed')
            self.assertEqual(receipt['passed'], 36)
            self.assertEqual((root/'evidence/quote-acceptance/source-manifest.json').read_bytes(), (root/'scripts/quote-acceptance/source-manifest.json').read_bytes())

    def test_nonzero_child_exit_cannot_be_hidden_by_green_output(self):
        with tempfile.TemporaryDirectory() as temp:
            root=Path(temp)
            self.fixture(root, 'test result: ok. 36 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1s', 7)
            self.assertNotEqual(gate.run_gate(root, 'quote-acceptance', root/'evidence'), 0)
            self.assertEqual(json.loads((root/'evidence/quote-acceptance/result.json').read_text())['exit_code'], 7)

    def test_missing_source_manifest_cannot_pass(self):
        with tempfile.TemporaryDirectory() as temp:
            root=Path(temp)
            self.fixture(root, 'test result: ok. 36 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1s', manifest=False)
            self.assertNotEqual(gate.run_gate(root, 'quote-acceptance', root/'evidence'), 0)

    def test_missing_paired_source_cannot_pass(self):
        with tempfile.TemporaryDirectory() as temp:
            root=Path(temp)
            self.assertNotEqual(gate.run_gate(root, 'quote-acceptance', root/'evidence'), 0)

    def test_workflow_has_independent_required_gates_and_retained_artifacts(self):
        import yaml
        source = yaml.safe_load((Path(__file__).resolve().parents[1]/'.github/workflows/ci.yml').read_text())
        for name, (_, required) in gate.GATES.items():
            job_name = 'native-test' if name == 'checkpoint-restore-contract' else 'postgres-security'
            job = source['jobs'][job_name]
            self.assertIn(job_name, source['jobs']['ci-required']['needs'])
            matches=[step for step in job['steps'] if f'focused_ci_gate.py {name}' in step.get('run','')]
            self.assertEqual(len(matches), 1, name)
            self.assertIn('!cancelled()', matches[0]['if'])
            if required:
                self.assertIn(required, matches[0]['env'])
                if name == 'checkpoint-restore-contract':
                    earlier = job['steps'][:job['steps'].index(matches[0])]
                    self.assertTrue(any('createdb ' in step.get('run', '') and 'ohc_checkpoint_test' in step['run'] for step in earlier))
                else:
                    self.assertIn('createdb ', matches[0]['run'])
        for job_name in ['postgres-security', 'native-test']:
            uploads=[s for s in source['jobs'][job_name]['steps'] if s.get('uses','').startswith('actions/upload-artifact@')]
            self.assertTrue(any(s.get('if')=='always()' and s['with']['path']=='target/focused-ci-results' for s in uploads), job_name)


class WorkflowWitnessLockTests(unittest.TestCase):
    def setUp(self):
        import shutil
        self.root = Path(__file__).resolve().parents[1]
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        sandbox = Path(temporary.name)
        self.here = sandbox/'scripts/agent-workflow-contract'
        self.here.mkdir(parents=True)
        client = sandbox/'src/ui/next'
        client.mkdir(parents=True)
        shutil.copyfile(self.root/'scripts/agent-workflow-contract/verify_node_lock.py', self.here/'verify_node_lock.py')
        shutil.copyfile(self.root/'src/ui/next/package-lock.json', client/'package-lock.json')
        canonical = json.loads((client/'package-lock.json').read_text())['packages']
        self.dependencies = {name: canonical[f'node_modules/{name}']['version'] for name in ['jose', 'typescript']}
        self.packages = {'': {'dependencies': self.dependencies.copy()},
                         **{f'node_modules/{name}': canonical[f'node_modules/{name}'].copy() for name in self.dependencies}}
        for name in self.dependencies:
            self.packages[f'node_modules/{name}'].pop('dev', None)

    def verify(self, dependencies, packages):
        import subprocess
        import sys
        (self.here/'package.json').write_text(json.dumps({'dependencies': dependencies}))
        (self.here/'package-lock.json').write_text(json.dumps({'packages': packages}))
        return subprocess.run([sys.executable, str(self.here/'verify_node_lock.py')], capture_output=True, text=True)

    def test_complete_witness_matches_both_canonical_packages(self):
        result = self.verify(self.dependencies, self.packages)
        self.assertEqual(result.returncode, 0, result.stderr)

    def test_typescript_only_witness_cannot_certify_the_session_proxy(self):
        del self.dependencies['jose']
        del self.packages['']['dependencies']['jose']
        del self.packages['node_modules/jose']
        self.assertNotEqual(self.verify(self.dependencies, self.packages).returncode, 0)

    def test_witness_rejects_package_drift_and_unpaired_or_extra_dependencies(self):
        import copy
        for name in self.dependencies:
            for field in ['version', 'resolved', 'integrity']:
                with self.subTest(package=name, field=field):
                    packages = copy.deepcopy(self.packages)
                    packages[f'node_modules/{name}'][field] = 'mismatched'
                    self.assertNotEqual(self.verify(self.dependencies, packages).returncode, 0)
        for change in ['manifest_pin', 'lock_pin', 'extra_manifest', 'extra_package', 'missing_package', 'dev_only']:
            with self.subTest(change=change):
                dependencies = self.dependencies.copy()
                packages = copy.deepcopy(self.packages)
                if change == 'manifest_pin': dependencies['jose'] = '^'+dependencies['jose']
                elif change == 'lock_pin': packages['']['dependencies']['jose'] = '^'+dependencies['jose']
                elif change == 'extra_manifest': dependencies['unrelated'] = '1.0.0'
                elif change == 'extra_package': packages['node_modules/unrelated'] = {'version': '1.0.0'}
                elif change == 'dev_only': packages['node_modules/jose']['dev'] = True
                else: del packages['node_modules/jose']
                self.assertNotEqual(self.verify(dependencies, packages).returncode, 0)

    def test_receipt_source_manifest_binds_the_shared_witness_installation(self):
        import hashlib
        import runpy
        folder = self.root/'scripts/agent-receipt-postgres-contract'
        runpy.run_path(str(folder/'prepare.py'))
        manifest = json.loads((folder/'source-manifest.json').read_text())
        paths = [self.root/'scripts/agent-workflow-contract'/name for name in ['package.json', 'package-lock.json']]
        paths += [self.root/path for path in [
            'src/server/lib.rs', 'src/server/api/dynamic_workflows.rs',
            'src/server/orchestration/dynamic_workflows.rs', 'src/server/queue.rs',
            'src/server/migrations/104_sub_agent_queue.sql',
            'scripts/agent-receipt-postgres-contract/dynamic_workflow_test.rs',
            'scripts/agent-receipt-postgres-contract/dynamic_queue_test.rs',
            'scripts/agent-receipt-postgres-contract/Cargo.toml',
            'scripts/agent-receipt-postgres-contract/Cargo.lock',
            'scripts/rust_source.py',
        ]]
        for path in paths:
            self.assertEqual(manifest.get(str(path.relative_to(self.root))), hashlib.sha256(path.read_bytes()).hexdigest())


if __name__ == '__main__':
    unittest.main()
