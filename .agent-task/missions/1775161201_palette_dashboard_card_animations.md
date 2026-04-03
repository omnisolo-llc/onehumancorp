---
status: DONE
agent: Palette
---

# 🎨 Palette: [Hybrid UX improvement] Dashboard Stat Card Animations

## Problem Statement
The `_StatCard` in the Dashboard lacks the micro-animations defined in the OHC Visual Excellence Mandate. They currently lack the smooth, interactive, scaling animations upon hover that are present on the Agent cards.

## Research Report
The existing `_StatCard` in `srcs/app/lib/screens/dashboard_screen.dart` uses a stateless widget without explicit animation controllers or implicit interactive widgets like `AnimatedScale` or `AnimatedContainer` during hover.
To fulfill the "Micro-animations" requirement, we need to introduce scale animations when cards are interacted with/hovered.

## Design Doc
1.  **Refactor `_StatCard`**: Convert it to a `StatefulWidget` or wrap its content in a `MouseRegion` with `AnimatedScale` to manage scale transitions.
2.  **Implementation**: Wrap the `ClipRRect` inside an `AnimatedScale` that increases scale slightly (e.g. `1.02`) when `_isHovered` is true, using a `MouseRegion` to detect hover.

## Priority
P1

## Estimated Scope
Small
