# Scout: Tool Integration Research

## SMS & Notifications
**Title**: Integrate MessageBird for Omnichannel Global Messaging (SMS + WhatsApp)
**Problem Statement**: International business owners find Twilio too complex or US-centric. They need a simple way to send order updates not just via SMS, but seamlessly falling back to WhatsApp, which is the dominant communication channel in LATAM, EMEA, and APAC.
**Research Report**:
- MessageBird (now Bird) excels in omnichannel communication, offering a unified API for SMS, WhatsApp, and even email.
- Its global carrier connections often provide better deliverability and pricing outside the US compared to competitors.
- Pricing: Competitive global SMS rates. WhatsApp messaging is priced per conversation, which is standard.
- Compatibility: Cloud mode can utilize a centralized account with omnichannel routing. Standalone mode requires users to provide their own API credentials.
**Design Doc**:
- Users configure their notification preferences in settings, opting into "Omnichannel Notifications."
- They authenticate with MessageBird and link their WhatsApp Business number.
- When an order is ready for pickup, the Operations agent sends a message. The MessageBird API automatically attempts WhatsApp first; if the user doesn't have WhatsApp or it fails, it falls back to standard SMS.
**Implementation Prompt**: Integrate the MessageBird Omnichannel API. Allow businesses to send automated order status updates (e.g., "Order Confirmed", "Ready for Pickup"). Implement fallback logic to try WhatsApp first, then SMS, providing the best experience for international customers.
**Priority**: P2
**Estimated Scope**: Medium