import os
from pathlib import Path
import subprocess
import unittest
from database_guard import validate

class DisposableDatabaseGuard(unittest.TestCase):
    def test_explicit_loopback_test_database(self):
        for raw in ('postgres://127.0.0.1:55439/ohc_milestone_test', 'postgresql://[::1]/ohc_milestone_test'):
            validate(raw)

    def test_unsafe_targets_are_rejected_without_connection(self):
        for raw in ('postgres://127.0.0.1/production', 'postgres://db.example.test/ohc_milestone_test',
                    'postgres://127.0.0.1/ohc__test', 'postgres://127.0.0.1/ohc_milestone_test?host=remote',
                    'postgres://127.0.0.1/ohc_milestone_test#fragment', 'postgres://127.0.0.1/ohc_milestone_test/extra',
                    'postgres:///ohc_milestone_test', 'postgres://127.0.0.1/ohc_%2F_test', 'mysql://127.0.0.1/ohc_milestone_test'):
            with self.subTest(raw=raw), self.assertRaises(ValueError):
                validate(raw)

    def test_wrapper_rejects_before_any_cargo_or_database_action(self):
        root = Path(__file__).resolve().parents[2]
        env = {**os.environ, 'OHC_MILESTONE_TEST_DATABASE_URL': 'postgres://127.0.0.1/production', 'PATH': '/usr/bin:/bin'}
        result = subprocess.run(['bash', str(root / 'scripts/order-milestones/run.sh')], env=env,
                                text=True, capture_output=True, timeout=5)
        self.assertEqual(result.returncode, 1)
        self.assertIn('explicit loopback PostgreSQL URL', result.stderr)
        self.assertNotIn('cargo', result.stderr.lower())
        self.assertEqual(result.stdout, '')

if __name__ == '__main__':
    unittest.main()
