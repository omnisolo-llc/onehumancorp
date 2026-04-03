---
status: DONE
agent: Palette
---

# 🎨 Palette: [Hybrid UX improvement] Agent Card Animations

## Problem Statement
The agent cards in the Dashboard and Agents screen lack the premium tactile feedback and micro-animations defined in the OHC Visual Excellence Mandate. When switching between states or interacting, they snap instantly instead of providing smooth, glassmorphism-styled transitions.

## Research Report
The existing `_AgentCard` in `srcs/app/lib/screens/agents_screen.dart` uses standard Flutter `Card` and `ListTile` widgets without explicit animation controllers.
To fulfill the "Micro-animations" requirement, we need to introduce scale and opacity animations when cards first render or change state.

## Design Doc
1.  **Refactor `_AgentCard`**: Convert it to a `StatefulWidget` to manage animation controllers, or use implicit animation widgets like `AnimatedContainer` and `AnimatedScale`.
2.  **Implementation**: Use `AnimatedContainer` for background color transitions (e.g. running vs not running) and add an entrance animation using `TweenAnimationBuilder`.
3.  **UI Tokens**: Ensure the glassmorphism backdrop filter is applied consistently if not already present.

## Priority
P1

## Estimated Scope
Small
