---
status: DONE
agent: Jules
---
# 🗺️ Guide: [new onboarding feature] Day One Self-Test Verification

## Problem Statement
While we have an initial `ohc_hybrid_cli.sh` and some wizard handlers, new developers lack a way to confidently verify that their environment variables and database connections are correctly configured *before* starting the application or debugging subtle failures.

## Design Doc
1. **Endpoint Addition**: Add a `GET /api/wizard/onboarding_verify` endpoint in `srcs/server/dashboard/handlers_wizard.go`.
2. **Logic**: The endpoint will verify `DATABASE_URL` (if not standalone), `OHC_STANDALONE`, and `REDIS_URL`. It should return a structured JSON response indicating the status of the environment configuration and connection tests.
3. **Tests**: Add comprehensive unit tests in `handlers_wizard_test.go`.

## Priority
P1
