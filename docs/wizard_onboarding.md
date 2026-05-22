# OHC Onboarding Wizards Architecture & Usage

## Overview
OneHumanCorp provides a suite of deeply integrated, highly polished onboarding wizards built with the Slint UI framework. These wizards fulfill the L7 Wizard & Onboarding Experience engineer mandate to allow zero-knowledge users to launch a business in minutes.

## Core Wizards

1. **Business Setup Wizard** (`setup_wizard.slint`)
   - Captures business intent, name, categories, product information, template selection, and domain preferences.
   - Includes AI-powered auto-suggest functions for business and product descriptions.

2. **Website Builder Onboarding** (`website_builder.slint`)
   - Guides users through template selection (Modern, Classic, Bold) and branding configurations.
   - Seamlessly integrates with the domain selection process.

3. **AI Agent Configuration** (`agent_config.slint`)
   - A non-technical interface for adding AI agents to the team.
   - Converts plain-language capability toggles (e.g., "Reply to customer messages") into backend API scopes (`read:messages`, `write:messages`).

4. **Prompt Tuning** (`prompt_tuning.slint`)
   - Allows users to define agent personas via tone, constraints, and few-shot examples.
   - Features a live chat preview to test configurations before saving.

5. **Ongoing Wizards**
   - **Grow My Business** (`grow_business.slint`): Recommends and executes strategic AI tasks like adding products or creating email campaigns.
   - **Fix Agent** (`ongoing_management.slint`): Guides users through reconnecting failed integrations.

## Progressive Disclosure
All wizards implement a consistent `AdvancedToggle` mechanism, ensuring non-technical users see simplified language by default, while developers can access raw JSON config blocks and API payload overrides.
