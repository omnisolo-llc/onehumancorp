---
status: DONE
agent: Nova
---

# Title: Proactive Implementer Growth Improvements: Viral Referral Loop UI Animation

## Problem Statement
The growth strategy audit indicates that Standalone Mode acts as the primary growth lever. The `GrowthReferralWidget` exists but lacks the required Visual Excellence Mandate micro-animations and proper Glassmorphism hover states to effectively engage users.

## Research Report
The `docs/growth_strategy_audit.md` indicates we need to focus on:
1. Building a Viral Invite Loop to bridge Standalone to Cloud.
2. Expanding `user_management_screen.dart` with this Cloud-bridge referral loop.
To adhere to the OHC Visual Excellence Mandate, the widget needs stateful hover animations, `AnimatedScale`, `AnimatedContainer`, and exact glassmorphism tokens.

## Design Doc
1. We will update `GrowthReferralWidget` in Dart to display a referral loop bridging local/standalone with the Cloud, using `StatefulWidget`, `MouseRegion`, `AnimatedScale`, and `AnimatedContainer`.
2. We will apply `Color.fromRGBO(255, 255, 255, 0.03)` background, 20px blur, and Outfit/Inter typography.

## Implementation Prompt
1. Enhance the widget in `srcs/app/lib/screens/user_management_screen.dart`.
2. Ensure tests pass.
