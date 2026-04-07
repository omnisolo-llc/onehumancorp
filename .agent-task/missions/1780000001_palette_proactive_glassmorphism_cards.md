---
status: DONE
agent: Palette
title: "🎨 Palette: [Hybrid UX improvement] Apply Glassmorphism to remaining Cards"
priority: P2
estimated_scope: Medium
---
# Problem Statement
Despite earlier refactoring efforts, numerous components across the Flutter application still use flat, standard Material `Card` widgets or custom implementations that do not fully adhere to the OHC Visual Excellence Mandate.
# Research Report
An audit of `srcs/app/lib/screens/` and `srcs/app/lib/widgets/` indicates that standard `Card()` is still being instantiated in multiple locations. Reusing a central `GlassCard` widget will ensure visual consistency across the entire application.
# Design Doc
- **Create `GlassCard` Widget**: Build a reusable `StatefulWidget` in `srcs/app/lib/widgets/glass_card.dart` that manages hover states using `MouseRegion`, applies a scaling animation via `AnimatedScale`, and uses `BackdropFilter` with `ImageFilter.compose` to apply the OHC specific color matrix and a 20.0px blur.
- **Refactor Flat Cards**: Replace standard `Card()` widgets with `GlassCard()` in all screens.
# Implementation Details
- Provide an API for `GlassCard(child: ...)`
- Standardize the hover scale to 1.02, matching previous implementations.
