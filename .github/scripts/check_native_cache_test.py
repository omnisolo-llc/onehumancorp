#!/usr/bin/env python3
"""Exercise action-owned setup and bounded dependency caches without installing tools."""
import json
from itertools import combinations
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
            'native-node', 'native-web', 'native-desktop', 'native-e2e', 'native-click-coverage', 'native-images',
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

    def test_complete_ci_graph_uses_at_most_eight_concurrent_runners(self):
        jobs = self.ci['jobs']

        def ancestors(name):
            needs = jobs[name].get('needs', [])
            if isinstance(needs, str):
                needs = [needs]
            return set(needs).union(*(ancestors(need) for need in needs))

        predecessors = {name: ancestors(name) for name in jobs}
        weights = {}
        for name, job in jobs.items():
            strategy = job.get('strategy', {})
            matrix = strategy.get('matrix', {})
            self.assertLessEqual(len(matrix), 1, 'extend runner accounting for multidimensional matrices')
            copies = len(next(iter(matrix.values()))) if matrix else 1
            weights[name] = min(copies, strategy.get('max-parallel', copies))

        peak = 0
        for size in range(1, len(jobs) + 1):
            for concurrent in combinations(jobs, size):
                if any(a in predecessors[b] or b in predecessors[a]
                       for a, b in combinations(concurrent, 2)):
                    continue
                peak = max(peak, sum(weights[name] for name in concurrent))
        self.assertLessEqual(peak, 8, f'CI can occupy {peak} runners concurrently')

    def test_browser_shards_keep_running_after_independent_quality_failure(self):
        job = self.ci['jobs']['native-e2e']
        self.assertEqual(job['strategy']['matrix']['shard'], list(range(1, 13)))
        self.assertIn('!cancelled()', job['if'])
        self.assertIn("needs.native-build.result == 'success'", job['if'])
        self.assertIn("needs.native-web.result == 'success'", job['if'])
        for name in ('dependency-audit', 'native-node', 'native-desktop'):
            self.assertIn(name, job['needs'])
            self.assertNotIn(f'needs.{name}.result', job['if'])
        self.assertNotIn('postgres-security', job['needs'])
        self.assertNotIn('needs.postgres-security.result', job['if'])
        self.assertIn('postgres-security', self.ci['jobs']['ci-required']['needs'])

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


    def test_rust_tests_run_after_lint_failure_and_pg_feed_queries_are_real(self):
        steps = self.ci['jobs']['native-test']['steps']
        setup = next(step for step in steps if step.get('uses') == './.github/actions/setup-native')
        self.assertEqual(setup.get('id'), 'rust-setup')
        test = next(step for step in steps if step.get('run') == 'make test-backend')
        self.assertEqual(test.get('if'), "${{ !cancelled() && steps.rust-setup.outcome == 'success' }}")
        self.assertFalse(test.get('continue-on-error', False))
        postgres = self.ci['jobs']['postgres-security']['steps']
        feed = next((step for step in postgres if 'agent_feed_query_regression.py --postgres' in step.get('run', '')), None)
        self.assertIsNotNone(feed, 'mirrored approvals need production PostgreSQL query coverage')
        self.assertIn('ohc_feed_query_test', feed['run'])
        self.assertIn('FEED_QUERY_TEST_DATABASE_URL', feed.get('env', {}))
        self.assertFalse(feed.get('continue-on-error', False))

    def test_rust_lint_diagnostics_are_uploaded_before_tests_without_masking_failure(self):
        steps = self.ci['jobs']['native-test']['steps']
        lint = next(step for step in steps if 'make lint-backend' in step.get('run', ''))
        upload = next((step for step in steps if step.get('with', {}).get('path') == 'target/ci-logs/lint-backend.log'), None)
        tests = next(step for step in steps if step.get('run') == 'make test-backend')
        self.assertIsNotNone(upload, 'the exact lint log must be available before a long test job finishes')
        self.assertLess(steps.index(lint), steps.index(upload))
        self.assertLess(steps.index(upload), steps.index(tests))
        self.assertEqual(upload.get('if'), '${{ always() }}')
        self.assertEqual(upload['uses'], 'actions/upload-artifact@v6')
        self.assertEqual(upload['with'].get('if-no-files-found'), 'error')
        self.assertFalse(lint.get('continue-on-error', False))
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            executable = root / 'make'
            executable.write_text('#!/usr/bin/env bash\necho "fixture stdout"\necho "fixture stderr" >&2\nexit 23\n')
            executable.chmod(0o700)
            result = subprocess.run(['bash', '-c', lint['run']], cwd=root,
                env={**os.environ, 'PATH': str(root) + os.pathsep + os.environ['PATH']},
                capture_output=True, text=True, timeout=10)
            self.assertEqual(result.returncode, 23, result.stdout + result.stderr)
            recorded = (root / 'target/ci-logs/lint-backend.log').read_text()
            self.assertIn('fixture stdout', recorded)
            self.assertIn('fixture stderr', recorded)


    def test_postgres_runs_product_seo_snapshot_regression(self):
        steps = self.ci['jobs']['postgres-security']['steps']
        seo = next((step for step in steps if 'product_seo_snapshot_regression.py' in step.get('run', '')), None)
        self.assertIsNotNone(seo, 'asynchronous SEO snapshots need a real PostgreSQL stale-write check')
        self.assertIn('ohc_seo_snapshot_test', seo['run'])
        self.assertIn('SEO_SNAPSHOT_TEST_DATABASE_URL', seo.get('env', {}))
        self.assertFalse(seo.get('continue-on-error', False))


    def test_postgres_runs_optional_quote_configuration_regression(self):
        steps = self.ci['jobs']['postgres-security']['steps']
        quote = next((step for step in steps if 'scripts/quote-taxjar/Cargo.toml' in step.get('run', '')), None)
        self.assertIsNotNone(quote, 'missing tax schema and hosted credential isolation need PostgreSQL coverage')
        self.assertIn('--locked', quote['run'])
        self.assertIn('--include-ignored', quote['run'])
        self.assertIn('OHC_QUOTE_TEST_DATABASE_URL', quote.get('env', {}))
        self.assertEqual(quote['env']['TAXJAR_API_KEY'], 'local-regression-taxjar-key')
        self.assertFalse(quote.get('continue-on-error', False))


    def test_postgres_runs_durable_sync_and_catalog_regressions(self):
        steps = self.ci['jobs']['postgres-security']['steps']
        sync = next((step for step in steps if 'scripts/sync-durability/run.sh' in step.get('run', '')), None)
        self.assertIsNotNone(sync, 'durable receipt and business mutation checks must run against PostgreSQL')
        self.assertIn('OHC_SYNC_TEST_DATABASE_URL', sync.get('env', {}))
        self.assertFalse(sync.get('continue-on-error', False))
        catalog = next((step for step in steps if 'scripts/catalog-edit/Cargo.toml' in step.get('run', '')), None)
        self.assertIsNotNone(catalog, 'real catalog edits need PostgreSQL and SQLite coverage')
        self.assertIn('--locked', catalog['run'])
        self.assertIn('--include-ignored', catalog['run'])
        self.assertIn('OHC_CATALOG_TEST_DATABASE_URL', catalog.get('env', {}))
        self.assertFalse(catalog.get('continue-on-error', False))


if __name__ == '__main__':
    unittest.main()
