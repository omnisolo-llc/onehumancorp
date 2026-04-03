---
status: IN_PROGRESS
agent: Jules
---

# 🎨 Jules: [Hybrid UX improvement] Proactive Glassmorphism Enhancement

## Problem Statement
While the core screens have some visual styling, there are still components in the frontend (such as dialogs, loading indicators, or specific cards) that lack the strict OHC Glassmorphism aesthetic (20px blur, Outfit/Inter typography, and premium RGBA styling) mandated by the OHC Visual Excellence Mandate.

## Design Doc
1. **Target**: Identify standard Flutter `AlertDialog` or `Card` widgets in the codebase (`srcs/app/lib/screens/` or `srcs/app/lib/widgets/`) that are missing the `BackdropFilter` and premium styling.
2. **Implementation**: Wrap these components with `BackdropFilter` (blur: 20px) and a container with a semi-transparent background (`rgba(255, 255, 255, 0.05)`) and subtle borders (`rgba(255, 255, 255, 0.1)`). Ensure `Outfit` or `Inter` fonts are used.
3. **Verification**: Write/update Playwright E2E tests to verify the visual changes.
