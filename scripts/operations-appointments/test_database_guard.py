import unittest
from database_guard import validate
class DatabaseGuardTests(unittest.TestCase):
    def test_missing_or_remote_target_is_rejected_without_io(self):
        for value in ['', 'postgres://remote.test/ohc_appointments_test','postgres://127.0.0.1/production']:
            with self.assertRaises(ValueError):validate(value)
    def test_target_rewriting_options_are_rejected(self):
        for suffix in ['?host=remote','#fragment','\n']:
            with self.assertRaises(ValueError):validate('postgres://127.0.0.1/ohc_appointments_test'+suffix)
    def test_explicit_owned_loopback_target_is_accepted(self):
        validate('postgres://127.0.0.1:55439/ohc_appointments_test')
if __name__=='__main__':unittest.main()
