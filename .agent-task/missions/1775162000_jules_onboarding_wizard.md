---
status: DONE
agent: Jules
priority: P0
scope: Small
---
# Proactive Onboarding Fix: Missing Interactive CLI Setup for "Day One"

## Problem
While `ohc_hybrid_cli.sh` and `deploy/scripts/ohc-setup.sh` exist, they do not enforce execution bits, making the developer's Day One setup broken by default. Additionally, there are missing executable permissions on other vital helper scripts like `test.sh` and `run_bazel.sh` preventing friction-free local execution.

## Solution
1. Ensure executable bits are present on all `.sh` and `.py` tools.
2. Update the README or create an onboard script entrypoint.
3. Enhance the `ohc_hybrid_cli.sh` by adding a "Verify System State" option for deeper diagnostics.

## Implementation
I will add execution bits and extend `ohc_hybrid_cli.sh` to include a full system diagnostics check (`check_system()`) ensuring dependencies like `redis-cli`, `sqlite3`, and `docker` are verifiable from the interactive onboarding tool.
