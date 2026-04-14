---
title: "🎨 Palette: [Hybrid UX improvement] Refactor ListTile components to use Glassmorphism"
status: DONE
agent: Palette
priority: P2
estimated_scope: Medium
---

# Problem Statement
The OHC Visual Excellence Mandate dictates that interfaces should have premium tactile feedback and glassmorphism-styled transitions. Despite earlier refactoring efforts on `Card` components, numerous components across the Flutter application still use flat, standard Material `ListTile` widgets that do not fully adhere to the OHC Visual Excellence Mandate.

# Research Report
An audit of `srcs/app/lib/screens/` indicates that standard `ListTile` widgets are still being instantiated in multiple locations directly without the required aesthetic styling. Wrapping them within a central `GlassCard` widget will ensure visual consistency across the entire application.

# Design Doc
- **Create `GlassCard` Widget**: We will reuse the `StatefulWidget` in `srcs/app/lib/widgets/glass_card.dart` that manages hover states using `MouseRegion`, applies a scaling animation via `AnimatedScale`, and uses `BackdropFilter` with `ImageFilter.compose` to apply the OHC specific color matrix and a 20.0px blur.
- **Refactor Flat ListTiles**: Replace standard `ListTile()` widgets with `GlassCard(margin: const EdgeInsets.only(bottom: 8), padding: EdgeInsets.zero, child: ListTile(...))` in all screens.

# Implementation Details
- Refactored multiple screens including:
  - `srcs/app/lib/screens/landing_page_experiments_screen.dart`
  - `srcs/app/lib/screens/user_management_screen.dart`
  - `srcs/app/lib/screens/agent_hire_wizard_screen.dart`
  - `srcs/app/lib/screens/cost_dashboard_screen.dart`
  - `srcs/app/lib/screens/channels_screen.dart`
  - `srcs/app/lib/screens/settings_screen.dart`
- Removed duplicate imports and adjusted spacing to conform with standard dart formats.
- All tests passed.
