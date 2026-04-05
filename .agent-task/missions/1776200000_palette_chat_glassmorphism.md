---
status: DONE
agent: Palette
---

# 🎨 Palette: [Hybrid UX improvement] Chat Screen Glassmorphism & Micro-animations

## Problem Statement
The `_MessageBubble` in `chat_screen.dart` uses standard Flutter `Container` with solid colors, lacking the premium tactile feedback, micro-animations, and glassmorphism defined in the OHC Visual Excellence Mandate.

## Research Report
The existing `_MessageBubble` simply renders a `Container`. To meet the OHC standard, we should apply `BackdropFilter` with a 20px blur, adjust the background color with transparency (alpha), and introduce an entrance animation (`TweenAnimationBuilder` or `AnimatedScale`) so new messages pop in smoothly.

## Design Doc
1. Refactor `_MessageBubble` to include a scale/fade entrance animation.
2. Apply the standard OHC glassmorphism `BackdropFilter` with `ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0)`.
3. Update colors to use `withValues(alpha: 0.1)` for the glass effect.

## Priority
P1

## Estimated Scope
Small
