---
status: DONE
agent: Palette
priority: P1
---

# 🎨 Palette: [Hybrid UX improvement] Refactor Dashboard Role Cards with Animated Entrance

## Problem Statement
The role scale cards (`_RoleScaleCard`) in the Dashboard screen lack the smooth animated entrance present on other dashboard widgets (e.g., `_StatCard`). While they have glassmorphism and hover effects, they abruptly appear when the dashboard loads, violating the "Premium Tactile Feedback" core value.

## Research Report
The existing `_RoleScaleCardState` inside `srcs/app/lib/screens/dashboard_screen.dart` is a `StatefulWidget` managing the scaling count and hover state, but does not use an `AnimationController` for an entrance animation (`SlideTransition` and `FadeTransition`) like `_StatCard` does.

## Design Doc
1. **Refactor `_RoleScaleCardState`**:
   - Add `SingleTickerProviderStateMixin`.
   - Initialize an `AnimationController` with `SlideTransition` and `FadeTransition` inside `initState`.
   - Start the animation with a slight delay (`Future.delayed`) when the widget first mounts to match the staggered animation feel.
   - Wrap the main tree of `_RoleScaleCard` with `SlideTransition` and `FadeTransition`.

## Implementation Prompt
Modify `_RoleScaleCardState` in `srcs/app/lib/screens/dashboard_screen.dart` to add the entrance animations, run Flutter tests, and submit a PR.

## Estimated Scope
Small
