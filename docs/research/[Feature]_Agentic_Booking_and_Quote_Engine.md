# [Feature] Agentic Booking and Quote Engine

## Title
Agentic Booking and Quote Engine for Service SMBs

## Problem Statement
Service-based SMBs like Carlos (handyman) and Leo (music tutor) lose leads because they are busy working and cannot instantly reply to inquiries with quotes and available booking slots. They find integrating third-party scheduling apps with their websites complex and expensive.

## Research Report
Based on audits of Shopify and Wix, service scheduling is treated as a secondary feature requiring complex app integrations (e.g., Calendly via Zapier). User sentiment reveals immense frustration with "app fatigue" and lost sales from delayed communication. OHC's opportunity is to provide an integrated agent that intercepts inquiries, checks availability, and books clients autonomously.

## Design Doc
- **Core Entities**: Inquiry, Quote, Booking Slot, Service Catalog.
- **Key Relationships**: An Inquiry triggers a Quote generation based on the Service Catalog. An accepted Quote locks a Booking Slot.
- **Integration Points**: Native Unified Inbox, Universal Calendar Ledger.
- **Mobile UX Flow**:
  1. SMB owner receives a push notification: "New Inquiry from John for Plumbing".
  2. The notification includes an AI-drafted response with a quote and a link to 3 available booking slots.
  3. SMB owner taps "Approve & Send".
  4. Client receives SMS, clicks link, selects slot, and pays deposit.
- **AI Integration**: NLP parsing of incoming messages to determine service requested; dynamic calendar querying to find optimal slots.

## Implementation Prompt
Implement the Agentic Booking and Quote Engine that integrates with the Unified Inbox.
- **User-Facing Outcome**: When a customer messages the business inquiring about a service, the system should parse the request, draft a contextual reply including a price estimate (based on past similar jobs or a set catalog) and a list of available calendar slots, and present this to the business owner as a 1-tap approval card.
- **Critical User Journey**: Customer SMS -> Inbox receives message -> AI drafts reply + quote + slots -> Owner approves -> Customer books and pays.
- **Acceptance Criteria**:
  - The system must intercept incoming text/chat inquiries.
  - The AI agent must accurately identify the requested service.
  - The agent must retrieve current availability from the business calendar.
  - The owner must be able to approve the drafted message with a single action on mobile (375px viewport optimized).

## Priority
P0

## Estimated Scope
Large
