---
status: DONE
agent: Palette
---

# 🎨 Palette: [Hybrid UX improvement] Chat Screen Premium Aesthetics

## Problem Statement
The current `ChatScreen` in `srcs/app/lib/screens/chat_screen.dart` uses standard Flutter Material widgets (basic `Container` for bubbles, simple `TextField` for input). It lacks the "Aesthetic Excellence" mandated by the OHC visual identity, specifically missing Glassmorphism, 20px blurs, Outfit/Inter typography, and entrance animations for messages.

## Research Report
The `_MessageBubble` and `_InputBar` widgets are statically styled.
To fulfill the "Undercover Mode" and "Micro-animations" requirement, we need to introduce scale and opacity animations when messages appear, and apply the OHC Glassmorphism backdrop filter to the input bar and chat bubbles.

## Design Doc
1.  **Refactor `_MessageBubble`**: Convert it to use an `AnimatedContainer` or `ScaleTransition` + `FadeTransition` for an entrance animation. Apply Glassmorphism styling with `BackdropFilter`.
2.  **Refactor `_InputBar`**: Wrap the input field in a Glassmorphic container.
3.  **Refactor `_RoomPickerDialog`**: Make it a premium dialog matching the aesthetic.
4.  **UI Tokens**: Use Outfit for headers/names and Inter for message body text. Ensure a 20px blur is applied for the glass effect.

## Priority
P1

## Estimated Scope
Medium
