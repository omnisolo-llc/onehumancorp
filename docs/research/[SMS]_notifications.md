# SMS Notifications Integration

**Problem Statement:**
Many customers and business owners (especially those with low English proficiency or limited computer access) rely heavily on mobile phones. Email updates are often missed, leading to no-shows or confused customers.

**Research Report:**
* **Tool Evaluated:** Twilio Programmable SMS
* **Ease of Use:** The API is robust. The main UX challenge is abstracting A2P 10DLC compliance rules away from the business owner.
* **Pricing:** Pay-as-you-go, very cheap per message.
* **Reputation:** The industry standard for programmatic SMS.
* **Hybrid Context:** Fully supported via REST API in all modes.

**Design Doc:**
* **Trigger:** An appointment is approaching, or an order is ready for pickup.
* **Action:** OHC sends a brief SMS reminder to the customer's phone number.
* **User Experience:** The business owner flips a toggle in settings: "Send SMS Reminders." Customers automatically get a text 24 hours before their booking. The owner doesn't have to manage phone numbers or carriers manually.

**Implementation Prompt:**
Implement an automated SMS notification system. Add a settings toggle for users to enable SMS reminders for appointments and order updates. Ensure the system handles phone number formatting and provides a fallback if the SMS fails to send.

**Priority:** P1
**Estimated Scope:** Medium
