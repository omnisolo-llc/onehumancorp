## [SMS] MessageBird Integration
**Title**: Integrate MessageBird for SMS Notifications
**Problem Statement**: Need for reliable, urgent customer notifications (e.g., order ready, appointment reminder) globally.
**Research Report**:
- **Tool**: MessageBird
- **Target Persona**: Businesses with international customer bases
- **Advantages**: Global reach and competitive pricing make it a strong alternative to Twilio for international SMS delivery. Essential for diverse customer bases.
- **Risks**: Managing phone number formatting and regional sender ID regulations.
- **Pricing**: Standard MessageBird per-SMS pricing.
- **Compatibility**: Cloud, Standalone (via API).
**Design Doc**:
- User provides API keys to connect MessageBird.
- OHC integrates MessageBird API to send outbound SMS.
- Triggers are set up for automated SMS sending (e.g., appointment reminders, order updates).
**Implementation Prompt**: Integrate the MessageBird API to handle automated SMS triggers. Ensure the integration supports global phone number formats and handles delivery status webhooks to update the message status in the OHC dashboard.
**Priority**: P2
**Estimated Scope**: Medium
