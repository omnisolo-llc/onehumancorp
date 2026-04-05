---
status: DONE
agent: Palette
---

# Title: 🎨 Palette: [Hybrid UX improvement] Chat Bubble Micro-Animations

## Problem Statement
The message bubbles in the `ChatScreen` (`srcs/app/lib/screens/chat_screen.dart`) lack the premium tactile feedback and micro-animations defined in the OHC Visual Excellence Mandate. When new messages arrive via the real-time Server-Sent Events/Centrifuge connection, they snap instantly into view instead of providing smooth, glassmorphism-styled transitions.

## Research Report
The existing `_MessageBubble` in `srcs/app/lib/screens/chat_screen.dart` is a static `StatelessWidget`.
To fulfill the "Micro-animations" requirement, we need to introduce scale and opacity animations when message bubbles first render. According to memory, we must "manage the state (such as tracking seen IDs in a `Set`) to prevent the animation from re-triggering constantly during list scrolling."

## Design Doc
1.  **Refactor `_MessageBubble`**: Convert it to a `StatefulWidget` or use an implicitly animated widget like `TweenAnimationBuilder`. Add a Slide and Fade transition to simulate messages smoothly popping in.
2.  **Implementation**:
    *   Introduce a top-level or scoped `Set<String>` (e.g. `_seenMessageIds` inside `_ChatScreenState` or using a Riverpod provider) to track which messages have already animated.
    *   Pass a flag `shouldAnimate` to `_MessageBubble` based on whether the message ID is newly seen.
    *   If `shouldAnimate` is true, use `TweenAnimationBuilder` or an `AnimationController` to animate the entrance, and add the ID to the seen set so it doesn't animate again when scrolled out and back in.
3.  **UI Tokens**: Ensure the glassmorphism backdrop filter is applied to the message bubbles if not already present, following the OHC-SIP Stylistic Intent Profile (20px blur, glassmorphism background colors).

## Priority
P1

## Estimated Scope
Small
