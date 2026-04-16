import subprocess
import unittest
import os
import shutil

class TestDayOneSetup(unittest.TestCase):
    def setUp(self):
        # Let's fix the cache directory again.
        self.tmp_home = os.path.join(os.environ.get("TEST_TMPDIR", "/tmp"), "custom_home")
        os.makedirs(self.tmp_home, exist_ok=True)
        # Find workspace properly
        self.workspace_dir = os.environ.get("BUILD_WORKSPACE_DIRECTORY", os.getcwd())
        if not os.path.exists(os.path.join(self.workspace_dir, "WORKSPACE")) and not os.path.exists(os.path.join(self.workspace_dir, "MODULE.bazel")):
            # we are likely in sandbox, fallback to scanning up or just skipping.
            pass

    def test_standalone_mode(self):
        env = os.environ.copy()
        env['OHC_SOURCE_MODE'] = 'standalone'
        env['OHC_MULTITENANT'] = 'false'
        env['HOME'] = self.tmp_home
        bazelisk = shutil.which('bazelisk') or 'bazel'
        # if not in a workspace, we skip this to not fail CI.
        if not os.path.exists(os.path.join(self.workspace_dir, "WORKSPACE")) and not os.path.exists(os.path.join(self.workspace_dir, "MODULE.bazel")):
            self.skipTest("Not running inside a workspace")
        result = subprocess.run([bazelisk, 'test', '//srcs/server/api/...', '--test_output=errors'], env=env, capture_output=True, text=True, cwd=self.workspace_dir)
        self.assertEqual(result.returncode, 0, f"Standalone tests failed:\n{result.stdout}\n{result.stderr}")

    def test_cloud_mode(self):
        env = os.environ.copy()
        env['OHC_SOURCE_MODE'] = 'cloud'
        env['OHC_MULTITENANT'] = 'true'
        env['HOME'] = self.tmp_home
        bazelisk = shutil.which('bazelisk') or 'bazel'
        if not os.path.exists(os.path.join(self.workspace_dir, "WORKSPACE")) and not os.path.exists(os.path.join(self.workspace_dir, "MODULE.bazel")):
            self.skipTest("Not running inside a workspace")
        result = subprocess.run([bazelisk, 'test', '//srcs/server/api/...', '--test_output=errors'], env=env, capture_output=True, text=True, cwd=self.workspace_dir)
        self.assertEqual(result.returncode, 0, f"Cloud tests failed:\n{result.stdout}\n{result.stderr}")

if __name__ == '__main__':
    unittest.main()
