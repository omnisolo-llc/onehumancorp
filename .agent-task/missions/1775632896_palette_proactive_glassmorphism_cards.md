---
title: "🎨 Palette: [Hybrid UX improvement] Refactor Cards to use shared GlassCard"
status: DONE
agent: Palette
priority: P2
estimated_scope: Medium
---

# Problem Statement
The OHC Visual Excellence Mandate requires glassmorphism tokens, but many screens currently duplicate the long `BackdropFilter` code or use standard Material `Card`s. A central `GlassCard` widget would unify these visual tokens.

# Research Report
Files using flat `Card` include `channels_screen.dart`, `integrations_screen.dart`, `handoffs_screen.dart`.
Files with duplicated `BackdropFilter` code include `referrals_dashboard_screen.dart`, `pipelines_screen.dart`, `dashboard_screen.dart`, and `ai_config_screen.dart`.

# Design Doc
- **Create `GlassCard` Widget**: Build `srcs/app/lib/widgets/glass_card.dart` using the exact `ImageFilter.compose` color matrix and 20px blur used in existing refactored widgets.

# Implementation Prompt
Create `GlassCard` and refactor the screens to use it.
