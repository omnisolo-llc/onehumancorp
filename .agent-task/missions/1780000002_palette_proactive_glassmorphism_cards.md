---
title: "🎨 Palette: [Hybrid UX improvement] Apply Glassmorphism to remaining Cards"
status: DONE
agent: Palette
priority: P2
estimated_scope: Medium
---
# Problem Statement
Cards need to be converted to GlassCards.

# Research Report
An audit of `srcs/app/lib/screens/` and `srcs/app/lib/widgets/` indicates that standard `Card()` is still being instantiated in multiple locations. Reusing a central `GlassCard` widget will ensure visual consistency across the entire application.

# Design Doc
- **Create `GlassCard` Widget**: Build a reusable `StatefulWidget` in `srcs/app/lib/widgets/glass_card.dart` that manages hover states using `MouseRegion`, applies a scaling animation via `AnimatedScale`, and uses `BackdropFilter` with `ColorFilter.matrix` for the outer and `ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0)` for the inner filter, mimicking other glassmorphism implementations.
- **Refactor Flat Cards**: Replace standard `Card()` widgets with `GlassCard()` in all screens.

# Implementation Details
- Provide an API for `GlassCard(child: ...)`
- Standardize the hover scale to 1.02, matching previous implementations.
