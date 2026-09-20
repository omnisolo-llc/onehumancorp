"""Regression tests for source-bound, complete Rust execution evidence."""
import copy
import importlib.util
import pathlib
import tempfile
import unittest

PATH = pathlib.Path(__file__).with_name('ci_rust.py')

class RustEvidenceTests(unittest.TestCase):
    def setUp(self):
        self.assertTrue(PATH.exists(), 'archive evidence verifier must exist')
        spec = importlib.util.spec_from_file_location('ci_rust', PATH)
        self.mod = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(self.mod)
        self.identity = {'sha': 'a' * 40, 'run': '123', 'attempt': '1', 'platform': 'Linux-x86_64', 'nextest': '0.9.145'}
        self.listing = {'test-count': 3, 'rust-suites': {'pkg': {'status': 'listed', 'binary-id': 'pkg', 'testcases': {
            'tests::one': {'ignored': False, 'filter-match': {'status': 'matches'}},
            'tests::two': {'ignored': False, 'filter-match': {'status': 'matches'}},
            'tests::ignored': {'ignored': True, 'filter-match': {'status': 'matches'}}}}}}

    def test_inventory_preserves_ignored_and_rejects_filtered_or_incomplete_listing(self):
        active, ignored = self.mod.inventory(self.listing)
        self.assertEqual(active, ['["pkg","tests::one"]', '["pkg","tests::two"]'])
        self.assertEqual(ignored, ['["pkg","tests::ignored"]'])
        for mutate in [lambda x: x.update({'test-count': 7}),
                       lambda x: x['rust-suites']['pkg'].update({'status': 'skipped'}),
                       lambda x: x['rust-suites']['pkg']['testcases']['tests::one']['filter-match'].update({'status': 'mismatch'})]:
            broken = copy.deepcopy(self.listing); mutate(broken)
            with self.assertRaises(ValueError): self.mod.inventory(broken)

    def test_exact_report_union_cannot_hide_missing_duplicate_or_wrong_source(self):
        ids, ignored = self.mod.inventory(self.listing)
        manifest = {'identity': self.identity, 'tests': ids, 'ignored': ignored}
        reports = [{'identity': self.identity, 'index': i + 1, 'total': 2, 'complete': True,
                    'selected': [name], 'results': {name: 'passed'}} for i, name in enumerate(ids)]
        self.assertEqual(self.mod.verify_reports(manifest, reports, self.identity, 2), 2)
        for broken in [reports[:1], [reports[0], reports[0]],
                       [reports[0], {**reports[1], 'complete': False}],
                       [reports[0], {**reports[1], 'results': {}}],
                       [reports[0], {**reports[1], 'results': {ids[1]: 'skipped'}}],
                       [reports[0], {**reports[1], 'identity': {**self.identity, 'attempt': '2'}}],
                       [reports[0], {**reports[1], 'identity': {**self.identity, 'sha': 'b' * 40}}]]:
            with self.subTest(broken=broken), self.assertRaises(ValueError):
                self.mod.verify_reports(manifest, broken, self.identity, 2)

    def test_junit_requires_unique_terminal_success_without_flaky_or_skipped_cases(self):
        good = '<testsuites><testsuite name="pkg"><testcase name="one"/></testsuite></testsuites>'
        self.assertEqual(self.mod.junit_results(good), {'["pkg","one"]': 'passed'})
        for child in ['failure', 'error', 'skipped', 'flakyFailure', 'rerunFailure']:
            value = good.replace('<testcase name="one"/>', f'<testcase name="one"><{child}/></testcase>')
            self.assertEqual(self.mod.junit_results(value), {'["pkg","one"]': 'failed'})
        for bad in [good.replace('<testcase name="one"/>', '<testcase name="one"/><testcase name="one"/>'),
                    '<!DOCTYPE foo><testsuites/>', '<testsuites><testcase name="one"/></testsuites>']:
            with self.assertRaises(ValueError): self.mod.junit_results(bad)

    def test_libtest_inventory_is_not_inferred_from_source_regexes(self):
        output = 'a::test_one: test\nb::test_two: test\n\n2 tests, 0 benchmarks\n'
        self.assertEqual(self.mod.libtest_names(output), {'a::test_one', 'b::test_two'})
        for bad in [output.replace('2 tests', '3 tests'), 'warning: custom harness\n', 'a: benchmark\n0 tests, 1 benchmark\n']:
            with self.assertRaises(ValueError): self.mod.libtest_names(bad)

    def test_archive_checksum_and_manifest_identity_fail_closed(self):
        with tempfile.TemporaryDirectory() as d:
            root = pathlib.Path(d); archive = root / 'tests.tar.zst'; archive.write_bytes(b'archive fixture')
            manifest = {'identity': self.identity, 'archive_sha256': self.mod.digest(archive)}
            self.mod.check_archive(manifest, archive, self.identity)
            archive.write_bytes(b'changed')
            with self.assertRaises(ValueError): self.mod.check_archive(manifest, archive, self.identity)

if __name__ == '__main__': unittest.main()
