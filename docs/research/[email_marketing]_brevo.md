# Scout: Tool Integration Research

## [Email Marketing] Issue Brief: Brevo Integration
**Title**: Integrate Brevo for Native Email Marketing Automation
**Problem Statement**:
Priya (Boutique) wants to notify her regular customers about a weekend sale. She finds Mailchimp too complex. She needs a simple, native way in OHC to send emails to her customer list without managing a separate, complicated platform.

**Research Report**:
- **Tool**: Brevo (formerly Sendinblue) API.
- **Evaluation**:
  - **Ease of Use**: High. Powerful API for transactional and marketing emails.
  - **Pricing**: Free tier (300 emails/day) is perfect for OHC's small business users.
  - **Reputation**: Reliable, privacy-focused, and user-friendly.
  - **Cloud vs. Standalone**: Works in both via API keys.
- **Key Advantages**: Keeps marketing workflows within OHC. "The Promoter" AI can generate and send campaigns directly.
- **Risks**: Ensuring deliverability and managing unsubscribe lists correctly.

**Design Doc**:
- **User Flow**: User clicks "Marketing" and connects their OHC account to Brevo (or uses OHC's shared account).
- **Integration**: OHC syncs customer emails to Brevo lists.
- **User Experience**: "The Promoter" AI drafts a sale email. The user clicks "Approve & Send" in the OHC dashboard.

**Implementation Prompt**:
Build an integration with the Brevo API. Implement automated synchronization of the OHC customer list to Brevo. Allow the OHC "Promoter" agent to generate email campaign content and trigger sends via the Brevo API directly from the OHC UI.

**Priority**: P1
**Estimated Scope**: Medium
