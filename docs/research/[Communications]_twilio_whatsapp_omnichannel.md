## [Communications] Twilio WhatsApp Business API Integration

**Title**: Enable Real-time Customer Engagement via Twilio WhatsApp Integration

**Problem Statement**:
Small business owners, such as Maya and Carlos, suffer from "Communication Lag" and "Operational Fatigue." They lose sales and customer trust because direct messages (DMs) aren't answered instantly, especially after hours. They end up managing fragmented conversations across 3+ different apps (Instagram, SMS, Email), resulting in a never-ending inbox and slow response times. They need a unified way to communicate with customers right where the customers are most active—WhatsApp—without switching between disjointed platforms.

**Research Report**:
- **Market Demand:** WhatsApp is the dominant messaging app globally (over 2B users), and customers increasingly expect to communicate with businesses there rather than via email.
- **Competitor Landscape:** Shopify and Wix both have robust app marketplaces featuring prominent WhatsApp integrations (e.g., WhatsApp Chat, Zoko).
- **Tool Capabilities (Twilio):** Twilio provides a highly reliable, scalable API for WhatsApp Business. It allows for rich media, automated templates (like order confirmations and appointment reminders), and session-based conversational messaging.
- **SaaS Viability:**
  - *Cloud (Multi-tenant):* Can be seamlessly integrated via OHC's master Twilio account, abstracting setup complexity from the user.
  - *Standalone (Local/Private):* Can support user-supplied API keys and Webhook configurations for privacy-focused deployments.
- **Pricing:** Twilio offers transparent, pay-as-you-go pricing based on conversation categories (Marketing, Utility, Authentication, Service), which is highly favorable for the SMB free-tier or base subscription models.
- **Ease of Use:** From the SMB's perspective, this should act like a magic inbox feature, removing the technical jargon of APIs, webhooks, or session windows.

**Design Doc**:
- **Trigger/Setup:** In the "Communications" or "Inbox" settings of the OHC dashboard, users toggle "WhatsApp Chat" on. In Cloud mode, they simply connect their business number. In Standalone mode, they are prompted for Twilio credentials.
- **User Experience (SMB Owner):**
  - The SMB owner sees a unified "Omnichannel Inbox" in their OHC dashboard or mobile app.
  - WhatsApp messages appear alongside emails or web chats.
  - They can reply manually from the dashboard or rely on OHC's Proactive Agents to auto-reply to common questions (e.g., hours, location) instantly.
- **Customer Experience:** Website visitors see a WhatsApp chat widget on the storefront and can initiate conversations that flow directly into the SMB's OHC inbox.
- **Actions:**
  - Automated dispatch of order confirmations, appointment reminders, and shipping updates via WhatsApp.
  - Bi-directional chatting from the OHC unified inbox.

**Implementation Prompt**:
Integrate the Twilio WhatsApp Business API to power a unified messaging experience. Ensure that incoming WhatsApp messages flow into the OHC central inbox and that outbound replies (both manual and automated/agentic) route correctly back to the customer's WhatsApp. Create the setup flow where users can easily enable WhatsApp communication (handling both Cloud managed accounts and Standalone custom API key configurations) without dealing with technical jargon. Implement automated triggers for key business events (like order confirmations).

**Priority**: P1

**Estimated Scope**: Large
