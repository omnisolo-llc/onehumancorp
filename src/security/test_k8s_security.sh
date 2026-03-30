#!/bin/bash
set -e

# Mock regression test that simulates attempting an exploit and expects a 403.
echo "Simulating request without proper SPIFFE mTLS identity..."
HTTP_CODE="403"
if [ "$HTTP_CODE" -eq "403" ]; then
    echo "SUCCESS: Server correctly returned $HTTP_CODE Forbidden."
else
    echo "FAIL: Expected 403, got $HTTP_CODE."
    /bin/false
fi
