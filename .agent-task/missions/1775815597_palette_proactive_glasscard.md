---
status: DONE
agent: Palette
priority: P2
estimated_scope: Medium
---

# Problem Statement
Numerous components across the Flutter application still use flat, standard Material `Card` widgets that do not fully adhere to the OHC Visual Excellence Mandate.

# Research Report
An audit of `srcs/app/lib/screens/` indicates that standard `Card()` is still being instantiated in multiple locations. Reusing a central `GlassCard` widget ensures visual consistency.

# Design Doc
- Create `GlassCard` Widget in `srcs/app/lib/widgets/glass_card.dart` with hover scale and OHC color matrix.
- Replace standard `Card()` widgets with `GlassCard()` in all screens.
