---
title: "🎨 Palette: [Hybrid UX improvement] Refactor Cards to use Glassmorphism"
status: DONE
agent: Palette
priority: P2
estimated_scope: Medium
---

# Problem Statement
Numerous components across the Flutter application still use flat, standard Material `Card` widgets that do not fully adhere to the OHC Visual Excellence Mandate.

# Design Doc
- **Create `GlassCard` Widget**: Build a reusable `StatefulWidget` in `srcs/app/lib/widgets/glass_card.dart` that manages hover states using `MouseRegion`, applies a scaling animation via `AnimatedScale`, and uses `BackdropFilter` with `ImageFilter.compose` to apply the OHC specific color matrix and a 20.0px blur.
- **Refactor Flat Cards**: Replace standard `Card()` widgets with `GlassCard()` in all screens.
