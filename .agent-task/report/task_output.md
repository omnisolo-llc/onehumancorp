# Wizard & Onboarding Form Improvements

## Problem Statement
The wizard & onboarding flow needs to collect essential information during business setup, specifically focusing on template selection, custom domain configuration, and product details.

## Research Report
The existing codebase has partial implementations for collecting `website_template`, `domain_choice`, `product_name`, and `product_price`. The `src/app/setup_wizard.slint` UI already captures this data, but the backend caller `src/app/main.rs` is hardcoding these to empty strings `"".to_string()`.

## Design Doc
Update the `setup_wizard_ui.on_launch` handler in `src/app/main.rs` to propagate the UI state values instead of hardcoding empty strings.
Currently, `ui.get_website_template()`, `ui.get_domain_choice()`, `ui.get_product_name()`, and `ui.get_product_price()` are available on the `ui` object but are completely ignored and replaced with hardcoded dummy strings when constructing `req_website_template`, `req_domain_choice`, `req_first_product_name`, and `req_first_product_price`. We must use the values from the slint UI component instead.

## Implementation Prompt
- Fix the `on_launch` handler in `src/app/main.rs` to pass actual UI state for `website_template`, `domain_choice`, `product_name`, and `product_price` to the backend. Modify lines `let req_website_template = website_template.to_string();` etc. to fetch the actual properties using getter methods from the slint instance.

## Priority
P1

## Estimated Scope
Small
