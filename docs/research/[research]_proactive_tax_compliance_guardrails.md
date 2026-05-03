# Issue Brief: Proactive Tax & Legal Compliance Guardrails

## Problem Statement
Small business owners (especially solopreneurs like Carlos the Handyman) are terrified of "doing it wrong" when it comes to taxes and legal policies. Competitors provide generic "Terms of Service" templates, but owners still feel the weight of liability and the complexity of tax nexus.

## Research Report
- **SMB Pain Point:** "Financial Fog" and "Support Deserts" around legal/tax issues are top 10 pain points (Rank 9 & 10).
- **Competitor State:** Shopify Tax is a paid add-on. Squarespace requires manual policy generation.
- **Opportunity:** Leverage "Department: Legal & Compliance" (The Protector) to provide proactive, invisible guardrails that keep the business safe while the owner sleeps.

## Design Doc
### High-Level Architecture
- **Compliance Agent:** Part of "The Protector" department. Scans business transactions and location data.
- **Auto-Policy Generation:** Automatically generates and updates Privacy Policies, Terms of Service, and Refund Policies as the business adds new product types (e.g., adding "Food" triggers a health-safety disclaimer).
- **Tax Nexus Watcher:** Monitors sales volume across different regions and alerts the owner *before* they hit a tax nexus threshold, providing a pre-filled registration task.

### Mobile UX Flow (375px First)
- **Shield Icon:** A small "Security Status" shield on the Dashboard.
- **Proactive Alerts:** "The Protector added a food safety disclaimer to your checkout because you added 'Pizza' to your menu."
- **Tax Summary:** A plain-language weekly brief: "You're 80% towards needing a sales tax permit in California. Don't worry, we'll tell you when to act."

### Implementation Prompt
Create the "Compliance Monitor" service for "The Protector" department. This service should analyze the tenant's product catalog and sales history to automatically maintain legal policies on the storefront. It must also implement a "Tax Threshold Alert" system that uses the Shared Task List to notify owners of emerging compliance requirements in a non-jargon, plain-language format.

## Priority
P0

## Estimated Scope
Medium
