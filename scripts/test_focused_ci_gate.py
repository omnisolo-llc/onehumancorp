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
        self.assertGreaterEqual(minimum, 56)
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


if __name__ == '__main__':
    unittest.main()
