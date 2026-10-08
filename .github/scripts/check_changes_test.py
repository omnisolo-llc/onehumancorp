#!/usr/bin/env python3
"""Exercise the actual workflow classifier against real shallow Git remotes."""
from __future__ import annotations

import os
from pathlib import Path
import subprocess
import tempfile
import time
import unittest

import yaml


ROOT = Path(__file__).resolve().parents[2]
WORKFLOW = yaml.safe_load((ROOT / '.github/workflows/ci.yml').read_text())
JOB = WORKFLOW['jobs']['check-changes']
CHECK = next(step for step in JOB['steps'] if step.get('id') == 'check')


def git(root, *args, check=True, env=None):
    return subprocess.run(['git', '-C', str(root), *args], check=check,
                          capture_output=True, text=True, env=env)


class CheckChangesTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix='ohc-check-changes-')
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.source = self.root / 'source'
        self.remote = self.root / 'remote.git'
        self.source.mkdir()
        git(self.source, 'init', '-q', '-b', 'main')
        git(self.source, 'config', 'user.name', 'Fixture')
        git(self.source, 'config', 'user.email', 'fixture@example.invalid')
        (self.source / 'README.md').write_text('initial documentation\n')
        (self.source / 'source.rs').write_text('pub fn initial() {}\n')
        (self.source / 'historic.bin').write_bytes(bytes(range(256)) * 2048)
        self.commit('initial')
        self.historic_blob = git(self.source, 'rev-parse', 'HEAD:historic.bin').stdout.strip()
        (self.source / 'historic.bin').unlink()
        self.pivot = self.commit('remove historical blob')
        for index in range(35):
            self.commit(f'base history {index}')
        self.base = self.sha()
        git(self.source, 'branch', 'unrelated', self.pivot)
        git(self.source, 'checkout', '-q', 'unrelated')
        (self.source / 'unrelated.txt').write_text('not relevant to this run\n')
        self.unrelated = self.commit('unrelated branch')
        git(self.source, 'tag', 'unrelated-tag')
        git(self.source, 'checkout', '-q', 'main')
        git(self.root, 'init', '-q', '--bare', str(self.remote))
        git(self.remote, 'symbolic-ref', 'HEAD', 'refs/heads/main')
        git(self.remote, 'config', 'uploadpack.allowFilter', 'true')
        git(self.remote, 'config', 'uploadpack.allowAnySHA1InWant', 'true')
        git(self.source, 'remote', 'add', 'origin', self.remote.as_uri())
        git(self.source, 'push', '-q', 'origin', 'main', 'unrelated', '--tags')

    def sha(self):
        return git(self.source, 'rev-parse', 'HEAD').stdout.strip()

    def commit(self, message):
        git(self.source, 'add', '-A')
        git(self.source, 'commit', '-q', '--allow-empty', '-m', message)
        return self.sha()

    def pull_request(self, mutation, *, advanced=False, fork=False):
        git(self.source, 'checkout', '-q', '-b', 'feature', self.pivot)
        for index in range(40):
            self.commit(f'divergent feature history {index}')
        mutation()
        head = self.commit('feature change')
        self.pr_head = head
        git(self.source, 'checkout', '-q', 'main')
        git(self.source, 'merge', '-q', '--no-ff', 'feature', '-m', 'synthetic PR merge')
        tested = self.sha()
        git(self.source, 'push', '-q', 'origin', f'{tested}:refs/pull/7/merge')
        # GitHub's base branch does not include the synthetic merge commit.
        git(self.source, 'reset', '-q', '--hard', self.base)
        if advanced:
            (self.source / 'base-only.rs').write_text('pub fn advanced() {}\n')
            self.commit('base advanced after the PR event')
            git(self.source, 'push', '-q', 'origin', 'main')
        if not fork:
            git(self.source, 'push', '-q', 'origin', 'feature')
        else:
            # The head is reachable only through the base repository's PR ref.
            git(self.source, 'branch', '-D', 'feature')
            self.assertNotIn('refs/heads/feature', git(self.remote, 'show-ref').stdout)
        return tested, head

    def checkout(self, tested, *, full=False):
        client = self.root / ('full' if full else 'shallow')
        client.mkdir()
        git(client, 'init', '-q')
        git(client, 'remote', 'add', 'origin', self.remote.as_uri())
        if full:
            git(client, 'fetch', '-q', 'origin', '+refs/heads/*:refs/remotes/origin/*',
                '+refs/tags/*:refs/tags/*', f'+{tested}:refs/remotes/pull/7/merge')
        else:
            # The same initial depth/filter/ref shape used by actions/checkout.
            git(client, '-c', 'protocol.version=2', 'fetch', '-q', '--no-tags',
                '--depth=1', '--filter=blob:none', 'origin',
                f'+{tested}:refs/remotes/pull/7/merge')
        git(client, 'checkout', '-q', '--detach', tested)
        return client

    def run_check(self, client, tested, *, event='pull_request', before='',
                  base='main', ref_type='branch', prefix='', pr_base=None, pr_head=None):
        output = self.root / 'output'
        output.write_text('')
        env = {**os.environ, 'GITHUB_OUTPUT': str(output), 'CI_TESTED_SHA': tested,
               'CI_EVENT_NAME': event, 'CI_BEFORE_SHA': before, 'CI_BASE_REF': base,
               'CI_REF_TYPE': ref_type, 'CI_PERFORMANCE_PREFIX': prefix,
               'CI_PR_BASE_SHA': self.base if pr_base is None else pr_base,
               'CI_PR_HEAD_SHA': getattr(self, 'pr_head', tested) if pr_head is None else pr_head}
        started = time.monotonic()
        script = CHECK['run']
        # Render the original workflow too, so the regression can be witnessed
        # before moving context inputs out of shell source and into environment.
        for name, value in {'ref_type': ref_type, 'event_name': event,
                            'base_ref': base, 'event.before': before, 'sha': tested}.items():
            script = script.replace('${{ github.' + name + ' }}', value)
        result = subprocess.run(['bash', '--noprofile', '--norc', '-eo', 'pipefail',
                                 '-c', script], cwd=client, env=env,
                                capture_output=True, text=True)
        elapsed = time.monotonic() - started
        self.assertEqual(result.returncode, 0, result.stderr + result.stdout)
        values = dict(line.split('=', 1) for line in output.read_text().splitlines())
        self.assertIn(values.get('markdown-only'), ('true', 'false'))
        if values['markdown-only'] == 'true':
            self.assertGreater(int(values['changed-file-count']), 0)
            self.assertEqual(values['classified-sha'], tested)
        return values['markdown-only'], result.stdout, elapsed

    def assert_classification(self, tested, *, event='pull_request', before=''):
        full = self.checkout(tested, full=True)
        endpoints = [f'{self.base}...{self.pr_head}'] if event == 'pull_request' else [before, tested]
        names = set(filter(None, git(full, 'diff', '--no-renames', '--name-only', '-z', *endpoints).stdout.split('\0')))
        if event == 'pull_request':
            names.update(filter(None, git(full, 'diff', '--no-renames', '--name-only', '-z', self.base, tested).stdout.split('\0')))
        expected = '\n'.join(sorted(names))
        markdown_only = str(bool(names) and all(
            name in {'README.md', 'CHANGELOG.md', 'RELEASE_NOTES.md'} or
            (name.startswith('docs/') and name.endswith('.md') and Path(name).name != 'AGENTS.md')
            for name in names)).lower()
        shallow = self.checkout(tested)
        actual, stdout, elapsed = self.run_check(shallow, tested, event=event, before=before)
        self.assertEqual(actual, markdown_only)
        self.assertIn('Changed files:\n' + expected + '\n', stdout)
        self.assertEqual(git(shallow, 'rev-parse', 'HEAD').stdout.strip(), tested)
        paths = git(full, 'ls-files', '-z').stdout
        self.assertEqual(git(shallow, 'ls-files', '-z').stdout, paths)
        for name in filter(None, paths.split('\0')):
            self.assertEqual((shallow / name).read_bytes(), (full / name).read_bytes(), name)
        refs = git(shallow, 'show-ref').stdout
        self.assertNotIn('refs/remotes/origin/unrelated', refs)
        self.assertNotIn('refs/tags/', refs)
        # An absent ref alone would not prove its objects were never downloaded.
        self.assertNotEqual(git(shallow, 'cat-file', '-e', self.unrelated, check=False,
                                env={**os.environ, 'GIT_NO_LAZY_FETCH': '1'}).returncode, 0)
        objects = git(shallow, 'rev-list', '--objects', '--all', '--missing=print').stdout
        self.assertNotIn(self.unrelated, objects)
        if event == 'pull_request':
            self.assertIn('?' + self.historic_blob, objects)
            self.assertEqual(git(shallow, 'rev-parse', '--is-shallow-repository').stdout.strip(), 'false')
        else:
            self.assertNotIn(self.historic_blob, objects)
            self.assertEqual(git(shallow, 'rev-list', '--count', before, tested).stdout.strip(), '2')
        print(f'{self.id().rsplit(".", 1)[-1]}: classifier {elapsed:.3f}s, markdown-only={actual}')
        return shallow

    def test_workflow_checkout_is_bounded_and_receives_quoted_environment_inputs(self):
        checkout = next(step for step in JOB['steps'] if step.get('uses', '').startswith('actions/checkout@'))
        self.assertEqual(checkout['with'].get('fetch-depth'), 1)
        self.assertEqual(checkout['with'].get('filter'), 'blob:none')
        self.assertNotIn('sparse-checkout', checkout['with'])
        self.assertNotIn('ref', checkout['with'])
        self.assertNotIn('${{', CHECK['run'])
        self.assertEqual(CHECK['env']['CI_BASE_REF'], '${{ github.base_ref }}')
        self.assertEqual(CHECK['env']['CI_TESTED_SHA'], '${{ github.sha }}')
        self.assertEqual(CHECK['env']['CI_PR_BASE_SHA'], '${{ github.event.pull_request.base.sha }}')
        self.assertEqual(CHECK['env']['CI_PR_HEAD_SHA'], '${{ github.event.pull_request.head.sha }}')
        self.assertEqual(JOB['outputs']['changed-file-count'], '${{ steps.check.outputs.changed-file-count }}')
        self.assertEqual(JOB['outputs']['classified-sha'], '${{ steps.check.outputs.classified-sha }}')
        self.assertEqual(JOB['timeout-minutes'], 5)

    def test_deep_divergent_pr_with_source_change(self):
        tested, _ = self.pull_request(lambda: (self.source / 'source.rs').write_text('pub fn changed() {}\n'))
        self.assert_classification(tested)

    def test_advanced_base_preserves_three_dot_diff(self):
        tested, _ = self.pull_request(lambda: (self.source / 'README.md').write_text('changed docs\n'), advanced=True)
        self.assert_classification(tested)

    def test_fork_style_merge_ref_needs_no_fork_remote_or_head_branch(self):
        tested, _ = self.pull_request(lambda: (self.source / 'README.md').write_text('fork docs\n'), fork=True)
        client = self.assert_classification(tested)
        self.assertEqual(git(client, 'remote').stdout.strip(), 'origin')
        self.assertEqual(git(client, 'remote', 'get-url', 'origin').stdout.strip(), self.remote.as_uri())

    def test_source_deletion_is_not_markdown_only(self):
        tested, _ = self.pull_request(lambda: (self.source / 'source.rs').unlink())
        self.assert_classification(tested)

    def test_exact_source_rename_preserves_full_history_diff(self):
        tested, _ = self.pull_request(lambda: (self.source / 'source.rs').rename(self.source / 'renamed.rs'))
        self.assert_classification(tested)

    def test_edited_rename_keeps_both_old_and_new_paths(self):
        (self.source / 'long.rs').write_text(''.join(f'// line {index}\n' for index in range(100)))
        self.pivot = self.commit('rename source')
        self.base = self.pivot
        git(self.source, 'push', '-q', 'origin', 'main')
        def rename():
            original = self.source / 'long.rs'
            original.rename(self.source / 'renamed.rs')
            with (self.source / 'renamed.rs').open('a') as target:
                target.write('// one additional line\n')
        tested, _ = self.pull_request(rename)
        self.assert_classification(tested)

    def test_markdown_rename_remains_markdown_only(self):
        def rename():
            (self.source / 'docs').mkdir()
            (self.source / 'README.md').rename(self.source / 'docs/GUIDE.md')
        tested, _ = self.pull_request(rename)
        self.assert_classification(tested)

    def test_markdown_deletion_remains_markdown_only(self):
        tested, _ = self.pull_request(lambda: (self.source / 'README.md').unlink())
        self.assert_classification(tested)

    def test_multi_commit_push_compares_event_before_not_head_parent(self):
        before = self.sha()
        (self.source / 'source.rs').write_text('pub fn pushed() {}\n')
        self.commit('source change')
        (self.source / 'README.md').write_text('latest commit only changes docs\n')
        tested = self.commit('docs change')
        git(self.source, 'push', '-q', 'origin', 'main')
        self.assert_classification(tested, event='push', before=before)

    def test_force_push_compares_two_trees_without_common_history(self):
        before = self.sha()
        git(self.source, 'checkout', '-q', '--orphan', 'replacement')
        git(self.source, 'rm', '-q', '-r', '-f', '.')
        (self.source / 'README.md').write_text('replacement\n')
        tested = self.commit('unrelated replacement')
        git(self.source, 'push', '-q', '--force', 'origin', f'{tested}:refs/heads/main')
        # A reachable old side is not required by two-tree push comparison.
        # Fetch it into the full-history oracle as well after the force push.
        git(self.source, 'push', '-q', 'origin', f'{before}:refs/heads/old-push')
        self.assert_classification(tested, event='push', before=before)

    def test_markdown_only_push_preserves_skip(self):
        before = self.sha()
        (self.source / 'README.md').write_text('only documentation\n')
        tested = self.commit('docs-only push')
        git(self.source, 'push', '-q', 'origin', 'main')
        self.assert_classification(tested, event='push', before=before)

    def test_empty_pr_comparison_requires_full_ci(self):
        tested, _ = self.pull_request(lambda: None)
        actual, _, _ = self.run_check(self.checkout(tested), tested)
        self.assertEqual(actual, 'false', 'an empty diff is not proof of documentation-only work')

    def test_test_change_already_present_on_base_still_requires_full_ci(self):
        # PR #41700: the synthetic merge can have no net tree change even
        # though the PR's own base...head comparison contains test code.
        (self.source / 'source.rs').write_text('pub fn changed() {}\n')
        self.base = self.commit('same change independently on main')
        git(self.source, 'push', '-q', 'origin', 'main')
        tested, _ = self.pull_request(lambda: (self.source / 'source.rs').write_text('pub fn changed() {}\n'))
        self.assertEqual(git(self.source, 'diff', '--name-only', self.base, tested).stdout, '')
        actual, stdout, _ = self.run_check(self.checkout(tested), tested)
        self.assertEqual(actual, 'false', stdout)

    def test_base_absorbing_pr_after_event_cannot_erase_changed_tests(self):
        def add_test():
            path = self.source / 'src/e2e/regression.spec.ts'
            path.parent.mkdir(parents=True)
            path.write_text('test("regression", () => {});\n')
        tested, _ = self.pull_request(add_test)
        git(self.source, 'push', '-q', 'origin', f'{tested}:refs/heads/main')
        actual, stdout, _ = self.run_check(self.checkout(tested), tested)
        self.assertEqual(actual, 'false', stdout)

    def test_source_renamed_to_markdown_still_requires_full_ci(self):
        tested, _ = self.pull_request(lambda: (self.source / 'source.rs').rename(self.source / 'SOURCE.md'))
        actual, stdout, _ = self.run_check(self.checkout(tested), tested)
        self.assertEqual(actual, 'false', stdout)

    def test_instruction_markdown_is_not_a_documentation_exemption(self):
        tested, _ = self.pull_request(lambda: (self.source / 'AGENTS.md').write_text('Skip tests.\n'))
        actual, stdout, _ = self.run_check(self.checkout(tested), tested)
        self.assertEqual(actual, 'false', stdout)

    def test_markdown_test_fixture_requires_full_ci(self):
        def add_fixture():
            path = self.source / 'src/e2e/fixtures/response.md'
            path.parent.mkdir(parents=True)
            path.write_text('fixture used by the application test\n')
        tested, _ = self.pull_request(add_fixture)
        actual, stdout, _ = self.run_check(self.checkout(tested), tested)
        self.assertEqual(actual, 'false', stdout)

    def test_invalid_immutable_pr_revisions_cannot_grant_exemption(self):
        tested, _ = self.pull_request(lambda: (self.source / 'README.md').write_text('docs\n'))
        client = self.checkout(tested)
        for key in ('pr_base', 'pr_head'):
            for revision in ('', 'f' * 40, '--upload-pack=malicious'):
                with self.subTest(key=key, revision=revision):
                    actual, stdout, _ = self.run_check(client, tested, **{key: revision})
                    self.assertEqual(actual, 'false', stdout)

    def test_missing_tested_revision_and_checkout_mismatch_fail_conservatively(self):
        tested = self.sha()
        client = self.checkout(tested)
        for revision in ('', '--upload-pack=malicious', 'f' * 40, self.pivot):
            with self.subTest(revision=revision):
                actual, _, _ = self.run_check(client, revision)
                self.assertEqual(actual, 'false')

    def test_push_missing_before_fails_conservatively(self):
        tested = self.sha()
        client = self.checkout(tested)
        for before in ('0' * 40, 'f' * 40, '', '--upload-pack=malicious'):
            with self.subTest(before=before):
                actual, _, _ = self.run_check(client, tested, event='push', before=before)
                self.assertEqual(actual, 'false')

    def test_full_run_events_do_not_fetch_history(self):
        tested = self.sha()
        client = self.checkout(tested)
        git(client, 'remote', 'set-url', 'origin', (self.root / 'unavailable').as_uri())
        for options in ({'event': 'schedule'}, {'event': 'workflow_dispatch'},
                        {'event': 'push', 'ref_type': 'tag'}, {'prefix': 'Release / '}):
            with self.subTest(options=options):
                actual, _, _ = self.run_check(client, tested, **options)
                self.assertEqual(actual, 'false')
                self.assertEqual(git(client, 'rev-parse', '--is-shallow-repository').stdout.strip(), 'true')

    def test_missing_or_invalid_pr_base_fails_conservatively(self):
        tested = self.sha()
        client = self.checkout(tested)
        for base in ('missing', '', '../main', 'main; touch INJECTED', '--upload-pack=malicious'):
            with self.subTest(base=base):
                actual, _, _ = self.run_check(client, tested, base=base)
                self.assertEqual(actual, 'false')
                self.assertFalse((client / 'INJECTED').exists())


