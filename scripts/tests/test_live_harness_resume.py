import copy
import importlib.util
from pathlib import Path
import unittest

spec = importlib.util.spec_from_file_location('resume', Path(__file__).resolve().parents[1] / 'live-harness-resume.py')
resume = importlib.util.module_from_spec(spec)
spec.loader.exec_module(resume)


def report_fixture():
    rows = []
    for index, name in enumerate(resume.HARNESSES):
        rows.append({'harness_id':name, 'model':'model', 'reasoning_effort':'max',
            'status':'failed' if index == 11 else 'passed', 'native_session_deleted':True,
            'integration_mode':'native' if index < 8 else 'openai_compatible',
            'evidence':{'credential_leak_observed':False, 'usage_observed':True, 'usage':[{'total_tokens':1}],
                'terminal_success_observed':True, 'provider_marker_observed':True,
                'shared_local_services':{'status':'operations_verified', 'writer_verified':True, 'reader_verified':True,
                    'writer_key':'key', 'writer_value':'value', 'cross_harness_read_verified':True,
                    'writer_operations':{'attempt_id':f'{name}-writer', 'operations':sorted(resume.WRITE)},
                    'reader_operations':{'attempt_id':f'{name}-reader', 'operations':sorted(resume.READ)}}}})
    return {'schema':'omnisolo.live_harness_matrix.v1', 'status':'failed', 'native_gate':'passed',
        'model':'model', 'reasoning_effort':'max', 'results':rows}


class ResumeTests(unittest.TestCase):
    def test_nested_native_token_counters_are_supported(self):
        self.assertTrue(resume.token_usage([{'tokens': {'input': 2, 'output': 1}}]))
        self.assertFalse(resume.token_usage([{'cost': 3, 'tokens': {'total': 0}}]))

    def test_complete_native_gate_can_resume_failed_shim(self):
        report = report_fixture()
        self.assertIs(resume.validate(report, 'model', 'max'), report)

    def test_invalid_gate_is_rejected(self):
        original = report_fixture()
        mutations = [
            lambda r:r.update(native_gate='failed'),
            lambda r:r['results'][0].update(status='failed'),
            lambda r:r['results'].pop(0),
            lambda r:r['results'][2].update(harness_id='codex'),
            lambda r:r.update(model='different'),
            lambda r:r.update(reasoning_effort='low'),
            lambda r:r['results'][1]['evidence'].update(credential_leak_observed=True),
            lambda r:r['results'][1]['evidence'].update(usage=[]),
            lambda r:r['results'][1]['evidence'].update(usage=[{'total_tokens':0}]),
            lambda r:r['results'][1]['evidence']['shared_local_services'].update(status='bound'),
            lambda r:r['results'][1]['evidence']['shared_local_services'].update(cross_harness_read_verified=False),
            lambda r:r['results'][1]['evidence']['shared_local_services']['writer_operations'].update(operations=[]),
            lambda r:r['results'][1]['evidence']['shared_local_services']['reader_operations'].update(attempt_id='codex-writer'),
        ]
        for mutate in mutations:
            report = copy.deepcopy(original)
            mutate(report)
            with self.subTest(mutation=mutate), self.assertRaises(ValueError):
                resume.validate(report, 'model', 'max')
