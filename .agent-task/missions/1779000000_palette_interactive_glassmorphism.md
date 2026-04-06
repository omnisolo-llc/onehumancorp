---
title: "Implement Interactive Glassmorphism Hover Effects for Swarm Memory Cards"
status: DONE
agent: Palette
priority: "P1"
estimated_scope: "Small"
---

# Problem Statement
The current implementation of the Swarm Memory Dashboard (`SwarmMemoryScreen`) uses static cards (`_GlassMessageCard` and `_MemoryCard`) that lack tactile feedback and premium interactivity. Furthermore, they use a standard blur rather than the official OHC Glassmorphism visual tokens which require a composed image filter and interactive `MouseRegion` based scaling animations.

# Research Report
- Visual excellence is a core mandate. OHC Glassmorphism tokens require `BackdropFilter` with a 20px blur via `ImageFilter.compose(ImageFilter.blur(sigmaX: 20, sigmaY: 20)...)` to create the correct saturation and blur matrix.
- Interactive states must be added using `StatefulWidget` and `MouseRegion` to trigger `AnimationController` for smooth scaling on hover.

# Implementation Prompt
You are Palette, Principal Flutter Developer.
1. Update `srcs/app/lib/screens/swarm_memory_screen.dart`.
2. Convert `_GlassMessageCard` and `_MemoryCard` to `StatefulWidget`s.
3. Wrap them in `MouseRegion` to detect hover states.
4. Implement an `AnimationController` to smoothly scale the cards (e.g., to 1.02x) when hovered.
5. Upgrade the `BackdropFilter` to use the official OHC `ImageFilter.compose` matrix.
6. Verify Flutter tests pass.
