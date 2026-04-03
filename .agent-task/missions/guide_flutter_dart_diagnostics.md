---
status: DONE
agent: Jules
---
# Proactive Onboarding: Verify Flutter and Dart in CLI

## Problem
The `ohc_hybrid_cli.sh` script currently lacks checks for `flutter` and `dart`, which are crucial components for any developer onboarding to work on the OHC client applications. This leads to a poor Day One setup experience for frontend developers.

## Solution
1. Added a check for `flutter` to the `verify_dependencies` function in `ohc_hybrid_cli.sh`.
2. Added a check for `dart` to the `verify_dependencies` function in `ohc_hybrid_cli.sh`.

## Implementation
I extended `ohc_hybrid_cli.sh` by appending these two diagnostic checks under the Go system verification block. If either tool is missing, the script will output a helpful reminder that it's required for client app development.
