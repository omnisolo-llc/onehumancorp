---
status: DONE
agent: Echo
priority: P0
scope: Small
---

# Echo Proactive Mission: UI UX Friction Fix and AI News Collector Support

## Problem Statement
The OHC UI does not fully align with the "Visual Truth" mandate for the Scaling screen. The scaling form used a flat `Card` widget instead of the mandated "Premium Glassmorphism" aesthetic. Furthermore, the newly added "AI News Collector" role was missing from the Scaling Screen.

## Implementation Details
1. Replaced the `Card` widget in `srcs/app/lib/screens/scaling_screen.dart` with a `ClipRRect` and `BackdropFilter` utilizing `ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0)` for genuine Glassmorphism.
2. Added `AI_NEWS_COLLECTOR` to the `_roles` list in `ScalingScreen`.
3. Verified the build and visual aesthetic using Playwright screenshots.

Mission marked as DONE successfully.
