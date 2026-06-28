issue_title: "[research] Architect Omnichannel Unified Customer Memory Graph"
issue_description: |
  # Research Report: Omnichannel Unified Customer Memory Graph

  ## 1. Problem Statement
  Small business owners (e.g., Priya the Boutique Operator, Carlos the Field Service Owner) engage with customers across multiple disjointed channels: Instagram DMs, SMS, in-store POS (tap-to-pay), online purchases, and booking requests. Currently, OHC handles these as separate transactional events. Without a unified memory structure, AI agents cannot provide context-aware support. For example, if a customer complains via WhatsApp about a missing delivery, the Customer Success Agent lacks immediate awareness that the customer just made an in-store POS purchase an hour ago, leading to confusing and disjointed automated responses. The business owner has to manually connect the dots.

  ## 2. Research Report
  - **Market Context**: Platforms like Shopify rely on monolithic CRM backends, while POS systems like Square treat customer profiles as static entities. Neither builds a temporal, event-driven graph that AI agents can easily query for context.
  - **The OHC Opportunity**: By establishing a "Unified Customer Memory Graph," every interaction—from a casual Instagram DM to a high-value physical purchase—is mapped to a single unified customer identity. This allows the AI to "remember" the customer's entire journey and act intelligently.
  - **Competitor Gaps**:
    - *Shopify*: Good customer profiles, but poor integration of conversational data (DMs, SMS) into the central memory for autonomous agents to use.
    - *Square*: Strong POS customer tracking, but weak online and conversational memory integration.
    - *HubSpot*: Excellent CRM, but far too complex and manual for non-technical small business operators like our personas.

  ## 3. Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Incoming Events: POS, DM, Web Order, Booking] --> B(Identity Resolution Engine)
      B --> C{Unified Customer Identity}
      C --> D[Temporal Event Graph]
      D --> E[Vector DB for Semantic Search]
      D --> F[Relational DB for Hard State]
      E --> G[Customer Success Agent]
      F --> G
      G --> H[Context-Aware Auto-Reply & Action]
  ```

  ### Data Model & Sync Protocol
  - **Identity Resolution**: Implement probabilistic and deterministic matching (e.g., linking a phone number from a POS receipt to an Instagram handle if they share the same email).
  - **Temporal Event Graph**: Store events as a time-series graph where nodes are entities (Customer, Product, Agent) and edges are interactions (Purchased, Sent_Message, Booked).
  - **Vector Embedding**: Each customer's holistic history is periodically summarized and embedded to allow agents to query: "What is the general sentiment of this customer and what did they last buy?"

  ### AI Agent Coordination
  - **Customer Success Agent ("The Ambassador")**: Queries the Memory Graph before drafting any reply. If Carlos gets an SMS from "John," the agent knows John had a repair job yesterday and automatically drafts a reply asking if the repair is holding up.
  - **Marketing Agent ("The Promoter")**: Uses the Memory Graph to identify VIP customers who engage across multiple channels and generates highly targeted retention campaigns.

  ### Mobile-First Implementation
  - Create a 375px mobile "Customer Context Card." When a business owner views a conversation, they see a clean, unified timeline showing "Bought in store (2 days ago)," "Sent DM (Yesterday)," "Current Order Status: Shipped." Touch targets for quick actions (e.g., "Draft Reply," "Issue Refund") must be ≥ 44x44px.

  ## 4. Implementation Prompt
  **Feature Name**: Omnichannel Unified Customer Memory Graph
  **Target Persona**: Priya (Boutique Operator)
  **Outcome**: Priya receives an Instagram DM from a customer. The OHC Mobile App instantly shows a unified context card indicating this customer bought a dress in-store yesterday. The AI drafts a contextually relevant reply referencing the recent purchase.

  **Next Actions for Engineering**:
  1. Design and implement the `CustomerIdentity` and `InteractionEvent` database schemas in PostgreSQL, ensuring strict multi-tenant isolation.
  2. Build the Identity Resolution Service to merge fragmented profiles (e.g., phone number matching email).
  3. Integrate the Memory Graph into the `CustomerSuccessAgent` so it automatically queries recent interactions before generating a draft reply.
  4. Build the Mobile-First (375px) "Unified Timeline" UI component for the business owner dashboard.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
