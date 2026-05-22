# OHC Onboarding Wizards Architecture & Usage

## Overview
OneHumanCorp provides onboarding flows through the current Tauri desktop shell
and packaged web UI assets. The historical Slint wizard files have been removed;
new onboarding work should target the Tauri app, the packaged static frontend,
and the Rust onboarding APIs.

## Core Wizards

1. **Business Setup Wizard**
   - Captures business intent, name, categories, product information, template selection, and domain preferences.
   - Includes AI-powered auto-suggest functions for business and product descriptions.

2. **Website Builder Onboarding**
   - Guides users through template selection (Modern, Classic, Bold) and branding configurations.
   - Seamlessly integrates with the domain selection process.

3. **AI Agent Configuration**
   - A non-technical interface for adding AI agents to the team.
   - Converts plain-language capability toggles (e.g., "Reply to customer messages") into backend API scopes (`read:messages`, `write:messages`).

4. **Prompt Tuning**
   - Allows users to define agent personas via tone, constraints, and few-shot examples.
   - Features a live chat preview to test configurations before saving.

5. **Ongoing Wizards**
   - **Grow My Business**: Recommends and executes strategic AI tasks like adding products or creating email campaigns.
   - **Fix Agent**: Guides users through reconnecting failed integrations.

## Progressive Disclosure
All wizards implement a consistent `AdvancedToggle` mechanism, ensuring non-technical users see simplified language by default, while developers can access raw JSON config blocks and API payload overrides.
