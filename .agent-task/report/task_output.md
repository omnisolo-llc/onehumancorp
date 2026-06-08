issue_title: "Autonomous Omni-Channel Customer Success & Sales Engine (The Ambassador)"
issue_description: |
  # Mission Queue Protocol: The Ambassador Engine

  ## Problem Statement
  Small business owners like Maya (baker) and Carlos (handyman) suffer from "Operational Fatigue" (68% frequency) and "Communication Lag" (40% frequency). They lose up to 30% of sales because they cannot respond instantly to inquiries across Instagram, WhatsApp, and SMS while working or sleeping. Current "unified inboxes" merely aggregate messages; they still require the owner to manually type replies, calculate prices, and send payment links. OHC needs an **invisible, autonomous engine** that answers FAQs, qualifies leads, and drafts quotes/payment links 24/7, escalating to the owner only when necessary.

  ## Research Report
  - **Competitor Audit**:
    - **Shopify Inbox**: Passive aggregation. AI is reactive (Sidekick) and merchant-facing, not customer-facing.
    - **Intercom/Zendesk**: Enterprise-grade complexity. "Grandmother Test" failure. High "App Tax."
    - **OHC Gap**: OHC's current `chat` service is a passive proxy. It lacks an event-driven listener to trigger the KAIROS Orchestrator for inbound customer events.
  - **Data Validation**: 80% of SMB inquiries are repetitive ("Do you do vegan cakes?", "What is your rate?"). Autonomous resolution of these queries creates a "Sales Wedge" that differentiates OHC.

  ## Design Doc

  ### Architecture Diagram (Orchestration)
  ```mermaid
  sequenceDiagram
      participant Customer as Customer (IG/WA/SMS)
      participant Ingress as Inbound Gateway (Webhooks)
      participant Mesh as NATS Event Mesh
      participant Ambassador as Ambassador Agent (CS)
      participant Sales as Sales Agent (Pricing)
      participant Finance as Accountant Agent (Payments)
      participant DB as Hybrid RAG (SQLite/Postgres)
      participant Owner as Owner Mobile (375px)

      Customer->>Ingress: "Need a custom cake for Sat. Do you do vegan?"
      Ingress->>Mesh: Publish: inbound_message_event
      Mesh->>Ambassador: Trigger: process_inquiry
      Ambassador->>DB: Query: tenant_catalog (Vegan?)
      DB-->>Ambassador: Match: "Vegan Choco Cake - $40"
      Ambassador->>Sales: Request: Draft Quote (1x Vegan Cake + Delivery)
      Sales->>Finance: Request: Secure Payment Link
      Finance-->>Ambassador: Link: https://ohc.pay/abc
      Ambassador->>Mesh: Publish: outbound_message_event (AI Draft)
      Mesh->>Owner: Push: "Draft Quote Ready: $45. Approve?"
      Owner->>Ambassador: 1-Tap [Approve]
      Ambassador->>Mesh: Dispatch: "Yes! Here is your link: [Link]"
      Mesh->>Customer: Delivered via WhatsApp
  ```

  ### Data Model & Invariants
  ```mermaid
  erDiagram
      TENANT ||--o{ CONVERSATION : owns
      CONVERSATION ||--o{ MESSAGE : contains
      CONVERSATION {
          uuid id PK
          string channel "whatsapp|instagram|sms"
          boolean ai_paused "Default: false"
          timestamp last_human_reply
      }
      MESSAGE {
          uuid id PK
          string sender_type "customer|ai_agent|human_owner"
          string content
          boolean is_draft "For owner approval"
      }
  ```
  - **Invariant 1 (Owner Shield)**: `ai_paused` is set to `true` for 2 hours if `sender_type == human_owner`. AI must never "talk over" the owner.
  - **Invariant 2 (Tenant Vault)**: Every RAG query must include a SPIFFE-signed `tenant_id` context to prevent cross-tenant catalog leaks.

  ### Mobile-First UX Flow (375px)
  - **Unified Inbox Feed**: Glassmorphic card list. Threads handled by AI feature a subtle purple "✨ Sparkle" badge. High-risk intents (Quotes) show a red "Needs Approval" badge.
  - **1-Tap Approval**: Quote drafts appear as interactive "Frosted Glass" cards within the thread. The owner can tap a single 88x88px [Send] button from the lock screen notification or the app.
  - **Optimistic UI**: AI-generated drafts are rendered instantly with a "Generating..." shimmer effect to mask LLM latency.

  ### AI Agent Coordination
  - **Ambassador (Department Head)**: Handles the customer relationship, tone matching, and triage.
  - **Salesperson (Pricing Sub-agent)**: Translates unstructured requests into multi-line quotes based on historical pricing memory.
  - **Accountant (Finance Sub-agent)**: Generates one-click Apple/Google Pay links via Stripe.

  ## Implementation Prompt
  **To the Implementer Swarm**:
  Build the Autonomous Omni-Channel Customer Success & Sales Engine (The Ambassador).
  1. Implement an inbound webhook listener in the `chat` service that publishes `inbound_message_event` to NATS.
  2. Create the `Ambassador` agent role in KAIROS. It must classify message intent and use Hybrid RAG to query the `tenant_catalog` (local SQLite or Postgres).
  3. Implement the `ai_paused` state machine: human replies silence the agent for that thread.
  4. Develop the "Quote-to-Cash" handoff: Ambassador triggers the Sales agent to draft a quote and the Finance agent to generate a Stripe payment link.
  5. Update the mobile UI (`UnifiedInboxView`) to support interactive "Draft Cards" and the "✨ Sparkle" badge system using OHC premium design tokens (20px blur, translucency).
  6. **Acceptance Criteria**: A customer sends "Price for cleaning?" via SMS -> Owner gets a push "Draft Quote: $100" -> Owner taps [Approve] -> Customer receives payment link.

  ## Priority: P0
  ## Estimated Scope: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
