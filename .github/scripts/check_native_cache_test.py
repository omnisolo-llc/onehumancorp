#!/usr/bin/env python3
"""Exercise action-owned setup and bounded dependency caches without installing tools."""
import json
import os
from pathlib import Path
import re
import subprocess
import tempfile
import tomllib
import unittest

import yaml

ROOT = Path(__file__).resolve().parents[2]


class NativeCacheTests(unittest.TestCase):
    def setUp(self):
        self.action = yaml.safe_load((ROOT / '.github/actions/setup-native/action.yml').read_text())
        self.steps = self.action['runs']['steps']
        self.ci = yaml.safe_load((ROOT / '.github/workflows/ci.yml').read_text())

    def step(self, name):
        return next(step for step in self.steps if step.get('id') == name or step.get('name') == name)

    def test_workflows_do_not_execute_local_bootstrap_or_doctor(self):
        for path in [*ROOT.glob('.github/workflows/*.yml'), *ROOT.glob('.github/actions/*/action.yml')]:
            for line in path.read_text().splitlines():
                if line.lstrip().startswith('#'):
                    continue
                self.assertIsNone(re.search(r'\bmake\s+(?:init|doctor)\b|\bpython3?\s+scripts/init_dev\.py\b', line), str(path))
        self.assertNotIn('native-init', self.ci['jobs'])
        self.assertTrue((ROOT / 'scripts/init_dev.py').exists(), 'local bootstrap must remain available')
        self.assertIn('python3 scripts/init_dev_test.py', str(self.ci['jobs']['check-changes']['steps']))

    def test_all_application_and_security_gates_remain_required(self):
        expected = {'check-changes', 'dependency-audit', 'native-build', 'native-test',
            'native-node', 'native-web', 'native-desktop', 'native-e2e', 'native-images',
            'postgres-security', 'kind-e2e', 'docker-e2e'}
        gate = self.ci['jobs']['ci-required']
        self.assertEqual(set(gate['needs']), expected)
        step = next(step for step in gate['steps'] if step.get('name') == 'Check required CI results')
        for lane in expected:
            key = lane.upper().replace('-', '_') + '_RESULT'
            self.assertIn(key, step['env'])
            for result in ('failure', 'cancelled', 'skipped', 'unknown'):
                with self.subTest(lane=lane, result=result):
                    env = {**os.environ, **{name: 'success' for name in step['env']},
                        'EVENT_NAME': 'push', 'MARKDOWN_ONLY': 'false', key: result}
                    completed = subprocess.run(['bash', '--noprofile', '--norc', '-c', step['run']],
                        env=env, capture_output=True, text=True, timeout=10)
                    self.assertNotEqual(completed.returncode, 0)

    def test_action_pins_and_cargo_dependency_boundary(self):
        rust = next(step for step in self.steps if step.get('uses', '').startswith('dtolnay/rust-toolchain@'))
        self.assertEqual(rust['with']['toolchain'], tomllib.loads((ROOT/'rust-toolchain.toml').read_text())['toolchain']['channel'])
        cache = self.step('rust-cache')
        self.assertLess(self.steps.index(rust), self.steps.index(cache))
        for field in ('cache-on-failure', 'cache-bin', 'cache-workspace-crates'):
            self.assertEqual(cache['with'][field], 'false')
        for field in ('runner.os', 'runner.arch', 'env.ImageOS', 'inputs.role'):
            self.assertIn(field, cache['with']['shared-key'])
        for variable in ('CARGO', 'RUST', 'OPENSSL', 'PKG_CONFIG', 'VCPKG', 'SDKROOT'):
            self.assertIn(variable, cache['with']['env-vars'].split())

    def test_node_cache_has_scoped_lock_hashes_and_bounded_fallback(self):
        restore = self.step('npm-restore')['with']
        metadata = self.step('npm-cache')
        key = restore['key']
        self.assertIn('steps.npm-cache.outputs.lock-hash', key)
        self.assertNotIn('github.sha', key)
        for field in ('runner.os', 'runner.arch', "hashFiles('.node-version')", 'inputs.node-scope'):
            self.assertIn(field, key)
            self.assertIn(field, restore['restore-keys'])
        expr = metadata['env']['NPM_LOCK_HASH']
        self.assertIn("inputs.node-scope == 'harness' && hashFiles('.github/test-tools/package-lock.json')", expr)
        self.assertIn("inputs.node-scope == 'root' && hashFiles('package-lock.json')", expr)
        self.assertIn("inputs.node-scope == 'web' && hashFiles('package-lock.json', 'src/ui/next/package-lock.json')", expr)
        self.assertNotIn('node_modules', str(restore))
        self.assertEqual(restore['path'], '${{ steps.npm-cache.outputs.path }}')
        self.assertIn('/_cacache', metadata['run'])
        node = next(s for s in self.steps if s.get('uses', '').startswith('actions/setup-node@'))
        self.assertFalse(node['with']['package-manager-cache'], 'no second implicit cache writer')

    def test_locked_installer_obeys_scope_and_propagates_failure(self):
        body = self.step('Install locked Node dependencies')['run']
        for scope, expected in [('root', ['.']), ('web', ['.', 'src/ui/next']),
                                ('all', ['.', 'src/ui/next', 'src/cli']),
                                ('harness', ['.github/test-tools'])]:
            for fail in (False, True):
                with self.subTest(scope=scope, fail=fail), tempfile.TemporaryDirectory() as tmp:
                    directory = Path(tmp)
                    fake = directory / 'npm'
                    fake.write_text('#!/usr/bin/env python3\nimport json,os,sys\n'
                        'with open(os.environ["CALLS"], "a") as f: f.write(json.dumps(sys.argv[1:])+"\\n")\n'
                        'raise SystemExit(17 if os.environ["FAIL"] == "1" else 0)\n')
                    fake.chmod(0o755)
                    env = {**os.environ, 'PATH': str(directory) + os.pathsep + os.environ['PATH'],
                        'NODE_SCOPE': scope, 'CALLS': str(directory / 'calls'), 'FAIL': str(int(fail))}
                    result = subprocess.run(['bash', '-c', body], cwd=directory, env=env,
                        text=True, capture_output=True, timeout=10)
                    calls = [json.loads(line) for line in (directory/'calls').read_text().splitlines()]
                    if fail:
                        self.assertNotEqual(result.returncode, 0)
                        self.assertEqual(len(calls), 1)
                        continue
                    self.assertEqual(result.returncode, 0, result.stderr)
                    trees = [args[args.index('--prefix')+1] if '--prefix' in args else '.' for args in calls]
                    self.assertEqual(trees, expected)
                    for args in calls:
                        self.assertIn('ci', args)
                        self.assertIn('--include=dev', args)
                        self.assertNotIn('install', args)

    def test_invalid_scope_fails_before_installing(self):
        with tempfile.TemporaryDirectory() as tmp:
            fake = Path(tmp) / 'npm'
            fake.write_text('#!/bin/sh\necho called > "$MARKER"\n')
            fake.chmod(0o755)
            env = {**os.environ, 'PATH': tmp+os.pathsep+os.environ['PATH'],
                'NODE_SCOPE': 'unknown', 'MARKER': str(Path(tmp)/'marker')}
            result = subprocess.run(['bash', '-c', self.step('Install locked Node dependencies')['run']],
                env=env, cwd=tmp, capture_output=True, timeout=10)
            self.assertNotEqual(result.returncode, 0)
            self.assertFalse((Path(tmp)/'marker').exists())

    def test_cold_mode_and_trusted_write_allowlist(self):
        for step in self.steps:
            if 'cache' in step.get('uses', '').lower():
                self.assertIn("inputs.cache == 'true'", step['if'])
                policy = step.get('with', {}).get('save-if')
                if '/save@' in step['uses']:
                    policy = step['if']
                if policy:
                    for required in ("github.event_name == 'push'", "github.event_name == 'workflow_dispatch'", "github.ref == 'refs/heads/main'"):
                        self.assertIn(required, policy)
                    self.assertNotIn("!= 'pull_request'", policy)
                    self.assertNotIn("== 'pull_request_target'", policy)
        for job in self.ci['jobs'].values():
            for step in job.get('steps', []):
                if step.get('uses') == './.github/actions/setup-native':
                    self.assertIn('cold_cache', step['with']['cache'])

    def test_harness_job_reuses_cached_action_and_still_verifies_real_binary(self):
        steps = self.ci['jobs']['native-test']['steps']
        setup = next(s for s in steps if s.get('uses') == './.github/actions/setup-native')
        self.assertEqual(setup['with']['node-scope'], 'harness')
        self.assertNotEqual(setup['with'].get('node'), 'false')
        self.assertFalse(any(s.get('uses', '').startswith('actions/setup-node@') for s in steps))
        verify = next(s for s in steps if 'opencode --version' in s.get('run', ''))
        self.assertIn('GITHUB_PATH', verify['run'])
        self.assertNotIn('npm ci', verify['run'], 'do not install a second time')

    def test_next_cache_is_compiler_only_and_runner_image_scoped(self):
        steps = self.ci['jobs']['native-web']['steps']
        restore = next(s for s in steps if s.get('id') == 'next-cache')
        self.assertEqual(restore['with']['path'], 'src/ui/next/.next/cache')
        for key in ('key', 'restore-keys'):
            self.assertIn('env.ImageOS', restore['with'][key])
        build = next(s for s in steps if s.get('run') == 'make build-web')
        self.assertNotIn('if', build, 'cache hits must never skip source compilation')


if __name__ == '__main__':
    unittest.main()
