---
title: "🎨 Palette: [Hybrid UX improvement] Refactor remaining Cards to use Glassmorphism"
status: DONE
agent: Palette
priority: P1
estimated_scope: Small
---

# Problem Statement
The OHC Visual Excellence Mandate dictates that interfaces should have premium tactile feedback and glassmorphism-styled transitions. However, there are a number of components across the Flutter application (such as the standard `Card` widget in `handoffs_screen.dart`, `scaling_screen.dart`, and `security_screen.dart`) that use flat Material `Card` widgets.

# Research Report
An audit of `srcs/app/lib/screens/` revealed several screens still using standard Material `Card` widgets instead of the mandated Glassmorphism tokens.
To fulfill the "Undercover Mode" and "Aesthetic Excellence" requirements, these must be refactored to use `AnimatedContainer`, `AnimatedScale`, and `BackdropFilter` with the OHC color matrix and a 20.0px blur.
Other components like `pipelines_screen.dart` and `referrals_dashboard_screen.dart` already implement this behavior correctly.

# Design Doc
- **Refactor Flat Cards**: Convert standard `Card` usage to `StatefulWidget`s to manage hover states.
- **Implementation**:
  - Add `MouseRegion` for hover detection.
  - Wrap the content in `AnimatedScale` (scale 1.02 on hover).
  - Use `ClipRRect` and `BackdropFilter` with `ImageFilter.compose` applying the specific color matrix and `ImageFilter.blur(sigmaX: 20.0, sigmaY: 20.0)`.
  - Use `AnimatedContainer` with background color and border opacity changing on hover.

# Implementation Details
Refactored the following components:
1. `_HandoffCard` in `srcs/app/lib/screens/handoffs_screen.dart`.
2. `_ScalingCard` in `srcs/app/lib/screens/scaling_screen.dart`.
3. The issue card in `srcs/app/lib/screens/security_screen.dart`.
Verified frontend visual tokens.
