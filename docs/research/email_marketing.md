# Title: Integrated Email Marketing Campaigns

## Problem Statement
Small business owners want to send promotions or newsletters to their existing customers but find tools like Mailchimp too complex and expensive. They need a simple way to email their customer list directly from where they manage their business.

## Research Report
*   **Tool Candidates**: SendGrid, Mailgun, Resend.
*   **Evaluation**: Resend offers a very modern, developer-friendly API and excellent deliverability. SendGrid is legacy but proven. Mailgun is solid for bulk.
*   **Ease of Use**: By abstracting the email provider, the business owner just types a subject, message, and clicks "Send to all customers". No list exports needed.
*   **Pricing**: Resend is affordable (free tier up to 3k emails/mo).
*   **Modes**: Cloud (uses OHC centralized API keys). Standalone (user must provide their own API key, which adds friction).

## Design Doc
*   **Integration Trigger**: User navigates to the "Marketing" tab and drafts an email.
*   **Action**: The system fetches all opted-in customer emails and dispatches the campaign via the email provider API.
*   **User Interface**: A simple rich-text editor, a recipient selector (e.g., "All Customers", "Recent Customers"), and a "Send" button.

## Implementation Prompt
Create an email marketing tool within OHC. Users should be able to draft an email using a basic text editor and send it to their customer list. The integration should handle unsubscribes automatically. Acceptance criteria: user can draft an email, select recipients, send it, and the system tracks successful delivery.

## Priority
P2

## Estimated Scope
Medium
