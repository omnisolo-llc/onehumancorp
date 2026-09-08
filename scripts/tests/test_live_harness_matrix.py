"""Offline behavioral checks for the opt-in live runner."""
import json
import os
from pathlib import Path
import subprocess
import tempfile
import unittest

RUNNER = Path(__file__).resolve().parents[1] / 'test-live-harness-matrix.sh'


class MatrixRunnerTests(unittest.TestCase):
    def run_matrix(self, successful=False, **options):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            log = root / 'calls'
            for command in ('curl', 'docker', 'cargo'):
                stub = root / command
                stub.write_text('#!/bin/bash\necho called >> "$CALL_LOG"\nexit 91\n')
                if successful:
                    stub.write_text("#!/usr/bin/env python3\n" + r'''import json, os, sys
args = sys.argv[1:]
command = os.path.basename(sys.argv[0])
if command == 'curl':
    path = args[args.index('--output') + 1]
    data = {'data':[{'id':'gpt-5.6-luna'}]} if args[-1].endswith('/models') else {'text':'OMNISOLO_PROVIDER_PREFLIGHT_OK'}
    open(path, 'w').write(json.dumps(data))
    print('200', end='')
elif command == 'docker':
    if args[0] == 'inspect': print('true')
elif command == 'cargo':
    if os.environ.get('FAIL_DIAGNOSTIC') == '1':
        print('test failed: sentinel mismatch ' + os.environ['OPENAI_API_KEY'])
        sys.exit(1)
    if os.environ.get('DROP_EVIDENCE') != '1':
        print('OMNISOLO_LIVE_RESULT=' + json.dumps({
            'schema':'omnisolo.live_harness_result.v1', 'status':'passed',
            'harness_id':os.environ['OMNISOLO_LIVE_HARNESS_ID'],
            'model':'gpt-5.6-luna', 'reasoning_effort':'max', 'native_session_deleted':True,
            'evidence':{'reasoning_translation':{'kind':'native','requested':'max','effective':'xhigh'},
                        'assistant_text':'OMNISOLO_LIVE_HARNESS_OK',
                        'usage':[{'total_tokens':3}], 'usage_observed':True,
                        'terminal_success_observed':True, 'provider_marker_observed':True,
                        'shared_local_services':{'status':'bound'}}}))
''')
                stub.chmod(0o755)
            env = {key: value for key, value in os.environ.items()
                   if not key.startswith(('OPENAI_', 'SUB2API_', 'OMNISOLO_LIVE_', 'OMNISOLO_RUN_LIVE_'))}
            env.update(PATH=f'{root}:{env["PATH"]}', CALL_LOG=str(log), **options)
            result = subprocess.run(['bash', str(RUNNER)], env=env, text=True,
                                    capture_output=True, timeout=10)
            calls = log.read_text() if log.exists() else ''
            return result, calls

    def test_without_opt_in_skips_before_credentials_or_external_commands(self):
        result, calls = self.run_matrix()
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(json.loads(result.stdout)['status'], 'skipped')
        self.assertEqual(calls, '')

    def test_unknown_selection_fails_before_external_commands(self):
        result, calls = self.run_matrix(OMNISOLO_RUN_LIVE_HARNESS_E2E='1',
                                       OPENAI_API_KEY='canary', OMNISOLO_LIVE_HARNESSES='typo')
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('unknown harness', result.stderr)
        self.assertEqual(calls, '')

    def test_empty_selection_fails_before_external_commands(self):
        result, calls = self.run_matrix(OMNISOLO_RUN_LIVE_HARNESS_E2E='1',
                                       OPENAI_API_KEY='canary', OMNISOLO_LIVE_HARNESSES=' , ')
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('empty harness selection', result.stderr)
        self.assertEqual(calls, '')

    def test_subset_report_preserves_evidence_without_claiming_native_gate(self):
        result, _ = self.run_matrix(successful=True, OMNISOLO_RUN_LIVE_HARNESS_E2E='1',
                                   OPENAI_API_KEY='canary', OMNISOLO_LIVE_HARNESSES='pi',
                                   OMNISOLO_LIVE_BUILD_IMAGES='0')
        self.assertEqual(result.returncode, 0, result.stderr)
        report = json.loads(result.stdout)
        self.assertEqual(report['native_gate'], 'not_run')
        self.assertEqual(report['results'][0]['evidence']['usage'], [{'total_tokens':3}])
        self.assertEqual(report['results'][0]['evidence']['reasoning_translation']['effective'], 'xhigh')
        self.assertNotIn('shared_service_probe', report['results'][0])

    def test_successful_test_exit_without_evidence_fails_matrix(self):
        result, _ = self.run_matrix(successful=True, OMNISOLO_RUN_LIVE_HARNESS_E2E='1',
                                   OPENAI_API_KEY='canary', OMNISOLO_LIVE_HARNESSES='pi',
                                   OMNISOLO_LIVE_BUILD_IMAGES='0', OMNISOLO_LIVE_ATTEMPTS='1',
                                   DROP_EVIDENCE='1')
        self.assertNotEqual(result.returncode, 0)

    def test_compatibility_harness_requires_complete_native_gate(self):
        result, _ = self.run_matrix(successful=True, OMNISOLO_RUN_LIVE_HARNESS_E2E='1',
                                   OPENAI_API_KEY='canary', OMNISOLO_LIVE_HARNESSES='aider',
                                   OMNISOLO_LIVE_BUILD_IMAGES='0')
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(json.loads(result.stdout)['results'][0]['failure_class'], 'native_gate_incomplete')

    def test_failed_test_diagnostics_are_visible_without_provider_credentials(self):
        result, _ = self.run_matrix(successful=True, OMNISOLO_RUN_LIVE_HARNESS_E2E='1',
                                   OPENAI_API_KEY='diagnostic-secret-canary', OMNISOLO_LIVE_HARNESSES='pi',
                                   OMNISOLO_LIVE_BUILD_IMAGES='0', OMNISOLO_LIVE_ATTEMPTS='1',
                                   FAIL_DIAGNOSTIC='1')
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('test failed: sentinel mismatch', result.stderr)
        self.assertNotIn('diagnostic-secret-canary', result.stdout + result.stderr)


if __name__ == '__main__':
    unittest.main()
