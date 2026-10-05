"""Safety checks run by native contracts without compiling or killing Redis clients."""
from pathlib import Path
import hashlib
import importlib.util
import json
import os
import shutil
import subprocess
import tempfile
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[2]


class RunnerTests(unittest.TestCase):
    def run_fixture(self, supplied_url=False):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            lane = root/'scripts/redis-reconnect'
            lane.mkdir(parents=True)
            shutil.copyfile(ROOT/'scripts/redis-reconnect/run.sh', lane/'run.sh')
            binaries = root/'bin'
            binaries.mkdir()
            for name, source in {
                'cargo': '#!/bin/sh\ntouch "$NATIVE_CAPTURE"\n',
                'redis-server': '#!/bin/sh\nexec sleep 30\n',
                'redis-cli': '#!/bin/sh\necho process_id:1\n',
            }.items():
                (binaries/name).write_text(source)
                (binaries/name).chmod(0o755)
            env = dict(os.environ, PATH=str(binaries)+os.pathsep+os.environ['PATH'],
                       NATIVE_CAPTURE=str(root/'native-started'))
            env.pop('OHC_TEST_REDIS_URL', None)
            env.pop('OHC_REDIS_SERVICE_ISOLATION', None)
            if supplied_url:
                env['OHC_TEST_REDIS_URL'] = 'redis://127.0.0.1:6379/'
                env['OHC_REDIS_SERVICE_ISOLATION'] = '1'
            result = subprocess.run(['bash', str(lane/'run.sh')], env=env,
                                    capture_output=True, text=True, timeout=10)
            self.assertNotEqual(result.returncode, 0)
            self.assertFalse((root/'native-started').exists())
            return result.stderr

    def test_runner_refuses_supplied_service_even_with_isolation_claim(self):
        self.assertIn('owns its Redis process', self.run_fixture(supplied_url=True))

    def test_runner_refuses_wrong_server_pid_before_cargo(self):
        self.assertIn('not owned by this test process', self.run_fixture())


class SourceProofTests(unittest.TestCase):
    def setUp(self):
        spec = importlib.util.spec_from_file_location('redis_source_proof', ROOT/'scripts/redis-reconnect/verify_source.py')
        self.proof = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(self.proof)
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.proof.ROOT = Path(self.temp.name)
        self.proof.LANE = self.proof.ROOT/'scripts/redis-reconnect'
        self.proof.LANE.mkdir(parents=True)
        self.queue = self.proof.ROOT/'src/server/queue.rs'
        self.queue.parent.mkdir(parents=True)
        self.queue.write_bytes(b'original queue source')
        (self.proof.LANE/'source-inputs.json').write_text('["src/server/queue.rs"]')
        self.records = [{'source': name, 'source_sha256': digest} for name, digest in self.proof.snapshot().items()]
        for kind, name, trait in [('struct', 'Job', None), ('trait', 'TaskQueue', None),
                                  ('struct', 'RedisTaskQueue', None), ('impl', 'RedisTaskQueue', None),
                                  ('impl', 'RedisTaskQueue', 'TaskQueue')]:
            self.records.append(dict(self.records[0], kind=kind, name=name, trait=trait,
                                     start=0, end=8, selection_sha256=hashlib.sha256(b'original').hexdigest()))

    def write_manifest(self):
        (self.proof.LANE/'source-manifest.json').write_text(json.dumps(self.records))

    def test_complete_unchanged_proof_passes(self):
        self.write_manifest()
        self.proof.verify()

    def test_changed_source_or_selected_bytes_fail(self):
        self.records[-1]['selection_sha256'] = '0'*64
        self.write_manifest()
        with self.assertRaisesRegex(ValueError, 'Selected queue bytes changed'):
            self.proof.verify()
        self.queue.write_bytes(b'changed queue source')
        with self.assertRaisesRegex(ValueError, 'complete current inputs'):
            self.proof.verify()

    def test_missing_input_or_selection_fails(self):
        self.records.pop()
        self.write_manifest()
        with self.assertRaisesRegex(ValueError, 'all five exact queue selections'):
            self.proof.verify()
        self.records.pop(0)
        self.write_manifest()
        with self.assertRaisesRegex(ValueError, 'complete current inputs'):
            self.proof.verify()

    def test_missing_manifest_restores_only_from_exact_cargo_package_output(self):
        artifact = self.proof.ROOT/'artifact'
        artifact.mkdir()
        (artifact/'source-manifest.json').write_text(json.dumps(self.records))
        output = self.proof.ROOT/'cargo-output.log'
        package = 'exact-current-manifest-id'
        metadata = {'packages': [{'id': package, 'manifest_path': str(self.proof.LANE/'Cargo.toml')}]}
        output.write_text(json.dumps({'reason': 'build-script-executed', 'package_id': package,
                                      'out_dir': str(artifact)})+'\nRust test output\n')
        with patch.object(self.proof.subprocess, 'check_output', return_value=json.dumps(metadata)):
            self.proof.restore(output)
            self.assertEqual(json.loads((self.proof.LANE/'source-manifest.json').read_text()), self.records)
            (self.proof.LANE/'source-manifest.json').unlink()
            output.write_text(json.dumps({'reason': 'build-script-executed', 'package_id': 'another-checkout',
                                          'out_dir': str(artifact)}))
            with self.assertRaisesRegex(ValueError, 'one exact build source proof'):
                self.proof.restore(output)
            self.assertFalse((self.proof.LANE/'source-manifest.json').exists())


if __name__ == '__main__':
    unittest.main()
