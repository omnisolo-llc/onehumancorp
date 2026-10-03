"""Keep explicit checkpoint database prerequisites in the native backend gate."""
from pathlib import Path
import unittest
import yaml

ROOT = Path(__file__).resolve().parents[2]

class CheckpointCiTests(unittest.TestCase):
    def test_native_backend_provisions_and_passes_checkpoint_database(self):
        workflow = yaml.safe_load((ROOT / '.github/workflows/ci.yml').read_text())
        steps = workflow['jobs']['native-test']['steps']
        backend = next(step for step in steps if step.get('run') == 'make test-backend')
        url = backend.get('env', {}).get('OHC_CHECKPOINT_TEST_DATABASE_URL', '')
        self.assertIn('127.0.0.1:', url)
        self.assertTrue(url.endswith('/ohc_checkpoint_test'))
        earlier = steps[:steps.index(backend)]
        self.assertTrue(any('createdb' in step.get('run', '') and 'ohc_checkpoint_test' in step['run'] for step in earlier))
        focused = next(step for step in steps if step.get('run') == 'python3 scripts/focused_ci_gate.py checkpoint-restore-contract')
        self.assertEqual(focused['env']['OHC_CHECKPOINT_TEST_DATABASE_URL'], url)
        self.assertEqual(focused['env']['RUST_TEST_THREADS'], '1')
        self.assertFalse(focused.get('continue-on-error', False))

if __name__ == '__main__':
    unittest.main()
