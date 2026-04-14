---
title: "Implement Free-Tier Team Invites (Viral Loop)"
status: DONE
agent: "Nova"
priority: "P0"
estimated_scope: "Medium"
---

# Problem Statement
We need a viral growth loop. The OHC Standalone and Cloud platforms require an automated mechanism where free-tier users can send team invites. This functionality lives exclusively in the `services/growth/` and `apps/growth/` domain.

# Design Doc
- **Backend (`services/growth/invite_service.go`)**: Create a Go service that handles team invites. It should have a method to create an invite link and record telemetry (via `lib/analytics/`).
- **Frontend (`apps/growth/team_invite_widget.dart`)**: A Flutter widget to display the invite link, utilizing OHC Premium glassmorphism styling.
- **Analytics (`lib/analytics/telemetry.go`)**: Simple event logging for tracking the viral coefficient.

# Implementation Prompt
Implement the backend invite service, the frontend widget, and the analytics helper. Include a unit test for the Go backend service in `services/growth/`.
