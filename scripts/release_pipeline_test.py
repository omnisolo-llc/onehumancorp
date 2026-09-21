#!/usr/bin/env python3
"""Release transport/collector regressions; fixture bytes are not product builds."""
import copy
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
import zipfile
import release_contract as contract

ROOT = Path(__file__).resolve().parents[1]


def identity():
    return contract.resolve_identity(ref='refs/tags/v1.2.3', event='push', run_number=120,
        attempt=1, revision='a' * 40, repository='omnisolo-llc/onehumancorp', base_version='1.2.3')


def module(test, name):
    path = ROOT / 'scripts' / (name + '.py')
    test.assertTrue(path.is_file(), name + ' must exist')
    spec = importlib.util.spec_from_file_location(name, path)
    loaded = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(loaded)
    return loaded


class DesktopCollectorTests(unittest.TestCase):
    def setUp(self):
        self.collector = module(self, 'collect_desktop')
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)

    def assets(self, names):
        paths = []
        for name in names:
            path = self.root / 'target' / 'release' / 'bundle' / name
            path.parent.mkdir(parents=True, exist_ok=True)
            if name.endswith('.app'):
                path.mkdir()
            else:
                path.write_bytes(b'fixture bytes ' + name.encode())
            paths.append(str(path))
        return paths

    def collect(self, os_name, arch, paths, **kwargs):
        return self.collector.collect(self.root, os_name, arch, paths, '1.2.3', identity(), **kwargs)

    def test_linux_collects_exact_reported_outputs_and_keeps_signature_bytes(self):
        paths = self.assets(['appimage/Space App_1.2.3_amd64.AppImage',
            'appimage/Space App_1.2.3_amd64.AppImage.sig', 'deb/Space App.deb', 'rpm/Space App.rpm'])
        self.collect('linux', 'x86_64', paths)
        self.assertEqual({p.name for p in (self.root / 'dist').iterdir()},
            set(contract.expected_assets(identity())['desktop-linux-x86_64']) | {'release-payload-desktop-linux-x86_64.json'})
        self.assertEqual((self.root / 'dist/OmniSolo-v1.2.3-Linux-x86_64.AppImage.sig').read_bytes(), Path(paths[1]).read_bytes())

    def test_empty_ambiguous_missing_signature_and_external_outputs_fail(self):
        valid = self.assets(['x.AppImage', 'x.AppImage.sig', 'x.deb', 'x.rpm'])
        external = self.root / 'outside.AppImage'; external.write_text('foreign')
        for paths in ([], valid[:-1], valid + [valid[0]], [str(external)] + valid[1:], valid[::2]):
            with self.subTest(paths=paths), self.assertRaises(ValueError):
                self.collect('linux', 'x86_64', paths)
        with self.assertRaises(ValueError):
            self.collector.collect(self.root, 'linux', 'x86_64', valid, '9.9.9', identity())
        self.assertFalse((self.root / 'dist').exists())

    def test_windows_portable_preserves_hidden_web_runtime_and_node(self):
        paths = self.assets(['msi/Space App.msi', 'msi/Space App.msi.sig'])
        (self.root / 'target/release/app.exe').write_bytes(b'fixture executable')
        runtime = self.root / 'src/ui/tauri/native-resources'
        for name in ('bin/node.exe', 'web/.next/server/file.js', 'runtime-manifest.json', 'node-distribution.json', 'NODE-LICENSE'):
            path = runtime / name; path.parent.mkdir(parents=True, exist_ok=True); path.write_text('fixture')
        self.collect('windows', 'x86_64', paths)
        with zipfile.ZipFile(self.root / 'dist/OmniSolo-v1.2.3-Windows-Portable.zip') as archive:
            self.assertIn('OmniSolo.exe', archive.namelist())
            self.assertIn('native-runtime/bin/node.exe', archive.namelist())
            self.assertIn('native-runtime/web/.next/server/file.js', archive.namelist())

    def test_macos_verifies_without_modifying_already_signed_updater(self):
        paths = self.assets(['macos/Space App.app', 'macos/Space App.app.tar.gz',
            'macos/Space App.app.tar.gz.sig', 'dmg/Space App.dmg'])
        calls = []
        from types import SimpleNamespace
        def run(args, **kwargs):
            calls.append(args)
            if args[0] == 'ditto': Path(args[-1]).write_bytes(b'fixture macOS zip')
            return SimpleNamespace(stdout='{"status":"Accepted"}')
        before = {p: Path(p).read_bytes() for p in paths if Path(p).is_file()}
        self.collect('macos', 'aarch64', paths, macos_signed=True, run=run)
        submit = next((c for c in calls if c[:3] == ['xcrun', 'notarytool', 'submit']), None)
        self.assertIsNotNone(submit, 'the DMG needs its own notarization submission')
        self.assertTrue(submit[3].endswith('.dmg'))
        self.assertIn('--keychain-profile', submit)
        self.assertNotIn('--password', submit)
        staples = [c for c in calls if c[:3] == ['xcrun', 'stapler', 'staple']]
        self.assertEqual(len(staples), 1)
        self.assertTrue(staples[0][-1].endswith('.dmg'))
        self.assertEqual(before, {p: Path(p).read_bytes() for p in before})

    def test_macos_notarization_rejection_stops_before_collecting_packages(self):
        from types import SimpleNamespace
        paths = self.assets(['macos/Rejected.app', 'macos/Rejected.app.tar.gz',
            'macos/Rejected.app.tar.gz.sig', 'dmg/Rejected.dmg'])
        calls = []
        def run(args, **kwargs):
            calls.append(args)
            return SimpleNamespace(stdout='{"status":"Invalid"}')
        with self.assertRaises(ValueError):
            self.collect('macos', 'aarch64', paths, macos_signed=True, run=run)
        self.assertFalse((self.root / 'dist').exists())
        self.assertFalse(any(c[:3] == ['xcrun', 'stapler', 'staple'] for c in calls))


