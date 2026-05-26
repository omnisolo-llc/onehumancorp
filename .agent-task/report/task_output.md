issue_title: "Autonomous Zero-Touch Business Migration & Import Engine"
issue_description: |
  ## Problem Statement
  Migrating an existing business to a new platform is universally painful. For non-technical small business owners, this is the single biggest barrier to adopting a new system.

  *   **Priya (Boutique Owner):** Wants to move 1,000 SKUs (with size/color variants) from her clunky Square POS to OneHumanCorp (OHC) to get better online integration. Currently, this requires exporting a complex CSV, understanding field mapping, manually uploading images, and fixing formatting errors. She simply doesn't have the time or technical skills.
  *   **Maya (Baker):** Sells exclusively via Instagram DMs. She doesn't have a "catalog" to export. Her catalog is her Instagram grid. Moving to a real platform currently means manually saving photos, writing descriptions from scratch, and pricing everything one by one.
  *   **Fatima (Food Cart):** Only has a printed physical paper menu and a few handwritten price updates.

  Current platforms force these users to conform to rigid data ingestion formats. OHC must reverse this: the platform must conform to whatever messy data the user already has. We need an engine that can ingest a CSV, an Instagram URL, a PDF menu, or a blurry photo, and autonomously structure it into a multi-tenant catalog in under 10 minutes.

  ## Research Report
  **Competitor Analysis:**
  *   **Shopify:** Relies heavily on rigid CSV imports. If a single column header is wrong, the entire import fails with cryptic error messages. Users often have to pay for third-party apps just to migrate data.
  *   **Wix/Squarespace:** Similar reliance on CSVs or direct API integrations with specific competitors. No ability to parse unstructured data (like a menu photo or Instagram feed).
  *   **OHC AI Vision:** The system acts as a white-glove migration concierge. The user provides the raw material (a link, a file, a photo), and the AI handles data extraction, cleaning, variant generation, and pricing normalization, requiring only a final 1-tap approval from the user.

  ## Design Doc
  ### Architecture Diagram

  ```mermaid
  graph TD;
      A[Mobile Client] -->|Upload: Image/PDF/CSV/URL| B(Ingestion Gateway)
      B --> C{Data Type}
      C -->|Unstructured Image/PDF| D[Vision AI Extraction Model]
      C -->|Unstructured URL/IG| E[Web Scraper & Parsing Agent]
      C -->|Structured CSV| F[CSV Mapping Agent]

      D --> G(Ops AI Agent: Normalization & Structuring)
      E --> G
      F --> G

      G --> H{Finance AI Agent: Pricing/Currency Check}
      H --> I[Draft Catalog Ledger]

      I --> J[User Dashboard: 1-Tap Approval Review]
      J -->|Approve| K[(Universal Catalog Ledger)]

      style K fill:#4CAF50,stroke:#388E3C,stroke-width:2px;
      style G fill:#2196F3,stroke:#1976D2,stroke-width:2px;

      subgraph Multi-Tenant Boundary
      D
      E
      F
      G
      H
      I
      K
      end
  ```

  ### UI Wireframes & Screen Flow (375px Mobile First)
  Adhering to macOS-style Translucent Glass materials and clean Ubiquiti UniFi modular dashboard cards.

  **Screen 1: The Ask**
  *   Header: "Let's move your business in 3 minutes."
  *   Content: A simple, conversational chat UI.
  *   Input Options (Large tap targets):
      *   [ 📸 Take a photo of your menu/price list ]
      *   [ 🔗 Paste your Instagram or website link ]
      *   [ 📄 Upload a CSV or PDF file ]
  *   No complex forms or mappings.

  **Screen 2: The Magic (Loading State)**
  *   Header: "Organizing your catalog..."
  *   Content: Shimmering placeholder cards.
  *   Micro-copy updating in real-time: "Found 45 items...", "Extracting prices...", "Enhancing photos..."

  **Screen 3: The Review**
  *   Header: "Ready to launch!"
  *   Content: A scrollable grid of generated product cards (Image, Title, Price, Variants).
  *   Action: A single massive [ Approve & Go Live ] button at the bottom.
  *   Secondary Action: A small "Edit" icon on individual cards for quick tweaks.

  ### Mobile UX Flow
  1. User taps "Import Existing Business" during onboarding.
  2. User selects input method (e.g., snaps a photo of their menu).
  3. The app uploads the asset securely to the Ingestion Gateway.
  4. User waits ~30-60 seconds on a dynamic loading screen.
  5. User reviews the structured list, makes any minor text/price adjustments inline, and taps Approve.
  6. Items are instantly available on their storefront and POS.

  ### AI Agent Integration Points
  *   **Operations AI Agent:** The core engine. It receives raw text/images, identifies what is a product, what is a description, what is a price, and what are variants (e.g., recognizing "S/M/L" implies size variants).
  *   **Finance AI Agent:** Reviews extracted prices to ensure they match the user's localized currency settings and flags anomalies (e.g., a $400 cupcake is probably a typo for $4.00).

  ### Key Design Decisions
  *   **Zero Trust & Multi-Tenant Isolation:** Every uploaded asset, extraction job, and draft catalog entry must be strictly scoped to the `tenant_id`. Migration jobs for Tenant A cannot read data or model context from Tenant B.
  *   **Asynchronous Processing with Optimistic UI:** Data extraction (especially from URLs or large PDFs) takes time. The backend must use a high-performance background job queue. The mobile UI should poll or use WebSockets for real-time progress updates without blocking the main thread.
  *   **"Grandmother Test" Compliance:** Absolutely no mapping UI ("Map column 'Price' to database field 'unit_amount'"). The AI must figure it out. If it's unsure, it leaves it blank for the user to fill in during the review step.

  ## Implementation Prompt
  **Target Audience:** Implementer Agent
  **Task:** Build the Autonomous Zero-Touch Business Migration & Import Engine. You must create the API endpoints, background job workers, and multi-tenant data structures to ingest raw data (images, PDFs, CSVs, or URLs), parse them using the AI Operations Agent, and generate draft catalog entries for user review.

  **Acceptance Criteria:**
  1. A secure, multi-tenant ingestion API that accepts various file types and URLs.
  2. Integration with the AI Operations Agent to parse unstructured text/images into structured product entities (Title, Description, Price, Variants).
  3. A background job mechanism to handle the parsing asynchronously without timing out mobile client requests.
  4. An API endpoint to fetch the status of an ongoing migration and retrieve the "draft" structured catalog.
  5. An endpoint to "commit" the draft catalog to the active Universal Catalog Ledger upon user approval.
  6. Strict Zero-Trust multi-tenant isolation on all database queries and job queue items.

  Do not prescribe specific ORMs, SQL schemas, or lower-level libraries. Focus on the API contract, the asynchronous event flow, and the multi-tenant safety guarantees.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []