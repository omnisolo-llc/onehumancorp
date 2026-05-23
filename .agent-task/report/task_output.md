issue_title: "[Architecture] Autonomous Zero-Friction Store Migration Engine"
issue_description: |
  # [Architecture] Autonomous Zero-Friction Store Migration Engine

  ## Problem Statement
  Small business owners like Priya (Boutique owner, 35) want to migrate from legacy, disjointed platforms like Shopify or Wix to OneHumanCorp (OHC) to take advantage of integrated AI agents, mobile-first management, and unified POS. However, the migration process is highly technical and fraught with risk. Non-technical users cannot export CSVs, map database columns, handle image hosting transitions, or correctly route domain DNS settings without causing downtime. The "Setup Complexity" of moving an existing catalog, customer list, and active subscriptions is overwhelming and fails the "grandmother test," causing many potential OHC users to abandon the switch and stay trapped in their expensive, fragmented ecosystems.

  ## Research Report
  *   **Competitor Analysis**:
      *   **Shopify**: Requires manual CSV imports for products and customers. Relies heavily on paid third-party apps (e.g., Matrixify, Cart2Cart) for complex migrations.
      *   **Wix**: Basic import tools; full site migrations often require hiring a "Wix Partner."
      *   **Squarespace**: Provides a generic import tool that frequently fails to preserve product variants or high-resolution images.
  *   **The OHC Differentiator**: OHC must provide an *Invisible, Autonomous Migration Engine*. A user simply provides the URL of their current store, and the OHC "Migration Agent" crawls the public site (or uses legacy APIs if credentials are provided), extracts the catalog, recreates the design aesthetic using OHC design tokens, and sets up the backend ledger. All the user does is review the generated preview on their mobile device and tap "Approve & Switch Domain."
  *   **Target Pain Point Resolution**: Eliminates the technical barrier to entry for established businesses, directly addressing the "Overwhelming Initial Setup" and "Setup Complexity" identified in our market gap analysis.

  ## Design Doc

  ### High-Level Architecture
  ```mermaid
  graph TD;
      User[Priya - Mobile App] -->|Inputs Legacy URL| Ingress[Zero-Trust API Ingress];
      Ingress --> KAIROS[KAIROS Orchestrator];

      subgraph Autonomous Migration Department
          KAIROS --> MigrationAgent[AI Migration Agent];
          MigrationAgent -->|Crawl & Extract| Scraper[Edge Scraper / Legacy API Adapter];
          Scraper -->|Raw Data| Normalizer[Data Normalization & Embedding Engine];
          Normalizer -->|Structured Entities| TenantSchema[(Tenant Data Schema RLS)];
          Normalizer -->|Images & Media| Storage[Secure Edge Cache / Object Store];
      end

      subgraph OHC Departments
          MigrationAgent --> OpsAgent[Operations Agent: Inventory Setup];
          MigrationAgent --> MarketingAgent[Marketing Agent: Storefront Generation];
          MarketingAgent --> Preview[Mobile Staging Environment];
      end

      Preview -->|1-Tap Approval| User;
      User -->|Approves| NetworkService[Automated DNS / Domain Handover];
  ```

  ### Key Design Decisions & Invariants
  *   **Zero-Input Crawling**: The engine prioritizes public data extraction (scraping product pages, extracting variant structures, downloading images) requiring only the URL. Legacy API keys are only requested if deep historical order data is required.
  *   **Mobile-First UX (375px)**:
      1.  **Input View**: A simple text field: "Paste your current website link."
      2.  **Loading View**: A friendly, animated progress card showing the agent "reading your menu," "organizing the shelves," and "painting the walls."
      3.  **Review View**: A split-screen or swipeable card interface showing "Old Store" vs "New OHC Store."
      4.  **Action**: A single large button: "Looks Great! Make it Live."
  *   **Zero-Trust & Multi-Tenancy**: The extracted data is strictly isolated via PostgreSQL Row Level Security (RLS) under a staging `tenant_id`. It only merges into production upon explicit user approval via SPIFFE/SPIRE authenticated requests.
  *   **Idempotent Extraction**: Re-running the migration engine on the same URL updates the staging environment without duplicating entities.
  *   **No Code/No Spreadsheets**: The user is completely shielded from JSON, CSVs, or mapping interfaces. If an item is ambiguous, the Migration Agent makes a high-confidence guess and flags it for simple "Yes/No" review.

  ## Implementation Prompt
  **User-Facing Outcome:**
  A non-technical business owner can migrate their entire existing online store (catalog, variants, images, basic design aesthetic) to OneHumanCorp simply by pasting their current URL into the OHC mobile app. The system autonomously extracts the data, normalizes it into the OHC ledger, generates a staged mobile-first storefront, and presents it for a 1-tap approval.

  **Critical User Journey (CUJ):**
  1. User enters the URL of their existing Shopify/Wix/Square store.
  2. The AI Migration Agent orchestrates background extraction, parsing product details, prices, variants, and media.
  3. The Operations and Marketing agents populate a staging environment within the user's isolated tenant schema.
  4. User receives a push notification: "Your new store is ready to review!"
  5. User reviews the mobile preview and taps "Approve."
  6. (Optional/Future Phase) The agent provides instructions or automates the DNS switch to make the OHC store live.

  **Acceptance Criteria:**
  *   Implement the core Migration Agent logic capable of receiving a URL and initiating the extraction process.
  *   Ensure the extracted data is securely mapped to the correct `tenant_id` respecting all PostgreSQL RLS policies.
  *   The system must correctly identify and import product variants (e.g., Size, Color) and link associated images to the OHC edge cache.
  *   Provide an endpoint/mechanism to generate the staging preview and handle the user's final approval event.
  *   DO NOT expose any CSV import UI, mapping tables, or technical configuration screens to the user.

  ## Priority
  **P1 (High)** - Essential for accelerating user acquisition from legacy platforms and reducing onboarding friction for established businesses.

  ## Estimated Scope
  **Large** - Requires robust scraping/extraction capabilities, intelligent data normalization, coordination between multiple agent departments, and secure staging environments.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
