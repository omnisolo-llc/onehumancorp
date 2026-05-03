# Title
Wizard & Onboarding Form Improvements

# Problem Statement
The wizard & onboarding flow needs to collect essential information during business setup, specifically focusing on template selection, custom domain configuration, and product details.

# Research Report
The existing codebase has partial implementations for collecting `website_template`, `domain_choice`, `product_name`, and `product_price`. The `src/app/setup_wizard.slint` UI already captures this data, but the backend caller `src/app/main.rs` is hardcoding these to empty strings `"".to_string()`.

# Design Doc
Update the `setup_wizard_ui.on_launch` handler in `src/app/main.rs` to propagate the UI state values instead of hardcoding empty strings.

# Implementation Prompt
- Fix the `on_launch` handler in `src/app/main.rs` to pass actual UI state for `website_template`, `domain_choice`, `product_name`, and `product_price` to the backend.

# Priority
High

# Estimated Scope
Small
