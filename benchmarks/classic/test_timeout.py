#!/usr/bin/env python3
"""Exercise cancellation across two real subprocess stages on an EC2 worker."""
import os
import shutil
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch
from scale import benchmark

class TimeoutTest(unittest.TestCase):
    @unittest.skipUnless(Path('/usr/bin/time').exists() and shutil.which('sleep'), 'GNU time and sleep required')
    def test_total_budget_carries_between_stages(self):
        with tempfile.TemporaryDirectory(prefix='ooff-timeout-test-') as tmp:
            bwa = Path(tmp)/'bwa'
            bwa.write_text('#!/bin/sh\nsleep 0.5\n')
            bwa.chmod(0o755)
            with patch.dict(os.environ, PATH=tmp+os.pathsep+os.environ['PATH']):
                result = benchmark('bwa', 1, 1, 10, 'timeout_test', run_timeout=.9)
            self.assertFalse(result['complete'])
            self.assertEqual(len(result['stages']), 2)
            import json
            first, second = [json.loads(Path(p).read_text()) for p in result['stages']]
            self.assertFalse(first['timeout'])
            self.assertTrue(second['timeout'])
            self.assertEqual(first['returncode'], 0)
            self.assertEqual(result['run_timeout_seconds'], .9)

if __name__ == '__main__': unittest.main()
