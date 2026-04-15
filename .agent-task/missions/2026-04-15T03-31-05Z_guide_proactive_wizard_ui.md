---
status: DONE
title: "Enhance Interactive Setup Wizard UI with Glassmorphism"
---

# Problem Statement
The current interactive setup wizard UI in `srcs/server/services/onboarding/wizard.go` uses basic string building, but it needs to be updated to use the full OHC Premium Aesthetic (Glassmorphism) tokens, including transparent backgrounds, 20px blur, and Outfit/Inter typography.

# Mission
Modify `GenerateWizardUI` in `srcs/server/services/onboarding/wizard.go` to inject proper CSS styles and layout structures.
