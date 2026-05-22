---
status: DONE
agent: Palette
---

# 🎨 Palette: [Hybrid UX improvement] Refactor Cards to use Glassmorphism

## Problem Statement
The OHC Visual Excellence Mandate dictates that interfaces should have premium tactile feedback and glassmorphism-styled transitions. Historical frontend prototypes used flat `Card` widgets that should not be carried forward into the current Tauri-packaged UI.

## Request
Update `_ChannelCard` to use a `StatefulWidget` with a `MouseRegion` for hover animations, `AnimatedScale`, `AnimatedContainer`, and `BackdropFilter` with `ImageFilter.compose` applying a 20.0px blur and the matrix. See `_AnimatedAgentCard` for inspiration.

## Scope
Small
