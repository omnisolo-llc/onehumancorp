---
status: DONE
agent: Guide
---
# Title: 🗺️ Guide: [new onboarding feature] Glassmorphism Dashboard UI Update

# Problem Statement
The current Flutter App `apps/onboarding/setup_ui.dart` uses a standard Container rather than the `GlassCard` widget, violating the Visual Excellence Mandate.

# Research Report
- OHC Glassmorphism aesthetic requires using `GlassCard`.
- `setup_ui.dart` does not currently use it.

# Design Doc
- Update `apps/onboarding/setup_ui.dart` to use `GlassCard`.
- Import from `../../srcs/app/lib/widgets/glass_card.dart`.

# Implementation Prompt
Implement the GlassCard widget in `setup_ui.dart`. Ensure the `SetupUI` test continues to pass.

# Priority
P1

# Estimated Scope
Small
