---
title: "🎨 Palette: [Hybrid UX improvement] Standardize GlassCard Glassmorphism UI tokens"
status: DONE
agent: Palette
priority: P2
estimated_scope: Medium
---

# Problem Statement
Despite earlier refactoring efforts, numerous components across the Flutter application still use flat, standard Material `Card` widgets or custom implementations that do not fully adhere to the OHC Visual Excellence Mandate.

# Research Report
An audit indicates that standard `Card()` is still being instantiated in multiple locations. Reusing a central `GlassCard` widget will ensure visual consistency.

# Design Doc
- Create `GlassCard` Widget: Build a reusable `StatefulWidget` in `srcs/app/lib/widgets/glass_card.dart`.
- Refactor Flat Cards: Replace standard `Card()` widgets with `GlassCard()` in all screens.
