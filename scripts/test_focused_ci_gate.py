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
        self.assertGreaterEqual(minimum, 13)
        self.assertEqual(database, 'OHC_CHAT_TEST_DATABASE_URL')
        root = Path(__file__).resolve().parents[1]
        self.assertTrue((root/'scripts/chat-tenant-isolation/run.sh').is_file())
        workflow = (root/'.github/workflows/ci.yml').read_text()
        self.assertIn('python3 scripts/focused_ci_gate.py chat-tenant-isolation', workflow)
        native = workflow.split('  native-test:', 1)[1].split('  native-node:', 1)[0]
        self.assertIn('OHC_CHAT_TEST_DATABASE_URL:', native)
        self.assertIn('ohc_chat_service_test', native)
        self.assertIn('make test-backend', native)

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
            self.fixture(root, 'test result: ok. 25 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1s')
            self.assertEqual(gate.run_gate(root, 'quote-acceptance', root/'evidence'), 0)
            receipt = json.loads((root/'evidence/quote-acceptance/result.json').read_text())
            self.assertEqual(receipt['passed'], 25)
            self.assertEqual(receipt['status'], 'passed')
            self.assertEqual((root/'evidence/quote-acceptance/source-manifest.json').read_bytes(), (root/'scripts/quote-acceptance/source-manifest.json').read_bytes())

    def test_nonzero_child_exit_cannot_be_hidden_by_green_output(self):
        with tempfile.TemporaryDirectory() as temp:
            root=Path(temp)
            self.fixture(root, 'test result: ok. 25 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1s', 7)
            self.assertNotEqual(gate.run_gate(root, 'quote-acceptance', root/'evidence'), 0)
            self.assertEqual(json.loads((root/'evidence/quote-acceptance/result.json').read_text())['exit_code'], 7)

    def test_missing_source_manifest_cannot_pass(self):
        with tempfile.TemporaryDirectory() as temp:
            root=Path(temp)
            self.fixture(root, 'test result: ok. 25 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1s', manifest=False)
            self.assertNotEqual(gate.run_gate(root, 'quote-acceptance', root/'evidence'), 0)

    def test_missing_paired_source_cannot_pass(self):
        with tempfile.TemporaryDirectory() as temp:
            root=Path(temp)
            self.assertNotEqual(gate.run_gate(root, 'quote-acceptance', root/'evidence'), 0)

    def test_workflow_has_independent_required_gates_and_retained_artifacts(self):
        import yaml
        source = yaml.safe_load((Path(__file__).resolve().parents[1]/'.github/workflows/ci.yml').read_text())
        job = source['jobs']['postgres-security']
        for name, (_, required) in gate.GATES.items():
            matches=[step for step in job['steps'] if f'focused_ci_gate.py {name}' in step.get('run','')]
            self.assertEqual(len(matches), 1, name)
            self.assertIn('!cancelled()', matches[0]['if'])
            if required:
                self.assertIn(required, matches[0]['env'])
                self.assertIn('createdb ', matches[0]['run'])
        self.assertIn('postgres-security', source['jobs']['ci-required']['needs'])
        uploads=[s for s in job['steps'] if s.get('uses','').startswith('actions/upload-artifact@')]
        self.assertTrue(any(s.get('if')=='always()' and s['with']['path']=='target/focused-ci-results' for s in uploads))


if __name__ == '__main__':
    unittest.main()
