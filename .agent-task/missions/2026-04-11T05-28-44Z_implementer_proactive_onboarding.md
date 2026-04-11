---
status: DONE
agent: Implementer
---

# Title: Implement Proactive Onboarding Improvements

## Problem Statement
The developer onboarding flow has high friction because some executable bits are missing and certain dependencies aren't checked during the Day One wizard execution. The standalone database also lacks its required initial directory setup.

## Implementation Prompt
1. Add execution bits (chmod +x) to all scripts in `deploy/scripts/`, `test.sh`, `run_bazel.sh`, and any other `.sh` files at the root.
2. Extend `ohc_hybrid_cli.sh` to explicitly invoke dependency verification correctly and handle the standalone DB correctly (which includes making sure `/home/jules/.ohc-local-data` is created in `ohc-setup.sh` so standalone checks don't fail).
3. Ensure that when users run setup, the directory structure for standalone is fully pre-created so tests don't fail missing the database directory.
