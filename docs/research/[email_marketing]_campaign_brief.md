# Title: Implement Self-Sovereign Email Marketing via Listmonk

## Problem Statement
Small business owners want to send newsletters or promotions to their customer list, but tools like Mailchimp become prohibitively expensive as their list grows. They also don't want their customer data held hostage by a third-party platform.

## Research Report
- **Tool Evaluated:** Listmonk
- **Benefits:** Open-source, high-performance standalone mailing list and newsletter manager. Zero vendor lock-in.
- **Ease of Use:** Offers a clean UI for writing newsletters and managing templates.
- **Pricing:** Free (Open Source). The only cost is the underlying SMTP relay (e.g., AWS SES, SendGrid) which is vastly cheaper than traditional SaaS marketing platforms.
- **Cloud/Standalone:** Perfect for Standalone, as it runs locally and keeps all customer PII on-device. In Cloud, OHC can host instances and manage the SMTP configuration for the user.

## Design Doc
1. **Trigger:** Business owner navigates to the "Marketing" tab and clicks "Create Campaign".
2. **Action:** OHC opens an embedded, simplified version of the Listmonk campaign editor.
3. **UI Outcome:** The user selects an audience segment (which is automatically synced from the OHC CRM), writes their email using a visual builder, and clicks send. Analytics (opens/clicks) are displayed back in the OHC dashboard.

## Implementation Prompt
Integrate Listmonk as the underlying engine for a new "Marketing Campaigns" feature. Build a simple email composer UI inside OHC that allows business owners to send rich-text emails to their customer segments. Automatically sync the OHC customer database with Listmonk's subscriber lists. Surface open and click rates in the OHC dashboard.

## Priority
P2

## Estimated Scope
Medium
