issue_title: "[Research] OHC Dynamic Multi-Tenant Commerce Architecture"
issue_description: |
  # Research Report: OHC Dynamic Multi-Tenant Commerce Architecture

  ## Problem Statement
  Currently, SMB platforms fall into two categories: generic website builders (Wix, Squarespace) that require disjointed apps for commerce and bookings, and powerful e-commerce monoliths (Shopify) that are too complex for non-technical owners to configure. For OHC to succeed as an autonomous work assistant for personas like Maya (baker), Carlos (handyman), and Priya (boutique), it needs a unified, multi-tenant commerce architecture where AI agents can autonomously construct and execute both service bookings and physical product sales without relying on a fragmented "app tax" ecosystem.

  ## Research Report & Gap Analysis
  Based on our analysis of competitor platforms (Shopify, Wix, Squarespace, GoDaddy) and the OHC codebase:
  1. **The "App Tax" Gap**: Competitors force users to install separate apps for physical products, service bookings, and subscriptions. This fractures data and increases cost.
  2. **AI Execution Gap**: Current AI assistants (like Shopify Sidekick) are advisory. They tell the user *how* to set up a booking system rather than *doing* it for them.
  3. **Multi-Tenant Isolation Gap**: For OHC to securely host thousands of autonomous SMBs on a shared backend, the commerce schema must enforce rigorous PostgreSQL Row Level Security (RLS) across all product, booking, and transaction tables.

  We need a unified `Commerce Object` model that natively handles Products, Services, and Bookings within a single multi-tenant schema, paired with an `Operations Agent` capable of executing CRUD operations on these entities securely.

  ## Architectural Design

  ### 1. Unified Commerce Concept
  Instead of separate schemas for "Products" and "Bookings", we introduce a unified catalog model that can represent physical goods, digital goods, and bookable services.

  ```mermaid
  erDiagram
      Tenant ||--o{ CatalogEntity : owns
      CatalogEntity ||--o{ ItemVariant : has
      CatalogEntity ||--o{ AvailabilitySlot : offers
      CatalogEntity {
          uuid id PK
          uuid tenant_id FK
          string item_type "physical | service | digital"
          string name
          decimal base_price
          boolean requires_booking
      }
      AvailabilitySlot {
          uuid id PK
          uuid catalog_entity_id FK
          timestamp start_time
          timestamp end_time
          int capacity
      }
      ItemVariant {
          uuid id PK
          uuid catalog_entity_id FK
          string name
          decimal price_override
      }
  ```

  - **Multi-Tenancy**: Every data entity must include `tenant_id` and have PostgreSQL RLS enabled.
  - **Zero Trust**: API access to these entities must be gated by tenant-scoped SPIFFE/SPIRE tokens.

  ### 2. Operations Agent Integration
  We will introduce an `OperationsAgent` protocol to the KAIROS Orchestrator. This agent will intercept natural language requests (e.g., "Set up a new vegan cake offering with a $50 deposit") and translate them into authenticated mutations against the unified commerce data model.

  ```mermaid
  sequenceDiagram
      participant Owner as User (Mobile UI)
      participant API as OHC API Layer
      participant Agent as Operations Agent
      participant DB as Postgres (RLS)

      Owner->>API: "Add a vegan cake option, $50 deposit"
      API->>Agent: Route intent to Operations
      Agent->>Agent: Determine intent: Create catalog entity (physical, deposit)
      Agent->>API: Execute mutation (Tenant Scoped)
      API->>DB: Insert catalog entity
      DB-->>API: Success
      API-->>Owner: "Done! Vegan cake is now on your storefront."
  ```

  ### 3. Mobile-First UX (375px)
  - The UI for managing this catalog must fit on a 375px screen without horizontal scrolling.
  - Instead of complex form builders, the UI will feature a chat-like "Operations Feed" where the owner simply tells the agent what to add, and the agent responds with a "Translucent Glass" styled summary card of the created offering.
  - Interactive elements must have 44x44px minimum touch targets.

  ## Implementation Prompt
  **Target Implementer:** Backend & Data Architect Agent

  **Task:** Implement the foundation for the Unified Commerce Architecture.
  1. Define the gRPC service definitions that support creating and retrieving unified catalog resources (which can represent physical products or bookable services) for the Go + Bazel backend.
  2. Implement the PostgreSQL database schema migrations. Ensure `tenant_id` is present on every table and that Row Level Security (RLS) is strictly enforced in the schema setup.
  3. Implement the Go backend logic to handle the defined endpoints, ensuring that every operation verifies the caller's tenant context.
  4. Create comprehensive unit tests for the Go service layer, and a new Playwright E2E test covering the scenario of a user seamlessly creating a new product via the UI (which calls this new backend).

  **Acceptance Criteria:**
  - `bazel test //...` passes, including 100% backend unit test coverage for the new service.
  - A new Playwright test successfully simulates a user creating a commerce item.
  - RLS is verifiably active on the new tables.

  ## Metadata
  - **Priority:** P0 (Critical path for core product value)
  - **Estimated Scope:** Large (Requires cross-stack proto, DB, and Go implementation)

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
