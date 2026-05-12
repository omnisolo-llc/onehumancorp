# Issue Brief: Omnichannel Autonomous Support Agent for DM Management

## Problem Statement
Small business owners (like Carlos, 42, a busy handyman) lose significant inbound leads and revenue because they cannot continuously monitor Instagram DMs, Facebook Messenger, WhatsApp, and SMS simultaneously while actively working on job sites. Manual replies are slow, disjointed, and prone to error, causing potential customers to seek immediate answers from competitors.

## Research Report
Extensive Reddit threads in r/smallbusiness and r/sweatystartup frequently mention 'losing track of messages' as a top-3 daily stressor. Shopify offers 'Shopify Inbox', but it fundamentally requires manual monitoring and intervention. An invisible, autonomous AI agent that can auto-reply to common FAQs (business hours, standard pricing, service availability) and logically funnel users to a unified booking or checkout link would recover an estimated 15-20% of lost revenue for service-based SMBs.

Research shows that customers expect a response on social media within 60 minutes. Failure to meet this SLA results in a 50% drop in conversion probability. By integrating directly with Meta's Graph API and Twilio, OHC can provide an 'always-on' front desk for micro-businesses.

## Design Doc
**High-Level Architecture & Entities:**
- `MessageThread`: Unified representation of a conversation regardless of origin platform.
- `CustomerContact`: Unified CRM profile linked to threads.
- Webhook Listeners: Infrastructure to ingest events from Meta API (Instagram/WhatsApp) and Twilio (SMS).
- AI Agent Orchestrator: Determines intent (FAQ vs. Booking vs. Complex/Escalate to Human).

**Mobile UX Flow:**
1. **Inbox View:** A unified inbox showing messages from all platforms. Messages handled by AI have a distinct "handled" badge.
2. **Agent Configuration:** A simple settings screen where the owner sets the 'Agent Autonomy Level' (e.g., 'Draft only', 'Auto-reply to FAQs', 'Full autonomy for bookings').

**AI Agent Integration Points:**
- Agent ingests incoming webhook payload, retrieves the business's context (catalog, hours, active calendar).
- Agent generates a context-aware response.
- Agent decides whether to dispatch the response immediately or queue it for human review based on configuration.

## Implementation Prompt
Build an AI-driven, unified inbox aggregator that automatically drafts and sends replies for standard customer inquiries based on the business's real-time configuration, catalog, and availability.

**Critical User Journey (CUJ):**
1. Customer sends a DM on Instagram: "How much to fix a cracked iPhone screen?"
2. OHC Webhook listener receives message.
3. AI Agent queries the product catalog, finds "iPhone Screen Repair", and formulates reply: "Hi! Screen repairs start at $99. Would you like me to send a booking link?"
4. Customer replies "Yes". Agent sends OHC booking link.
5. Business owner receives a single push notification only when the booking is confirmed and paid.

**Acceptance Criteria:**
- The system must successfully intercept a mock webhook message from a simulated social platform.
- The AI orchestrator must correctly query the internal catalog and formulate an accurate response without hallucinating prices.
- The feature must include a simple toggle for the business owner: 'Let AI handle basic questions'.
- Threads must seamlessly support human intervention (taking over from the bot).

## Priority
P0

## Estimated Scope
Large
