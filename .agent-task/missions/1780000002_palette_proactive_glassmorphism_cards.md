---
title: "🎨 Palette: [Hybrid UX improvement] Refactor Cards to use Glassmorphism across remaining screens"
status: DONE
agent: Palette
priority: P2
estimated_scope: Medium
---

# Problem Statement
Despite earlier refactoring efforts, numerous components across the Flutter application still use flat, standard Material `Card` widgets or custom implementations that do not fully adhere to the OHC Visual Excellence Mandate. There are multiple screens that still use standard Card widgets.

# Research Report
We need to introduce a generic `GlassCard` widget and then apply it throughout the app instead of `Card`. Since I noticed that we have `srcs/app/lib/widgets/` but we didn't find `glass_card.dart` yet, I will create it and apply it.

# Design Doc
- **Create `GlassCard` Widget**: Build a reusable `StatefulWidget` in `srcs/app/lib/widgets/glass_card.dart` that manages hover states using `MouseRegion`, applies a scaling animation via `AnimatedScale`, and uses `BackdropFilter` with `ImageFilter.compose` to apply the OHC specific color matrix and a 20.0px blur.
- **Refactor Flat Cards**: Replace standard `Card()` widgets with `GlassCard()` in all screens where possible.

# Implementation Details
- Ensure it takes parameters like `child`, `padding` (default to `EdgeInsets.zero`), `onTap`, and styling parameters matching other Glass components.
- Use `backdrop-filter: blur(20px) saturate(200%)`, `background: rgba(255, 255, 255, 0.03)` (via flutter equivalents).
