"""Dependency setup integrity and error propagation, without network or builds."""
from pathlib import Path
import os
import subprocess
import tempfile
import unittest

HERE = Path(__file__).resolve().parent


class FocusedDependencyFetch(unittest.TestCase):
    def invoke(self, *, mismatch=False, cargo_exit=0):
        with tempfile.TemporaryDirectory(prefix='ohc-publication-fetch-') as directory:
            root = Path(directory)
            harness = root/'scripts/site-publication'
            harness.mkdir(parents=True)
            for name in ['fetch.sh', 'verify_lock.py']:
                (harness/name).write_bytes((HERE/name).read_bytes())
            lock = 'version = 4\n[[package]]\nname = "fixture"\nversion = "1.0.0"\nsource = "registry+https://github.com/rust-lang/crates.io-index"\nchecksum = "' + 'a'*64 + '"\n'
            (root/'Cargo.lock').write_text(lock)
            (harness/'Cargo.lock').write_text(lock.replace('a'*64, 'b'*64) if mismatch else lock)
            marker = root/'called'
            for name, content in {
                'rustc': '#!/bin/sh\nprintf "host: x86_64-unknown-linux-gnu\\n"\n',
                'cargo': '#!/bin/sh\nprintf "%s\\n" "$*" > "$OHC_FETCH_MARKER"\nexit '+str(cargo_exit)+'\n',
            }.items():
                command = root/name
                command.write_text(content)
                command.chmod(0o700)
            env = dict(os.environ, PATH=str(root)+os.pathsep+os.environ['PATH'], OHC_FETCH_MARKER=str(marker))
            result = subprocess.run(['bash', str(harness/'fetch.sh')], cwd=root, env=env, capture_output=True, text=True, timeout=10)
            return result.returncode, marker.read_text().strip() if marker.exists() else None

    def test_drift_stops_before_dependency_fetch(self):
        status, call = self.invoke(mismatch=True)
        self.assertNotEqual(status, 0)
        self.assertIsNone(call)

    def test_only_the_locked_host_focused_graph_is_fetched(self):
        status, call = self.invoke()
        self.assertEqual(status, 0)
        self.assertEqual(call, 'fetch --locked --manifest-path scripts/site-publication/Cargo.toml --target x86_64-unknown-linux-gnu')

    def test_dependency_failure_is_not_hidden(self):
        self.assertEqual(self.invoke(cargo_exit=97)[0], 97)


if __name__ == '__main__': unittest.main()
