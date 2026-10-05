import pathlib
import unittest

import yaml

ROOT = pathlib.Path(__file__).resolve().parents[2]


class IgnoredRustCiContract(unittest.TestCase):
    def test_affiliate_database_is_created_before_backend_tests_with_its_own_url(self):
        workflow = yaml.safe_load((ROOT / '.github/workflows/ci.yml').read_text())
        job = workflow['jobs']['native-test']
        steps = job['steps']
        backend = next(s for s in steps if s.get('run') == 'make test-backend')
        self.assertEqual(
            backend.get('env', {}).get('OHC_AFFILIATE_TEST_DATABASE_URL'),
            'postgres://postgres:ignored_fixture@127.0.0.1:${{ job.services.ignored_postgres.ports[5432] }}/ohc_affiliate_test',
        )
        fixture = next((s for s in steps if s.get('run') ==
            'createdb --host=127.0.0.1 --port=${{ job.services.ignored_postgres.ports[5432] }} --username=postgres ohc_affiliate_test'), None)
        self.assertIsNotNone(fixture, 'affiliate aggregates require their own real database')
        self.assertLess(steps.index(fixture), steps.index(backend))
        self.assertEqual(fixture['env']['PGPASSWORD'], 'ignored_fixture')
        self.assertEqual(fixture['if'], backend['if'])
        self.assertFalse(fixture.get('continue-on-error', False))
        self.assertFalse(backend.get('continue-on-error', False))
        for env in (workflow.get('env', {}), job.get('env', {}), backend['env']):
            self.assertNotIn('OMNISOLO_DATABASE_URL', env)

    def test_native_job_runs_exact_safe_inventory_after_primary_tests(self):
        job = yaml.safe_load((ROOT / '.github/workflows/ci.yml').read_text())['jobs']['native-test']
        steps = job['steps']
        gate = next((s for s in steps if 'ignored_rust_gate.py run' in s.get('run', '')), None)
        self.assertIsNotNone(gate, 'safe ignored tests need a real mandatory execution step')
        self.assertFalse(gate.get('continue-on-error', False))
        self.assertIn('!cancelled()', gate['if'])
        self.assertGreater(steps.index(gate), next(i for i, s in enumerate(steps) if s.get('run') == 'make test-backend'))
        self.assertEqual(set(job['services']), {'ignored_postgres', 'ignored_mysql', 'ignored_redis'})
        self.assertEqual(job['services']['ignored_postgres']['env']['POSTGRES_DB'], 'ohc_ignored_postgres')
        self.assertEqual(job['services']['ignored_mysql']['env']['MYSQL_DATABASE'], 'ohc_ignored_mysql')
        self.assertEqual(gate['env']['OHC_IGNORED_SERVICE_ISOLATION'], '1')
        self.assertNotIn('OMNISOLO_LIVE_CODEX_E2E', gate['env'])
        self.assertNotIn('OMNISOLO_LIVE_HARNESS_E2E', gate['env'])

    def test_sdk_source_is_exact_pinned_and_reports_are_retained_on_failure(self):
        steps = yaml.safe_load((ROOT / '.github/workflows/ci.yml').read_text())['jobs']['native-test']['steps']
        checkout = next((s for s in steps if s.get('with', {}).get('repository') == 'AgentBoardTT/openharness'), None)
        self.assertIsNotNone(checkout)
        self.assertEqual(checkout['with']['ref'], '85c54682a209ca7c3fc8b1ab2e820b6724dc3028')
        self.assertFalse(checkout['with']['persist-credentials'])
        upload = next((s for s in steps if s.get('with', {}).get('name') == 'ignored-rust-execution'), None)
        self.assertIsNotNone(upload)
        self.assertIn('always()', upload['if'])
        self.assertEqual(upload['with']['if-no-files-found'], 'error')


if __name__ == '__main__':
    unittest.main()
