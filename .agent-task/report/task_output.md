issue_title: "Implement Architectural Capability: Unified Multi-Channel Intent Orchestrator (The Ambassador)"
issue_description: |
  # The Ambassador Agent Architecture Issue Brief

  ## Problem Statement
  Solopreneurs like Maya (Home Baker) and Carlos (Handyman) miss critical sales and appointments because they are unable to monitor disparate inbound channels (Instagram DMs, WhatsApp, SMS, Web Forms) while engaged in physical operations. Current industry solutions require complex workflow builders (e.g., ManyChat, Zapier) that are completely inaccessible to non-technical operators. OHC requires a seamless, zero-configuration "Ambassador" architecture that intercepts multi-channel demand, extracts intent, queries internal state (inventory, bookings), and drafts actionable replies for 1-tap owner approval.

  ## Research Report (Track 1)
  - **Shopify/Wix:** Rely on disjointed 3rd-party apps with explicit rules engines.
  - **HubSpot:** Offers "Breeze" agents, but heavily skewed toward B2B sales CRMs, lacking 375px mobile-first execution.
  - **AI-Native Rivals (11x, Lindy):** Offer autonomous workers but lack deep integrations into the SMB's core ledger and inventory state without API configuration.
  - **The OHC Differentiator:** A unified inbound event bus that normalizes all messages regardless of source, coupled with an LLM intent router that securely queries the PostgreSQL multitenant state and generates a drafted action in the unified Agent Feed, requiring zero setup from the user beyond OAuth connection.

  ## Design Doc (Track 2 & Track 3)

  ### Architecture Diagram (Mermaid)
  ```mermaid
  graph TD;
      Webhooks[Meta / WhatsApp / Web Form Ingress] --> Normalizer[Channel Normalizer Service];
      Normalizer --> Queue[PostgreSQL Job Queue];
      Queue --> Classifier[KAIROS LLM Intent Classifier];
      Classifier --> ContextRouter{Intent Router};
      ContextRouter -->|Pricing| RAG_Price[Catalog / Inventory Query];
      ContextRouter -->|Availability| RAG_Book[Calendar / Booking Query];
      RAG_Price --> Drafter[Draft Generator Worker];
      RAG_Book --> Drafter;
      Drafter --> TriageDB[(Triage Items DB)];
      TriageDB --> UIMobile[375px Mobile Feed UI];
      UIMobile --> Approve[User 'Approve & Send' Action];
      Approve --> Egress[Egress API / Webhook Reply];
  ```

  ### Mobile UX Flow (375px first)
  1. The user opens the app on a 375px viewport.
  2. The unified Agent Feed displays high-priority pending drafts.
  3. A Glassmorphism card appears: "Drafted reply for @customer on Instagram: 'Yes, vegan chocolate cake is available! Booking link: [Link]'"
  4. The card contains two massive, minimum 44x44px touch targets: "Approve & Send" and "Edit".
  5. Tapping "Approve" transitions the card to a checked state and asynchronously dispatches the response via the Egress API.

  ### AI Agent Integration Points & Security
  - **SPIFFE/SPIRE Identity:** All intra-agent service calls (e.g., Ambassador querying the Operations Agent's inventory state) must be mTLS verified.
  - **Multitenant RLS:** All RAG queries must enforce PostgreSQL Row-Level Security (`tenant_id`).
  - **LLM Abstraction:** The intent classification and drafting must utilize the swappable `OHC_LLM_PROVIDER` interface to ensure resilience against provider outages.

  ## Implementation Prompt (Track 4)
  **Objective:** Implement the core infrastructure and initial flow for the "Ambassador" multi-channel orchestrator.
  **Task Requirements:**
  1.  **Ingress/Egress APIs:** Define the gRPC/REST contracts for receiving normalized inbound messages and sending outbound replies.
  2.  **Intent Classification Worker:** Implement a background worker (utilizing the existing job queue) that consumes inbound messages, calls the LLM provider to classify intent (e.g., inquiry, booking, complaint), and extracts relevant entities.
  3.  **Context Assembly (RAG):** Implement a service that retrieves necessary context (mocked or real catalog/booking data) based on the classified intent, ensuring strict `tenant_id` isolation.
  4.  **Draft Generation:** Produce a drafted response and persist it in the `triage_items` and `triage_proposed_actions` tables (or equivalent schema) for user review.
  5.  **E2E Testing:** Create Playwright E2E tests simulating an inbound webhook and verifying the drafted response appears in the UI and can be approved.

  **Acceptance Criteria:**
  - The system can process a simulated inbound message end-to-end resulting in a pending UI action.
  - The architecture enforces multitenant boundaries.
  - 100% Unit test coverage on the new worker and service layers.

  ## Priority: P0
  ## Estimated Scope: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
