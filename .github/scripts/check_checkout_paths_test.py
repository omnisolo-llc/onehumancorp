#!/usr/bin/env python3
"""Checkout portability must fail on Linux before Windows workers are queued."""
import importlib.util
from pathlib import Path
import subprocess
import tempfile
import unittest

SCRIPT = Path(__file__).with_name('check_checkout_paths.py')

class CheckoutPathTests(unittest.TestCase):
    def setUp(self):
        self.assertTrue(SCRIPT.is_file(), 'portable-checkout guard is required')
        spec = importlib.util.spec_from_file_location('checkout_paths', SCRIPT)
        self.checker = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(self.checker)

    def test_rejects_reserved_characters_devices_and_ambiguous_suffixes(self):
        for name in (':memory:', 'a/b:c', 'bad\\name', 'quote"name', 'line\nbreak',
                     'NUL', 'nested/con.txt', 'COM¹.log', 'LPT9', 'ends.', 'ends ', 'a/../b'):
            with self.subTest(name=name):
                self.assertTrue(self.checker.check_paths([name]))

    def test_accepts_next_routes_unicode_spaces_and_native_source_names(self):
        paths = ['src/app/[id]/page.tsx', 'src/app/[...path]/route.ts', '.cargo/config.toml',
                 'documentation with spaces/日本語.md', 'company/converter.rs', 'COM10.txt']
        self.assertEqual(self.checker.check_paths(paths), [])

    def test_rejects_case_and_unicode_collisions_in_parent_directories(self):
        for paths in (['src/Foo/a.rs', 'src/foo/b.rs'], ['README.md', 'readme.md'],
                      ['café/a.md', 'cafe\u0301/b.md']):
            with self.subTest(paths=paths):
                self.assertTrue(self.checker.check_paths(paths))

    def test_checks_index_without_reading_or_echoing_file_contents(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            subprocess.run(['git', 'init', '-q', temp], check=True)
            (root / ':memory:').write_text('synthetic-private-content')
            subprocess.run(['git', '--literal-pathspecs', '-C', temp, 'add', '--', ':memory:'], check=True)
            result = subprocess.run(['python3', str(SCRIPT.resolve())], cwd=root, capture_output=True, text=True)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn(':memory:', result.stderr)
            self.assertNotIn('synthetic-private-content', result.stderr + result.stdout)

class WorkflowStartupTests(unittest.TestCase):
    def test_android_setup_does_not_request_retired_sdk_tools(self):
        import yaml
        root = SCRIPT.parent.parent.parent
        workflow = yaml.safe_load((root / '.github/workflows/release.yml').read_text())
        step = next(s for s in workflow['jobs']['build-mobile-artifacts']['steps']
                    if s.get('uses', '').startswith('android-actions/setup-android@'))
        self.assertEqual(step['with']['packages'], 'platform-tools')

    def test_web_builder_contains_the_shared_typechecked_database_helper(self):
        root = SCRIPT.parent.parent.parent
        dockerfile = (root / 'deploy/docker/web/Dockerfile').read_text()
        self.assertIn('COPY src/e2e/db_utils.ts ./src/e2e/', dockerfile)
        self.assertIn('RUN npm run build:web', dockerfile)

if __name__ == '__main__':
    unittest.main()
