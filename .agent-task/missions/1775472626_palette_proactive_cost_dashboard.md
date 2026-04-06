---
status: DONE
agent: Palette
---

# 🎨 Palette: [Hybrid UX improvement] CostDashboardScreen Glassmorphism

## Problem Statement
The `CostDashboardScreen` (`srcs/app/lib/screens/cost_dashboard_screen.dart`) violates the OHC Visual Excellence Mandate. It uses standard Flutter `Card` widgets and lacks micro-animations, appearing flat and inconsistent with the premium aesthetic required for the Hybrid OS.

## Research Report
The screen uses `_SummaryCard`, a static stateless widget, and raw `Card` widgets for "Usage per Agent" and "Organization View". To meet the "Micro-animations" and "Glassmorphism" requirements, we need to introduce scale/hover animations and backdrop filters.

## Design Doc
1.  **Refactor `_SummaryCard`**: Convert it to a `StatefulWidget` to manage hover animation controllers. Add `BackdropFilter` and `AnimatedScale`.
2.  **Refactor Main Cards**: Replace the raw `Card` widgets wrapping the charts/lists with a glassmorphic container style matching the theme.
3.  **UI Tokens**: Ensure the glassmorphism backdrop filter is applied consistently (`sigmaX: 20.0, sigmaY: 20.0`) and use `AnimatedContainer` for background color transitions.

## Priority
P1

## Estimated Scope
Small
