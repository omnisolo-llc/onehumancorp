# Resend API Integration

**Title**: Implement Email Marketing Campaigns via Resend API
**Problem Statement**: Business owners (like Priya the boutique owner) need to send newsletters, promotional blasts, and automated "back in stock" emails to their customer list. Standard transactional email APIs are too complex for non-technical users to design templates.
**Research Report**:
- **Tool**: Resend API.
- **Ease of Use (End User)**: Transparent. Users write plain text or use an AI-assisted rich text editor in OHC; Resend handles the delivery and rendering.
- **Pricing**: Generous free tier (3,000 emails/mo). Paid tiers are affordable ($20/mo for 50,000 emails).
- **Cloud vs. Standalone**: Cloud-only API. Standalone users would need to provide their own Resend API key or use a local SMTP relay.
**Design Doc**:
- **Trigger**: User initiates an email campaign via the "Marketing" department, or an automated trigger fires (e.g., abandoned cart).
- **Action**: The AI agent drafts the content. OHC compiles it into an HTML template and sends it via the Resend API to the target audience segment.
- **UI**: A "Campaigns" tab where users can view sent emails, open rates, and click rates (via Resend webhooks).
**Implementation Prompt**: Integrate the Resend API to enable outbound email marketing. Provide a UI for users to draft and send broadcast emails to their customer lists. Implement webhook listeners to track and display open and click metrics in the OHC dashboard.
**Priority**: P1
**Estimated Scope**: Medium
