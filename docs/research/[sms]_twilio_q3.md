## [SMS & Notifications] Issue Brief: Twilio Integration for Global SMS

**Title**: Scout 🔍: Integrate Twilio for Reliable Order and Marketing SMS
**Problem Statement**:
Many customers prefer text messages for order updates, and SMS marketing has higher open rates than email. Additionally, non-technical or low-English proficiency users often rely heavily on SMS for business operations.
**Research Report**:
- **Tool**: Twilio
- **Evaluation**: The industry standard for programmatic SMS. High deliverability and global reach.
- **Ease of Use**: Requires the user to have a Twilio account and manage phone numbers, which can be slightly technical.
- **Pricing**: Per-message pricing, varies significantly by country.
- **Cloud vs. Standalone**: Works in both. In Cloud, OHC could abstract the Twilio account and bill the user. In Standalone, the user must provide their own API keys.
**Design Doc**:
- User enters their Twilio API credentials and selected phone number in settings.
- OHC uses the Twilio API to send automated notifications (e.g., "Your order has shipped").
- Two-way SMS can be routed back to the OHC unified inbox via Twilio webhooks.
**Implementation Prompt**:
Integrate the Twilio SMS API. Provide a secure settings page for users to input their credentials. Implement notification triggers for key events (order confirmation, shipping update). Allow users to define custom SMS templates. Support incoming SMS webhooks to route replies to the unified inbox.
**Priority**: P1
**Estimated Scope**: Medium
