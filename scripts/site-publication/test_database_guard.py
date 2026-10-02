"""Prove the real runner rejects unsafe DSNs before invoking any native command."""
from pathlib import Path
import os
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]


class PublicationDatabaseGuardTests(unittest.TestCase):
    def invoke(self, url):
        with tempfile.TemporaryDirectory(prefix="ohc-publication-preflight-") as directory:
            folder = Path(directory)
            marker = folder / "native-called"
            cargo = folder / "cargo"
            cargo.write_text('#!/bin/sh\nprintf called > "$OHC_GUARD_CALLED"\nexit 97\n')
            cargo.chmod(0o700)
            env = dict(os.environ, PATH=str(folder) + os.pathsep + os.environ["PATH"], OHC_GUARD_CALLED=str(marker))
            env.pop("OHC_PUBLICATION_TEST_DATABASE_URL", None)
            if url is not None:
                env["OHC_PUBLICATION_TEST_DATABASE_URL"] = url
            result = subprocess.run(["bash", "scripts/site-publication/run.sh"], cwd=ROOT, env=env, capture_output=True, text=True, timeout=10)
            return result.returncode, marker.exists()

    def test_missing_owned_database_stops_before_native_work(self):
        status, called = self.invoke(None)
        self.assertNotEqual(status, 0)
        self.assertFalse(called)

    def test_nonlocal_or_unowned_database_stops_before_native_work(self):
        for url in ["postgres://fixture@example.invalid/ohc_publication_test", "postgres://fixture@127.0.0.1/customer_production", "mysql://fixture@127.0.0.1/ohc_publication_test"]:
            with self.subTest(url=url):
                status, called = self.invoke(url)
                self.assertNotEqual(status, 0)
                self.assertFalse(called)

    def test_connection_override_parameters_are_rejected(self):
        status, called = self.invoke("postgres://fixture@127.0.0.1/ohc_publication_test?host=elsewhere.invalid")
        self.assertNotEqual(status, 0)
        self.assertFalse(called)

    def test_explicit_owned_loopback_reaches_only_the_recording_native_command(self):
        self.assertEqual(self.invoke("postgres://fixture@127.0.0.1:55439/ohc_publication_test"), (97, True))


if __name__ == "__main__":
    unittest.main()
