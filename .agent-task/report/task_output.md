issue_title: "[Architecture] Invisible Physical-to-Digital Bridge (Dynamic QR & NFC Mesh)"
issue_description: |
  # Architecture Brief: Invisible Physical-to-Digital Bridge (Dynamic QR & NFC Mesh)

  ## Problem Statement
  Small business owners straddle the physical and digital worlds, but bridging them is difficult. Carlos (the handyman) installs a water heater, but when it breaks two years later, the customer doesn't remember who installed it. Priya (the boutique owner) has beautiful dresses on racks but struggles to capture the emails of window shoppers who leave without buying. Fatima (the food cart operator) deals with long lines and wants customers to order from their phones while waiting. Generating static QR codes is tedious, they often break or point to generic homepages, and printing NFC tags feels like "developer magic" out of reach for regular people. The bridge between the real world and their online OHC ecosystem is fundamentally broken.

  ## Research Report
  - **Competitive Benchmark**:
    - **Shopify**: Provides "Shopcodes" but they are basic static links to products. Requires a specific app and manual printing.
    - **Wix/Squarespace**: Offers basic QR code generators, but they simply link to a static URL. No context or lifecycle management.
    - **Square**: Good for table-side ordering, but terrible for service-based post-installation flows or retail context capturing.
  - **Market Gap**: No platform seamlessly integrates a *dynamic* asset (NFC/QR) engine directly into the business owner's physical workflow with zero setup.
  - **Opportunity**: OHC can automatically generate and associate unique, context-aware digital touchpoints (QR/NFC) for *every* product, service, and invoice. Scanning a code placed by Carlos on a water heater instantly opens an AI support chat with his context. Scanning Fatima’s food cart QR instantly brings up the localized menu and ordering flow.

  ## Design Doc

  ### High-Level Architecture (Mermaid.js)
  ```mermaid
  graph TD
      Physical[Physical Touchpoint: QR/NFC] --> EdgeCache[OHC Edge CDN]
      EdgeCache -->|Resolve Entity Identity| Router[Phygital Routing Mesh]
      Router -->|Context: Product/Service| AgentRuntime[AI Conversational Receptionist]
      Router -->|Context: Table/Cart| Checkout[Omnichannel Checkout Engine]
      Router -->|Context: Support/Warranty| Inbox[Unified Omnichannel Inbox]

      AgentRuntime <--> Memory[Tenant Context & Ledger]
      Checkout <--> Memory

      Inbox -->|Push Notification| App[OHC Mobile App - Merchant]
  ```

  ### Data Model & Invariants (Mermaid.js ER Diagram)
  ```mermaid
  erDiagram
      MERCHANT ||--o{ PHYSICAL_TOUCHPOINT : generates
      MERCHANT {
          string tenant_id
          string business_name
      }
      PHYSICAL_TOUCHPOINT ||--o| PRODUCT_CONTEXT : links_to
      PHYSICAL_TOUCHPOINT ||--o| INVOICE_CONTEXT : links_to
      PHYSICAL_TOUCHPOINT {
          string touchpoint_id
          string hash_id
          string qr_image_url
          string type
          boolean is_active
      }
      PRODUCT_CONTEXT {
          string product_id
          string fallback_url
      }
      INVOICE_CONTEXT {
          string invoice_id
          date service_date
      }
      PHYSICAL_TOUCHPOINT ||--o{ SCAN_EVENT : records
      SCAN_EVENT {
          string scan_id
          datetime scanned_at
          string buyer_device_hash
      }
  ```

  ### Mobile UX Flow (375px First)
  1. **The "Print" Trigger**: In the OHC Mobile App, Maya views a product ("Vegan Chocolate Cake") or Carlos views an invoice ("Water Heater Install"). A prominent, jargon-free button reads: "Create Smart Label" or "Print QR."
  2. **Instant Output**: A beautiful, print-ready digital card with a QR code is generated instantly. It clearly indicates what it does (e.g., "Scan to reorder this exact cake" or "Scan for 24/7 support with Carlos").
  3. **The Buyer Experience**: The customer scans the QR with their native smartphone camera. A lightning-fast, edge-cached web-app opens natively (no app install).
     - For Carlos: An AI chat interface opens: "Hi, I'm Carlos's assistant. Need help with the water heater installed on Oct 12?"
     - For Fatima: The food menu opens directly to checkout.
  4. **Dashboard View (Advanced)**: Business owners can see an activity feed: "Your water heater tag in Austin was scanned. AI assistant scheduled a maintenance visit."

  ### AI Agent Integration Points
  - **Phygital Context Agent**: Sits between the edge router and the user. When a tag is scanned, it reads the exact parameters (which product, when it was sold, to whom) and dynamically generates a personalized landing page or chat interface.
  - **Marketing Agent**: Automatically tracks conversion rates of different physical touchpoints and suggests improvements ("Customers who scan your shop window QR often leave. Let's offer a 10% instant discount").

  ### Key Design Decisions
  - **Dynamic Routing Over Static URLs**: QR codes never point directly to a static product page. They point to an OHC edge-resolver that dynamically routes the user based on the current context (e.g., if out of stock, it opens a waitlist chat; if available, it opens checkout).
  - **Zero-Config NFC/QR Generation**: The OHC app automatically handles the generation, sizing, and styling of QR codes so they look professional and are ready to print from any mobile device or Bluetooth thermal printer.
  - **Privacy-First Context & Zero Trust Security**: When scanning, the user's location and identity are protected via zero-trust architecture. Multi-tenant isolation is enforced at the DB level (`tenant_id`), and all inter-agent communications for context gathering are cryptographically signed via SPIFFE/SPIRE, ensuring Tenant A's physical touchpoints can never leak data to Tenant B.

  ## Implementation Prompt
  **To Implementer Agent:**
  Implement the "PhygitalMesh" capability. Build an edge-level URL resolver that maps unique, physical-world identifiers (QR/NFC hashes) to dynamic OHC contexts (products, invoices, bookings). Create a mobile-first (375px) UI where the merchant can generate these codes with a single tap from any entity (product, order, service). Ensure that the resulting buyer experience is instant, loads under 1.5s on 4G, and connects directly to the relevant AI Agent (e.g., Support or Checkout) without requiring the buyer to download an app. Ensure the generation flow is "grandmother tested" and seamlessly integrates with standard mobile printing options.

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
