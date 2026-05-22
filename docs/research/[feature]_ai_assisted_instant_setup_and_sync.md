# [feature] AI-Assisted Instant Setup & Zero-Touch Sync

## Problem Statement
Small business owners (like Maya the baker and Priya the boutique owner) are overwhelmed by the initial setup of traditional platforms like Shopify and Wix. They struggle with complex "Theme Editors," confusing shipping zone logic, and fragmented inventory syncing between their physical Point-of-Sale (POS) and online store. These toolkits act as disjointed software rather than an active employee, resulting in an "Initial Setup Chasm" and constant fear of overselling.

## Research Report
Based on a deep-dive audit of Shopify (including Trustpilot, Reddit, and App Store reviews):
- **73% of negative sentiment** focuses on the learning curve of themes and shipping.
- **Inventory Mismatches:** Users frequently report Shopify POS sync breaking, leading to double-selling and forced refunds.
- **Lost Omnichannel Sales:** Business owners manually copy-paste checkout links into Instagram DMs, losing leads in the process.

For full findings, see the `.agent-task/report/task_output.md` research artifact.

## Design Doc
### High-Level Architecture
- **Onboarding Agent (The Promoter):** A chat-based interface that handles the zero-to-live setup flow.
- **Unified Capacity Mesh:** A global, real-time inventory ledger powered by NATS websockets.
- **Omnichannel Triage Engine:** A service that ingests Instagram DMs, emails, and WhatsApp messages into a single thread.

### Mobile UX Flow (375px first)
1. User opens OHC app and is greeted by a conversational AI.
2. AI asks: "What do you sell?" and "Where are you located?"
3. AI generates a fully populated storefront, drafts local shipping zones, and generates sample inventory.
4. User taps "Approve & Publish."
5. On the "Activity Feed," the user sees real-time inventory locks when a POS transaction occurs, instantly updating the online storefront.

### AI Agent Integration Points
- **Setup Generation:** The Onboarding Agent translates natural language intent into structured system configurations (shipping, taxes, layout).
- **Triage Parsing:** The Omnichannel Agent parses incoming DMs to extract intent (e.g., "I want to buy 2 cookies") and proactively generates an invoice draft for the user to 1-tap approve.

## Implementation Prompt
Implement a conversational onboarding flow that takes a user from zero to a fully published store in under 10 minutes without exposing them to traditional settings menus. Create a unified real-time synchronization mesh that locks inventory instantly across online and POS channels to prevent overselling. Provide an omnichannel inbox view where DMs automatically trigger draft invoice suggestions. Ensure the entire experience is fully manageable from a mobile device (375px). Let the implementer define the exact API endpoints and database schemas required to support these flows.

## Priority
P0

## Estimated Scope
Large
