# Integrated Customer Email Campaigns

## Problem Statement
Small businesses collect customer emails but don't know how to use them. Setting up Mailchimp is complicated and requires syncing lists manually. They need a simple way to send updates or promotions directly to their customer base.

## Research Report
**Competitive Landscape:**
1. **Mailchimp:** Feature-rich but increasingly expensive and complex for basic needs.
2. **Resend / Loops:** Developer-first, excellent deliverability, but requires OHC to build the campaign UI.
3. **Listmonk:** Open-source, good for Standalone, but UI is technical.

**Evaluation:**
- **Ease of Use:** OHC must provide the campaign builder UI; the underlying tool should just be an API (like Resend).
- **Deliverability:** Critical. If emails go to spam, the feature is useless.
- **Cloud vs Standalone:** Cloud uses Resend. Standalone might need to use the user's own SMTP server to avoid platform costs.

## Design Doc
- **Trigger:** User selects a group of customers in the OHC CRM and clicks 'Send Campaign'.
- **Action:** OHC provides a simple rich-text editor, compiles the email, and sends via the integrated provider (e.g., Resend API).
- **User Experience:** A 'Broadcast' tab in the CRM. Simple text/image editor, preview, and send. Basic analytics (open rate).

## Implementation Prompt
Build a 'Customer Broadcast' feature. The user selects a segment of their customers and writes an email using a block editor. The system sends the emails via a background job to ensure reliability. Provide basic open/click tracking. Hide all DNS/SMTP configuration from the user in Cloud mode; provide a simple SMTP setup wizard in Standalone mode.

## Priority
P2

## Estimated Scope
Medium
