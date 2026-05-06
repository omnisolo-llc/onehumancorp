# [SMS and Notifications] Twilio Integration

**Title**: Integrate Twilio for reliable SMS notifications and client communications

**Problem Statement**: Users like Fatima, who have low English proficiency or serve local communities, rely heavily on SMS. They need a way to send appointment reminders, pickup notifications, or promotional texts directly from their business number without using their personal phone plan.

**Research Report**: Twilio is a premier cloud communications platform providing programmable SMS, Voice, and WhatsApp APIs.
- **Ease of use**: End-users do not interact with Twilio directly; they just use the OHC interface. Twilio handles the complex carrier routing.
- **Pricing**: Pay-as-you-go. Extremely cheap (fractions of a cent per message in the US, varying globally).
- **Reputation**: Gold standard for programmable communications. High deliverability and compliance tools.
- **Cloud/Standalone**: Cloud mode works natively. Standalone mode requires internet access to hit Twilio APIs.

**Design Doc**:
- **Trigger**: System events (e.g., an order is ready, an appointment is tomorrow) or manual user action (typing an SMS in the inbox).
- **Action**: OHC calls the Twilio API to send the SMS from a dedicated business number provisioned for the user.
- **User Experience**: The business owner sees a simple "Send Text" button in the customer's profile. Automated reminders are toggled via simple checkboxes (e.g., "Send SMS reminder 24h before").

**Implementation Prompt**: Integrate an SMS sending capability. Give the business owner a simple toggle to enable "Automated Appointment Reminders via SMS" and a text box to send manual SMS messages directly to a client's phone number.

**Priority**: P1
**Estimated Scope**: Medium