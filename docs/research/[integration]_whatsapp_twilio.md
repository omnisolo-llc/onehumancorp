# 🔍 Scout: Tool Integration Research - WhatsApp via Twilio

**Title**: Implement WhatsApp Business Messaging via Twilio Integration

**Problem Statement**:
Small business owners often struggle to maintain communication with their customers, who are increasingly relying on messaging apps rather than email. Carlos (the tech-savvy restaurateur) and Maya (the boutique owner) need a fast, reliable, and familiar way to send order updates, appointment reminders, and quick support responses directly to their customers' phones. Relying solely on email means missed messages, slower response times, and lower customer satisfaction. WhatsApp is globally ubiquitous, and a seamless integration would allow these business owners to reach their customers where they already spend their time.

**Research Report**:
- **Tool**: Twilio API for WhatsApp
- **Market Context**: WhatsApp Business is a highly requested feature across e-commerce platforms (like Shopify and Wix) for transactional updates and support. Competitors often offer SMS/WhatsApp add-ons that are highly rated.
- **Ease of Use (Non-Technical User)**: The business owner simply authenticates their WhatsApp Business account via OHC. Once linked, the system handles the rest. They don't need to understand API keys; they just see "Connect WhatsApp" and can start configuring message templates (e.g., "Order Shipped").
- **Pricing**: Twilio offers a pay-as-you-go model (per conversation), which is highly scalable for small businesses. There is a free tier for testing, and conversation costs are relatively low (varies by region, typically a few cents per conversation). This is viable for a SaaS model where costs can be passed on or absorbed in a premium tier.
- **Reputation**: Twilio is the industry standard for communications APIs. It offers robust documentation, high reliability, and excellent support.
- **SaaS Viability**: Twilio's API is fully capable of operating in both Cloud (multi-tenant) and Standalone modes. In Cloud mode, OHC would manage the master account and sub-accounts for tenants. In Standalone mode, the local user can provide their own API credentials if needed.

**Design Doc**:
- **Integration Point**: The integration will live in the OHC integrations dashboard as a "WhatsApp Messaging" card.
- **Trigger**: System events such as order confirmation, shipping updates, or appointment bookings will trigger predefined messaging workflows.
- **Action**: OHC will dispatch the appropriate pre-approved template message via the Twilio API to the customer's phone number on file.
- **User Experience**: The business owner will go to Settings > Integrations, click "Connect WhatsApp," and go through an OAuth/linking flow. They will then have a simple interface to toggle which notifications (e.g., "Order Confirmed") should be sent via WhatsApp.

**Implementation Prompt**:
- Integrate Twilio's WhatsApp API to allow businesses to send transactional messages (like order updates) to customers.
- Create an intuitive UI in the Integrations settings for the business owner to connect their WhatsApp account without needing to handle API keys manually.
- Provide a simple toggle interface for owners to select which system events (e.g., New Order, Shipped) trigger a WhatsApp notification.
- Ensure the integration handles errors gracefully (e.g., invalid phone numbers) and provides clear status logs to the business owner.

**Priority**: P1 (High)
**Estimated Scope**: Medium
