# Integration Issue Brief: Email Marketing (Mailchimp)

## Title
Email Marketing Integration: Mailchimp

## Problem Statement
Small business owners have customer lists but often lack the time or tools to engage them effectively to drive repeat business. Sending bulk emails from a standard Gmail account risks being marked as spam and lacks analytics. Owners need a way to easily send professional, branded newsletters and promotions to their customer base.

## Research Report
*   **Tool Evaluated**: Mailchimp
*   **Ease of Use**: Highly accessible for beginners, featuring a drag-and-drop email builder, pre-made templates, and straightforward list management.
*   **Market Position & Reputation**: The dominant player in small business email marketing. It is widely recognized, though some users feel pricing gets expensive as their list grows.
*   **Pricing**:
    *   **Free**: Up to 250 contacts, 500 emails/month (1 seat).
    *   **Essentials**: Starts at $13/month (up to 50,000 contacts, 3 seats).
    *   **Standard**: Starts at $20/month.
    *   **Premium**: Starts at $350/month.
*   **Cloud vs. Standalone Compatibility**: Mailchimp provides robust APIs. Both OHC Cloud and Standalone modes would connect securely to Mailchimp's cloud infrastructure via OAuth/API keys to sync contacts and trigger sends.

## Design Doc
*   **Integration Trigger**: User connects Mailchimp via OAuth in OHC settings.
*   **Action Flow**:
    1.  Two-way contact sync: Customers added to OHC are automatically pushed to a designated Mailchimp audience list.
    2.  Analytics pull: OHC retrieves high-level campaign metrics (open rates, click rates) via API to display in the OHC dashboard.
*   **User Experience**: The business owner manages their customer database in OHC, knowing it seamlessly updates their Mailchimp list. They can view the success of their latest email campaign directly on their OHC home screen without logging into Mailchimp.

## Implementation Prompt
Build a Mailchimp integration focusing on contact synchronization and campaign reporting. Create an OAuth connection flow. Once connected, implement a background sync that ensures any new contact added to OHC is pushed to a selected Mailchimp Audience, and vice versa. On the OHC marketing dashboard, build a widget that pulls the latest campaign's Open Rate and Click Rate from Mailchimp's API so the user can gauge performance at a glance.

## Priority
P1

## Estimated Scope
Medium
