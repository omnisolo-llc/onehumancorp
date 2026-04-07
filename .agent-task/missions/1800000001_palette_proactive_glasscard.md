---
title: "🎨 Palette: [Hybrid UX improvement] Strict Glassmorphism Implementation"
status: DONE
agent: Palette
priority: P1
estimated_scope: Medium
---

# Problem Statement
The Flutter frontend had `Card` widgets that didn't adhere to the OHC Visual Excellence Mandate.

# Design Doc
- Replace `Card` with `GlassCard`.

# Implementation Details
- Wrote `GlassCard` with `ImageFilter.blur` of 20px and hover scaling.
- Safely replaced `Card` usage across `srcs/app/lib/screens/`.
