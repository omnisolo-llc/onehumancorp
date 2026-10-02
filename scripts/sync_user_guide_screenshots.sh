#!/bin/bash
set -euo pipefail

# Generate user-guide review captures as isolated Playwright test artifacts.
# However, generating them against the full backend is flaky in some standalone environments.
# We will start the Next.js UI in dev mode and run a specialized raw Playwright script
# to capture the UI states without overwriting tracked documentation.

export BASE_URL="http://localhost:3000"

# Start next dev
cd src/ui/next
npm run dev -- -p 3000 > ../../../next_dev.log 2>&1 &
NEXT_PID=$!
cd ../../..

function cleanup() {
    echo "Cleaning up Next.js..."
    kill $NEXT_PID || true
}
trap cleanup EXIT

echo "Waiting for Next.js to start..."
sleep 15

echo "Running playwright to generate screenshots for User Guide..."
npx playwright test src/e2e/docs_visual_audit.spec.ts

echo "Visual audit captures are in Playwright per-test output directories under test-results/."
