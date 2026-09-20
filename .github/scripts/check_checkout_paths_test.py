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
    def test_lint_has_independent_required_jobs_not_serial_test_steps(self):
        import yaml
        root = SCRIPT.parent.parent.parent
        jobs = yaml.safe_load((root / '.github/workflows/ci.yml').read_text())['jobs']
        for lane, command in (('native-rust-lint', 'make lint-backend'),
                              ('native-node-lint', 'make lint-node')):
            with self.subTest(lane=lane):
                self.assertIn(lane, jobs, 'lint must run concurrently with tests')
                self.assertEqual(jobs[lane]['needs'], ['check-changes'])
                self.assertIn(lane, jobs['ci-required']['needs'])
                self.assertTrue(any(s.get('run') == command for s in jobs[lane]['steps']))
                self.assertFalse(jobs[lane].get('continue-on-error', False))
        for lane in ('native-test', 'native-node'):
            self.assertEqual(jobs[lane]['needs'], ['check-changes'])
            commands = '\n'.join(s.get('run', '') for s in jobs[lane]['steps'])
            self.assertNotIn('make lint', commands)
            self.assertNotIn('npm run lint:node', commands)
            self.assertNotIn('npm run typecheck:web', commands)

    def test_ci_uses_setup_actions_not_the_local_initializer(self):
        import yaml
        root = SCRIPT.parent.parent.parent
        jobs = yaml.safe_load((root / '.github/workflows/ci.yml').read_text())['jobs']
        self.assertNotIn('native-init', jobs)
        for name in ('native-build', 'native-test', 'native-node', 'native-web', 'native-desktop', 'native-e2e'):
            self.assertTrue(any(step.get('uses') == './.github/actions/setup-native'
                                for step in jobs[name]['steps']), name)
        self.assertNotIn('make init', str(jobs))
        self.assertNotIn('make doctor', str(jobs))

    def test_required_gate_still_checks_all_application_test_and_security_lanes(self):
        import yaml
        root = SCRIPT.parent.parent.parent
        jobs = yaml.safe_load((root / '.github/workflows/ci.yml').read_text())['jobs']
        self.assertEqual(set(jobs['ci-required']['needs']), {'check-changes', 'dependency-audit',
            'native-build', 'native-test', 'native-rust-lint', 'native-node-lint', 'native-rust-execute', 'native-rust-results', 'native-browser-inventory', 'native-browser-results', 'native-e2e', 'native-web', 'native-node',
            'native-images', 'native-desktop', 'kind-e2e', 'docker-e2e', 'postgres-security'})

    def test_bootstrap_contracts_execute_before_expensive_builds(self):
        import yaml
        root = SCRIPT.parent.parent.parent
        jobs = yaml.safe_load((root / '.github/workflows/ci.yml').read_text())['jobs']
        steps = jobs['check-changes']['steps']
        self.assertTrue(any('python3 .github/scripts/check_checkout_paths_test.py' in step.get('run', '')
                            and 'python3 scripts/init_dev_test.py' in step.get('run', '') for step in steps))

    def test_native_harness_tests_install_their_locked_external_executable(self):
        import json
        import yaml
        root = SCRIPT.parent.parent.parent
        steps = yaml.safe_load((root / '.github/workflows/ci.yml').read_text())['jobs']['native-test']['steps']
        setup = next((i for i, step in enumerate(steps)
                      if step.get('uses') == './.github/actions/setup-native'
                      and step.get('with', {}).get('node-scope') == 'harness'), None)
        self.assertIsNotNone(setup, 'real OpenCode lifecycle tests need the cached harness setup action')
        verify = next(i for i, step in enumerate(steps) if 'opencode --version' in step.get('run', ''))
        tests = next(i for i, step in enumerate(steps) if step.get('run') == 'python3 scripts/ci_rust.py build')
        self.assertLess(setup, verify)
        self.assertLess(verify, tests)
        self.assertIn('GITHUB_PATH', steps[verify]['run'])
        action = yaml.safe_load((root / '.github/actions/setup-native/action.yml').read_text())
        self.assertIn('npm ci --prefix .github/test-tools --include=dev', str(action))
        package = json.loads((root / '.github/test-tools/package.json').read_text())
        lock = json.loads((root / '.github/test-tools/package-lock.json').read_text())
        self.assertEqual(package['dependencies']['opencode-ai'], '1.18.15')
        self.assertEqual(lock['packages']['node_modules/opencode-ai']['version'], '1.18.15')
        self.assertTrue(lock['packages']['node_modules/opencode-ai']['integrity'].startswith('sha512-'))

    def test_desktop_builds_expose_packager_failures_without_skipping_bundles(self):
        import yaml
        root = SCRIPT.parent.parent.parent
        job = yaml.safe_load((root / '.github/workflows/release.yml').read_text())['jobs']['build-desktop-installers']
        step = next(s for s in job['steps'] if s.get('uses', '').startswith('tauri-apps/tauri-action@'))
        self.assertIn('--verbose', step['with']['args'])
        self.assertIn('--bundles', step['with']['args'])
        self.assertIn('--locked', step['with']['args'])
        self.assertNotIn('--no-bundle', step['with']['args'])

    def test_independent_node_checks_run_after_setup_without_masking_failures(self):
        import yaml
        root = SCRIPT.parent.parent.parent
        job = yaml.safe_load((root / '.github/workflows/ci.yml').read_text())['jobs']['native-node']
        commands = ('make test-contracts', 'make test-node')
        for command in commands:
            step = next((s for s in job['steps'] if command in s.get('run', '')), None)
            self.assertIsNotNone(step, command)
            self.assertEqual(step.get('if'), "${{ !cancelled() && steps.native-setup.outcome == 'success' }}")
            self.assertFalse(step.get('continue-on-error', False))
        self.assertFalse(job.get('continue-on-error', False))

    def test_windows_cargo_uses_powershell_without_git_bash_linker_shadowing(self):
        import yaml
        root = SCRIPT.parent.parent.parent
        workflow = yaml.safe_load((root / '.github/workflows/release.yml').read_text())
        steps = workflow['jobs']['build-release-artifacts']['steps']
        windows = next(s for s in steps if s.get('name') == 'Build native Cargo release on Windows')
        unix = next(s for s in steps if s.get('name') == 'Build native Cargo release')
        self.assertEqual(windows.get('shell'), 'pwsh')
        self.assertEqual(windows.get('if'), "runner.os == 'Windows'")
        self.assertEqual(unix.get('if'), "runner.os != 'Windows'")
        self.assertEqual(windows['run'], unix['run'])
        self.assertIn('--locked', windows['run'])

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
