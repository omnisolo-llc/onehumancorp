# Invisible AI Customer Support

**Priority:** P1
**Scope:** Large

## Problem Statement
Carlos (handyman) misses leads because he can't answer messages while working. Customers expect instant replies.

## Research Report
- **Shopify/Wix:** Rely on basic chatbots or manual Inbox management.
- **Data:** 82% of consumers expect immediate responses to sales inquiries.
- **Conclusion:** An agent that can answer FAQs and schedule appointments automatically will capture lost revenue for service businesses.

## Design Doc
- **Architecture:** `MessageReceiver` feeds into `SupportAgent`. Agent accesses `BusinessContext` to answer questions or trigger `BookingAction`.
- **UX Flow:** Customer messages via SMS/WhatsApp. Agent replies. Owner sees transcript in "Inbox" with an "AI Handled" badge.

## Implementation Prompt
Implement a unified inbox backed by an LLM agent that can autonomously respond to customer inquiries based on the business's saved context (hours, pricing, services). Acceptance Criteria: The agent must correctly answer 3 common questions without owner intervention and escalate complex issues.
