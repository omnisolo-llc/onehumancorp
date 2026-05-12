# [SMS & Notifications] Twilio SMS

**Problem Statement**: Businesses need to reach customers who don't regularly check email (e.g., for appointment reminders or urgent updates). They need a way to send text messages directly from their management dashboard.

**Research Report**:
- **Target Persona**: Local services, salons, clinics, retail.
- **Ease of Use**: Twilio is a developer tool, so OHC must abstract it completely. The business owner should just type a message and hit send.
- **Pricing**: Pay-per-message (fractions of a cent to a few cents depending on country). OHC might need a billing mechanism or allow users to bring their own Twilio keys (though BYOK is complex for grandmas).
- **Reputation**: Industry standard, extremely reliable global coverage.
- **Cloud/Standalone**: API-based, works in both.

**Design Doc**:
- **Trigger**: Business owner types an SMS in the unified inbox or an automated reminder is triggered.
- **Action**: OHC sends the payload to Twilio API.
- **User View**: SMS appears as just another channel in the unified inbox. Owner can select "SMS" when sending a message to a customer.

**Implementation Prompt**: Enable sending and receiving SMS messages via Twilio. Surface SMS as a communication channel in the unified inbox.

**Priority**: P2
**Estimated Scope**: Medium
