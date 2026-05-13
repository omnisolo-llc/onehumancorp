# [onboarding]_autonomous_setup_wizard.md

## Title: Zero-Friction Autonomous Setup Wizard

## Problem Statement
Small business owners find onboarding overwhelming. They need a system that builds itself.

## Research Report
- Shopify requires ~25 distinct decisions to launch.

## Design Doc
### High-Level Architecture
- **Entity Types**: `Storefront`, `OnboardingSession`.
- **UI Screen Flow**: Welcome -> Photo Upload -> Processing -> Success.
- **AI Agent Integration**: The `OnboardingAgent` configures the store based on 3 uploaded photos.

## Implementation Prompt
Implement the backend and UI components for the Autonomous Setup Wizard. Ensure the process is idempotent to support Cloud/Standalone switching.

## Priority
P0

## Estimated Scope
Large
