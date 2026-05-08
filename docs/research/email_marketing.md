# [Email Marketing] Customer Campaigns and Newsletters

## Problem Statement
Small businesses want to re-engage their existing customers with promotions or newsletters. Existing tools like Mailchimp can be overwhelming and disconnected from their core customer list, leading to out-of-sync contacts and low usage.

## Research Report
**Tools Evaluated:** Mailchimp, SendGrid, Resend

*   **Ease of Use:** Mailchimp has a strong UI but complex list management. APIs like Resend and SendGrid are developer-friendly and allow us to build a heavily simplified UI within OHC. The biggest hurdle for users is domain verification (SPF/DKIM/DMARC) which is required for good deliverability.
*   **Pricing:** Mailchimp has a decent free tier but scales up quickly. API providers like Resend/SendGrid charge per 1000 emails, which is generally very cheap for small volumes.
*   **Reputation:** Mailchimp is the most recognized name by business owners; Resend/SendGrid are industry standards for API delivery.

## Design Doc
**Trigger:** User wants to send a mass email to their customer list.
**Action:** User creates a campaign in OHC and clicks send.
**User Sees:** A "Campaigns" interface in OHC where they can select an audience (e.g., "All Customers", "Recent Buyers"), write an email using a simple rich-text editor or basic template, and click send. They should also see basic analytics (open rates).

## Implementation Prompt
Implement a simplified email marketing module. Integrate with an email sending API (e.g., Resend). The feature should allow the user to select contacts from the OHC CRM, compose an email, and dispatch it. Critical: You must design a workflow that either helps the user verify their own domain for sending, or uses a shared OHC sending domain (for cloud mode) to abstract the technical setup away from the user.

## Priority
P2

## Estimated Scope
Medium

## Mode Compatibility
*   **Cloud:** Excellent. We can utilize a centralized billing account and potentially sub-domain routing to simplify user setup.
*   **Standalone:** Good, but requires the user to input their own API key (e.g., Resend key) to handle the actual delivery and billing of the emails, which adds a setup step.
