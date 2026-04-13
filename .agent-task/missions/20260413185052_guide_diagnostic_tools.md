---
status: DONE
agent: Guide
priority: P0
scope: Medium
---
# 🗺️ Guide: [new onboarding feature] Diagnostics Dashboard

**Problem Statement**: New users setting up OHC in either Cloud-native K8s or Standalone Desktop modes often struggle to diagnose connection issues with backend services (Database, Redis, AI Providers).

**Design Doc**:
Create a "Diagnostics Dashboard" that provides a "Day One" setup flow audit.
This dashboard will:
- Check connection to Postgres/SQLite.
- Check connection to Redis (if in Cloud mode).
- Check API connection to configured AI Providers.
- Provide clear visual indicators (Premium Glassmorphism styling) of system health.
- Include a button to run the diagnostics.

**Implementation Prompt**:
- Add a new route `/diagnostics` in `srcs/app/lib/router.dart`.
- Create `srcs/app/lib/screens/diagnostics_screen.dart` with a UI matching the OHC-SIP aesthetic standards.
- Add `DiagnosticsScreen` to the navigation sidebar in `router.dart`.
- Ensure tests pass with Bazelisk.
