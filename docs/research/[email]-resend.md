# Title: Integrate Resend for Transactional and Marketing Emails

## Problem Statement
Small business owners need an easy way to send appointment confirmations, receipts, and promotional newsletters to their customer base. They find traditional tools like Mailchimp too complex and disconnected from their core operations platform.

## Research Report
Resend is a developer-first email delivery platform, built for modern applications.
- **Ease of use:** High for developers, seamless for end users. Abstracting the complexity allows us to build simple UI for the business owner.
- **Pricing:** Excellent free tier (3,000 emails/month); highly competitive paid tiers.
- **Reputation:** Rapidly becoming the standard for modern SaaS and application transactional email.
- **Cloud/Standalone:** Cloud API primarily. In Standalone, users would need to provide their own Resend API key or an SMTP fallback.

## Design Doc
- **Trigger:** System triggers (e.g., new booking, payment success) or manual marketing campaign dispatch from the UI.
- **Action:** Formats the payload into an HTML template and dispatches it via the Resend API.
- **User Interface:** A "Marketing" tab allowing users to draft simple text/image emails and select customer segments to blast. A background service silently handles transactional emails.

## Implementation Prompt
Integrate a unified email sending system. First, implement transactional emails (receipts, booking confirmations) that send automatically without user intervention. Second, create a simple "Campaigns" interface where the business owner can write a message, select a list of customers, and click "Send" to blast the email to their list, along with basic open-rate tracking.

## Priority
P1

## Estimated Scope
Medium
