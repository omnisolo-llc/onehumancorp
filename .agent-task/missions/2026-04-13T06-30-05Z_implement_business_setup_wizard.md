---
status: DONE
agent: Implementer
priority: P0
scope: Medium
---
# Implement Business Setup Wizard (New User — Day One)

## Problem Statement
There are currently no PENDING missions in the wizard/onboarding domain. As the Principal UX Wizard & Onboarding Experience Engineer, I am autonomously creating this mission to implement the highest-impact wizard feature: The Business Setup Wizard.

This is the very first screen a new user sees, designed to collect just enough information to auto-configure the platform with zero jargon.

## Requirements
- Welcome screen with hero animation and value proposition.
- Business profile: Company name, industry, size.
- Goal selection: Multi-select tiles (Support, Build software, Marketing, Data, Custom).
- Deployment preference: Cloud, Desktop, Mobile-only.
- Administrator account: Name, email, password.
- Review & Launch: Summary card with a pulsing "Launch My AI Team" CTA.

## Design Protocol
- Adhere to the OHC Premium Design Standards: Glassmorphism (`GlassCard`), 20px blur, Outfit/Inter typography.
- Progressive Disclosure: Keep the interface clean and friendly.
- Persist state to the backend.

## Execution Plan
1.  **UI Component**: Create `srcs/app/lib/screens/business_setup_wizard_screen.dart`.
2.  **State Management**: Implement a Riverpod provider to manage the wizard state.
3.  **API Integration**: Ensure it sends the collected data to the backend.
4.  **Testing**: Write a test for the wizard flow.
