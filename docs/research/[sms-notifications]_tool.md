# [sms-notifications] Global SMS Customer Notifications

**Title:** Integrate SMS Notifications for Appointments and Orders

**Problem Statement:**
Email notifications often go unread or end up in spam. For critical updates—like appointment reminders for a salon or delivery updates for a local shop—small business owners need to reach customers via SMS. This is especially crucial for demographics or regions with low email usage (like Fatima's use case).

**Research Report:**
* **Tools Evaluated:** Twilio, MessageBird, Vonage.
* **Ease of Use:** Twilio provides robust APIs but requires strict compliance registration (like A2P 10DLC in the US), which can be daunting for small business owners. OHC should abstract this compliance layer where possible.
* **Key Advantages:**
  - Massive increase in read rates and immediate customer action.
  - Global carrier coverage.
  - Reduces no-show rates for appointments dramatically.
* **Risks:**
  - Compliance (opt-in requirements, STOP handlers).
  - Cost per message can add up quickly, especially internationally.
* **Pricing Estimate:** ~$0.0079 to $0.05 per message depending on the destination country.
* **Environment Support:** fully supported in Cloud mode. Standalone mode can connect to the API via external network requests.

**Design Doc:**
* **Trigger:** The business owner enables "SMS Notifications" in their settings and purchases an SMS credit bundle.
* **Actions:** OHC automatically sends predefined SMS templates triggered by specific events (e.g., `appointment.confirmed`, `order.shipped`).
* **User Experience:** The owner toggles on the feature and doesn't have to think about it again. Customers receive a clean, branded text message with a link to their order or appointment details.

**Implementation Prompt:**
Integrate an SMS delivery provider (like Twilio) to support automated transactional messages. Implement a settings panel where merchants can enable SMS alerts for specific events (e.g., Order Confirmation, Appointment Reminder). Ensure the system handles opt-outs automatically and tracks message delivery status. Expose an interface for merchants to view their SMS credit usage.

**Priority:** P0
**Estimated Scope:** Medium