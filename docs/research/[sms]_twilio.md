# Twilio - SMS & Notifications

## Problem Statement
SMS is critical for reaching customers instantly, especially for appointment reminders or urgent updates, and is often preferred by demographics with lower email usage.

## Research Report
Twilio is the industry standard for programmatic SMS and voice communication.
- **Ease of Use for SMBs**: High (from the UI perspective). OHC handles all the API complexity.
- **Pricing**: Pay-per-message. Can become expensive at high volumes, but reasonable for typical SMB use cases.
- **Reputation**: Highly reliable globally.
- **Competitive Analysis**: The most robust global network, despite complex regulatory compliance (like 10DLC in the US).

## Design Doc
**Trigger**: A high-priority event occurs (e.g., upcoming appointment) or business owner sends an SMS blast.
**Actions**:
- OHC formats the message and sends it via Twilio API.
- Twilio delivers the SMS to the customer's phone.
**User Experience**: Business owner can configure SMS notifications in settings or send direct SMS messages to customers from the CRM.

## Implementation Prompt
**User-facing Outcome**: A business owner can reliably send SMS notifications and messages to their customers globally.
**Acceptance Criteria**:
- System can send automated SMS reminders (e.g., for appointments).
- Business owner can send manual SMS messages to customers.
- OHC handles necessary compliance and opt-out logic (STOP messages).

## Priority
P1 (High)

## Estimated Scope
Medium
