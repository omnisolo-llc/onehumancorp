import subprocess
import unittest

class DayOneAuditTest(unittest.TestCase):
    def test_cli_help(self):
        """Test that the ohc_hybrid_cli.sh script exists and runs."""
        result = subprocess.run(
            ['bash', 'deploy/scripts/ohc_hybrid_cli.sh'],
            input=b'0\n',
            capture_output=True
        )
        self.assertIn(b'Exiting...', result.stdout)
        self.assertEqual(result.returncode, 0)

if __name__ == '__main__':
    unittest.main()
