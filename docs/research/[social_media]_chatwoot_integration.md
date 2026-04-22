# [Social Media] Integrate Chatwoot for Unified Inbox

## Problem Statement
Small business owners often struggle to keep up with customer messages scattered across multiple platforms—Instagram DMs, Facebook comments, WhatsApp, and their website chat. Missing a message often means missing a sale. They need a single, unified inbox where they can see and reply to all customer inquiries in one place, without switching between apps.

## Research Report
**Tool Analyzed:** Chatwoot (Open Source Omnichannel Customer Support)

*   **Capabilities:** Chatwoot provides a unified inbox connecting Facebook, Instagram, Twitter, WhatsApp, LINE, SMS, and email.
*   **Ease of Use (for Non-Technical Users):** Once set up, the interface is similar to standard email or messaging apps, making it highly intuitive. The initial integration (OAuth flows) can be complex but can be abstracted away by the OHC platform.
*   **Cloud vs. Standalone:**
    *   *Cloud:* Can be hosted as a multi-tenant service or integrated via their cloud API.
    *   *Standalone:* Being open-source, it can be containerized and run locally for Standalone mode, ensuring data privacy.
*   **Pricing:** Free tier available for limited agents/features. Open-source self-hosted version is free. Paid cloud plans start around $19/agent/month.
*   **Reputation:** Well-regarded open-source alternative to Intercom/Zendesk, specifically favored for its extensibility and omnichannel capabilities.

## Design Doc
**Integration with OHC:**
*   **Trigger:** User clicks "Connect Social Media" in the OHC Customer Success ("The Ambassador") dashboard.
*   **Action:** OHC handles the OAuth flow with Facebook/Instagram/WhatsApp and provisions a Chatwoot inbox in the background. Messages from these platforms are routed to the unified OHC inbox UI (backed by Chatwoot APIs).
*   **User Interface:** The business owner sees a simple "Messages" tab in their OHC mobile or web app. They read and reply there; OHC handles sending the reply back through the correct social channel via Chatwoot.
*   **AI Agent Synergy:** "The Ambassador" AI agent can monitor incoming Chatwoot messages, generate draft responses, or auto-reply to common questions (e.g., "Do you do vegan cakes?").

## Implementation Prompt
Integrate Chatwoot to power a unified inbox feature for OHC users.
1.  Add a "Unified Inbox" screen to the UI that allows users to connect their Instagram and WhatsApp accounts.
2.  When connected, incoming messages from these platforms should appear in this inbox.
3.  Users must be able to reply to messages directly from the OHC app, and the response should be delivered to the customer on their original platform (e.g., Instagram DM).
4.  Ensure the setup process abstracts away technical complexities (like setting up webhooks manually).

## Priority
P0 (Critical) - Communication is essential for sales and support.

## Estimated Scope
Large
