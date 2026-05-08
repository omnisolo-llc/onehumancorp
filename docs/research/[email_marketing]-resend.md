# Title: Seamless Customer Updates via Resend Email Marketing

## Problem Statement
Small business owners want to announce sales, new services, or holiday hours to their past customers, but traditional tools like Mailchimp are too complex and expensive for simple blasts. They need a way to send a beautiful, plain-text or simple HTML email to their customer list directly from where that list lives.

## Research Report
- **Tool Evaluated**: Resend API
- **Benefit to Users**: Allows targeted email blasts directly from the OHC CRM without learning a new marketing platform.
- **Ease of Use**: Owner types a message in a simple compose window inside OHC, selects "All Customers" or a specific group, and clicks send. No drag-and-drop builders or DNS configuration required (OHC handles default sending domains).
- **Pricing**: Extremely developer-friendly with 3,000 free emails per month. Very cheap per-email cost thereafter.
- **Integration Risks**: Spam compliance (CAN-SPAM/GDPR). If OHC users send spam, the OHC shared sending domain could get blacklisted, affecting other tenants.
- **Environment**: Cloud mode perfectly suited for centralized domain reputation management. Standalone mode works by users providing their own API key, keeping reputation isolated.

## Design Doc
- **Trigger**: User navigates to the "Customers" tab, selects a group, and clicks "Send Broadcast".
- **Action**: User writes their message. OHC formats it, appends a mandatory unsubscribe link, and dispatches it via Resend.
- **User Interface**: A clean, distraction-free text editor. Basic stats (Sent, Opened) appear next to the broadcast history.

## Implementation Prompt
Integrate the Resend API to allow users to send email broadcasts to their customer lists. Provide a simple text editor UI for composing the message. Automatically append an unsubscribe link to comply with anti-spam laws. Display basic delivery and open metrics for past broadcasts.

## Priority
P2

## Estimated Scope
Medium