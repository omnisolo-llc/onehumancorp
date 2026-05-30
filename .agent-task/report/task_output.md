issue_title: "[Architecture] Cross-Channel Identity Resolution & Customer Profile Consolidation Engine"
issue_description: |
  # Issue Brief: Cross-Channel Identity Resolution Engine

  ## Problem Statement
  Small business owners often interact with the same customer across multiple fragmented channels—Instagram DMs, email, phone, in-person (Stripe Terminal tap-to-pay), and online storefront. Currently, these interactions create duplicate, disconnected records. For example, when Maya (the baker) talks to "Sarah" on Instagram about a vegan cake, and then "Sarah Smith" pays a deposit via Stripe using her email, Maya doesn't automatically know this is the same person. This fragmentation breaks the `Customer360` view, dilutes the loyalty engine, and makes AI agents (like "The Ambassador") hallucinate or miss critical context because they only see partial interaction timelines.

  Existing platforms (Shopify, Wix) struggle with this, often requiring expensive third-party CDPs (Customer Data Platforms) or manual deduplication, which non-technical users will not do. OHC needs a silent, continuous, multi-tenant-safe engine that automatically merges identities across touchpoints into a single, canonical `Customer360` profile, enabling true AI-driven proactive engagement.

  ## Research Report
  - **Competitive Audit**:
    - **Shopify**: Relies on email/phone matching at checkout. Doesn't natively merge social identities (IG DMs) with checkout identities without complex API integrations or expensive apps (e.g., Klaviyo CDP).
    - **Wix**: Basic CRM functionality; manual merging is required if a user contacts via different methods.
    - **OHC Advantage**: Because OHC integrates the entire stack (Storefront, Unified Inbox, AI Agents), it has visibility into all touchpoints. By implementing a probabilistic/deterministic Identity Resolution Engine within the KAIROS memory layer, OHC can silently unify profiles, making AI interactions drastically more personalized.
  - **Key Findings**:
    - Over 60% of small business interactions start on social media before moving to a formal booking/checkout flow.
    - Duplicate customer profiles lead to inaccurate loyalty tier calculations and redundant or contradictory AI-drafted messages.
    - Identity unification is the foundational enabler for advanced features like "Zero-Click Abandoned Cart" and "Autonomous Customer Lifecycle & Loyalty".

  ## Design Doc

  ### Data Model (Identity Graph)
  The existing `Customer360` model must be expanded to support an identity graph concept, allowing multiple aliases/identifiers to link back to a single canonical customer record.

  ```mermaid
  erDiagram
      TENANT ||--o{ CUSTOMER360 : "owns"
      CUSTOMER360 ||--o{ IDENTITY_ALIAS : "has many"
      CUSTOMER360 ||--o{ INTERACTION_TIMELINE : "consolidates"

      CUSTOMER360 {
          uuid canonical_id PK
          string primary_email
          string primary_phone
          string primary_name
          string confidence_score "High, Medium, Low"
      }

      IDENTITY_ALIAS {
          uuid id PK
          uuid canonical_id FK
          string identifier_type "Email, Phone, IG_Handle, Cookie_ID, Stripe_Customer_ID"
          string identifier_value
          timestamp last_seen_at
      }
  ```

  ### AI Agent Coordination (The Librarian / Identity Engine)
  A background process (part of the memory/data layer) continuously evaluates incoming events to deterministic (e.g., exact email match) or probabilistic (e.g., same name + same location + similar time context) matches.

  ```mermaid
  sequenceDiagram
      participant External as Social/Payment API
      participant EventMesh as OHC Event Mesh
      participant IdEngine as Identity Resolution Engine
      participant DB as Customer360 DB
      participant User as Mobile Dashboard (Owner)

      External->>EventMesh: Event: IG DM from @sarah_bakes
      EventMesh->>IdEngine: New Identifier Seen (@sarah_bakes)
      IdEngine->>DB: Create/Update Alias for Unknown User A
      External->>EventMesh: Event: Stripe Payment (sarah@example.com)
      EventMesh->>IdEngine: New Identifier Seen (sarah@example.com)
      IdEngine->>IdEngine: AI/Heuristics flag potential match (Name + Time Proximity + Content context)
      IdEngine->>DB: Merge Unknown User A into Canonical User Sarah Smith
      IdEngine->>EventMesh: Emit: ProfileMerged Event
      EventMesh->>User: Activity Feed: "Linked IG @sarah_bakes to Sarah Smith's profile."
  ```

  ### Key Architectural Invariants
  1. **Strict Multi-Tenant Isolation**: Identity resolution must NEVER cross `tenant_id` boundaries. A customer's profile for Maya the Baker is completely separate from their profile for Carlos the Handyman. RLS must enforce this.
  2. **Non-Destructive Merges**: When two profiles are merged, the original identifiers and interaction timelines must be preserved and linked to the new canonical ID. Un-merging should be possible if a false positive occurs.
  3. **Privacy & Compliance**: Handling of PII (identifiers) must comply with GDPR/CCPA. Alias resolution logic must not expose PII inappropriately in logs or unauthenticated contexts.

  ### Mobile-First UX & Wireframes (375px First)
  1. **Customer Profile View**:
     - **Visual**: Translucent glass card showing the canonical name. Below it, small chips/icons indicating linked channels (e.g., [IG icon @sarah_bakes], [Email icon]).
  2. **Merge Suggestion Card (Activity Feed)**:
     - **Visual**: If the engine is unsure (Medium confidence), it prompts the owner.
     - **Content**: "Is Instagram user @sarah_bakes the same as Sarah Smith? They both asked about vegan cakes recently."
     - **Interaction**: Large, thumb-friendly "Yes, Merge" or "No" buttons.
  3. **Zero Jargon**: Avoid terms like "Identity Resolution", "Probabilistic Matching", or "Canonical ID". Use "Linked Contacts" or "Combined Profile".

  ## Implementation Prompt
  **Goal**: Build the "Cross-Channel Identity Resolution Engine" to automatically merge fragmented customer interactions into a unified `Customer360` profile, enabling smarter AI responses and accurate loyalty tracking.

  **Core User Journey (CUJ)**:
  1. **The Silent Merge**: A customer messages Priya's boutique on Instagram asking about a red dress. Later, that customer buys the dress in-store using Tap-to-Pay (Stripe Terminal) and provides their phone number for a receipt. The Identity Engine recognizes the overlapping context/name/timing, silently links the IG handle to the phone number under a single `Customer360` profile, and updates the timeline. Priya sees one cohesive history.
  2. **The Assisted Merge**: The engine flags a possible match between an email inquiry and a WhatsApp message based on similar names but different numbers. It surfaces a simple "1-Tap Merge Suggestion" in Maya's Activity Feed. Maya taps "Yes, it's the same person," instantly consolidating the histories.

  **Acceptance Criteria**:
  - **Schema Update**: Implement the `IDENTITY_ALIAS` table linked to `CUSTOMER360` with strict RLS multi-tenant isolation.
  - **Resolution Logic**: Build the background worker/service that processes incoming events from the Mesh and performs deterministic and probabilistic matching within a single tenant boundary.
  - **Merge Handling**: Ensure merging is non-destructive (timelines combine gracefully, soft-delete or redirect old canonical IDs).
  - **UI/UX**: Surface merged identifiers in the mobile Customer Profile view and create the "Merge Suggestion" actionable card for the Activity Feed.

  ## Priority
  P1 (High) - Foundational for CRM, Loyalty, and AI context.

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
