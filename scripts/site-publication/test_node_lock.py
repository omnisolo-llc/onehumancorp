from pathlib import Path
import json
import tempfile
import unittest
from verify_node_lock import verify


class NodeWitnessLock(unittest.TestCase):
    def test_matching_manifest_and_integrity_are_required(self):
        here = Path(__file__).resolve().parent
        with tempfile.TemporaryDirectory() as temp:
            client = Path(temp)
            for name in ['package.json', 'package-lock.json']:
                (client/name).write_bytes((here/name).read_bytes())
            verify(here, client)
            for field, value in [('version', '5.0.0'), ('integrity', 'sha512-different')]:
                lock = json.loads((here/'package-lock.json').read_text())
                lock['packages']['node_modules/canonicalize'][field] = value
                (client/'package-lock.json').write_text(json.dumps(lock))
                with self.assertRaises(AssertionError): verify(here, client)

    def test_missing_client_pin_fails_closed(self):
        here = Path(__file__).resolve().parent
        with tempfile.TemporaryDirectory() as temp:
            client = Path(temp)
            (client/'package.json').write_text('{"dependencies":{}}')
            (client/'package-lock.json').write_bytes((here/'package-lock.json').read_bytes())
            with self.assertRaises(KeyError): verify(here, client)


if __name__ == '__main__': unittest.main()
