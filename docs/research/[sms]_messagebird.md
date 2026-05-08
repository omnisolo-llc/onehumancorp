## [SMS] MessageBird Integration
**Title**: Integrate MessageBird for Global SMS Notifications
**Problem Statement**: Business owners interacting with customers who don't use email (or have low English proficiency, like Fatima) need reliable SMS messaging for appointment reminders and order updates.
**Research Report**:
- **Tool**: MessageBird (now Bird)
- **Target Persona**: Fatima (Local Service Provider), Global SMBs
- **Advantages**: Strong international coverage, competitive pricing outside the US, supports WhatsApp as well.
- **Risks**: Deliverability regulations vary wildly by country.
- **Pricing**: Pay per message (varies by destination country).
- **Compatibility**: Cloud (API Keys). Standalone (API Keys).
**Design Doc**:
- User configures their sender ID in OHC.
- OHC orchestration triggers SMS notifications for key events (Order Ready, Appointment Reminder).
- Agents can send ad-hoc SMS messages to customers via the unified inbox.
**Implementation Prompt**: Integrate the MessageBird SMS API. Create a unified sending interface that the Operations Agent can call to dispatch SMS alerts. Implement delivery status webhooks to update the OHC database.
**Priority**: P2
**Estimated Scope**: Medium
