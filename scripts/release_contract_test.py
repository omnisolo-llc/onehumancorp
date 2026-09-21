#!/usr/bin/env python3
"""Behavioral release identity, packaging and publication contract regressions."""
import importlib.util
import json
from pathlib import Path
import tempfile
import unittest
import sys
try:
    import yaml
    has_yaml = True
except ImportError:
    has_yaml = False
import sys
try:
    import yaml
except ImportError:
    pass

SCRIPT = Path(__file__).with_name('release_contract.py')


@unittest.skipIf(not has_yaml, 'pyyaml not installed')
class ReleaseContractTests(unittest.TestCase):
    def setUp(self):
        self.assertTrue(SCRIPT.is_file(), 'shared release contract must exist')
        spec = importlib.util.spec_from_file_location('release_contract', SCRIPT)
        self.contract = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(self.contract)

    def identity(self, ref='refs/tags/v1.2.3', event='push', **extra):
        return self.contract.resolve_identity(ref=ref, event=event, run_number=120,
            attempt=2, revision='a' * 40, repository='omnisolo-llc/onehumancorp',
            base_version='0.4.47', **extra)

    def test_tag_is_authoritative_for_all_formats(self):
        value = self.identity()
        self.assertEqual(value['version'], '1.2.3')
        self.assertEqual(value['tag'], 'v1.2.3')
        self.assertEqual(value['wix_version'], '1.2.3.0')
        self.assertFalse(value['prerelease'])
        self.assertTrue(value['publish'])

    def test_every_semver_prerelease_is_non_latest(self):
        for suffix in ('rc.1', 'preview.2', 'nightly.99', 'custom-test'):
            with self.subTest(suffix=suffix):
                value = self.identity('refs/tags/v1.2.3-' + suffix)
                self.assertTrue(value['prerelease'])
                self.assertEqual(value['tag'], 'v' + value['version'])

    def test_nightly_and_manual_versions_are_consistent(self):
        value = self.identity('refs/heads/main', 'schedule')
        self.assertEqual(value['version'], '0.4.47-nightly.120.2')
        self.assertEqual(value['tag'], 'v' + value['version'])
        self.assertTrue(value['publish'])
        manual = self.identity('refs/heads/main', 'workflow_dispatch')
        self.assertFalse(manual['publish'])
        self.assertTrue(manual['prerelease'])

    def test_invalid_tags_and_non_release_events_fail(self):
        for ref in ('refs/tags/v1.2', 'refs/tags/v01.2.3', 'refs/tags/v1.2.3-01',
                    'refs/tags/v1.2.3\nINJECT=1', 'refs/tags/v256.1.1'):
            with self.subTest(ref=ref), self.assertRaises(ValueError):
                self.identity(ref)
        with self.assertRaises(ValueError):
            self.identity('refs/tags/v1.2.3', 'pull_request_target')

    def test_prepare_updates_product_versions_and_preserves_dependency_lock(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            for filename, name in self.contract.PRODUCT_CRATES.items():
                p = root / filename
                p.parent.mkdir(parents=True, exist_ok=True)
                p.write_text('[package]\nname = "' + name + '"\nversion = "0.1.0"\n')
            lock = 'version = 4\n\n' + ''.join('[[package]]\nname = "' + name + '"\nversion = "0.1.0"\n\n'
                for name in self.contract.PRODUCT_CRATES.values())
            external = '[[package]]\nname = "serde"\nversion = "1.0.0"\nsource = "registry+https://example.test"\nchecksum = "KEEP"\n'
            (root / 'Cargo.lock').write_text(lock + external)
            config = root / 'src/ui/tauri/tauri.conf.json'
            config.write_text(json.dumps({'version': '0.1.0', 'plugins': {'updater': {'pubkey': 'KEEP'}}}))
            for name in self.contract.NODE_PACKAGES:
                p = root / name
                p.parent.mkdir(parents=True, exist_ok=True)
                p.write_text(json.dumps({'name': 'product', 'version': '0.1.0'}))
                p.with_name('package-lock.json').write_text(json.dumps({'version': '0.1.0', 'lockfileVersion': 3,
                    'packages': {'': {'version': '0.1.0'}, 'node_modules/lib': {'version': '9.0.0'}}}))
            self.contract.prepare(root, self.identity())
            for filename in self.contract.PRODUCT_CRATES:
                self.assertIn('version = "1.2.3"', (root / filename).read_text())
            self.assertIn(external, (root / 'Cargo.lock').read_text())
            updated = json.loads(config.read_text())
            self.assertEqual(updated['version'], '1.2.3')
            self.assertEqual(updated['plugins']['updater']['pubkey'], 'KEEP')
            self.assertIn('/omnisolo-llc/onehumancorp/', updated['plugins']['updater']['endpoints'][0])
            self.assertEqual(updated['bundle']['android']['versionCode'], 120)
            for name in self.contract.NODE_PACKAGES:
                self.assertEqual(json.loads((root / name).read_text())['version'], '1.2.3')
                installed = json.loads((root / name).with_name('package-lock.json').read_text())
                self.assertEqual(installed['packages']['node_modules/lib']['version'], '9.0.0')

    def payloads(self, root, identity):
        for group, names in self.contract.expected_assets(identity).items():
            directory = root / ('release-payload-' + group)
            directory.mkdir(parents=True)
            for name in names:
                (directory / name).write_bytes(b'contract test payload, not a real installer')
            self.contract.record_payload(directory, group, identity)

    def test_complete_matrix_assembles_updater_and_checksums_before_upload(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            identity = self.identity()
            self.payloads(root / 'incoming', identity)
            self.contract.assemble(root / 'incoming', root / 'dist', identity)
            latest = json.loads((root / 'dist/latest.json').read_text())
            self.assertEqual(latest['version'], '1.2.3')
            self.assertEqual(set(latest['platforms']), {'darwin-aarch64', 'darwin-x86_64',
                'windows-x86_64', 'linux-x86_64', 'linux-aarch64'})
            self.assertTrue(all('/v1.2.3/' in row['url'] for row in latest['platforms'].values()))
            sums = (root / 'dist/checksums.txt').read_text()
            self.assertIn('latest.json', sums)
            self.assertIn('release-manifest.json', sums)
            self.assertEqual(len(sums.splitlines()), len(list((root / 'dist').iterdir())) - 1)

    def test_missing_platform_corruption_and_mismatched_version_fail_closed(self):
        for fault in ('missing', 'corrupt', 'version', 'extra'):
            with self.subTest(fault=fault), tempfile.TemporaryDirectory() as temp:
                root = Path(temp)
                identity = self.identity()
                self.payloads(root / 'incoming', identity)
                group, names = next(iter(self.contract.expected_assets(identity).items()))
                directory = root / 'incoming' / ('release-payload-' + group)
                manifest = directory / ('release-payload-' + group + '.json')
                if fault == 'missing': manifest.unlink()
                elif fault == 'corrupt': (directory / names[0]).write_bytes(b'corrupt')
                elif fault == 'extra': (directory / 'unreviewed.env').write_text('not a secret')
                else:
                    value = json.loads(manifest.read_text()); value['version'] = '9.9.9'
                    manifest.write_text(json.dumps(value))
                with self.assertRaises(ValueError):
                    self.contract.assemble(root / 'incoming', root / 'dist', identity)

    def test_build_metadata_remains_in_release_but_uses_a_valid_image_tag(self):
        value = self.identity('refs/tags/v1.2.3+build.7')
        self.assertEqual(value['version'], '1.2.3+build.7')
        self.assertEqual(value['image_tag'], 'v1.2.3_build.7')
        self.assertFalse(value['prerelease'])

    def test_existing_releases_and_conflicting_tags_cannot_be_overwritten(self):
        value = self.identity()
        self.contract.check_publication(value, lambda route: None)
        def existing(route):
            return {'id': 1} if route.startswith('releases/') else None
        with self.assertRaises(ValueError):
            self.contract.check_publication(value, existing)
        def conflict(route):
            return None if route.startswith('releases/') else {'object': {'type': 'commit', 'sha': 'b' * 40}}
        with self.assertRaises(ValueError):
            self.contract.check_publication(value, conflict)
        def matching(route):
            return None if route.startswith('releases/') else {'object': {'type': 'commit', 'sha': 'a' * 40}}
        self.contract.check_publication(value, matching)
        def denied(route):
            raise OSError('API unavailable')
        with self.assertRaises(OSError):
            self.contract.check_publication(value, denied)

    def test_actual_workflow_matrix_and_publish_dependencies_match_the_contract(self):
        if 'yaml' not in sys.modules:
            return self.skipTest('pyyaml not installed')
        root = SCRIPT.parent.parent
        workflow = yaml.safe_load((root / '.github/workflows/release.yml').read_text())
        jobs = workflow['jobs']
        backend = jobs['build-release-artifacts']['strategy']['matrix']['include']
        self.assertEqual({row['target'] for row in backend}, set(self.contract.TARGETS))
        desktop = jobs['build-desktop-installers']['strategy']['matrix']['include']
        groups = {'backend-' + row['target'] for row in backend}
        groups |= {'desktop-' + row['os_name'] + '-' + row['arch'] for row in desktop}
        groups |= {'mobile-' + row['name'] for row in jobs['build-mobile-artifacts']['strategy']['matrix']['include']}
        groups.add('server-web-image')
        self.assertEqual(groups, set(self.contract.expected_assets(self.identity())))
        builders = ('build-release-artifacts', 'build-desktop-installers', 'build-mobile-artifacts', 'build-server-web-image')
        for name in builders:
            self.assertEqual(jobs[name]['needs'], 'release-metadata')
            actions = [s.get('uses') for s in jobs[name]['steps']]
            self.assertIn('./.github/actions/prepare-release', actions)
            self.assertFalse(jobs[name].get('continue-on-error', False))
        self.assertEqual(jobs['release-qualification']['uses'], './.github/workflows/ci.yml')
        publisher = jobs['publish-release']
        self.assertTrue(set(builders) | {'release-metadata', 'release-qualification'} <= set(publisher['needs']))
        steps = publisher['steps']
        create_index = next(i for i, step in enumerate(steps) if step.get('uses', '').startswith('softprops/action-gh-release@'))
        publish = steps[create_index]
        self.assertIn("outputs.publish == 'true'", publish['if'])
        self.assertEqual(publish['with']['target_commitish'], '${{ needs.release-metadata.outputs.revision }}')
        self.assertTrue(publish['with']['fail_on_unmatched_files'])
        self.assertFalse(publish['with']['overwrite_files'])
        self.assertTrue(any('release_contract.py assemble' in step.get('run', '') for step in steps[:create_index]))
        self.assertTrue(any('release_contract.py check-publication' in step.get('run', '') for step in steps[:create_index]))
        self.assertEqual(next(s['with']['pattern'] for s in steps if s.get('uses', '').startswith('actions/download-artifact@')), 'release-payload-*')
        event = workflow.get('on', workflow.get(True))
        self.assertEqual(event['push']['tags'], ['v*'])
        self.assertFalse(event['workflow_dispatch']['inputs']['publish']['default'])
        self.assertFalse(event['workflow_dispatch']['inputs']['publish_cloud_images']['default'])

    def test_native_archives_contain_versioned_manifests_binaries_and_migrations(self):
        import tarfile
        import zipfile
        spec = importlib.util.spec_from_file_location('native_release', SCRIPT.with_name('native-release.py'))
        package = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(package)
        identity = self.identity()
        package.environment_identity = lambda: identity
        for target in self.contract.TARGETS:
            with self.subTest(target=target), tempfile.TemporaryDirectory() as temp:
                root = Path(temp)
                package.ROOT = root
                (root / 'Cargo.toml').write_text('[package]\nname = "omnisolo"\nversion = "1.2.3"\n')
                suffix = '.exe' if 'windows' in target else ''
                binaries = root / 'target' / target / 'release'
                binaries.mkdir(parents=True)
                for name in ('server', 'omnisolo-builtin-agent', 'omnisolo-harness-worker'):
                    (binaries / (name + suffix)).write_bytes(b'Packaging fixture; not an executable')
                for name in ('src/server/migrations', 'src/server/db/migrations'):
                    directory = root / name
                    directory.mkdir(parents=True)
                    (directory / '001.sql').write_text('SELECT 1;')
                archive = package.package(target)
                self.assertEqual(archive.name, self.contract.archive_name(target, identity))
                if suffix:
                    with zipfile.ZipFile(archive) as opened:
                        manifest = json.loads(opened.read('omnisolo/manifest.json'))
                else:
                    with tarfile.open(archive) as opened:
                        manifest = json.load(opened.extractfile('omnisolo/manifest.json'))
                self.assertEqual(manifest['version'], identity['version'])
                self.assertEqual(manifest['tag'], identity['tag'])
                self.assertEqual(manifest['revision'], identity['revision'])
                self.assertEqual(manifest['target'], target)
                self.assertIn('bin/server' + suffix, manifest['files'])
                self.assertIn('src/server/migrations/001.sql', manifest['files'])
                self.assertIn('src/server/db/migrations/001.sql', manifest['files'])
                with self.assertRaises(ValueError):
                    package.package(target, 'wrong-version.zip')
                (binaries / ('server' + suffix)).unlink()
                with self.assertRaises(FileNotFoundError):
                    package.package(target)

    def test_preserved_native_platform_matrix(self):
        targets = self.contract.TARGETS
        self.assertEqual(len(targets), 7)
        for target in ('x86_64-unknown-linux-gnu', 'aarch64-unknown-linux-gnu',
                       'x86_64-apple-darwin', 'aarch64-apple-darwin',
                       'x86_64-pc-windows-gnu', 'x86_64-pc-windows-msvc', 'aarch64-pc-windows-msvc'):
            self.assertIn(target, targets)


if __name__ == '__main__':
    unittest.main()
