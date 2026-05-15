# Resend - Email Marketing & Notifications

## Problem Statement
Business owners need to easily reach out to their customer lists with promotions, updates, or automated receipts without wrestling with complex templates or dealing with severe spam delivery issues.

## Research Report
Resend is a modern, developer-friendly email API platform.
- **Ease of Use for SMBs**: High. Business owners use an intuitive UI in OHC to compose emails, while Resend handles the backend delivery.
- **Pricing**: Generous free tier and reasonable pricing for scaling up.
- **Reputation**: High deliverability rates and modern developer experience.
- **Competitive Analysis**: Better developer experience and deliverability out-of-the-box compared to legacy tools like SendGrid or Mailgun.

## Design Doc
**Trigger**: Business owner clicks "Send Email Campaign" or system triggers a transactional email.
**Actions**:
- OHC formats the email content and sends it via Resend API.
- Resend handles delivery, bounces, and complaints.
**User Experience**: A simple email composer in the OHC dashboard.

## Implementation Prompt
**User-facing Outcome**: A business owner can easily send transactional emails and simple marketing campaigns to their customers with high deliverability.
**Acceptance Criteria**:
- System can send transactional emails (e.g., receipts) reliably.
- Business owner can compose and send a broadcast email to their customer list.
- Bounce and complaint handling is implemented.

## Priority
P2 (Medium)

## Estimated Scope
Medium
