# Title: Instagram Direct Message Unified Inbox Integration

## Problem Statement
Small business owners often struggle to manage incoming messages across various social media platforms. Specifically, Instagram Direct is a high-volume channel for customer inquiries, but managing it natively means constantly switching apps. This leads to missed sales opportunities and delayed customer support, particularly for businesses that rely heavily on visual marketing like boutiques or local services. A non-technical user needs a way to view and respond to Instagram DMs directly from their central dashboard without needing to understand APIs or complex setups.

## Research Report
**Market Analysis & Pain Points:**
- **High Friction:** Currently, business owners manage DMs natively on the Instagram app, which doesn't easily scale to multiple team members or integrate with order management.
- **Competitors:** Tools like Sprout Social, Hootsuite, and ManyChat offer this, but they are often too complex or expensive for very small businesses.
- **The Instagram Messenger API:** Meta's API allows third-party platforms to receive webhooks for new DMs and send replies. It supports rich media (images, quick replies).
- **Reputation & Ease of Use:** The end-user experience (the business owner) is highly dependent on the integration's UI. If done right, they simply click "Connect Instagram" and messages flow in. Meta's OAuth flow is standard but can be confusing if permissions aren't clearly explained.
- **Pricing:** Meta doesn't charge for standard DM API usage, but third-party unified inbox tools typically charge $15-$50/month. We can offer this natively as a value-add.

**Key Advantages:**
- Centralizes a massive communication channel.
- Enables future AI-assisted replies or routing.

**Integration Risks:**
- Meta's strict review process for API access.
- 24-hour response window policy (messages must be replied to within 24 hours).

**Environment Support:**
- **Cloud:** Easily supported via webhooks to our backend.
- **Standalone:** More complex. Would require a relay service or long-polling from the desktop app, as Meta cannot send webhooks directly to a local machine behind a NAT.

## Design Doc
**Trigger:**
The user navigates to "Integrations" -> "Social Media" and clicks "Connect Instagram". They are redirected to Meta's OAuth flow to grant permission to their Instagram Professional account.

**Action:**
Once connected, OHC registers webhooks with Meta. When a customer sends a DM to the business's Instagram, Meta sends a webhook payload to the OHC backend. The backend normalizes this message into the OHC unified inbox format.

**User View:**
The business owner sees the Instagram message appear in their OHC Unified Inbox alongside emails and SMS. They type a reply in the OHC interface, and OHC sends it back to Instagram via the API. The owner never has to open the Instagram app.

## Implementation Prompt
Implement a unified inbox channel for Instagram Direct Messages.
- Create an OAuth connection flow for users to link their Instagram Professional accounts.
- Build a webhook receiver to ingest incoming DMs and store them in the unified inbox data model.
- Provide a UI for the user to view the conversation thread and send text/image replies back to the customer on Instagram.
- Ensure the UI highlights the 24-hour response window limit imposed by Meta.
- (Do not prescribe specific database schemas or API routes; design the backend to support these user-facing requirements.)

## Priority
P1

## Estimated Scope
Medium
