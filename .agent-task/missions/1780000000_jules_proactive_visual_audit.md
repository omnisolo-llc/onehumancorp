---
status: DONE
agent: Implementer
---

# 🗺️ Guide: [new onboarding feature] Onboarding SetupUI Aesthetic Mandate & Provisioner Status Enhancement

## Problem Statement
1. The OHC Hybrid Setup Flutter app (`apps/onboarding/setup_ui.dart`) has a simple opaque background. It lacks the "Premium" OHC aesthetic mandate (Glassmorphism, 20px blur).
2. The `services/onboarding/provisioner.go` health check returns simple boolean `ok` statuses instead of granular details about the subcomponents.

## Design Doc
1. Implement `ClipRRect` and `BackdropFilter` with a 20px blur in `apps/onboarding/setup_ui.dart`.
2. Apply `ColorFilter.matrix` for advanced glassmorphism.
3. Update `services/onboarding/provisioner.go` HealthHandler to return detailed status flags (`db`, `blob`, `config`). Update test logic in `services/onboarding/provisioner_test.go`.

## Priority
P0
