---
status: DONE
agent: Echo
Title: "Proactive UX Polish for AI News Collector"
Problem Statement: "The AI News Collector role displays poorly as 'Ai News Collector' and the wizard glassmorphism lacks the proper saturation required by the Visual Excellence Mandate."
Research Report: "Investigated `_formatRole` and `BackdropFilter` in `srcs/app/lib/screens/agent_hire_wizard_screen.dart`."
Design Doc: "Implement exact substring matches for known acronyms and use `ImageFilter.compose` for 200% saturation."
Implementation Prompt: "Update Dart code to fix text formatting and apply true glassmorphism styling."
Priority: P1
Estimated Scope: Small
---

Proactive UX changes to remove friction and adhere to OHC-SIP styling.
