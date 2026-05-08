# Scout: Tool Integration Research [Q2]

## [Email Marketing] Issue Brief: Brevo Integration

**Title**: Native Email Marketing & Automation via Brevo

**Problem Statement**:
Priya (Boutique Owner) wants to keep her customers engaged by sending them updates about new stock or special offers. However, she finds dedicated email tools like Mailchimp too expensive and intimidating. She needs a simple, "one-tap" way to send professional emails to her customer list directly from her OHC dashboard, without learning a new platform.

**Research Report**:
- **Tool**: Brevo (formerly Sendinblue) API.
- **Evaluation**: Brevo is highly developer-friendly and offers an all-in-one suite including marketing campaigns and transactional emails.
- **Ease of Use**: Excellent. OHC can abstract the entire Brevo interface, letting the user simply "Approve" an AI-generated newsletter.
- **Pricing**: Very SMB-friendly. Free tier allows 300 emails per day. Paid plans are based on volume, not contact list size (unlike Mailchimp).
- **Reputation**: High. Known for great deliverability and simple pricing.
- **Cloud vs. Standalone**: Cloud mode uses a centralized OHC account; Standalone mode allows the user to input their own API key.

**Design Doc**:
- OHC automatically syncs the "Customers" table with a Brevo contact list.
- "The Promoter" AI agent suggests a campaign (e.g., "Holiday Sale").
- The user reviews the AI-generated template in the Marketing dashboard.
- Upon approval, OHC dispatches the email via Brevo's API.
- Results (opens, clicks) are pulled back into the OHC dashboard for a simplified view.

**Implementation Prompt**:
Integrate the Brevo API for native email campaign management. Implement contact synchronization and a simplified campaign sending interface. Allow the AI Marketing agent to generate and queue email content for user approval.
- **Acceptance Criteria**: Customer list syncs to Brevo. User can send an email campaign from OHC. Open and click rates are displayed in the OHC dashboard.
- **Priority**: P1
- **Estimated Scope**: Medium
