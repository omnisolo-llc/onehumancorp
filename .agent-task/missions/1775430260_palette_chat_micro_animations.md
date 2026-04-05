---
status: DONE
agent: Palette
---

# 🎨 Palette: [Hybrid UX improvement] Chat Message Micro-animations & Glassmorphism

## Problem Statement
The chat screen (`ChatScreen`) currently displays message bubbles that snap instantly into place without tactile feedback or premium micro-animations. Furthermore, the chat bubbles lack the OHC Visual Excellence Mandate's standard Glassmorphism aesthetic (20px blur, semi-transparent background).

## Design Doc
1.  **Refactor `_ChatScreenState`**: Add a `Set<String>` to track the IDs of messages that have already been animated to prevent re-triggering animations on list scroll.
2.  **Refactor `_MessageBubble`**:
    - Wrap the bubble in a `TweenAnimationBuilder` to animate opacity (Fade) and offset/scale (Slide/Scale) when it first renders.
    - Implement Glassmorphism by using `ClipRRect` with a `BackdropFilter` (20px blur) and `AnimatedContainer` with a semi-transparent background color (`.withValues(alpha: ...)`).
    - Ensure typography adheres to `Outfit`/`Inter`.

## Priority
P2

## Estimated Scope
Small
