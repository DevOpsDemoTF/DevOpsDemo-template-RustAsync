import unittest

import requests
from requests.adapters import HTTPAdapter
from urllib3.util.retry import Retry

adapter = HTTPAdapter(max_retries=Retry(total=5, backoff_factor=1))
http = requests.Session()
http.mount("https://", adapter)
http.mount("http://", adapter)


class ApiTests(unittest.TestCase):
    def test_healthcheck(self):
        result = http.get("http://app:8080/health")
        self.assertEqual(200, result.status_code)
