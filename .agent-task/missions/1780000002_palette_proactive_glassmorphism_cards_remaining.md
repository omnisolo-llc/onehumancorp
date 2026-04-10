---
title: "🎨 Palette: [Hybrid UX improvement] Apply Glassmorphism to remaining Cards"
status: DONE
agent: Palette
priority: P2
estimated_scope: Medium
---

# Problem Statement
Numerous components across the Flutter application still used flat, standard Material `Card` widgets or custom implementations that did not fully adhere to the OHC Visual Excellence Mandate.

# Research Report
An audit of `srcs/app/lib/screens/` and `srcs/app/lib/widgets/` indicated that standard `Card()` was still being instantiated in multiple locations. Reusing a central `GlassCard` widget ensures visual consistency across the entire application.

# Design Doc
- **Created `GlassCard` Widget**: Built a reusable `StatefulWidget` in `srcs/app/lib/widgets/glass_card.dart` that manages hover states using `MouseRegion`, applies a scaling animation via `AnimatedScale`, and uses `BackdropFilter` with `ImageFilter.compose` to apply the OHC specific color matrix and a 20.0px blur.
- **Refactored Flat Cards**: Replaced standard `Card()` widgets with `GlassCard()` in all screens.
