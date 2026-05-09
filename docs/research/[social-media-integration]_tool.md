# [social-media] Unified Social Inbox Integration

**Title:** Integrate Unified Omnichannel Social Media Inbox

**Problem Statement:**
Small business owners often struggle to keep up with customer messages scattered across multiple platforms (Instagram DMs, Facebook comments, WhatsApp, TikTok). Checking each app manually is time-consuming, and missed messages lead to lost sales and poor customer service. They need a single, unified inbox to view and respond to all social interactions without leaving the OHC platform.

**Research Report:**
* **Tools Evaluated:** Respond.io, ManyChat, Meta Business API.
* **Ease of Use:** Tools like Respond.io provide a non-technical setup for business owners to connect their social accounts without dealing directly with Meta Developer consoles.
* **Key Advantages:**
  - Supports WhatsApp, Instagram DMs, Facebook Messenger, and more out-of-the-box.
  - Reliable webhooks for incoming messages.
  - Handles the complexity of different message types (images, videos, quick replies).
* **Risks:**
  - Third-party dependency.
  - Pricing might be steep for very small businesses (Respond.io starts around $79/mo). Direct Meta API is free but requires more engineering effort on our side.
* **Pricing Estimate:** $0 - $79/month depending on the provider chosen.
* **Environment Support:** Webhooks can be routed to Cloud mode easily. For Standalone mode, webhooks require a tunneling service or an OHC cloud-relay to function properly.

**Design Doc:**
* **Trigger:** The user navigates to the "Integrations" page and clicks "Connect Social Inbox". They authenticate via an OAuth flow with the provider.
* **Actions:** OHC registers webhooks to listen for incoming messages. When a message is received, it appears in a new "Inbox" widget on the OHC dashboard.
* **User Experience:** The business owner sees a unified chat interface. They can reply directly from OHC, and the message is routed back to the native platform (e.g., as an Instagram DM).

**Implementation Prompt:**
Implement a unified inbox feature that allows users to connect their social media accounts. Ensure incoming messages appear in real-time in the OHC dashboard. The user should be able to reply directly from the dashboard, and the reply must be delivered to the customer on their original platform. Provide a seamless connection experience and clearly display the connection status of each platform.

**Priority:** P1
**Estimated Scope:** Large