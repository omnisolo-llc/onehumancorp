import subprocess
import unittest
import os

class DayOneAuditTest(unittest.TestCase):
    def test_cli_help(self):
        """Test that the ohc_hybrid_cli.sh script exists and runs."""
        script_path = os.path.join(os.path.dirname(__file__), '..', '..', 'deploy', 'scripts', 'ohc_hybrid_cli.sh')
        result = subprocess.run(
            ['bash', script_path],
            input=b'0\n',
            capture_output=True
        )
        self.assertIn(b'Exiting...', result.stdout)
        self.assertEqual(result.returncode, 0)

    def test_cli_menu_options(self):
        """Test that the ohc_hybrid_cli.sh script prints menu options."""
        script_path = os.path.join(os.path.dirname(__file__), '..', '..', 'deploy', 'scripts', 'ohc_hybrid_cli.sh')
        result = subprocess.run(
            ['bash', script_path],
            input=b'0\n',
            capture_output=True
        )
        self.assertIn(b'Run Developer Setup', result.stdout)
        self.assertIn(b'Configure Environment', result.stdout)
        self.assertIn(b'Run Diagnostics', result.stdout)

if __name__ == '__main__':
    unittest.main()
