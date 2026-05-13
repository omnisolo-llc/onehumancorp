# Issue Brief: Integrated Email Campaigns and Audience Management

**Category**: Email Marketing

## Problem Statement
Business owners rely on expensive third-party tools to send newsletters. Moving contacts between their CRM and email tool is tedious and error-prone.

## Research Report

### Tool Evaluations

**1. Mailchimp**
- **Ease of Use for User**: Excellent drag-and-drop editor and list management.
- **Pricing**: Becomes very expensive very quickly as the subscriber list grows (e.g., $50+/month for moderate lists).
- **Integration**: Syncing OHC contacts to Mailchimp is possible, but it means the user has to log into Mailchimp to send emails, breaking the unified experience.

**2. SendGrid (API)**
- **Ease of Use for User**: SendGrid is a backend tool; the user would never see it. OHC would build the UI.
- **Pricing**: Extremely cheap for bulk sending.
- **Deliverability**: High reputation, ensuring emails don't go to spam.
- **Mode Compatibility**: Cloud mode handles this perfectly.

**3. Amazon SES**
- **Ease of Use for User**: Same as SendGrid (invisible to user).
- **Pricing**: The cheapest option on the market ($0.10 per 1000 emails).
- **Deliverability**: Requires strict handling of bounces and complaints to maintain account health.

**4. Resend**
- **Ease of Use for User**: Invisible to user.
- **Pricing**: Modern API, slightly more expensive than SES but much better developer experience for creating React-based email templates.

**Summary Recommendation**: OHC should build a native email template builder using a library like React Email, and route the actual sending through Amazon SES or Resend. This provides a Mailchimp-like experience directly inside OHC without the Mailchimp price tag.


## Design Doc
Integrate with SendGrid or Amazon SES for email delivery. Build a drag-and-drop template editor in OHC. Store campaign performance (open/click rates) alongside customer profiles. Cloud mode handles mass sending; Standalone mode securely batches emails.

## Implementation Prompt
Implement an email campaign builder. Users should be able to select a list of customers, design an email with a visual editor, and schedule it for delivery.

## Priority
P1

## Estimated Scope
Large
