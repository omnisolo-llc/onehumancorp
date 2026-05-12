# SMS & Notifications Brief

## Problem Statement
Important updates like appointment reminders or delivery notifications often get lost in email inboxes. SMS provides a more immediate and reliable communication channel, especially for users less reliant on email.

## Research Report
**Tool Evaluated:** Twilio
**Findings:** Twilio is the industry leader for reliable, global SMS delivery. It ensures high visibility for critical notifications.
**Pricing:** Pay-as-you-go, usually fractions of a cent per message.
**Ease of Use:** While the API is developer-friendly, the business owner experience depends entirely on how OHC builds the interface.
**Risks:** Costs can scale quickly with volume. Strict regulatory compliance (e.g., A2P 10DLC in the US) can be complex to manage on behalf of small businesses.

## Design Doc
**Trigger:** A critical event occurs (e.g., an upcoming appointment, a shipped order).
**Action:** An automated SMS notification is dispatched to the customer's phone number.
**User Experience:** Business owners can toggle SMS notifications on or off for specific events. They might also have a simple interface to send manual, one-off text updates to customers.

## Implementation Prompt
**Outcome:** The ability for business owners to enable automated SMS notifications for key customer interactions, improving engagement and reducing no-shows.
**Acceptance Criteria:**
- Owner can opt-in to SMS notifications for specific triggers.
- Customers receive timely SMS messages for those triggered events.
- The system handles basic opt-out requests (e.g., "STOP").

## Priority
P1

## Estimated Scope
Medium
