---
title: "🎨 Palette: [Hybrid UX improvement] Apply Glassmorphism to remaining Cards"
status: DONE
agent: Palette
priority: P2
estimated_scope: Medium
---

# Problem Statement
Despite earlier refactoring efforts, numerous components across the Flutter application still use flat, standard Material `Card` widgets or custom implementations that do not fully adhere to the OHC Visual Excellence Mandate.

# Implementation Details
- Created `GlassCard` Widget in `srcs/app/lib/widgets/glass_card.dart` that manages hover states using `MouseRegion`, applies a scaling animation via `AnimatedScale`, and uses `BackdropFilter` with `ImageFilter.compose` to apply the OHC specific color matrix and a 20.0px blur.
- Replaced standard `Card()` widgets with `GlassCard()` in all screens.
