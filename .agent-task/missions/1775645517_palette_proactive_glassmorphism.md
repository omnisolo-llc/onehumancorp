---
status: DONE
agent: Palette
title: "🎨 Palette: [Hybrid UX improvement] Apply Glassmorphism across UI"
priority: P1
scope: Medium
---

# Problem Statement
The OHC Visual Excellence Mandate dictates that interfaces should have premium tactile feedback and glassmorphism-styled transitions. There were a number of components across the Flutter application (such as `ChannelsScreen`, `IntegrationsScreen`, `HandoffsScreen`, `ChatScreen`, and `LogsScreen`) that still used flat Material `Card` or `Container` widgets without the proper styling.

# Research Report
An audit of `srcs/app/lib/screens/` revealed several screens lacking the mandated Glassmorphism tokens. To fulfill the "Undercover Mode" and "Aesthetic Excellence" requirements, these have been proactively refactored.

# Design Doc
- **Refactor Flat Cards/Containers**: Convert standard `Card` usage to `StatefulWidget`s or animated containers where appropriate.
- **Implementation**:
  - Applied `MouseRegion` for hover detection.
  - Wrapped content in `AnimatedScale` (scale 1.02 on hover).
  - Used `ClipRRect` and `BackdropFilter` with `ImageFilter.compose` applying the specific color matrix and `ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0)`.
  - Used `AnimatedContainer` with background color and border opacity changing on hover or states.

# Implementation Details
Refactored the following components:
1. `_ChannelCard` in `srcs/app/lib/screens/channels_screen.dart`.
2. `_IntegrationCard` and `_MCPToolTile` in `srcs/app/lib/screens/integrations_screen.dart`.
3. `_HandoffCard` in `srcs/app/lib/screens/handoffs_screen.dart`.
4. `_MessageBubble` in `srcs/app/lib/screens/chat_screen.dart`.
5. Logs container in `srcs/app/lib/screens/logs_screen.dart`.
