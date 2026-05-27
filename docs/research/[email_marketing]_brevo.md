## [Email Marketing] Issue Brief: Brevo Integration

**Title**: Scout 🔍: Integrate Brevo for Simplified Email Marketing
**Problem Statement**:
Business owners like Priya want to notify their existing customers about new stock or holiday sales. Traditional tools like Mailchimp are too complex and require manual template design, list management, and campaign scheduling. Engaging past customers without violating spam laws or using complex tools is a challenge.
**Research Report**:
- **Tool**: Brevo (formerly Sendinblue).
- **Evaluation**: Brevo provides a generous free tier (300 emails/day) which is perfect for many SMBs just starting out. It also offers a very intuitive drag-and-drop editor that is less overwhelming than Mailchimp.
- **Ease of Use**: High. The focus here is a one-way contact sync from OHC to Brevo, allowing the business owner to log into Brevo simply to hit "Send" on basic newsletters.
- **Pricing**: Generous free tier. Paid plans start around $25/mo, which is cost-effective for growing businesses.
- **Cloud vs. Standalone**: Works seamlessly in both. In Cloud, OHC could potentially offer a managed integration. In Standalone, the user just drops in their API key.
**Design Doc**:
- "Marketing" tab -> "Integrations" -> "Connect Brevo".
- User provides Brevo API key (or uses OAuth if we implement it).
- Background job runs daily (or on specific triggers like 'New Customer') to sync the OHC customer list to a specific Brevo list.
- One-way sync: OHC -> Brevo.
**Implementation Prompt**:
Create a background worker that performs a one-way sync of opted-in customers from the OHC database to a configured Brevo account using their API. Provide a UI in the Marketing settings to input the Brevo API key and select/create the target list. Handle rate limits and basic error logging.
**Priority**: P2
**Estimated Scope**: Medium
