import subprocess
import unittest
import os
import shutil

class TestDayOneSetup(unittest.TestCase):
    def setUp(self):
        # We need a writable temporary home directory for bazelisk cache
        self.tmp_home = os.path.join(os.environ.get("TEST_TMPDIR", "/tmp"), "custom_home")
        os.makedirs(self.tmp_home, exist_ok=True)
        # Find the actual workspace directory
        self.workspace_dir = os.environ.get("BUILD_WORKSPACE_DIRECTORY", os.getcwd())

    def test_standalone_mode(self):
        env = os.environ.copy()
        env['OHC_SOURCE_MODE'] = 'standalone'
        env['OHC_MULTITENANT'] = 'false'
        env['HOME'] = self.tmp_home
        bazelisk = shutil.which('bazelisk') or 'bazel'
        result = subprocess.run([bazelisk, 'test', '//srcs/server/api/...', '--test_output=errors'], env=env, capture_output=True, text=True, cwd=self.workspace_dir)
        self.assertEqual(result.returncode, 0, f"Standalone tests failed:\n{result.stdout}\n{result.stderr}")

    def test_cloud_mode(self):
        env = os.environ.copy()
        env['OHC_SOURCE_MODE'] = 'cloud'
        env['OHC_MULTITENANT'] = 'true'
        env['HOME'] = self.tmp_home
        bazelisk = shutil.which('bazelisk') or 'bazel'
        result = subprocess.run([bazelisk, 'test', '//srcs/server/api/...', '--test_output=errors'], env=env, capture_output=True, text=True, cwd=self.workspace_dir)
        self.assertEqual(result.returncode, 0, f"Cloud tests failed:\n{result.stdout}\n{result.stderr}")

if __name__ == '__main__':
    unittest.main()
