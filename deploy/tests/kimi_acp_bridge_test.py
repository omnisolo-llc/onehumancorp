import importlib.util
import json
import os
import stat
import tempfile
import unittest
from pathlib import Path


BRIDGE = (
    Path(__file__).resolve().parents[2]
    / "src/server/harness/sidecars/kimi_acp_bridge.py"
)


def load_bridge():
    spec = importlib.util.spec_from_file_location("omnisolo_kimi_acp", BRIDGE)
    module = importlib.util.module_from_spec(spec)
    assert spec.loader is not None
    spec.loader.exec_module(module)
    return module


class KimiAcpBridgeTest(unittest.TestCase):
    def test_static_provider_writes_secret_free_isolated_config(self):
        bridge = load_bridge()
        with tempfile.TemporaryDirectory() as directory:
            environment = {
                "OPENAI_API_KEY": "secret-canary",
                "OPENAI_API_BASE_URL": "https://llmapi.omnisolo.co/v1",
                "OPENAI_MODEL": "gpt-5.6-luna",
                "KIMI_SHARE_DIR": directory,
            }

            configured = bridge.prepare_static_provider(environment)

            self.assertTrue(configured)
            self.assertEqual(
                environment["OPENAI_BASE_URL"],
                "https://llmapi.omnisolo.co/v1",
            )
            config_path = Path(directory) / "config.json"
            config = json.loads(config_path.read_text())
            self.assertEqual(config["default_model"], "gpt-5.6-luna")
            self.assertFalse(config["default_thinking"])
            self.assertEqual(
                config["providers"]["omnisolo"]["type"], "openai_responses"
            )
            self.assertEqual(
                config["models"]["gpt-5.6-luna"]["model"], "gpt-5.6-luna"
            )
            self.assertIn(
                "thinking",
                config["models"]["gpt-5.6-luna"]["capabilities"],
            )
            self.assertNotIn("secret-canary", config_path.read_text())
            self.assertEqual(stat.S_IMODE(config_path.stat().st_mode), 0o600)

    def test_exact_reasoning_uses_wire_override_without_mutating_native_provider(self):
        import asyncio
        import copy
        bridge = load_bridge()
        class Provider:
            def __init__(self):
                self.options = {"reasoning_effort": "high"}
            def with_generation_kwargs(self, **kwargs):
                cloned = copy.copy(self)
                cloned.options = {**self.options, **kwargs}
                return cloned
            async def generate(self, *args, **kwargs):
                return self.options
        bridge.bind_requested_reasoning("max", Provider)
        original = Provider()
        result = asyncio.run(original.generate("system", [], []))
        self.assertEqual(result["extra_body"]["reasoning"]["effort"], "max")
        self.assertEqual(original.options, {"reasoning_effort": "high"})
        with self.assertRaises(ValueError):
            bridge.bind_requested_reasoning("unsupported", Provider)

    def test_missing_static_route_preserves_upstream_oauth_gate(self):
        bridge = load_bridge()
        with tempfile.TemporaryDirectory() as directory:
            environment = {
                "OPENAI_API_KEY": "secret-canary",
                "KIMI_SHARE_DIR": directory,
            }

            self.assertFalse(bridge.prepare_static_provider(environment))
            self.assertFalse((Path(directory) / "config.json").exists())

    def test_default_share_directory_is_private_and_process_isolated(self):
        bridge = load_bridge()
        with tempfile.TemporaryDirectory() as directory:
            environment = {
                "OPENAI_API_KEY": "secret-canary",
                "OPENAI_API_BASE_URL": "https://llmapi.omnisolo.co/v1",
                "OPENAI_MODEL": "gpt-5.6-luna",
                "TMPDIR": directory,
            }

            self.assertTrue(bridge.prepare_static_provider(environment))
            share_dir = Path(environment["KIMI_SHARE_DIR"])
            self.assertEqual(share_dir.parent, Path(directory))
            self.assertTrue(share_dir.name.startswith("omnisolo-kimi-"))
            self.assertEqual(stat.S_IMODE(share_dir.stat().st_mode), 0o700)

    def test_malformed_provider_routes_do_not_bypass_oauth(self):
        bridge = load_bridge()
        for base_url in (
            "file:///tmp/provider",
            "https://user:password@example.test/v1",
            "https://example.test/v1?token=value",
            "https://example.test/v1#fragment",
            "https://example.test/white space",
            "https:///missing-host",
            "https://example.test:invalid/v1",
        ):
            with self.subTest(base_url=base_url), tempfile.TemporaryDirectory() as directory:
                environment = {
                    "OPENAI_API_KEY": "secret-canary",
                    "OPENAI_API_BASE_URL": base_url,
                    "OPENAI_MODEL": "gpt-5.6-luna",
                    "KIMI_SHARE_DIR": directory,
                }

                self.assertFalse(bridge.prepare_static_provider(environment))
                self.assertNotIn("OPENAI_BASE_URL", environment)
                self.assertFalse((Path(directory) / "config.json").exists())

    def test_control_characters_in_model_id_do_not_bypass_oauth(self):
        bridge = load_bridge()
        with tempfile.TemporaryDirectory() as directory:
            environment = {
                "OPENAI_API_KEY": "secret-canary",
                "OPENAI_API_BASE_URL": "https://llmapi.omnisolo.co/v1",
                "OPENAI_MODEL": "gpt-5.6-luna\nmalicious",
                "KIMI_SHARE_DIR": directory,
            }

            self.assertFalse(bridge.prepare_static_provider(environment))
            self.assertFalse((Path(directory) / "config.json").exists())


if __name__ == "__main__":
    unittest.main()
