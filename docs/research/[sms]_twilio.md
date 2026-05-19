## 2. SMS & Notifications: Twilio
**Problem Statement:** Many small businesses (especially service-oriented ones like hair salons or repair shops) serve clients who prefer text messages over emails. Relying on personal phone numbers is unscalable and unprofessional, and missing an appointment reminder costs money.
**Research Report:**
- **Tool:** Twilio
- **Persona Benefit:** Enables automated, reliable text reminders and notifications to customers' phones, ensuring higher appointment attendance and better communication.
- **Key Advantages:** Industry standard, highly reliable, global reach, and pay-as-you-go pricing.
- **Risks:** Requires careful handling of opt-out compliance (A2P 10DLC regulations in the US) which can be confusing for small business owners.
- **Pricing:** Pay-as-you-go (approx. $0.0079 per SMS in the US).
- **Environment:** Cloud (API based).
**Design Doc:**
- **Trigger:** An appointment is booked, or an order is ready for pickup.
- **Action:** An automated SMS is dispatched to the customer's provided phone number.
- **User View:** The business owner sees a log of sent messages and delivery statuses in their customer's timeline.
**Implementation Prompt:** Integrate Twilio API to allow the platform to send automated SMS notifications for key events (e.g., appointment reminders). The business owner should be able to toggle these notifications on or off in their settings.
**Priority:** P0
**Estimated Scope:** Large
