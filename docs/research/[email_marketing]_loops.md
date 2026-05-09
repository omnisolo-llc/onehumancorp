# Scout 🔍: Integrate Loops.so for Simple, Beautiful Email Marketing

## Problem Statement
Priya (Boutique Owner) wants to send newsletters and product updates but finds legacy tools like Mailchimp too complex, "heavy," and filled with technical jargon. She needs a "no-nonsense" tool that focuses on simplicity and clean design, allowing her to send beautiful emails without getting lost in a sea of features she doesn't need.

## Research Report
- **Tool**: Loops.so
- **Target Persona**: Priya (Boutique Owner), Leo (Music Tutor).
- **Evaluation**: Loops is built specifically for modern businesses that value simplicity. It has a high-quality editor and focuses on the essentials of email marketing and transactional emails.
- **Ease of Use**: Extremely high. The UI is clean, and the editor is intuitive for non-technical users.
- **Pricing**: Free for up to 1,000 subscribers and 2,000 emails/month. Paid plans start at a reasonable price for small businesses.
- **Reputation**: Excellent. Known for being the "Linear of email marketing."
- **Cloud vs. Standalone**: Compatible with both. Cloud can use a centralized OHC account with sub-accounts, or Standalone can use a user-provided API key.

## Design Doc
- **User Experience**: User connects Loops.so via a simple API key entry in the "Marketing" tab.
- **Automation**: OHC automatically syncs the customer list (from orders and signups) to Loops in the background.
- **AI Integration**: "The Promoter" (Marketing Agent) drafts email campaigns directly in OHC using the Loops API, suggesting copy and timing based on business activity.
- **Frictionless Sending**: The user approves the AI-drafted email, and it is sent via Loops without ever needing to leave the OHC dashboard.

## Implementation Prompt
Integrate Loops.so for native email marketing. Implement customer list synchronization (Audience API) and a basic campaign triggering mechanism. Ensure the UI remains radically simple, focusing on the user's content rather than complex layout settings.
- **Acceptance Criteria**: Merchant can connect Loops API key. Customer list syncs automatically. Merchant can trigger a basic email blast from OHC.
- **Priority**: P2
- **Estimated Scope**: Medium
