# [Feature Gap] AI-Managed Omnichannel Inbox

## Title
The AI Auto-Responder & Booking Agent

## Problem Statement
Small service business owners like **Carlos (handyman)** and **Leo (music tutor)** lose revenue because they are busy *doing* the work and cannot instantly reply to inquiries. Leads message via Instagram, SMS, or email. If Carlos doesn't reply within an hour, the lead moves to another handyman. Managing multiple inboxes manually is chaotic and stressful.

## Research Report
- **Competitor Landscape:**
  - *Shopify:* Has "Shopify Inbox", which is a unified inbox, but relies heavily on manual replies or rigid, pre-programmed FAQs.
  - *Wix:* Has a unified inbox, but lacks autonomous conversational booking capabilities.
- **User Pain Points:**
  - "I missed a booking because I didn't see the email."
  - "Managing Instagram DMs and emails is overwhelming."
- **Market Opportunity:** Transforming the inbox from a "place to read messages" into an "agent that secures bookings" directly impacts the bottom line of service businesses.

## Design Doc
- **High-Level Architecture:**
  - Ingestion webhooks for Instagram DMs, Email, and SMS.
  - A unified internal messaging schema.
  - An LLM-powered Auto-Responder Agent that reads incoming messages, determines intent (e.g., FAQ, pricing inquiry, booking request), and formulates a response based on the business's context (availability, pricing catalog).
  - A UI for the business owner to review agent conversations and take over if necessary.
- **UI Wireframes / Screen Flow (Mobile 375px):**
  1. **Inbox View:** A unified list of threads. Threads handled successfully by the AI have a subtle "AI Handled" badge. Threads needing attention are pinned to the top.
  2. **Thread View:** Standard chat interface. AI replies are visually distinct.
  3. **Intervention Action:** User can type a message at any time, instantly pausing the AI for that specific thread.
- **AI Agent Integration Points:**
  - The Auto-Responder Agent requires access to the business's schedule (for bookings) and catalog (for pricing).

## Implementation Prompt
**User-Facing Outcome:** When a potential customer DM's Carlos on Instagram asking, "Are you free to fix a leaky pipe this Thursday?", the OHC AI Agent immediately replies: "Hi! Yes, Carlos has an opening this Thursday at 2 PM. The standard rate is $100/hr. Would you like me to lock that in for you?" If the customer says yes, the agent books the slot and notifies Carlos.
**Critical User Journey (CUJ):**
1. Customer sends a message on a connected channel.
2. AI Agent interprets the message and replies contextually.
3. AI Agent successfully drives the conversation to a booking or answers the FAQ.
4. Business owner receives a summary notification ("New booking secured via Instagram").
**Acceptance Criteria:**
- The agent must be able to understand natural language inquiries about availability and pricing.
- The agent must be able to execute a booking action on behalf of the customer.
- The business owner must have clear visibility into what the agent is saying and the ability to intervene instantly.

## Priority
P1

## Estimated Scope
Large
