#!/usr/bin/env python3
"""Bootstrap contracts use temporary homes and fake executables, never sudo/network."""
import hashlib
import importlib.util
import json
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[1]

class InitDevTests(unittest.TestCase):
    def setUp(self):
        path = ROOT / 'scripts/init_dev.py'
        self.assertTrue(path.is_file(), 'make init needs an implemented bootstrap')
        spec = importlib.util.spec_from_file_location('init_dev', path)
        self.init = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(self.init)

    def test_repository_pins_and_all_dependency_trees_are_authoritative(self):
        pins = self.init.load_pins(ROOT)
        self.assertEqual(pins['node'], (ROOT / '.node-version').read_text().strip())
        self.assertEqual(pins['rust'], '1.95.0')
        self.assertEqual(self.init.NPM_TREES, ('.', 'src/ui/next', 'src/cli', '.github/test-tools'))
        self.assertIn('make init', (ROOT / 'Makefile').read_text())

    def test_plan_has_no_side_effects_and_includes_full_host_test_prerequisites(self):
        with tempfile.TemporaryDirectory() as temp:
            home = Path(temp)
            result = subprocess.run(['python3', str(ROOT/'scripts/init_dev.py'), '--plan'], cwd=ROOT,
                env={'HOME': str(home), 'PATH': '/usr/bin:/bin'}, text=True, capture_output=True)
            self.assertEqual(result.returncode, 0, result.stderr)
            for term in ('rustfmt', 'clippy', 'npm ci', 'Chromium', 'Docker', 'OpenCode'):
                self.assertIn(term, result.stdout)
            self.assertEqual(list(home.iterdir()), [])

    def test_supported_system_plans_do_not_remove_packages_or_grant_docker_root_access(self):
        ubuntu = self.init.system_packages('linux', 'ubuntu')
        debian = self.init.system_packages('linux', 'debian')
        for packages in (ubuntu, debian):
            self.assertIn('libwebkit2gtk-4.1-dev', packages)
            self.assertIn('libssl-dev', packages)
            self.assertIn('python3-venv', packages)
        self.assertIn('pkg-config', self.init.system_packages('darwin', ''))
        with self.assertRaises(ValueError): self.init.system_packages('linux', 'unsupported')
        source = (ROOT/'scripts/init_dev.py').read_text()
        self.assertNotIn('usermod', source)
        self.assertNotIn('rustup default', source)
        self.assertNotIn('shell=True', source)
        self.assertNotIn('--break-system-packages', source)

    def test_download_checksum_rejects_corruption_and_does_not_trust_filename(self):
        with tempfile.TemporaryDirectory() as temp:
            path = Path(temp) / 'node.tar.gz'
            path.write_bytes(b'synthetic download')
            self.init.verify_download(path, hashlib.sha256(path.read_bytes()).hexdigest())
            with self.assertRaises(ValueError): self.init.verify_download(path, '0'*64)
            path.write_bytes(b'')
            with self.assertRaises(ValueError): self.init.verify_download(path, hashlib.sha256(b'').hexdigest())

    def test_dependency_stamp_binds_both_manifests_node_and_install_success(self):
        with tempfile.TemporaryDirectory() as temp:
            tree = Path(temp)
            (tree/'package.json').write_text('{"name":"test"}')
            (tree/'package-lock.json').write_text('{"lockfileVersion":3}')
            expected = self.init.dependency_fingerprint(tree, '22.22.1')
            self.assertFalse(self.init.dependencies_current(tree, '22.22.1'))
            (tree/'node_modules').mkdir()
            self.init.record_dependencies(tree, '22.22.1')
            self.assertTrue(self.init.dependencies_current(tree, '22.22.1'))
            self.assertFalse(self.init.dependencies_current(tree, '22.22.2'))
            (tree/'package.json').write_text('{"name":"changed"}')
            self.assertNotEqual(self.init.dependency_fingerprint(tree, '22.22.1'), expected)
            self.assertFalse(self.init.dependencies_current(tree, '22.22.1'))

    def test_missing_or_foreign_docker_is_not_a_successful_setup(self):
        def probe(argv):
            if argv[1:3] == ['context', 'inspect']:
                return '[{"Endpoints":{"docker":{"Host":"ssh://production.example"}}}]'
            return '24.0.0'
        with self.assertRaises(ValueError): self.init.check_docker(probe)
        def local(argv):
            if argv[1:3] == ['context', 'inspect']:
                return '[{"Endpoints":{"docker":{"Host":"unix:///var/run/docker.sock"}}}]'
            return '24.0.0'
        self.init.check_docker(local)
        def unavailable(argv): raise RuntimeError('daemon unavailable')
        with self.assertRaises(RuntimeError): self.init.check_docker(unavailable)

    def test_docker_context_takes_precedence_over_a_local_docker_host(self):
        seen = []
        def probe(argv):
            seen.append(argv)
            return '[{"Endpoints":{"docker":{"Host":"ssh://production.example"}}}]'
        with patch.dict('os.environ', {'DOCKER_CONTEXT': 'production', 'DOCKER_HOST': 'unix:///var/run/docker.sock'}):
            with self.assertRaises(ValueError): self.init.check_docker(probe)
        self.assertEqual(seen, [['docker', 'context', 'inspect', 'production']])

    def test_dependency_install_stamp_cannot_cross_host_architectures(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            for name in ('package.json', 'package-lock.json'): (root/name).write_text('{}')
            with patch.object(self.init.platform, 'machine', return_value='x86_64'):
                first = self.init.dependency_fingerprint(root, '22.22.1')
            with patch.object(self.init.platform, 'machine', return_value='aarch64'):
                self.assertNotEqual(first, self.init.dependency_fingerprint(root, '22.22.1'))

    def test_local_tool_links_are_repeatable_and_do_not_replace_regular_files(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp); binary = root/'real'; binary.write_text('fixture')
            link = root/'link'
            self.init.link_tool(binary, link); self.init.link_tool(binary, link)
            self.assertEqual(link.resolve(), binary)
            link.unlink(); link.write_text('user file')
            with self.assertRaises(RuntimeError): self.init.link_tool(binary, link)
            self.assertEqual(link.read_text(), 'user file')

    def test_local_node_installs_npx_and_locked_development_dependencies(self):
        source = (ROOT/'scripts/init_dev.py').read_text()
        self.assertIn("TOOLS/'bin/npx'", source)
        self.assertIn('--include=dev', source)

    def test_python_tooling_rejects_unsupported_old_python(self):
        with patch.object(self.init.shutil, 'which', side_effect=lambda name: '/usr/bin/python3' if name == 'python3' else None):
            with self.assertRaisesRegex(RuntimeError, '3.11'):
                self.init.select_python(lambda argv: '[3, 10]')
            self.assertEqual(self.init.select_python(lambda argv: '[3, 11]'), '/usr/bin/python3')

    def test_doctor_disables_implicit_toolchain_downloads_for_every_probe(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            for name in self.init.NPM_TREES:
                (root/name/'node_modules').mkdir(parents=True)
            (root/'.github/test-tools/package.json').write_text(json.dumps({'dependencies': {'opencode-ai': '1.18.15'}}))
            calls = []
            def execute(argv, **options):
                calls.append(argv)
                self.assertEqual(options['env']['RUSTUP_AUTO_INSTALL'], '0')
                self.assertEqual(options['env']['CARGO_NET_OFFLINE'], 'true')
                if argv[:2] == ['node', '-p']: return '22.22.1'
                if argv[0] == 'rustc': return 'rustc 1.95.0 (fixture)'
                if argv[0] == 'opencode': return '1.18.15'
                return ''
            with patch.object(self.init, 'ROOT', root), patch.object(self.init, 'check_host'), patch.object(self.init, 'check_docker'), patch.object(self.init, 'execute', side_effect=execute):
                self.init.doctor({'node': '22.22.1', 'rust': '1.95.0'}, 'linux')
            self.assertIn(['cargo', 'clippy', '--version'], calls)
            self.assertTrue(any('chromium.launch' in str(argv) for argv in calls))

    def test_repeated_setup_does_not_modify_shell_profiles_or_start_business_services(self):
        source = (ROOT/'scripts/init_dev.py').read_text()
        for forbidden in ('.bashrc', '.zshrc', 'docker compose up', 'gh release', 'kubectl apply'):
            self.assertNotIn(forbidden, source)
        self.assertIn('--no-modify-path', source)
        self.assertIn('--locked', source)

if __name__ == '__main__': unittest.main()
