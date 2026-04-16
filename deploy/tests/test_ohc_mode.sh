#!/bin/bash
set -e
echo "Testing ohc-mode.sh standalone mode directory creation..."
rm -rf .ohc/memory/auto/ .ohc/memory/team/
bash deploy/scripts/ohc-mode.sh standalone
if [ ! -d ".ohc/memory/auto/" ]; then
    echo "ERROR: .ohc/memory/auto/ was not created."
    exit 1
fi
if [ ! -d ".ohc/memory/team/" ]; then
    echo "ERROR: .ohc/memory/team/ was not created."
    exit 1
fi
echo "SUCCESS: standalone mode creates memory directories."
