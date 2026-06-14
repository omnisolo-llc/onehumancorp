issue_title: "[Architectural Design] Customer Identity Resolution & Omnichannel Memory Architecture"
issue_description: |
  ## Core Mission
  Design the definitive Customer Identity Resolution Engine and Omnichannel Memory architecture for OHC.

  ## The Problem
  Currently, small business owners manage fragmented customer identities. If Maya the baker gets an Instagram DM from "@cake_lover99", she does not natively know this is the same "Sarah Smith" who bought a cake last month via Stripe and emailed her yesterday. Legacy platforms (Shopify, Wix) aggregate inboxes but fail to resolve fragmented identities into a single, unified "Customer Graph." This breaks the "Invisible AI Automation" promise, as agents lack complete context to draft accurate responses.

  ## Research & Market Gap
  - **Traditional SMB E-commerce (Shopify/Wix)**: Treat emails and checkout details as the primary key. Social handles are often disconnected.
  - **Enterprise CRMs (HubSpot/Salesforce)**: Have identity resolution but are far too complex and expensive for micro-SMEs.
  - **OHC Opportunity**: Provide a Zero-Configuration Identity Graph. When a message arrives via any channel, the system autonomously attempts to link it to an existing profile using fuzzy matching, phone numbers, and cross-referenced data, presenting a unified history to the Customer Success Agent ("The Ambassador").

  ## Design Document: Customer Identity Resolution Engine

  ### 1. Data Model (PostgreSQL with RLS)
  We need a unified structure to hold identities and their various aliases across channels.
  - `ohc_customers`: The canonical customer record (Canonical ID, Primary Name, Primary Email, Primary Phone).
  - `ohc_customer_aliases`: Maps external identifiers to the canonical customer.
    - Fields: `customer_id`, `channel` (e.g., 'instagram', 'whatsapp', 'email', 'stripe'), `external_id` (e.g., '@cake_lover99', '+15551234567'), `verified` (boolean).
  - `ohc_omnichannel_messages`: All communications linked back to the *canonical* `customer_id`.

  ### 2. Architecture Diagram (Mermaid)
  ```mermaid
  graph TD
      A[Incoming Message: IG, WA, Email] -->|Webhook| B(Omnichannel Gateway)
      B --> C[Identity Resolution Service]
      C -->|Extract Selectors: Phone, Email, Handle| D{Alias Lookup}
      D -- Found --> E[Link to Canonical Customer ID]
      D -- Not Found --> F[Fuzzy Match / Create Tentative Profile]
      E --> G[Store Message with Customer ID]
      F --> G
      G --> H[Event Bus: message.received]
      H --> I[The Ambassador Agent]
      I -->|RAG Query| J[(Omnichannel Memory DB)]
      I --> K[Draft Contextual Reply]
  ```

  ### 3. Agent Coordination (The Ambassador)
  When "The Ambassador" agent receives a `message.received` event, it doesn't just see "Message from @cake_lover99". The Identity Resolution Service ensures the agent receives the full Canonical Customer Profile, including:
  - Past orders (from the `ledger` / Stripe).
  - Previous support tickets (from Emails).
  - Past DMs.
  The Agent can then draft a reply like: "Hi Sarah! Yes, we can do the same vegan cake you ordered last November for $50. Shall I send a deposit link?"

  ### 4. Mobile-First UX (375px)
  - **Customer Profile View**: A unified timeline showing every interaction (Order, IG DM, Email) in one vertical scroll, without the user needing to switch "apps".
  - **Identity Merging Card**: An Action Card in the Agent Feed: "Agent thinks @cake_lover99 on Instagram is Sarah Smith based on phone number. Tap to merge profiles." (1-tap interaction).

  ## Implementation Prompt (For Engineering Swarm)
  **Feature**: Omnichannel Customer Identity Resolution
  **Target Persona**: Maya the Baker

  **Outcome**: When Maya gets an IG DM, the system automatically recognizes if the user is an existing customer and provides the full purchase/chat history to the Ambassador agent to draft a highly contextual reply.

  **Next Actions**:
  1. **Schema**: Create migrations for `ohc_customer_aliases` linked to the existing customer tables, enforcing strict `tenant_id` RLS.
  2. **Service**: Build the `IdentityResolutionService` in Rust. It should take incoming message metadata, check `ohc_customer_aliases`, and either return an existing canonical `customer_id` or create a new profile.
  3. **Agent Integration**: Update `The Ambassador` agent to query the full omnichannel history using the resolved canonical `customer_id` before generating a draft.
  4. **UX**: Create an E2E test verifying that a message from an unlinked Instagram handle that contains a known phone number prompts a profile merge or auto-resolves.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
