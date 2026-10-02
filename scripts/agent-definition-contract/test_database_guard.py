import os
from pathlib import Path
import subprocess
import unittest
from database_guard import validate

class DisposableTarget(unittest.TestCase):
    def test_owned_loopback_address(self):
        for value in ['postgres://127.0.0.1:55439/ohc_definition_test','postgresql://[::1]/ohc_definition_test']:
            validate(value)
    def test_unsafe_target_is_rejected_without_connection(self):
        for value in ['postgres://db.example.test/ohc_definition_test','postgres://127.0.0.1/production','postgres://127.0.0.1/ohc__test','postgres://127.0.0.1/ohc_definition_test?host=remote','postgres:///ohc_definition_test','postgres://127.0.0.1/ohc_definition_test#x','postgres://127.0.0.1/ohc_%2f_test']:
            with self.subTest(value=value),self.assertRaises(ValueError): validate(value)
    def test_wrapper_refuses_before_cargo(self):
        runner=Path(__file__).resolve().parent/'run.sh'
        result=subprocess.run(['bash',str(runner)],env={**os.environ,'PATH':'/usr/bin:/bin','OHC_AGENT_DEFINITION_TEST_DATABASE_URL':'postgres://127.0.0.1/production'},capture_output=True,text=True,timeout=5)
        self.assertEqual(result.returncode,1)
        self.assertIn('explicit loopback PostgreSQL',result.stderr)
        self.assertNotIn('cargo',result.stderr.lower())
        self.assertEqual(result.stdout,'')
if __name__=='__main__':unittest.main()
