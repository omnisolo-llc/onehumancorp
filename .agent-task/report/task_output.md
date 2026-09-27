# Onboarding / First-Day Experience Connection Lifecycle - No-Work Finding

## Customer / Workflow
New owner/operator onboarding flow.

## Stable Target / Issue
Onboarding wizard UI and setup process across `src/ui/next/src/app/onboarding` and `src/ui/next/src/app/onboarding/zero-click`.

## Evidence
- The current implementation of the onboarding wizard lives in the Next.js UI (`src/ui/next/src/app/onboarding/page.tsx`, `src/ui/next/src/app/onboarding/zero-click/page.tsx`).
- `docs/business/market_research/ux_analysis_onboarding.md` explicitly names these as "legacy Next.js onboarding wizards."
- `make lint` and `make test-backend` timed out during the build process, preventing validation of any changes to the current UI.
- `docs/research/native_migration_and_remediation.md` notes the codebase is migrating to native Rust/Cargo, Tauri, and Node.js.

## Expected Owner Result
A functional onboarding flow that establishes business context, connections, authority, and cost limits.

## Justification for Blocked Outcome
The current Next.js onboarding routes are considered legacy. Attempting to optimize or redesign the Next.js UI would be throwaway work. Furthermore, the local build environment is timing out on standard acceptance gates (`make lint`, `make test-backend`). Without a stable target architecture and a passing local build environment, no meaningful friction reduction can be durably implemented.