class DraftVerificationTests(unittest.TestCase):
    def setUp(self):
        self.verifier = module(self, 'verify_release_upload')
        self.temp = tempfile.TemporaryDirectory(); self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        for group, names in contract.expected_assets(identity()).items():
            folder = self.root / 'incoming' / group; folder.mkdir(parents=True)
            for name in names: (folder / name).write_bytes(b'fixture-not-a-real-package')
            contract.record_payload(folder, group, identity())
        self.dist = self.root / 'dist'
        contract.assemble(self.root / 'incoming', self.dist, identity())
        self.release = {'id': 72, 'draft': True, 'tag_name': 'v1.2.3', 'target_commitish': 'a' * 40, 'prerelease': False}
        self.assets = [{'id': i + 100, 'name': p.name, 'size': p.stat().st_size,
            'digest': 'sha256:' + contract.digest(p), 'state': 'uploaded'} for i, p in enumerate(sorted(self.dist.iterdir()))]

    def verify(self, release=None, assets=None):
        return self.verifier.verify_snapshot(identity(), self.dist, 72,
            self.release if release is None else release, self.assets if assets is None else assets)

    def test_complete_draft_is_verified_including_json_and_checksums(self):
        self.verify()
        self.assertEqual(len(self.assets), 32)

    def test_missing_corrupted_duplicate_pending_or_extra_assets_never_verify(self):
        for kind in ('missing', 'digest', 'size', 'duplicate', 'pending', 'extra', 'no_digest'):
            assets = copy.deepcopy(self.assets)
            if kind == 'missing': assets.pop()
            elif kind == 'digest': assets[0]['digest'] = 'sha256:' + '0' * 64
            elif kind == 'size': assets[0]['size'] += 1
            elif kind == 'duplicate': assets.append(assets[0])
            elif kind == 'pending': assets[0]['state'] = 'starter'
            elif kind == 'extra': assets.append({**assets[0], 'name': 'unexpected.zip', 'id': 999})
            else: del assets[0]['digest']
            with self.subTest(kind=kind), self.assertRaises(ValueError): self.verify(assets=assets)

    def test_wrong_release_commit_tag_channel_or_public_state_never_verify(self):
        for key, value in [('id', 73), ('draft', False), ('tag_name', 'v9.9.9'),
                           ('target_commitish', 'main'), ('prerelease', True)]:
            with self.subTest(key=key), self.assertRaises(ValueError): self.verify(release={**self.release, key: value})

    def test_local_manifest_tampering_is_rejected_even_with_matching_uploaded_hash(self):
        manifest = self.dist / 'release-manifest.json'
        value = json.loads(manifest.read_text()); value['revision'] = 'b' * 40
        manifest.write_text(json.dumps(value))
        for asset in self.assets:
            if asset['name'] == manifest.name:
                asset.update(size=manifest.stat().st_size, digest='sha256:' + contract.digest(manifest))
        with self.assertRaises(ValueError): self.verify()


