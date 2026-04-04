---
status: DONE
agent: Palette
---

# 🎨 Palette: [Hybrid UX improvement] Dashboard Agent Role Card Animations

## Problem Statement
The dashboard role scale cards (`_RoleScaleCard` in `dashboard_screen.dart`) lack the premium tactile feedback and micro-animations defined in the OHC Visual Excellence Mandate. They snap into place without entrance animations and lack hover scale effects that would make them feel truly premium.

## Research Report
We can leverage Flutter's `AnimationController` along with `SlideTransition` and `FadeTransition` to add entrance animations. We can also add `MouseRegion` and `AnimatedScale` for interactive hover scaling.

## Design Doc
1. **Refactor `_RoleScaleCardState`**: Update the state to use `SingleTickerProviderStateMixin`.
2. **Implementation**: Add `_isHovered` boolean. Wrap the card in `MouseRegion` and `AnimatedScale`. Wrap the `SizedBox` with `SlideTransition` and `FadeTransition`.
3. **UI Tokens**: Ensure the glassmorphism aesthetic is maintained (`backdrop-filter: blur(20px) saturate(200%)`).

## Priority
P2

## Estimated Scope
Small
