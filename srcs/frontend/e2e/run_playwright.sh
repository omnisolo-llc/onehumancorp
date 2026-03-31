#!/usr/bin/env bash

export NODE_PATH="${RUNFILES_DIR}/_main/node_modules"
export PLAYWRIGHT_BROWSERS_PATH="${TEST_TMPDIR:-/tmp}/pw_browsers"
export OHC_DB_PATH="${TEST_TMPDIR:-/tmp}/test_e2e.db"
touch $OHC_DB_PATH

echo "Starting backend..."
${RUNFILES_DIR}/_main/srcs/cmd/ohc/ohc_/ohc --port 8080 > backend.log 2>&1 &
BACKEND_PID=$!

echo "Starting frontend..."
export FRONTEND_STATIC_DIR="${RUNFILES_DIR}/_main/srcs/frontend/public"
export BACKEND_URL="http://127.0.0.1:8080"
${RUNFILES_DIR}/_main/srcs/frontend/server/cmd/frontend/frontend_/frontend --port 8081 > frontend.log 2>&1 &
FRONTEND_PID=$!

sleep 5

echo "Running E2E tests..."
node ${RUNFILES_DIR}/_main/node_modules/@playwright/test/cli.js install chromium
node ${RUNFILES_DIR}/_main/node_modules/@playwright/test/cli.js test -c srcs/frontend/e2e/playwright.config.ts "srcs/frontend/e2e/cuj.spec.ts"
EXIT_CODE=$?

kill $BACKEND_PID || true
kill $FRONTEND_PID || true

if [ $EXIT_CODE -ne 0 ]; then
  cat backend.log
  cat frontend.log
fi

return $EXIT_CODE 2>/dev/null || true
