# Mission: Autonomous Customer Support Agent

## Problem Statement
Service providers like Carlos (handyman) and Fatima (food cart) miss leads because they cannot answer messages or calls while actively working. They need a system that handles initial inquiries invisibly.

## Research Report
Many SMBs rely on auto-replies in Instagram or WhatsApp, which are static and often frustrating for customers. Shopify's Sidekick helps the *merchant*, but not the *customer*. High perceived value lies in an agent that can actually converse and convert leads while the owner is busy.

## Design Doc

### High-Level Architecture
- **Entities**: Support Conversation, Customer Inquiry, Knowledge Base (Business Hours, FAQs, Service List).
- **Key Relationships**: Conversations belong to a Store and interact with the Knowledge Base.
- **Integration Points**: LLM integration to handle conversational flow; Webhook/SMS integration for incoming messages.

### Mobile UX Flow (375px first)
1. **Settings**: Simple toggle: "Enable AI Assistant".
2. **Knowledge Base**: Text area: "Tell your assistant important things" (e.g., "I don't work Sundays").
3. **Dashboard Feed**: Inbox shows conversations handled by AI, marked "Resolved" or "Needs your attention".

## Implementation Prompt
Create an autonomous agent service that can intercept incoming customer messages (via SMS or web chat) for a specific OHC store. The agent should use the store's configured business details (hours, location, catalog) to answer common questions and guide the customer toward a purchase or booking. It must elegantly hand off the conversation to the human owner if it encounters a question it cannot confidently answer.

## Priority
P1

## Estimated Scope
Medium
