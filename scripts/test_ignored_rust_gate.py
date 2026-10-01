import importlib.util
import pathlib
import tempfile
import unittest
from unittest.mock import patch
import json
import os
import signal
import sys
import time

spec = importlib.util.spec_from_file_location('ignored_gate', pathlib.Path(__file__).with_name('ignored_rust_gate.py'))
gate = importlib.util.module_from_spec(spec)
spec.loader.exec_module(gate)


class ExactIgnoredGate(unittest.TestCase):
    def test_application_mode_uses_only_the_validated_disposable_database(self):
        with tempfile.TemporaryDirectory() as temp:
            root = pathlib.Path(temp)
            sdk = root / 'target/ignored-prerequisites/openharness'
            (sdk / 'src/harness').mkdir(parents=True)
            (sdk / 'src/harness/__init__.py').write_text('')
            pg = 'postgres://fixture:fixture@127.0.0.1:15432/ohc_ignored_postgres'
            source = {
                'PATH': '/bin', 'OHC_IGNORED_SERVICE_ISOLATION': '1',
                'OHC_IGNORED_POSTGRES_URL': pg,
                'OHC_IGNORED_MYSQL_URL': 'mysql://fixture:fixture@127.0.0.1:13306/ohc_ignored_mysql',
                'OHC_IGNORED_REDIS_URL': 'redis://127.0.0.1:16379',
                'OPENHARNESS_PINNED_SDK_SOURCE': str(sdk),
                'DATABASE_URL': 'postgres://unrelated.example:5432/production',
                'OMNISOLO_DATABASE_URL': 'postgres://unrelated.example:5432/production',
                'DATABASE_URL_FILE': '/unrelated/database-secret',
                'OMNISOLO_STANDALONE_MODE': 'true',
            }
            with patch.object(gate.subprocess, 'check_output',
                              side_effect=[gate.SDK_REVISION + '\n', '', '1.18.15\n']):
                env = gate.prerequisites(root, source)
            self.assertEqual(env.get('OMNISOLO_DATABASE_URL'), pg)
            self.assertEqual(env['OMNISOLO_STANDALONE_MODE'], 'false')
            self.assertEqual(env['REDIS_URL'], source['OHC_IGNORED_REDIS_URL'])
            self.assertNotIn('DATABASE_URL', env)
            self.assertNotIn('DATABASE_URL_FILE', env)

    @unittest.skipUnless(os.name == 'posix', 'owned process groups require POSIX')
    def test_timeout_retains_output_and_stops_owned_descendants(self):
        with tempfile.TemporaryDirectory() as temp:
            root = pathlib.Path(temp)
            marker = root / 'owned-pids.json'
            child = root / 'child.py'
            child.write_text("import subprocess,sys,os,json,time,signal\n"
                "signal.signal(signal.SIGTERM, signal.SIG_IGN)\n"
                "grandchild=subprocess.Popen([sys.executable,'-c','import signal,time;signal.signal(signal.SIGTERM,signal.SIG_IGN);time.sleep(60)'])\n"
                "open(sys.argv[1],'w').write(json.dumps([os.getpid(),grandchild.pid]))\n"
                "print('owned child started',flush=True)\n"
                "time.sleep(60)\n")
            try:
                code, output = gate.execute_case([sys.executable, str(child), str(marker)], root,
                    gate.child_environment(os.environ), root / 'case.log', timeout=0.5)
                self.assertNotEqual(code, 0)
                self.assertIn('owned child started', output)
                self.assertIn('owned child started', (root / 'case.log').read_text())
                self.assertIn('timed out', output)
                _parent, descendant = json.loads(marker.read_text())
                for _ in range(40):
                    status = pathlib.Path(f'/proc/{descendant}/stat')
                    if not status.exists() or status.read_text().split()[2] == 'Z':
                        break
                    time.sleep(0.025)
                else:
                    self.fail('owned descendant still executes after timeout')
            finally:
                if marker.exists():
                    parent, _ = json.loads(marker.read_text())
                    try: os.killpg(parent, signal.SIGKILL)
                    except ProcessLookupError: pass

    def test_inventory_is_fourteen_exact_local_cases(self):
        self.assertEqual(len(gate.CASES), 14)
        self.assertEqual(len({case[3] for case in gate.CASES}), 14)
        for case in gate.CASES:
            command = gate.command(case)
            self.assertIn('--exact', command)
            self.assertIn('--ignored', command)
            self.assertNotIn('--include-ignored', command)
            self.assertNotIn('live_', case[3])

    def test_success_requires_exact_name_and_one_actual_execution(self):
        gate.validate_result('named_case', 'test named_case ... ok\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 11 filtered out;', 0)
        for name, log, code in [
            ('other', 'test named_case ... ok\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;', 0),
            ('named_case', 'test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out;', 0),
            ('named_case', 'test named_case ... ignored\ntest result: ok. 0 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out;', 0),
            ('named_case', 'test named_case ... ok\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;', 1),
            ('named_case', 'test named_case ... ok\ntest result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;', 0),
        ]:
            with self.subTest(log=log, code=code), self.assertRaises(ValueError):
                gate.validate_result(name, log, code)

    def test_credentials_and_live_optins_are_not_inherited(self):
        result = gate.child_environment({'PATH': '/bin', 'CARGO_TARGET_DIR': '/target',
            'OPENAI_API_KEY': 'private', 'SUB2API_API_KEY': 'private', 'GITHUB_TOKEN': 'private',
            'OMNISOLO_LIVE_CODEX_E2E': '1', 'OMNISOLO_LIVE_HARNESS_E2E': '1'})
        self.assertEqual(result['PATH'], '/bin')
        self.assertEqual(result['CARGO_TARGET_DIR'], '/target')
        for key in ['OPENAI_API_KEY', 'SUB2API_API_KEY', 'GITHUB_TOKEN', 'OMNISOLO_LIVE_CODEX_E2E', 'OMNISOLO_LIVE_HARNESS_E2E']:
            self.assertNotIn(key, result)

    def test_service_validation_rejects_remote_or_non_disposable_targets(self):
        good = 'postgres://fixture:fixture@127.0.0.1:15432/ohc_ignored_postgres'
        self.assertEqual(gate.validate_service_url(good, 'postgres', 'ohc_ignored_postgres'), good)
        for bad in ['postgres://hosted.example:5432/ohc_ignored_postgres', 'postgres://127.0.0.1:5432/production',
                    'postgres://127.0.0.1/ohc_ignored_postgres', good+'?options=remote', good+'#fragment']:
            with self.subTest(url=bad), self.assertRaises(ValueError):
                gate.validate_service_url(bad, 'postgres', 'ohc_ignored_postgres')

    def test_missing_prerequisites_leave_a_failed_receipt_without_running_tests(self):
        with tempfile.TemporaryDirectory() as temp:
            destination = pathlib.Path(temp) / 'evidence'
            with patch.dict('os.environ', {}, clear=True), patch.object(gate.subprocess, 'check_output', return_value=b'') as listing, patch.object(gate.subprocess, 'Popen') as process:
                self.assertEqual(gate.run(pathlib.Path(temp), destination), 1)
            process.assert_not_called()
            receipt = json.loads((destination / 'result.json').read_text())
            self.assertEqual(receipt['status'], 'failed')
            self.assertEqual(receipt['cases'], [])
            self.assertTrue((destination / 'source-manifest.json').is_file())
            self.assertIn('explicitly declared', receipt['error'])


if __name__ == '__main__':
    unittest.main()
