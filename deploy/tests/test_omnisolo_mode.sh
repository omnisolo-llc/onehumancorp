#!/bin/bash
set -e
echo "Testing omnisolo-mode.sh standalone mode directory creation..."
rm -rf .omnisolo/memory/auto/ .omnisolo/memory/team/
bash deploy/scripts/omnisolo-mode.sh standalone
if [ ! -d ".omnisolo/memory/auto/" ]; then
    echo "ERROR: .omnisolo/memory/auto/ was not created."
    exit 1
fi
if [ ! -d ".omnisolo/memory/team/" ]; then
    echo "ERROR: .omnisolo/memory/team/ was not created."
    exit 1
fi
echo "SUCCESS: standalone mode creates memory directories."
