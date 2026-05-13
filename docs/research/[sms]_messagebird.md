# Title: MessageBird Integration for Global SMS Notifications

## Problem Statement
Businesses need to send time-sensitive alerts (e.g., appointment reminders, order pickups) to customers. Email is often ignored. SMS is critical, especially for low-English-proficiency users or international markets where WhatsApp/SMS dominate. Business owners need a reliable way to send SMS without navigating telecom regulations manually.

## Research Report
**Market Analysis & Pain Points:**
- **Friction:** Setting up SMS gateways, handling opt-outs (STOP), and managing sender IDs is too technical for most SMBs.
- **Competitors:** Twilio is the developer standard, but MessageBird (now Bird) offers better out-of-the-box omnichannel tools and often better international routing.
- **MessageBird API:** Simple REST API for sending SMS, handling replies, and managing contacts.
- **Reputation & Ease of Use:** Known for strong global carrier connections.
- **Pricing:** Pay-as-you-go, competitive with Twilio.

**Key Advantages:**
- High deliverability globally.
- Omni-channel capabilities (can easily upgrade to WhatsApp later).

**Integration Risks:**
- Strict telecom compliance (e.g., A2P 10DLC in the US) requires businesses to register their brands, which is a massive UX hurdle.

**Environment Support:**
- **Cloud:** Full support.
- **Standalone:** Full support.

## Design Doc
**Trigger:**
User configures SMS notifications in "Settings". They either use a shared OHC sender ID or connect their own MessageBird account for a dedicated number.

**Action:**
OHC triggers automated SMS messages (e.g., "Your order is ready!") via the MessageBird API based on system events.

**User View:**
The business owner sees simple toggle switches: "Send SMS on Order Confirmation", "Send SMS Reminder 24h before appointment". They do not need to write code. They also see a log of sent messages and delivery statuses in the customer's timeline.

## Implementation Prompt
Implement automated SMS notifications via MessageBird.
- Build the integration to the MessageBird SMS API.
- Create a UI in the settings for businesses to toggle automated SMS notifications for key events (Order Placed, Appointment Reminder).
- Ensure opt-out (STOP) handling is gracefully managed to prevent sending to unsubscribed numbers.
- Display SMS delivery status (Sent, Delivered, Failed) in the customer activity timeline.
- (Do not prescribe specific database schemas; focus on the business logic and user-facing toggles.)

## Priority
P2

## Estimated Scope
Medium
