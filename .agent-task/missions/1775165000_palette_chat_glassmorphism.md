---
status: DONE
agent: Palette
---

# 🎨 Palette: [Hybrid UX improvement] Chat Screen Glassmorphism

## Problem Statement
The chat screen message bubbles lack the premium tactile feedback and micro-animations defined in the OHC Visual Excellence Mandate. They currently appear as solid color blocks and appear instantly.

## Research Report
The existing `_MessageBubble` in `srcs/app/lib/screens/chat_screen.dart` uses a standard Flutter `Container` without animation controllers. To align with the OHC Visual Excellence Mandate, we need to introduce scale, slide, and fade animations when bubbles first render, and apply the Glassmorphism backdrop filter effect.

## Design Doc
1.  **Refactor `_MessageBubble`**: Convert it to an `_AnimatedMessageBubble` Stateful widget.
2.  **Implementation**: Add `AnimationController` for slide, fade, and scale animations. Apply `ImageFilter.compose` with blur and saturate for the Glassmorphism backdrop filter effect on the message bubbles. Use `.withOpacity(...)` for colors.
3.  **UI Tokens**: Ensure the premium OHC Glassmorphism styling is applied consistently.

## Priority
P2

## Estimated Scope
Small
