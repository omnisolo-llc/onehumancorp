### 4. Loops (Email)
**Title**: Loops Integration for Email Marketing

**Problem Statement**:
Small business owners need an easy way to send newsletters or promotional emails to their customer base directly from their CRM. Existing tools like Mailchimp are often too complex, bloated, or expensive for simple use cases. They need a straightforward way to email their synced customer list.

**Research Report**:
- **Tool**: Loops (Email Marketing).
- **Ease of Use**: Very high. Designed for modern SaaS and simple campaign management. Clean UI and excellent template quality.
- **Pricing**: Free tier up to 1,000 contacts. Then scales predictably.
- **Reputation**: Highly regarded in the startup/modern business ecosystem for its simplicity and excellent API.
- **Compatibility**: Works well in Cloud mode via API. In Standalone mode, users would need their own API keys, which is standard for email sending.

**Design Doc**:
- **Trigger**: User selects a segment of customers in OHC and clicks "Send Email Campaign".
- **Action**: OHC syncs the selected contacts to Loops via API and triggers an email send, or uses the API to send a transactional/broadcast email directly.
- **User Interface**: A simple "Campaigns" tab where users can draft an email and select recipients. The actual sending is offloaded to Loops to handle spam compliance and deliverability.
- **Integration Flow**: User enters their Loops API Key in Settings -> Email Integrations.

**Implementation Prompt**:
Integrate Loops for email marketing. Allow users to configure their Loops API key. Add functionality to sync customer lists to Loops contacts. Enable sending basic email broadcasts to selected customer segments directly from the OHC interface using the Loops API.

**Priority**: P2
**Estimated Scope**: Medium
