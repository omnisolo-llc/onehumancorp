# [SMS & Notifications] Global Transactional SMS via Twilio

**Title**: Implement Global Transactional SMS Notifications via Twilio

**Problem Statement**:
Business owners like Fatima (The Food Cart Operator) and her customers are not always looking at their email. When a customer places a pre-order for pickup, Fatima needs an immediate, loud notification on her phone (which may not always have a strong data connection for push notifications). Similarly, her customers expect a text message saying "Your food is ready for pickup!". SMS remains the most reliable, universally understood communication method for urgent transactional updates.

**Research Report**:
I evaluated Twilio, MessageBird, and AWS SNS for SMS delivery.
- **Twilio**: The undisputed industry leader in global SMS delivery. Extremely high reliability, massive global carrier network, and robust handling of international number formatting (E.164) and opt-out compliance (STOP messages). Pricing is straightforward.
- **MessageBird**: Also excellent (and chosen for our Omnichannel Inbox), but Twilio's raw SMS deliverability in certain emerging markets is often cited as slightly superior, and their API specifically for pure transactional SMS is deeply battle-tested. (Note: Using MessageBird for *both* Inbox and SMS is also a valid architectural choice to reduce vendor sprawl, but Twilio is the gold standard for SMS).
- **AWS SNS**: Difficult to configure for two-way communication or handling opt-outs gracefully. Not developer-friendly for this specific use case.
- **Conclusion**: Twilio is the safest choice for guaranteed delivery of critical alerts (like "New Order"), especially in low-data environments.

**Design Doc**:
- **Integration Point**: Resides within the "Operations" (The Manager) and "Customer Success" (The Ambassador) departments.
- **Triggers & Flow**:
  1. A customer places an order on Fatima's OHC site.
  2. The OHC backend immediately dispatches an SMS via Twilio to Fatima's phone: "New order! 2x Falafel. Reply 1 to Accept."
  3. Fatima replies "1" (or clicks a link).
  4. Twilio webhook hits OHC, updating the order status to "Preparing".
  5. The "Ambassador" dispatches an SMS to the customer: "Fatima is preparing your order! We'll text you when it's ready."
- **User View**: Business owner receives standard text messages for critical alerts. Customers receive branded text messages for order updates. A simple toggle in OHC settings: "Send me an SMS for new orders."

**Implementation Prompt**:
Integrate the Twilio SMS API for transactional notifications. Implement a robust notification system that triggers SMS alerts to the business owner for critical events (e.g., new orders, cancellations) based on their notification preferences. Implement an outward-facing SMS flow to update customers on their order status (e.g., "Order Confirmed", "Ready for Pickup"). Ensure all phone numbers are properly validated and formatted to E.164 standard before sending. The system must gracefully handle SMS delivery failures and provide fallback mechanisms (like push notifications or email).

**Priority**: P1
**Estimated Scope**: Medium
