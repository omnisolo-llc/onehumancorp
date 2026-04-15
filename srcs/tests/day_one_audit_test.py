import subprocess
import unittest
import os
import shutil

class TestDayOneSetup(unittest.TestCase):
    def test_standalone_mode(self):
        env = os.environ.copy()
        env['OHC_SOURCE_MODE'] = 'standalone'
        env['OHC_MULTITENANT'] = 'false'
        bazelisk = shutil.which('bazelisk') or 'bazel'
        result = subprocess.run([bazelisk, 'test', '//srcs/server/api/...', '--test_output=errors'], env=env, capture_output=True, text=True)
        self.assertEqual(result.returncode, 0, f"Standalone tests failed:\n{result.stdout}\n{result.stderr}")

    def test_cloud_mode(self):
        env = os.environ.copy()
        env['OHC_SOURCE_MODE'] = 'cloud'
        env['OHC_MULTITENANT'] = 'true'
        bazelisk = shutil.which('bazelisk') or 'bazel'
        result = subprocess.run([bazelisk, 'test', '//srcs/server/api/...', '--test_output=errors'], env=env, capture_output=True, text=True)
        self.assertEqual(result.returncode, 0, f"Cloud tests failed:\n{result.stdout}\n{result.stderr}")

if __name__ == '__main__':
    unittest.main()
