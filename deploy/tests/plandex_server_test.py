"""Native Plandex service database-secret loading and deployment isolation."""
import importlib.util
from pathlib import Path
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
spec = importlib.util.spec_from_file_location('entrypoint', ROOT / 'deploy/harness/plandex/server-entrypoint.py')
entrypoint = importlib.util.module_from_spec(spec)
spec.loader.exec_module(entrypoint)


class DatabaseConfigurationTests(unittest.TestCase):
    def test_database_url_file_is_loaded_and_pointer_removed(self):
        with tempfile.TemporaryDirectory() as directory:
            secret = Path(directory) / 'url'
            secret.write_text('postgres://service:fixture@db/plandex\n')
            env = {'DATABASE_URL_FILE': str(secret)}
            entrypoint.configure_database(env)
            self.assertEqual(env, {'DATABASE_URL': 'postgres://service:fixture@db/plandex'})

    def test_ambiguous_source_is_rejected_without_printing_secret(self):
        env = {'DATABASE_URL': 'private-credential', 'DATABASE_URL_FILE': '/missing'}
        with self.assertRaises(ValueError) as caught:
            entrypoint.configure_database(env)
        self.assertNotIn('private-credential', str(caught.exception))

    def test_invalid_files_are_rejected(self):
        with tempfile.TemporaryDirectory() as directory:
            secret = Path(directory) / 'password'
            for content in (b'', b'\xff', b'x' * 65537, b'password\x00', b'password\nembedded'):
                secret.write_bytes(content)
                with self.subTest(content_length=len(content)), self.assertRaises(ValueError):
                    entrypoint.configure_database({'DB_PASSWORD_FILE': str(secret)})

    def test_incomplete_configuration_is_rejected(self):
        with self.assertRaises(ValueError):
            entrypoint.configure_database({'DB_HOST': 'database'})

    def test_split_database_configuration_accepts_secret_file(self):
        with tempfile.TemporaryDirectory() as directory:
            secret = Path(directory) / 'password'
            secret.write_text('fixture-password\n')
            env = dict(DB_HOST='db', DB_PORT='5432', DB_USER='plandex', DB_NAME='plandex', DB_PASSWORD_FILE=str(secret))
            entrypoint.configure_database(env)
            self.assertEqual(env['DB_PASSWORD'], 'fixture-password')
            self.assertNotIn('DB_PASSWORD_FILE', env)


if __name__ == '__main__':
    unittest.main()
