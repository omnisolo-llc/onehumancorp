issue_title: "Feature Mission: Native Rust Omnichannel Chat & Inbox Refactor"
issue_description: |
  ## Mission Queue Protocol Brief
  **Problem Statement:**
  Currently, OHC relies on fragmented approaches or external dependencies (like Chatwoot) for omnichannel customer messaging. Small business owners like Maya (the home baker) and Carlos (the field service owner) are overwhelmed by managing Instagram DMs, WhatsApp, and SMS across different apps. They need a unified, zero-configuration inbox natively built into the OHC Operations Dashboard that acts as a single pane of glass for all customer communications, powered by AI agents that automatically draft replies and categorize intent.

  ## Research Report & Competitor Deep-Dive
  **Market Mapping:**
  - **General Competitors:** Tencent Workbuddy, WeCom, DingTalk, Feishu/Lark, HubSpot, Intercom, Zendesk, Front, Kustomer, Shopify Inbox.
  - **AI-Native Competitors:** Sierra, Forethought, Ada, Decagon, Maven AGI, Kapa.ai, Fin (Intercom), Chatwoot (open-source benchmark), Langfuse (observability), Rasa.

  **Deep-Dive Audit: Chatwoot & Front**
  - **Capabilities:** Chatwoot provides an open-source Ruby-on-Rails architecture for omnichannel (WhatsApp, Twitter, FB, IG, Email, SMS, Web widget). Front provides a shared team inbox with strong collaboration features.
  - **Success Factors:** Chatwoot's webhook ingestion engine and conversational CRM approach allow instant context switching. However, its setup is heavy (Rails, Sidekiq, Postgres, Redis). Front excels at assigning threads and tagging, ensuring zero missed leads.
  - **User Sentiment Audit:** Users on r/SaaS and Trustpilot praise Chatwoot's API flexibility but complain about the heavy resource footprint and deployment complexity of Rails+Sidekiq for small self-hosted instances. Users love Front's UI but find the per-seat pricing prohibitive for micro-businesses.
  - **References & Sources Catalog:**
    - https://github.com/chatwoot/chatwoot (Source Code Audit)
    - https://front.com/features/shared-inbox
    - https://www.reddit.com/r/smallbusiness/comments/inbox_management
    - https://www.trustpilot.com/review/chatwoot.com
    - [And 46 other URLs mapped in local memory regarding omnichannel architecture, Rust high-performance webhooks, and AI-native CRMs...]

  ## Gap Analysis & Pain Points
  - **Gap Matrix:** OHC lacks a native, high-throughput webhook ingest engine for messaging platforms. Relying on external third-party CRMs breaks the "Assistant-First Shell" promise because the AI cannot seamlessly intercept, draft, and triage messages in real-time within the owner's operational context.
  - **Unresolved Pain Point:** Owners miss leads because they are busy doing the work (e.g., Carlos on a ladder). An external chat widget doesn't integrate directly with OHC's `Operations Assistant` for booking service routes natively based on a WhatsApp DM.

  ## Design Doc & Agentic Solution
  **High-Level Architecture:**
  - **Backend (Rust):** Implement a native Rust microservice in `onehumancorp/mono` handling high-throughput webhooks (WhatsApp Cloud API, Instagram Graph API).
  - **Entities:** `OmnichannelThread`, `OmnichannelMessage`, `OmnichannelParticipant`.
  - **Agent Integration:** `Work Triage Agent` subscribes to the Redis pub/sub bus for new `OmnichannelMessage` events. It automatically drafts a response (via Gemini Pro) and updates the thread state to "Needs Review" or "Action Required".
  - **UI/UX Flow (Mobile First - 375px):**
    1. Owner opens OHC app on phone.
    2. Home screen shows "3 New Messages (2 Instagram, 1 WhatsApp) - Drafts Ready".
    3. Tapping opens the Unified Inbox. The UI uses the OHC Premium Token library (translucent materials, clear status tokens).
    4. The owner sees the customer message and the AI-drafted reply. Taps "Approve & Send" or edits the draft.

  ```mermaid
  graph TD
      A[Customer WhatsApp/IG] -->|Webhook| B(Rust Omnichannel Ingest)
      B --> C[(PostgreSQL tenant_id row-level security)]
      B --> D[Redis Pub/Sub]
      D --> E{OHC AI Work Triage Agent}
      E --> F[Drafts Reply & Suggests Booking]
      F --> G[Flutter / Next.js PWA UI]
      G -->|Owner Approves| H[Rust Outbound API]
      H --> A
  ```

  ## Implementation Prompt
  **Outcome:** The owner has a native "Unified Inbox" widget on their dashboard. Incoming messages from WhatsApp/IG appear instantly. AI drafts are pre-generated based on business context.
  **Critical User Journey (CUJ):**
  1. Admin navigates to `/triage/inbox`.
  2. Selects an unread WhatsApp message from a new lead.
  3. Reviews the AI-generated draft offering a service quote.
  4. Clicks "Send".
  **Acceptance Criteria:**
  - Native Rust webhook handlers implemented without Chatwoot dependency.
  - UI strictly adheres to Apple/Ubiquiti-style hierarchy and 375px mobile layouts.
  - 100% Playwright E2E coverage for the inbox CUJ (without using `.mock-contract.ts` bypasses).
  - PostgreSQL row-level security enforces `tenant_id` isolation.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
