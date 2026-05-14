# Global SMS Notifications

**Problem Statement**: Customers don't always check email. SMS is needed for urgent updates (appointments, shipping).

**Research Report**: Twilio is standard but expensive globally. MessageBird or Vonage might be better. Must handle opt-outs strictly. Very useful for low-tech customer bases.

**Design Doc**: SMS provider integration. Trigger rules (e.g., 'Order Shipped'). Twilio API for sending.

**Implementation Prompt**: Integrate Twilio to allow sending automated order and appointment reminders via SMS.

**Priority**: P1
**Estimated Scope**: Medium
