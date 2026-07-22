issue_title: "Unified Omnichannel Agent Architecture"
issue_description: |
  ## Title
  Unified Omnichannel Agent Architecture

  ## Problem Statement
  Currently, incoming signals and messages (e.g. from Instagram, Webhooks, WhatsApp, or Emails) are not unified and are processed disparately. A key pain point for OHC’s targeted users, such as Carlos (handyman) or Maya (baker), is missing out on client queries across various unlinked channels. There is a need for a unified Omnichannel Agent Architecture where any incoming customer intent is resolved into a centralized identity graph. The AI agents (like The Ambassador) can then use context to generate drafted replies and proactive notifications that land directly on the owner’s mobile dashboard.

  ## Research Report
  - **Market Context**: Competing solutions (Shopify Inbox, Zendesk, Wix Inbox) aggregate messages but fall short in autonomously predicting context and generating actionable replies based on the full scope of business rules and history.
  - **The OHC Opportunity**: Integrating RAG (Retrieval-Augmented Generation) with a central Omnichannel Gateway to parse incoming events natively. This creates an event mesh that feeds into an Identity Resolution Engine, allowing The Ambassador agent to understand if an Instagram DM from "@sarah_cakes" matches the email "sarah@example.com" in the ledger.
  - **Competitor Gaps**: Traditional systems are heavily reactive and only suggest rudimentary auto-replies. OHC’s agentic workflow dictates that the agent *drafts the reply based on the customer’s purchase and interaction history*, and requires only a one-tap approval from the owner.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Various Inbound Channels: DMs, SMS, Webhooks] -->|Webhook/Event| B(Omnichannel Gateway)
      B --> C{Customer Identity Resolution}
      C -->|Identify| D[Unified Customer DB]
      C --> E[Event Mesh / Message Bus]
      E --> F[The Ambassador Agent]
      F -->|RAG Lookup| D
      F -->|Generate Draft| G[Action Required Queue]
      G --> H[Mobile Owner Dashboard 375px]
      H -->|1-Tap Approve| I[Omnichannel Dispatcher]
      I --> A
  ```

  ### AI Integration & Mobile UX Flow (375px)
  - **Event Mesh**: Messages are standardized into internal `OmnichannelEvent` objects.
  - **Identity Resolution Engine**: Attempts to map social handles, phone numbers, and emails to a single `Customer` identity per tenant using probabilistic and deterministic matching.
  - **Agent Intervention**: The Ambassador Agent is invoked, queries the unified DB for context (e.g. recent orders), and drafts a highly personalized response.
  - **Mobile Approval**: The owner receives a push notification and sees an Action Card on their 375px home feed. The card highlights the message intent, the drafted reply, and features prominent "Approve", "Edit", and "Discard" actions.

  ## Implementation Prompt
  **Feature Name**: Omnichannel Event Gateway & Identity Resolution
  **Target Persona**: Maya the Baker
  **Outcome**: Maya receives DMs from various sources, but OHC unifies them into single customer profiles. The Ambassador Agent drafts replies contextually based on previous interactions, and Maya simply taps "Approve" from her phone.

  **Next Actions**:
  1. Build the `Omnichannel Gateway` to receive and standardize incoming webhooks.
  2. Implement the `Customer Identity Resolution Engine` to merge customer profiles based on identifiers.
  3. Wire the `Event Mesh` to trigger `The Ambassador Agent` upon message receipt.
  4. Design the 375px Mobile Action Card to display the drafted response and allow for 1-tap approval.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
