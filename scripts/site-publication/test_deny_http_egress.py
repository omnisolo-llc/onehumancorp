import importlib.util
from pathlib import Path
import subprocess
import sys
import unittest
from unittest.mock import patch

spec = importlib.util.spec_from_file_location('deny_http', Path(__file__).with_name('deny_http_egress.py'))
guard = importlib.util.module_from_spec(spec)
spec.loader.exec_module(guard)

class DenyHttpTests(unittest.TestCase):
    def test_real_proxy_rejects_without_forwarding_or_recording_private_data(self):
        import http.client
        import urllib.parse
        observed = {}
        def child(command, *, env, check):
            del command, check
            proxy = urllib.parse.urlparse(env['HTTP_PROXY'])
            observed.update(env)
            connection = http.client.HTTPConnection(proxy.hostname, proxy.port, timeout=2)
            connection.request('POST', 'http://edge-cache/purge', body=b'test-only-no-secret')
            response = connection.getresponse()
            self.assertEqual(response.status, 502)
            self.assertEqual(response.read(), b'')
            connection.close()
            return subprocess.CompletedProcess([], 0)
        with patch.object(guard.subprocess, 'run', side_effect=child):
            self.assertEqual(guard.run(['test-child']), 1)
        for name in ['HTTPS_PROXY','ALL_PROXY','http_proxy','https_proxy','all_proxy']:
            self.assertEqual(observed[name], observed['HTTP_PROXY'])
        self.assertEqual(observed['NO_PROXY'], '127.0.0.1,localhost,::1')
        self.assertNotIn('REDIS_URL', observed)

    def test_no_request_preserves_actual_child_status(self):
        with patch.object(guard.subprocess, 'run', return_value=subprocess.CompletedProcess([], 7)):
            self.assertEqual(guard.run(['test-child']), 7)
        with patch.object(guard.subprocess, 'run', return_value=subprocess.CompletedProcess([], 0)):
            self.assertEqual(guard.run(['test-child']), 0)

if __name__ == '__main__':
    unittest.main()
