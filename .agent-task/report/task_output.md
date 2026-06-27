issue_title: "Implement Universal Customer Identity (CRM Engine) for Cross-Channel Context"
issue_description: |
  ## Title
  Implement Universal Customer Identity (CRM Engine) for Cross-Channel Context

  ## Problem Statement
  Business owners currently suffer from fragmented customer data across different channels. Maya (the baker) might receive an Instagram DM from "alex_bakes", an email inquiry from "alex.smith@email.com", and a Stripe payment from "Alexander Smith". Without a unified customer view, she loses the context that this is the same repeat customer. For OHC to truly act as a unified "Customer & Relationship Assistant", the AI needs a singular, authoritative `Customer` entity that bridges social DMs, online orders, physical tap-to-pay (POS), and bookings. Without this, the AI agent lacks context, and owners are forced to manually piece together a customer's history.

  ## Research Report
  - **Codebase Audit:** Currently, OHC handles interactions in silos, with data models focused heavily on operations like `inbox` or `booking`, but no centralized Customer or Identity entity across the system. State is siloed by interaction type.
  - **Competitive Analysis:** Shopify unifies customers via email/phone across online and Shopify POS. Wix has a built-in CRM for contacts. Square uses universal customer profiles linked to payment cards (hashed). OHC needs a simpler, AI-driven identity resolution engine that doesn't require the owner to manually merge records.
  - **Key Gap:** Missing a unified `CustomerIdentity` model with a many-to-one relationship to `ChannelIdentity` (e.g., IG handle, WhatsApp number, Email).
  - **AI Opportunity:** The AI Customer Assistant can proactively suggest merging profiles (e.g., "It looks like IG user @alex_bakes is Alex Smith based on their phone number. Merge them?").

  ## Design Doc

  ### Architecture Diagram (Mermaid)
  ```mermaid
  erDiagram
      TENANT ||--o{ CUSTOMER : owns
      CUSTOMER ||--o{ CHANNEL_IDENTITY : has
      CUSTOMER ||--o{ ORDER : places
      CUSTOMER ||--o{ BOOKING : schedules
      CUSTOMER ||--o{ INBOX_MESSAGE : sends

      CUSTOMER {
          uuid id PK
          uuid tenant_id FK
          string primary_name
          string primary_email
          string primary_phone
          text ai_summary_notes
          datetime last_interaction_at
      }

      CHANNEL_IDENTITY {
          uuid id PK
          uuid customer_id FK
          string provider "e.g., 'instagram', 'whatsapp', 'email'"
          string provider_id "e.g., '@alex_bakes'"
      }
  ```

  ### Mobile UX Flow (375px First)
  - **Customer Profile View:** A unified card layout. Top section shows Avatar, Name, and Lifetime Value.
  - **Activity Timeline:** A single vertically scrolling feed showing "IG DM (Yesterday)", "Booked Lesson (Last Week)", "Bought Cake (3 mos ago)".
  - **AI Action Bar:** A sticky bottom sheet with actions: "Draft Reply", "Send Promo", "Generate Quote".
  - **1-Tap Merge:** When viewing a new message from an unknown number, the AI suggests: "Matches Alex Smith. Merge? [Yes] [No]".

  ### AI Agent Integration Points
  - **Customer Success Agent:** Uses the `CUSTOMER.ai_summary_notes` as context when drafting replies to DMs.
  - **Operations Agent:** Updates the `last_interaction_at` and triggers "Win-back" campaigns if a high-value customer hasn't purchased in 6 months.

  ### Key Design Decisions
  - **Lazy Resolution:** Channel identities are created first. If the AI detects a match (e.g., same phone number from a Stripe checkout and a WhatsApp message), it queues a low-priority async job for the owner to approve the merge.
  - **Strict Multi-Tenancy:** The `tenant_id` must be enforced via PostgreSQL Row Level Security (RLS) on both `customers` and `channel_identities`.
  - **Zero-Config CRM:** The owner does not manually create contacts. Contacts are automatically generated from inbound DMs, bookings, and checkouts.

  ## Implementation Prompt
  Implement the Universal Customer Identity (CRM Engine) in the backend (Go + Bazel). Create the core data models (`Customer` and `ChannelIdentity`) mapping to PostgreSQL with strict `tenant_id` based RLS. Expose standard gRPC/REST endpoints for querying a customer's unified timeline. Implement a background job queue worker using PostgreSQL `SKIP LOCKED` that attempts to auto-associate new inbound messages or orders to existing customers based on matching email or phone number. Ensure all database operations are tenant-isolated and covered by 100% unit tests. In the frontend (Flutter), create a mobile-optimized (375px) "Customer Profile" screen that displays this unified timeline and allows the owner to view AI-generated relationship summaries. Provide Playwright E2E tests verifying that an inbound order and a subsequent booking from the same email are correctly linked to the same Customer entity.

  ## Estimated Scope
  Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
