issue_title: "[Architecture] Autonomous Visual Product Configuration Engine"
issue_description: |
  # Issue Brief: Autonomous Visual Product Configuration Engine

  ## Problem Statement
  Small business owners selling highly customizable products or services (e.g., Maya the baker creating custom tiered cakes with specific vegan frosting and fondant figures) currently face immense friction. They must manually negotiate every custom order via fragmented Instagram DMs, text messages, and emails. This back-and-forth "quoting" process is slow, drops leads, and forces the business owner to manually transcribe requirements into a separate invoice or booking system. Current platforms like Shopify or Wix require complex, clunky multi-step forms that overwhelm both the buyer and the seller. Maya needs an AI teammate to effortlessly guide a customer through a custom visual configuration, collect a deposit, and hand her a clean, structured order brief.

  ## Research Report
  *   **Competitor Analysis**:
      *   **Shopify/Wix**: Rely on static variant dropdowns. Complex configurations require expensive third-party apps (like "Product Options") that fail the grandmother test and do not natively support chat-based negotiation.
      *   **The Gap**: No existing platform seamlessly bridges conversational commerce (Instagram DMs/WhatsApp) with structured, visual product variant configuration.
  *   **Pain Points Addressed**:
      *   **Lead Drop-off**: Customers abandon complex forms. A conversational AI approach drastically reduces friction.
      *   **Context Switching**: Owners lose hours moving from IG DM -> Notes App -> Square Invoice.
      *   **Miscommunication**: Mistakes in manual transcription lead to incorrect orders and refunds.
  *   **Proposed Solution**: The Autonomous Visual Product Configuration Engine. This engine allows the AI (Customer Success/Sales Agents) to dynamically generate and present visual configuration mini-apps directly within the chat stream based on the buyer's natural language requests.

  ## Design Doc

  ### UX Flow (Mobile-First 375px)
  1.  **Trigger**: Customer DMs Maya on Instagram: "I need a vegan cake for a 5-year-old's dinosaur party."
  2.  **AI Intercept**: The OHC Customer Success Agent understands the intent and replies natively in IG: "I can help with that! Let's get the details."
  3.  **Dynamic Mini-App Delivery**: The Agent sends a magic, single-use link.
  4.  **Configuration Interface**: Tapping the link opens an instant, edge-cached, 375px mobile view.
      *   Clean, UniFi-style cards for selections (Tier size, Flavor, Frosting).
      *   *Crucially*, the AI has already pre-selected "Vegan" based on the chat context.
      *   Image upload zone for inspiration photos.
  5.  **Instant Quote & Deposit**: As the user toggles options, a dynamic price updates. A one-tap Apple Pay/Google Pay button collects the deposit immediately.
  6.  **Handoff**: Maya receives a unified notification: "New Custom Order Confirmed. Deposit Paid. Here is the spec sheet."

  ### Architecture Diagrams

  #### Component Interaction (Sequence)
  ```mermaid
  sequenceDiagram
      participant C as Customer (IG/WhatsApp)
      participant OM as Omnichannel Inbox
      participant AA as Autonomous Agent (Sales/CS)
      participant VC as Visual Config Engine
      participant DB as Postgres (Tenant Ledger)

      C->>OM: Natural language custom request
      OM->>AA: Ingest intent & context
      AA->>VC: Request dynamic configuration schema
      VC-->>AA: Generate unique edge-cached token link
      AA->>OM: Send link to Customer
      C->>VC: Open mobile config UI
      VC->>VC: User selects visual variants
      VC->>DB: Calculate dynamic pricing
      C->>VC: 1-Tap Checkout (Deposit)
      VC->>DB: Persist order & payment intent
      DB-->>OM: Trigger business owner notification
  ```

  #### Data Model & Invariants (Entity-Relationship)
  ```mermaid
  erDiagram
      TENANT ||--o{ VISUAL_CONFIG_TEMPLATE : owns
      VISUAL_CONFIG_TEMPLATE ||--o{ CONFIG_VARIANT : has
      CONFIG_VARIANT ||--o{ VARIANT_OPTION : contains
      TENANT ||--o{ CUSTOM_ORDER : manages
      VISUAL_CONFIG_TEMPLATE ||--o{ CUSTOM_ORDER : instantiates
      CUSTOM_ORDER ||--o{ ORDER_SELECTION : contains
      VARIANT_OPTION ||--o{ ORDER_SELECTION : chosen_as
      CUSTOM_ORDER ||--|| PAYMENT_INTENT : requires

      TENANT {
          uuid id PK
          string organization_name
          string stripe_account_id
      }
      VISUAL_CONFIG_TEMPLATE {
          uuid id PK
          uuid tenant_id FK
          string base_product_name
          decimal base_price
          json exclusion_rules "e.g. if Vegan disable Buttercream"
      }
      CONFIG_VARIANT {
          uuid id PK
          uuid template_id FK
          string category_name "e.g. Frosting"
          boolean is_required
      }
      VARIANT_OPTION {
          uuid id PK
          uuid variant_id FK
          string option_name "e.g. Vegan Vanilla"
          decimal price_modifier
          string image_url
      }
      CUSTOM_ORDER {
          uuid id PK
          uuid tenant_id FK
          uuid template_id FK
          string customer_contact
          decimal total_price
          string status "Pending, Deposit_Paid, Fulfilled"
      }
  ```

  ### Key Design Decisions
  *   **Magic Links over App Downloads**: Buyers will not download an app to order a cake. The configuration UI must be instantly accessible via a web link.
  *   **Edge-Cached UI**: The mini-app must load in < 500ms even on slow mobile networks to prevent drop-off.
  *   **Context Preservation**: The UI must inherit all context from the preceding chat (e.g., locking the 'dietary' option to 'vegan').
  *   **Zero-Trust Isolation**: The configuration engine must strictly adhere to the `TenantRegistry` rules to ensure data leakage between distinct merchants is impossible. All queries must enforce `tenant_id`.

  ## Implementation Prompt
  **Objective**: Implement the `Visual Config Engine` core data structures and the dynamic link generation service.
  **User Journey**: When a buyer expresses intent for a custom item via an integrated messaging channel, the AI agent must be able to call a service that returns a unique URL. This URL must host a lightweight, tenant-isolated React/Tauri view where the buyer can select variants, upload images, and pay a deposit.
  **Acceptance Criteria**:
  1.  Service can ingest a JSON schema defining variant options and rules (e.g., if Vegan, disable Buttercream).
  2.  Service generates a short-lived, cryptographically signed URL.
  3.  The frontend view is responsive to 375px viewports, uses the OHC design tokens (glassmorphism, UniFi cards), and passes the "grandmother test" for simplicity.
  4.  Selections update a live price quote.
  5.  Completion generates a structured event in the KAIROS Orchestrator to notify the business owner.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
