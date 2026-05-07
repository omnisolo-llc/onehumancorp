# Native Integration of Brevo for Email Marketing

## Title
Native Integration of Brevo for Email Marketing

## Problem Statement
Small business owners want to retain customers and drive repeat sales through newsletters and promotions, but they lack the design skills and time to use standalone tools like Mailchimp. They need a simple way to send beautiful, AI-generated emails to their customer list directly from the platform where their sales happen.

## Research Report
- **Strategy**: Direct integration with Brevo's (formerly Sendinblue) API for email campaign delivery and contact management.
- **Target Persona**: Boutique owners, home bakers, local service providers looking to build customer loyalty.
- **Advantages**: Brevo has a very generous free tier compared to competitors (up to 300 emails/day), making it ideal for the OHC target market. Their API is robust and supports transactional and marketing emails.
- **Risks**: Ensuring high deliverability relies on domain authentication, which can be technical for small business owners. We need to simplify the DNS setup process.
- **Pricing**: Free tier (300 emails/day). Starter plan is $25/mo for 20k emails/mo.
- **Compatibility**: Works well in Cloud mode using a centralized account with dedicated IPs or sub-accounts. In Standalone mode, users would provide their own Brevo API key.

## Design Doc
- In the OHC "Marketing" tab, users see an option to "Create Email Campaign".
- OHC automatically syncs the internal customer list to a Brevo contact list in the background.
- Users describe their campaign (e.g., "Announce our new summer collection").
- The AI Agent generates the email content and subject line, applying the business's branding.
- Upon user approval, OHC uses the Brevo API to schedule and dispatch the campaign.
- Campaign analytics (opens, clicks) are fetched from Brevo and displayed in the OHC dashboard.
- **AI Integration**: The Marketing Agent analyzes open rates and suggests the best days/times to send future campaigns.

## Implementation Prompt
Integrate Brevo to enable native email marketing campaigns. Sync the OHC customer database with Brevo contact lists. Allow the AI to generate email content that the user can approve and send via the Brevo API. Display basic campaign performance metrics (open rates, click rates) back in the OHC dashboard.
- **Acceptance Criteria**: Merchant can trigger an email campaign from OHC. OHC customers are synced to Brevo. AI-generated email is delivered via Brevo. Open/click metrics are visible in OHC.
- **Priority**: P1
- **Estimated Scope**: Large
