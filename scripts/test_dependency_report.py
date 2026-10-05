"""Exercise scanner protocols through real subprocesses, without network access."""
import importlib.util
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from datetime import datetime, timedelta, timezone

SCRIPT = Path(__file__).with_name('dependency_report.py')


class ReportTests(unittest.TestCase):
    def setUp(self):
        self.assertTrue(SCRIPT.exists(), 'manual dependency reporter is missing')
        spec = importlib.util.spec_from_file_location('dependency_report', SCRIPT)
        self.report = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(self.report)
        self.directory = tempfile.TemporaryDirectory()
        self.addCleanup(self.directory.cleanup)
        self.root = Path(self.directory.name)
        self.source = self.root / 'lock'
        self.source.write_text('unchanged')

    def run_cli(self, kind, payload, code=0, *, stderr='', extra='', expected=None):
        fake = self.root / 'scanner.py'
        fake.write_text('import sys\n' + extra + '\n'
                        + f'print({json.dumps(payload)!r})\n'
                        + f'print({stderr!r}, file=sys.stderr)\n'
                        + f'sys.exit({code})\n')
        return self.report.scan(kind, [sys.executable, str(fake)], self.root,
                                self.root / 'raw', [self.source], 2, expected_python=expected)

    def npm(self, total=0):
        return {'auditReportVersion': 2, 'vulnerabilities': ({'affected': {}} if total else {}),
                'metadata': {'vulnerabilities': {'total': total}}}

    def test_complete_npm_reports_distinguish_findings_from_clean(self):
        self.assertEqual(self.run_cli('npm', self.npm())['state'], 'clean')
        result = self.run_cli('npm', self.npm(1), 1)
        self.assertEqual(result['state'], 'findings')
        self.assertEqual(result['summary']['affected_nodes'], 1)
        self.assertTrue((self.root / 'raw.stdout').read_text().strip())

    def test_network_error_and_truncated_json_never_become_clean(self):
        self.assertEqual(self.run_cli('npm', {'error': {'code': 'ENETUNREACH'}}, 1)['state'],
                         'operational_error')
        self.assertEqual(self.run_cli('npm', 'not a report')['state'], 'operational_error')

    def test_nonzero_without_findings_and_crash_with_findings_are_errors(self):
        self.assertEqual(self.run_cli('npm', self.npm(), 1)['state'], 'operational_error')
        self.assertEqual(self.run_cli('npm', self.npm(1), 2)['state'], 'operational_error')

    def test_python_skipped_dependencies_are_incomplete_even_when_exit_is_zero(self):
        payload = {'dependencies': [{'name': 'missing', 'version': '1', 'vulns': [],
                                     'skip_reason': 'package not found'}]}
        self.assertEqual(self.run_cli('python', payload, expected={'missing': '1'})['state'], 'operational_error')

    def test_python_aliases_are_raw_records_not_invented_unique_counts(self):
        payload = {'dependencies': [{'name': 'sample', 'version': '1', 'vulns': [
            {'id': 'PYSEC-EXAMPLE', 'aliases': ['CVE-EXAMPLE'], 'fix_versions': ['2']}]}]}
        result = self.run_cli('python', payload, 1, expected={'sample': '1'})
        self.assertEqual(result['state'], 'findings')
        self.assertEqual(result['summary']['advisory_records'], 1)

    def cargo(self, records, errors):
        summary = {'type': 'summary', 'fields': {
            'advisories': {'errors': errors, 'warnings': 0, 'notes': 0, 'helps': 0},
            'sources': {'errors': 0, 'warnings': 0, 'notes': 0, 'helps': 0}}}
        return '\n'.join(json.dumps(record) for record in [*records, summary])

    def run_cargo(self, records, errors, code):
        # cargo-deny writes its JSONL protocol to stderr, unlike npm/pip-audit.
        return self.run_cli('cargo', None, code, stderr=self.cargo(records, errors),
                            extra='sys.stdout = open("/dev/null", "w")')

    def test_cargo_requires_completion_and_keeps_unmaintained_separate(self):
        self.assertEqual(self.run_cargo([], 0, 0)['state'], 'clean')
        records = [{'type': 'diagnostic', 'fields': {'severity': 'error',
                    'code': 'unmaintained', 'advisory': {'id': 'RUSTSEC-EXAMPLE'}}}]
        result = self.run_cargo(records, 1, 1)
        self.assertEqual(result['state'], 'findings')
        self.assertEqual(result['summary']['categories'], {'unmaintained': 1})
        self.assertEqual(self.run_cli('cargo', None, 1, stderr='metadata failed')['state'],
                         'operational_error')

    def test_cargo_fatal_log_cannot_be_hidden_by_a_partial_summary(self):
        records = [{'type': 'log', 'fields': {'level': 'ERROR', 'message': 'stale database'}}]
        self.assertEqual(self.run_cargo(records, 0, 1)['state'], 'operational_error')

    def test_source_change_during_successful_scan_invalidates_receipt(self):
        extra = f'from pathlib import Path\nPath({str(self.source)!r}).write_text("changed")'
        result = self.run_cli('npm', self.npm(), extra=extra)
        self.assertEqual(result['state'], 'operational_error')
        self.assertIn('source', result['error'])

    def test_missing_command_and_timeout_are_reported(self):
        result = self.report.scan('npm', ['/does/not/exist'], self.root,
                                  self.root / 'missing', [self.source], 1)
        self.assertEqual(result['state'], 'operational_error')
        result = self.report.scan('npm', [sys.executable, '-c', 'import time; time.sleep(1)'],
                                  self.root, self.root / 'timeout', [self.source], 0.01)
        self.assertEqual(result['state'], 'operational_error')

    def test_stale_and_future_database_timestamps_are_errors(self):
        now = datetime.now(timezone.utc)
        self.report.validate_db_time(now.isoformat(), now)
        for value in [now - timedelta(days=8), now + timedelta(hours=1)]:
            with self.assertRaises(ValueError):
                self.report.validate_db_time(value.isoformat(), now)

    def test_python_scans_use_an_explicit_writable_report_cache(self):
        from types import SimpleNamespace
        args = SimpleNamespace(ecosystem=['python'], audit_python=Path(sys.executable),
                               root=self.root, output=self.root / 'report')
        policy = {'python': [{'lock': 'requirements.lock', 'hashed': True}]}
        _, _, command, _ = next(self.report.plans(args, policy))
        self.assertIn('--cache-dir', command)
        self.assertEqual(Path(command[command.index('--cache-dir') + 1]).parent, args.output)

    def test_python_partial_wrong_version_extra_and_duplicate_rows_are_errors(self):
        expected = {'requests': '2.34.2', 'urllib3': '2.8.0'}
        good = [{'name': name, 'version': version, 'vulns': []} for name, version in expected.items()]
        self.assertEqual(self.run_cli('python', {'dependencies': good}, expected=expected)['state'], 'clean')
        for rows in [good[:1], [*good, good[0]], [*good, {'name': 'extra', 'version': '1', 'vulns': []}],
                     [good[0], {'name': 'urllib3', 'version': '2.7.0', 'vulns': []}]]:
            with self.subTest(rows=rows):
                self.assertEqual(self.run_cli('python', {'dependencies': rows}, expected=expected)['state'],
                                 'operational_error')

    def test_declared_python_rejects_wildcard_versions_from_pinned_parser(self):
        # The real parser marks ==2.* as pinned; it still describes multiple versions.
        parsed = {'invalid_lines': [], 'options': [], 'requirements': [{
            'name': 'requests', 'specifier': ['==2.*'], 'is_pinned': True,
            'is_editable': False, 'link': None, 'marker': None, 'is_constraint': False,
            'extras': [], 'install_options': [], 'global_options': [], 'invalid_options': {}}]}
        parser = self.root / 'parser'
        parser.write_text(f'#!{sys.executable}\nprint({json.dumps(parsed)!r})\n')
        parser.chmod(0o755)
        with self.assertRaisesRegex(ValueError, 'exact package pins'):
            self.report.declared_python(parser, self.source)

    def test_wrong_object_shapes_remain_saved_operational_errors(self):
        for kind, payload, stderr in [('python', {'dependencies': ['bad']}, ''),
                                       ('python', {'dependencies': [{'name': 'x', 'version': '1', 'vulns': ['bad']}]}, ''),
                                       ('cargo', None, '["not an object"]')]:
            with self.subTest(kind=kind, payload=payload):
                result = self.run_cli(kind, payload, stderr=stderr, expected={'x': '1'})
                self.assertEqual(result['state'], 'operational_error')
                self.assertTrue((self.root / 'raw.stdout').exists())

    def test_cargo_nonerror_summary_cannot_hide_missing_diagnostics(self):
        for severity in ['warnings', 'notes', 'helps']:
            data = json.loads(self.cargo([], 0))
            data['fields']['advisories'][severity] = 1
            result = self.run_cli('cargo', None, stderr=json.dumps(data),
                                  extra='sys.stdout = open("/dev/null", "w")')
            self.assertEqual(result['state'], 'operational_error')

    def test_cargo_config_bytes_are_part_of_scan_evidence_and_drift_detection(self):
        config = self.root / 'deny.toml'
        config.write_text('[graph]\nall-features = false\n')
        fake = self.root / 'fake.py'
        fake.write_text('import sys\nprint(' + repr(self.cargo([], 0)) + ', file=sys.stderr)\n')
        command = [sys.executable, str(fake), '--config', str(config)]
        result = self.report.scan('cargo', command, self.root, self.root / 'bound', [self.source], 2)
        self.assertEqual(result['state'], 'clean')
        self.assertEqual(result.get('configuration', {}).get('sha256'), self.report.sha256(config))
        fake.write_text('from pathlib import Path\nPath(' + repr(str(config)) + ').write_text("changed")\n'
                        'import sys\nprint(' + repr(self.cargo([], 0)) + ', file=sys.stderr)\n')
        self.assertEqual(self.report.scan('cargo', command, self.root, self.root / 'changed',
                                         [self.source], 2)['state'], 'operational_error')

    def test_inventory_rejects_extra_tracked_lock_and_retains_exact_orphan(self):
        subprocess.run(['git', 'init', '-q', str(self.root)], check=True)
        for path in ['Cargo.lock', 'package-lock.json', 'src/server/Cargo.lock']:
            target = self.root / path
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text('')
        subprocess.run(['git', '-C', str(self.root), 'add', '.'], check=True)
        policy = {'npm': ['.'], 'cargo_focused': [], 'python': [],
                  'inactive_cargo_locks': {'src/server/Cargo.lock': 'No adjacent manifest'}}
        self.report.validate_inventory(self.root, policy)
        (self.root / 'extra').mkdir()
        (self.root / 'extra/Cargo.lock').write_text('')
        subprocess.run(['git', '-C', str(self.root), 'add', '.'], check=True)
        with self.assertRaisesRegex(ValueError, 'inventory'):
            self.report.validate_inventory(self.root, policy)

    def test_npm_child_does_not_inherit_conflicting_graph_environment(self):
        from unittest.mock import patch
        values = {'NODE_ENV': 'production', 'NPM_CONFIG_OMIT': 'dev', 'npm_config_include': 'dev',
                  'NPM_CONFIG_PRODUCTION': 'true', 'NPM_CONFIG_ONLY': 'prod'}
        extra = ('import os\nassert not any(key.lower() in '
                 + repr({key.lower() for key in values}) + ' for key in os.environ)')
        with patch.dict('os.environ', values):
            self.assertEqual(self.run_cli('npm', self.npm(), extra=extra)['state'], 'clean')

    def test_npm_graphs_explicitly_cover_optional_peer_and_development_scope(self):
        from types import SimpleNamespace
        args = SimpleNamespace(ecosystem=['npm'], root=self.root)
        runs = list(self.report.plans(args, {'npm': ['.']}))
        production, full = [command for _, _, command, _ in runs]
        for command in [production, full]:
            self.assertIn('--include=optional', command)
            self.assertIn('--include=peer', command)
        self.assertIn('--omit=dev', production)
        self.assertNotIn('--include=dev', production)
        self.assertIn('--include=dev', full)


if __name__ == '__main__':
    unittest.main()
