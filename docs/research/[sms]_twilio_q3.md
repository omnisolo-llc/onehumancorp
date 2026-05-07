# Title: Implement Reliable SMS via Twilio

**Problem Statement:** Fatima (Food Cart) and her customers need instant, reliable SMS alerts for order readiness, as she doesn't use email while operating her cart.

**Research Report:** Twilio is the industry leader for programmable SMS and Voice, offering 99.99% uptime and <500ms latency globally. Plivo is also a strong alternative, but Twilio's extensive docs and enterprise scale make it the top choice.

**Design Doc:** Integrate a unified messaging service. Automatically trigger SMS dispatches for critical order status updates (like "Ready for Pickup") to keep customers informed without requiring internet access.

**Implementation Prompt:** Add automated SMS notifications to the order flow. When an order status changes to "Ready for Pickup", use Twilio to text the customer.

**Priority:** P0

**Estimated Scope:** Small
