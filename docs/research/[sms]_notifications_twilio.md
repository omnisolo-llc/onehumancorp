**Title**: Global SMS Notifications with Twilio
**Problem Statement**: Many of our users' customers (e.g., Fatima's food cart patrons) may not check email frequently or might have low English proficiency, preferring simple text messages. Businesses need a reliable way to send order updates, appointment reminders, and promotional blasts via SMS globally.
**Research Report**:
- **Twilio**: An American cloud communications company based in San Francisco. It provides programmable communication tools via web APIs.
- **Ease of Use**: Twilio is the industry standard for developer APIs for SMS. For the merchant, it's a matter of provisioning a phone number and letting OHC handle the rest.
- **Pricing**: Pay-as-you-go pricing per SMS segment. It's affordable but can add up at scale. OHC needs a way to pass these costs to the merchant or offer an SMS quota.
- **Reputation**: Extremely high. Tech giant that powers communications for Uber, Airbnb, etc.
- **Cloud/Standalone**: API calls are identical. Cloud mode requires multi-tenant isolation of Twilio accounts or numbers. Standalone mode allows the user to input their own Twilio Account SID and Auth Token directly.
**Design Doc**:
- **Trigger**: An order status changes to "Ready for Pickup" or an appointment is 24 hours away.
- **Action**: OHC backend formats a short, localized text message and calls the Twilio SMS API to deliver it to the customer's verified phone number.
- **UI**: A settings toggle in the OHC dashboard: "Enable SMS Notifications". If enabled, OHC asks the merchant to connect a Twilio account or uses an OHC-managed number (Cloud).
**Implementation Prompt**: Integrate the Twilio SDK. Create an event-driven notification service in OHC that listens for key events (Order Created, Order Ready, Appointment Reminder). When triggered, send an SMS via Twilio. Ensure phone numbers are validated (E.164 format) before sending. Implement an SMS opt-out mechanism to comply with global telecom regulations.
**Priority**: P0
**Estimated Scope**: Medium
