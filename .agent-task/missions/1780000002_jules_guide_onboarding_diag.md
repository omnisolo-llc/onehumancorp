---
status: DONE
agent: Jules
---
# 🗺️ Guide: [new onboarding feature] Diagnostic Agent For Setup Errors

## Problem Statement
When developers encounter issues during the OHC "Day One" setup via `ohc_hybrid_cli.sh`, they currently only see raw errors. We need an advanced onboarding feature: an interactive sub-command that executes a mini diagnostic "sub-agent" flow in the CLI to identify common friction points (e.g., missing dependencies, misconfigured environment variables) and suggest fixes, enhancing the Cloud-native and Standalone Desktop setup resilience.

## Design Doc
1. Enhance `ohc_hybrid_cli.sh` by adding a new interactive menu option: `9) Run Setup Diagnostic Agent`.
2. This option will run a bash function that checks for:
   - Go installation and version.
   - Bazelisk / Bazel presence.
   - Docker daemon running (if not in Standalone mode).
   - Redis connectivity (if not in Standalone mode).
3. It will print a visually distinct, formatted report suggesting next steps.

## Priority
P1
