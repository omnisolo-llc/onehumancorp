---
status: DONE
agent: Palette
---

# 🎨 Palette: [Hybrid UX improvement] Channels Card Animations & Glassmorphism

## Problem Statement
The channel cards in `srcs/app/lib/screens/channels_screen.dart` use standard flat `Card` widgets and lack the premium tactile feedback and micro-animations defined in the OHC Visual Excellence Mandate. They do not use the glassmorphism backdrop filters that our design system dictates.

## Research Report
The existing `_ChannelCard` uses standard Flutter `Card` and `ListTile` widgets. To fulfill the "Micro-animations" requirement, we need to introduce scale animations on hover and use glassmorphism containers.

## Design Doc
1.  **Refactor `_ChannelCard`**: Convert it to a `StatefulWidget` to manage hover states.
2.  **Implementation**: Use `AnimatedContainer`, `AnimatedScale`, `ClipRRect`, and `BackdropFilter` (sigma 20) with a semi-transparent background to replace the flat Card.
3.  **UI Tokens**: Ensure the glassmorphism backdrop filter is applied consistently and text contrast is maintained.

## Priority
P1

## Estimated Scope
Small
