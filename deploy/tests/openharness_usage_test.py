"""Regression for usage arriving after the native provider finish event."""
import asyncio
import importlib.util
import sys
import unittest
from dataclasses import dataclass
from pathlib import Path
from types import SimpleNamespace

path = Path(__file__).resolve().parents[2] / "src/server/harness/sidecars/openharness_bridge.py"
spec = importlib.util.spec_from_file_location("openharness_usage_bridge", path)
bridge = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = bridge
spec.loader.exec_module(bridge)

@dataclass(frozen=True)
class Event:
    type: str
    usage: dict | None = None

class UsageTest(unittest.IsolatedAsyncioTestCase):
    async def test_trailing_usage_is_attached_to_delayed_native_terminal(self):
        async def create(**kwargs):
            self.assertTrue(kwargs["stream_options"]["include_usage"])
            async def chunks():
                yield SimpleNamespace(finish=False, usage=None)
                yield SimpleNamespace(finish=True, usage=None)
                yield SimpleNamespace(finish=False, usage=SimpleNamespace(prompt_tokens=17, completion_tokens=9))
            return chunks()
        class NativeProvider:
            async def chat_completion_stream(self):
                source = await self._client.chat.completions.create(stream_options={"include_usage": True})
                async for chunk in source:
                    if chunk.finish:
                        yield Event("message_end")
                    elif chunk.usage is None:
                        yield Event("text_delta")
        provider = bridge.usage_complete_provider(NativeProvider)()
        client = SimpleNamespace(chat=SimpleNamespace(completions=SimpleNamespace(create=create)))
        provider._client = client
        events = [e async for e in provider.chat_completion_stream()]
        self.assertEqual([e.type for e in events], ["text_delta", "message_end"])
        self.assertEqual(events[-1].usage, {"input_tokens":17, "output_tokens":9})
        self.assertIs(provider._client, client)
        self.assertIs(provider._client.chat.completions.create, create)

    async def test_absent_usage_is_not_invented(self):
        async def create():
            async def chunks():
                yield SimpleNamespace(usage=None)
            return chunks()
        class NativeProvider:
            async def chat_completion_stream(self):
                async for _ in await self._client.chat.completions.create():
                    yield Event("message_end")
        provider = bridge.usage_complete_provider(NativeProvider)()
        provider._client = SimpleNamespace(chat=SimpleNamespace(completions=SimpleNamespace(create=create)))
        events = [e async for e in provider.chat_completion_stream()]
        self.assertIsNone(events[-1].usage)

if __name__ == "__main__":
    unittest.main()
