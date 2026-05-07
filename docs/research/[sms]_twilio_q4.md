# [SMS] Global Text Alerts with Twilio

**Title**: Implement Twilio for Automated SMS Notifications

**Problem Statement**:
Many customers, particularly in certain demographics or regions, do not regularly check email. Business owners need a way to send critical updates (like appointment reminders or shipping notifications) via SMS to ensure they are seen immediately, reducing no-shows and support queries.

**Research Report**:
- **Evaluated Tools**: Twilio, MessageBird, Plivo.
- **Findings**: Twilio is the undisputed industry leader for SMS APIs, offering unparalleled global reach and reliability. MessageBird is a strong alternative (especially in Europe), but Twilio's extensive documentation and ubiquity make it the safest choice for a robust integration.
- **Ease of Use**: The business owner simply toggles "Enable SMS Alerts" in the UI. All complex telecom routing is handled by Twilio via the OHC backend.
- **Pricing**: Very affordable for low-volume transactional use (e.g., $0.0079 per message in the US). Small business owners can pre-load a small balance or be billed directly by OHC.
- **Cloud vs Standalone**: In Cloud mode, OHC manages the primary Twilio account and bills tenants. In Standalone, users provide their own Twilio Account SID and Auth Token.

**Design Doc**:
- **Trigger**: An event occurs in OHC (e.g., a meeting is booked, or an order is shipped).
- **Action**: OHC checks if the customer has a valid phone number and if SMS notifications are enabled. It then calls the Twilio API to send a templated message.
- **User View**: In the "Automations" or "Settings" tab, the business owner sees toggles for "Send SMS Appointment Reminders" and "Send SMS Shipping Updates". There is no complex logic to configure.

**Implementation Prompt**:
Add an SMS notification system powered by an integration like Twilio. Provide a simple settings UI where users can toggle SMS notifications on or off for specific system events (e.g., Appointment Reminders). When these events occur, the system must automatically dispatch the SMS to the relevant customer's phone number. Ensure there is a basic log view so the business owner can verify messages were sent.

**Priority**: P1
**Estimated Scope**: Medium
