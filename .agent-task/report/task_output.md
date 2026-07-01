issue_title: "Architectural Gap: Unified Multi-Channel Omnicommerce Aggregation & Action Engine"
issue_description: |
  # Unified Multi-Channel Omnicommerce Aggregation & Action Engine

  ## Problem Statement
  Small business owners (like Maya the Baker or Priya the Boutique Operator) are currently managing incoming demand, operations, and transactions across isolated platforms (Instagram DMs, physical POS, WhatsApp, Web Storefront). This fragmentation forces the owner to manually piece together context, risking dropped leads and disjointed customer interactions. The core gap is the absence of a localized, high-scale, omni-channel aggregation layer that unifies these streams into a single, actionable work feed on a 375px mobile device.

  ## Research Report
  ### Context & Gap Analysis
  - **Current State:** OHC currently provides capabilities like booking, deposits, and service routes, but lacks a centralized, real-time message and action bus that seamlessly correlates a WhatsApp DM with a Shopify order and a physical Tap-to-Pay transaction.
  - **Competitive Landscape:** Shopify Inbox handles store chats, and Wix unifies some social channels, but neither integrates deep AI agents (like the OHC Customer Assistant) to pre-draft quotes, identify intent, or generate instant payment links from a single continuous thread.
  - **The Missing Link:** A unified data pipeline and presentation layer that treats every inbound signal (chat, payment, review) as a standardized "Work Item" for the AI to triage and the owner to act upon.

  ## Design Doc
  ### Architectural Overview
  The architecture introduces the **OmniChannel Aggregation Engine (OCAE)**:
  1. **Ingestion Layer:** Webhook endpoints and polling workers (using Postgres `SKIP LOCKED` job queue) to ingest signals from Meta (Instagram/WhatsApp), Stripe, and internal Storefront APIs.
  2. **Normalization & Correlation:** Maps incoming payloads to a canonical `WorkItem` entity linked to a unified `CustomerProfile`.
  3. **Agent Integration:** Triggers the AI Triage Agent to score priority, tag intent, and draft a response or next action (e.g., generate a quote).
  4. **Real-time Synchronization:** Uses Redis Pub/Sub to push updates instantly to the mobile client.

  ### Mobile UX Flow (375px)
  - **Unified Feed:** The home screen presents a single, chronological list of actionable cards. No horizontal scrolling.
  - **Contextual Actions:** Tapping a card opens a detailed view showing the customer's history. The AI draft response is prominently displayed at the bottom with a simple "Approve & Send" or "Edit" button.
  - **Quick Resolve:** Swiping left on a card marks it as resolved, swiping right flags it for follow-up.

  ### Data Model (Mermaid)
  ```mermaid
  erDiagram
    TENANT ||--o{ CUSTOMER_PROFILE : has
    CUSTOMER_PROFILE ||--o{ WORK_ITEM : generates
    WORK_ITEM ||--o{ AGENT_DRAFT : receives
    WORK_ITEM {
      uuid id
      uuid tenant_id
      string source
      string payload
      string status
    }
  ```

  ## Implementation Prompt
  **To the Implementer:**
  Implement the backend ingestion pipeline and the mobile-first "Unified Feed" UI.
  1. Create the `WorkItem` and `CustomerProfile` ingestion logic.
  2. Integrate the AI Triage Agent to generate an actionable draft response for new `WorkItems`.
  3. Build the 375px mobile UI showing the unified feed and the contextual action view with the "Approve & Send" capability.
  Ensure strict multi-tenant isolation via Row-Level Security in Postgres.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
