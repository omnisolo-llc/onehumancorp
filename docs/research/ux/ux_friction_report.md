# UX Friction Report: OHC Web Application

**Author:** Principal UX Strategist & Palette (L8)
**Date:** 2023-10-27
**Target Product Goal:** "AI News Collector"

## Overview
During the execution of a complete customer journey to build an "AI News Collector," I logged into the system, created a small business via the Setup Wizard, and attempted to hire a full team using the Agent Hire Wizard. The core functionality is present, but the UX falls severely short of the Aesthetic Excellence Mandate.

## Identified Friction Points

1. **Non-Premium Feel (Aesthetics Gap):**
   - The UI currently uses standard, default Material styling (Cards, basic ElevatedButtons, generic ListTiles).
   - **Friction:** It does not look like a market-leading OS. The absence of Glassmorphism (`backdrop-filter: blur(20px) saturate(200%)`) makes it feel like an internal prototype rather than a premium platform. The mandate strictly requires Glassmorphism composition for all major panels.
   - **Remediation:** Both `SetupWizardScreen` and `AgentHireWizardScreen` must be refactored to utilize a custom glassmorphic container with `ImageFilter.compose(outer: ImageFilter.blur(sigmaX: 20, sigmaY: 20), inner: ColorFilter.matrix([...]))` or standard styling achieving the mandated CSS equivalent (`backdrop-filter: blur(20px) saturate(200%)`).

2. **Typography Misalignment:**
   - The application does not strictly enforce the `Outfit` or `Inter` typography as required by the aesthetic guidelines.
   - **Remediation:** Ensure that text styles throughout the wizard screens adopt the correct typography family.

3. **Clunky Flow & Semantics:**
   - Standard Stepper controls feel outdated for a modern "Agentic OS."
   - Missing tooltips or confusing state transitions during deployment.

## Autonomous Remediation Plan
1. Rewrite the UI components in `srcs/app/lib/screens/wizard_screen.dart` and `srcs/app/lib/screens/agent_hire_wizard_screen.dart`.
2. Introduce a reusable `GlassContainer` widget to inject the `ClipRRect` and `BackdropFilter` with `ImageFilter.blur(sigmaX: 20, sigmaY: 20)` and a background color of `Colors.white.withOpacity(0.03)` / border `1px solid rgba(255, 255, 255, 0.08)`.
3. Update Typography.
4. Ensure tests still pass.