@unittest.skipIf(not has_yaml, 'pyyaml not installed')
class PipelineWorkflowTests(unittest.TestCase):
    def test_each_builder_attests_its_own_outputs_and_linux_baseline_is_pinned(self):
        workflow = yaml.safe_load((ROOT / '.github/workflows/release.yml').read_text())
        jobs = workflow['jobs']
        for name in ('build-release-artifacts', 'build-desktop-installers', 'build-mobile-artifacts', 'build-server-web-image'):
            job = jobs[name]
            step = next((s for s in job['steps'] if s.get('uses', '').startswith('actions/attest-build-provenance@')), None)
            self.assertIsNotNone(step, name + ' must attest its own outputs')
            self.assertRegex(step['uses'], r'@[a-f0-9]{40}$')
            self.assertEqual(step['with']['subject-path'], 'dist/*')
            self.assertEqual(job['permissions']['id-token'], 'write')
            self.assertEqual(job['permissions']['attestations'], 'write')
            self.assertIn("outputs.publish == 'true'", step['if'])
        linux = next(row for row in jobs['build-release-artifacts']['strategy']['matrix']['include'] if row['name'] == 'linux-x86_64')
        self.assertEqual(linux['runner'], 'ubuntu-22.04')
        action = next(s for s in jobs['build-desktop-installers']['steps'] if s.get('uses', '').startswith('tauri-apps/tauri-action@'))
        self.assertIn('-- --locked', action['with']['args'])

    def test_official_tauri_outputs_drive_collection_and_never_publish_directly(self):
        workflow = yaml.safe_load((ROOT / '.github/workflows/release.yml').read_text())
        job = workflow['jobs']['build-desktop-installers']
        steps = job['steps']
        action = next((s for s in steps if s.get('uses', '').startswith('tauri-apps/tauri-action@')), None)
        self.assertIsNotNone(action, 'use the official desktop action')
        self.assertRegex(action['uses'], r'@[a-f0-9]{40}$')
        self.assertEqual(action['with']['projectPath'], 'src/ui/tauri')
        self.assertEqual(action['with']['tauriScript'], 'node ../../../scripts/run-tauri.mjs')
        self.assertFalse(any(k in action['with'] for k in ('tagName', 'releaseName', 'releaseId')))
        collector = next((s for s in steps if 'collect_desktop.py' in s.get('run', '')), None)
        self.assertIsNotNone(collector)
        self.assertIn('artifactPaths', collector['env']['TAURI_ARTIFACT_PATHS'])
        self.assertIn('appVersion', collector['env']['TAURI_APP_VERSION'])
        self.assertNotIn('find "$BUNDLE_ROOT"', str(steps))

    def test_small_macos_runners_bound_compilers_without_dropping_targets(self):
        jobs = yaml.safe_load((ROOT / '.github/workflows/release.yml').read_text())['jobs']
        for name in ('build-release-artifacts', 'build-desktop-installers'):
            self.assertEqual(jobs[name].get('env', {}).get('CARGO_BUILD_JOBS'),
                "${{ startsWith(matrix.name, 'macos-') && '1' || '2' }}")
        setup = next(step for step in jobs['build-desktop-installers']['steps'] if step.get('uses') == './.github/actions/setup-native')
        self.assertEqual(setup['with']['node-scope'], 'web')

    def test_upload_stays_draft_until_remote_asset_verification_succeeds(self):
        workflow = yaml.safe_load((ROOT / '.github/workflows/release.yml').read_text())
        steps = workflow['jobs']['publish-release']['steps']
        create = next(i for i, s in enumerate(steps) if s.get('uses', '').startswith('softprops/action-gh-release@'))
        self.assertTrue(steps[create]['with']['draft'])
        verify = next((i for i, s in enumerate(steps) if 'verify_release_upload.py' in s.get('run', '')), -1)
        promote = next((i for i, s in enumerate(steps) if 'draft=false' in s.get('run', '')), -1)
        self.assertGreater(verify, create)
        self.assertGreater(promote, verify)
        self.assertIn('verified_release_id', str(steps[promote]))
        self.assertNotIn('always()', steps[promote].get('if', ''))


if __name__ == '__main__':
    unittest.main()
