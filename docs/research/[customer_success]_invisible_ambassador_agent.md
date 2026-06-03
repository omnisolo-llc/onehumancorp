# [customer_success] Invisible Ambassador Agent

## Problem Statement
Business owners like Maya are overwhelmed by repetitive customer inquiries via Instagram DMs and WhatsApp (e.g., "Do you do vegan cakes?", "Where are you located?"). Answering these takes time away from their core work.

## Research Report
A deep dive into Zendesk and Intercom reviews shows they are too expensive and complex for solopreneurs. Competitors like Shopify offer basic chat, but it's restricted to the website. SMBs need an omnichannel auto-responder that learns from their business data.

## Design Doc
- **Architecture**: Integration with Chatwoot or a similar headless omnichannel inbox. The `Customer Success` agent acts as a middleware, intercepting messages before notifying the owner.
- **Data Model**: Embedded knowledge base (pgvector) populated from the user's business description, FAQs, and product catalog.
- **UI/UX**:
  - Unified Inbox view in the OHC app.
  - Messages that the AI is confident about are auto-drafted and await a 1-tap "Send" approval from the owner.

## Implementation Prompt
Build the Invisible Ambassador agent workflow. Integrate a unified inbox UI that connects to social channels. Implement a background job that uses the LLM to draft replies to incoming messages based on the business's existing data (location, hours, product catalog). Show these drafted replies in the inbox with an "Approve & Send" button.
Ensure the UI passes the Grandmother Test for simplicity.

## Priority
P2

## Estimated Scope
Large
