# Automated Sales Tax Calculation (TaxJar)

## Title
Automated Sales Tax Calculation (TaxJar)

## Problem Statement
Maya (Home Baker) and Priya (Boutique Owner) struggle with calculating the correct sales tax for different states, counties, and cities when selling online. Doing this manually is a nightmare and carries legal risks. They need a system that automatically calculates and applies the correct tax rate at checkout without requiring them to become tax experts.

## Research Report
- **Strategy**: Direct integration with TaxJar API for real-time tax calculations during checkout and tax reporting.
- **Target Persona**: Maya (Home Baker), Priya (Boutique Owner).
- **Advantages**: Eliminates manual tax compliance work. Highly accurate. TaxJar automatically handles changing tax rules across jurisdictions.
- **Risks**: TaxJar pricing scales with order volume. Requires passing precise shipping addresses during checkout, which can fail if the address is invalid.
- **Pricing**: SaaS model with a base fee, scaling per transaction. Suitable for Cloud (multi-tenant where OHC might offer it as a premium add-on or pass through API keys) and Standalone (API key configured by merchant).
- **Reputation**: Highly rated for ease of use and API reliability. Competes with Stripe Tax and Avalara, but often simpler for SMBs.

## Design Doc
- When a customer enters their shipping address at checkout, OHC asynchronously calls the TaxJar API to fetch the exact tax amount for the cart.
- The tax is dynamically added to the order total before payment capture.
- The Finance agent aggregates tax collected in a simple monthly report.
- The user enables it via the settings dashboard by checking "Enable Automatic Sales Tax" and selecting the states they have nexus in.

## Implementation Prompt
Build a native integration with the TaxJar API to calculate sales tax at checkout dynamically based on the customer's shipping address. Ensure the merchant dashboard includes a simple settings panel to enable/disable automated tax collection and view a summary of taxes collected.
- **Acceptance Criteria**: Live tax rate is fetched at checkout based on the shipping address. Merchant can view total tax collected per month. Setting exists to enable/disable automated tax.
- **Priority**: P1
- **Estimated Scope**: Medium
