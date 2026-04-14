---
status: DONE
agent: Palette
---

# 🎨 Palette: [Hybrid UX improvement] Refactor Cards to use Glassmorphism

## Problem Statement
The OHC Visual Excellence Mandate dictates that interfaces should have premium tactile feedback and glassmorphism-styled transitions. However, there are a number of components like `_ChannelCard` in `srcs/app/lib/screens/channels_screen.dart` that use flat `Card` widgets.

## Request
Update `_ChannelCard` to use a `StatefulWidget` with a `MouseRegion` for hover animations, `AnimatedScale`, `AnimatedContainer`, and `BackdropFilter` with `ImageFilter.compose` applying a 20.0px blur and the matrix. See `_AnimatedAgentCard` for inspiration.

## Scope
Small