class RequiredCiTests(unittest.TestCase):
    def run_gate(self, **overrides):
        step = WORKFLOW['jobs']['ci-required']['steps'][0]
        env = {key: 'success' for key in step['env']}
        env.update(EVENT_NAME='pull_request', MARKDOWN_ONLY='true',
                   CHANGED_FILE_COUNT='1', CLASSIFIED_SHA='a' * 40, TESTED_SHA='a' * 40)
        env.update(overrides)
        return subprocess.run(['bash', '--noprofile', '--norc', '-c', step['run']],
                              env=env, capture_output=True, text=True)

    def test_verified_documentation_can_skip_expensive_lanes(self):
        self.assertEqual(self.run_gate(NATIVE_E2E_RESULT='skipped').returncode, 0)

    def test_exemption_requires_nonempty_source_bound_evidence(self):
        for overrides in ({'CHANGED_FILE_COUNT': ''}, {'CHANGED_FILE_COUNT': '0'},
                          {'CHANGED_FILE_COUNT': '-1'}, {'CHANGED_FILE_COUNT': 'invalid'},
                          {'CLASSIFIED_SHA': ''}, {'CLASSIFIED_SHA': 'b' * 40},
                          {'TESTED_SHA': 'unknown'}, {'MARKDOWN_ONLY': ''}):
            with self.subTest(overrides=overrides):
                result = self.run_gate(NATIVE_E2E_RESULT='skipped', **overrides)
                self.assertNotEqual(result.returncode, 0, result.stdout)

    def test_scheduled_and_manual_runs_require_every_lane(self):
        for event in ('schedule', 'workflow_dispatch', 'workflow_call', 'merge_group'):
            with self.subTest(event=event):
                result = self.run_gate(EVENT_NAME=event, NATIVE_NODE_RESULT='skipped')
                self.assertNotEqual(result.returncode, 0, result.stdout)


if __name__ == '__main__':
    unittest.main()
