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
    def test_service_creation_gate_keeps_canonical_pg_and_original_sqlite_cases(self):
        minimum, database = gate.GATES['service-creation']
        self.assertGreaterEqual(minimum, 6)
        self.assertEqual(database, 'OHC_SERVICE_TEST_DATABASE_URL')
        runner = Path(__file__).resolve().parents[1]/'scripts/service-creation/run.sh'
        self.assertTrue(runner.is_file())

    def test_agent_workflow_gate_keeps_its_full_offline_inventory(self):
        minimum, database = gate.GATES['agent-workflow-contract']
        self.assertGreaterEqual(minimum, 9)
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
