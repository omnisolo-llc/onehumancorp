import unittest
import os
from validate_architecture import validate_design_docs

class TestValidateArchitecture(unittest.TestCase):
    def test_validation(self):
        success, msg = validate_design_docs()
        self.assertTrue(success, msg)

if __name__ == '__main__':
    unittest.main()
