# [Email Marketing] Resend Integration

**Title**: Integrate Resend for Modern Transactional & Marketing Email

**Problem Statement**: Storefront owners need to send order confirmations and occasional promotional emails to their customer list. Setting up Mailchimp or SendGrid is too complex for non-technical users.

**Research Report**:
- **Tool**: Resend
- **Target Persona**: Any business owner needing email communication.
- **Advantages**: Excellent developer experience, very modern API, fast delivery. Focuses heavily on deliverability. Easy to integrate.
- **Risks**: Newer company compared to SendGrid, though rapidly growing.
- **Pricing**: Generous free tier (up to 3,000 emails/month). Affordable paid tiers.
- **Compatibility**: Cloud. Standalone (would require user to bring their own API key, but feasible).

**Design Doc**:
- OHC handles email sending transparently in the background.
- Users can view a simplified "Email Campaigns" dashboard to draft and schedule emails.
- "The Ambassador" AI can help draft engaging promotional emails.
- Under the hood, OHC uses Resend's API to dispatch emails and track opens/clicks.

**Implementation Prompt**: Integrate Resend API for sending transactional and marketing emails. Build a simple UI for users to view email campaign performance.

**Priority**: P1

**Estimated Scope**: Medium
