import importlib.util
from pathlib import Path
import sqlite3
import tempfile
import unittest


class LiveServiceConfigurationTests(unittest.TestCase):
    def test_sessions_are_admitted_to_shared_services_without_process_credentials(self):
        spec = importlib.util.spec_from_file_location(
            "services", Path(__file__).parents[1] / "live-harness-services.py"
        )
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            manifest = module.initialize(root, ["codex", "pi"])
            self.assertEqual(len(manifest["scopes"]), 4)
            self.assertEqual(
                len({scope["session_id"] for scope in manifest["scopes"]}), 4
            )
            self.assertEqual(
                {scope["workspace_id"] for scope in manifest["scopes"]},
                {"workspace-live-harness-matrix"},
            )
            self.assertTrue(
                all(scope["attempt_id"] is None for scope in manifest["scopes"])
            )
            self.assertNotIn("token", str(manifest))
            with sqlite3.connect(root / "memory.db") as db:
                self.assertEqual(
                    db.execute("SELECT count(*) FROM agent_memory").fetchone()[0], 0
                )
            self.assertTrue((root / "blobs").is_dir())
