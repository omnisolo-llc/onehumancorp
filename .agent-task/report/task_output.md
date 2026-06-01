issue_title: "Universal AI Dynamic Catalog Localization & Translation Mesh"
issue_description: |
  # Universal AI Dynamic Catalog Localization & Translation Mesh

  ## Problem Statement
  Small business owners serving diverse, multilingual communities—like Fatima, who operates a halal food cart and requires Arabic and English support—struggle with manually translating their product catalogs, menus, and service descriptions. Updating sold-out toggles, adjusting prices, and adding new items across multiple languages creates significant friction. Non-technical users need an automated system that intelligently auto-translates, localizes, and instantly synchronizes their entire storefront and backend operations without them ever touching a language setting. If a customer views the menu in Spanish, it should instantly appear perfectly translated, while Fatima still manages everything in her preferred language on her low-end Android phone.

  ## Research Report
  Our target users encompass various profiles, including Maya (baker), Carlos (handyman), Priya (boutique owner), Leo (music tutor), and especially Fatima (food cart). A core missing capability across OneHumanCorp is native, zero-configuration dynamic localization of user-generated content (catalogs, service listings, menus, and booking calendars).

  **Competitive Analysis:**
  - **Shopify:** Requires third-party apps (e.g., Translate & Adapt) or complex manual configuration to enable multi-language storefronts. Not zero-touch.
  - **Wix:** Has Wix Multilingual, but it requires the user to manually trigger translations, review them, and manage separate language versions of pages. High friction for mobile-only users.
  - **Squarespace:** Multi-language is cumbersome, often requiring external integrations like Weglot, which adds cost and complexity.
  - **GoDaddy:** Offers basic translation but lacks contextual AI awareness for culturally relevant descriptions and dynamic syncing.

  **The Gap in OHC:**
  Currently, our architecture handles multi-tenancy and high-performance edge-caching but lacks a foundational data model and agent integration for "Localization as a Service" (LaaS). A business owner should be able to input an item ("Spicy Chicken Over Rice") and the system should automatically generate contextualized, culturally appropriate translations for any requested locale on the fly or pre-computed at the edge. The system needs to support Bi-Directional (RTL/LTR) layouts natively.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      subgraph Mobile Client 375px
          FatimaApp[Fatima's Android POS]
          CustomerBrowser[Customer Mobile Browser]
      end

      subgraph OHC Edge & API Mesh
          EdgeCache[Global Edge Cache Config]
          Gateway[API Gateway / Auth]
          LocaleDetector[Auto-Locale Detector]
      end

      subgraph Core Persistence & Services
          SIPDB[(SQLite SIPDB Local Sync)]
          PGVector[(Postgres Catalog Data)]
          TranslationQueue[NATS Background Job Queue]
      end

      subgraph KAIROS AI Swarm
          MarketingAgent[Marketing & Content Agent]
          CSAgent[Customer Success Agent]
      end

      FatimaApp -->|Adds 'Chicken Over Rice' in Arabic| Gateway
      Gateway --> PGVector
      Gateway --> TranslationQueue

      TranslationQueue -->|Trigger localization| MarketingAgent
      MarketingAgent -->|Generate En, Es, etc.| PGVector

      CustomerBrowser -->|Request Storefront| EdgeCache
      EdgeCache -->|Cache Miss / Pre-fetch| LocaleDetector
      LocaleDetector -->|Fetch Locale Data| PGVector
      PGVector --> EdgeCache
      EdgeCache -->|Serve Translated UI| CustomerBrowser

      classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
      class FatimaApp,CustomerBrowser,EdgeCache,Gateway,LocaleDetector,SIPDB,PGVector,TranslationQueue,MarketingAgent,CSAgent premium;
  ```

  ### UI Wireframes / Screen Flow Description (375px First)
  **Owner View (Fatima's Android Phone):**
  - **Catalog List Screen:** Standard view. A grid of product cards. A floating "+" button at the bottom.
  - **Add Product Screen:**
    - Image uploader at the top.
    - "Item Name" text field (Fatima types in Arabic).
    - "Price" numeric field.
    - "Save to Menu" massive, easy-to-tap button.
    - *No language selectors.* The system implicitly knows her default language.

  **Customer View (Browser):**
  - **Storefront Home:** Clean layout. A discreet but accessible translucent floating action button in the bottom right corner showing the current language (e.g., "🌐 EN").
  - **Auto-Detection:** On first load, the browser language is detected. The entire UI, including product names and descriptions, instantly renders in the user's language. If Arabic, the layout automatically flips to Right-to-Left (RTL).

  ### Mobile UX Flow
  1. Fatima taps "+" and takes a photo of a new dish.
  2. She types the name in her native language and enters a price.
  3. She taps "Save".
  4. *Invisible Magic:* The `MarketingAgent` instantly processes the image and native text, extracting context, generating SEO-friendly descriptions, and auto-translating the core entities into top target languages (e.g., English, Spanish) in the background.
  5. A customer scanning a QR code on the food cart opens the site. Their phone is set to English. The site loads instantly from the edge cache, displaying the menu perfectly in English.

  ### AI Agent Integration Points
  - **Marketing & Content Agent:** Acts on a pub/sub event when a new catalog item is created or updated. It performs translation, cultural localization (ensuring idioms make sense), and generates descriptive copy.
  - **Customer Success Agent:** If a customer replies or asks a question in a different language via the unified inbox, this agent auto-translates the message for the owner and translates the owner's reply back to the customer.

  ### Key Design Decisions & Why
  - **Zero-Config Localization:** We do not expose translation toggles or language setup to the owner during item creation. This passes the "grandmother test." It removes all cognitive load.
  - **Asynchronous Translation via NATS:** Translations are processed in the background to ensure the owner's "Save" action is instant (under 200ms latency).
  - **Edge Caching by Locale:** Storefront reads are cached at the edge per locale to hit strict performance targets, critical for mobile web users on 3G/4G networks.
  - **RTL Support Baked In:** The UI layout engine must support bi-directional text natively so languages like Arabic don't break the glassmorphism design.

  ## Implementation Prompt
  **Role:** Implementer Swarm
  **Context:** We need to implement the Universal AI Dynamic Catalog Localization & Translation Mesh to enable zero-touch, invisible translation for our business owners.

  **User Journey (CUJ):**
  1. The business owner adds or updates a catalog item in their native language (e.g., Arabic).
  2. The item saves instantly.
  3. In the background, the system translates the item's name and description into a predefined set of target languages using the AI swarm.
  4. A customer visits the storefront. The system detects their locale.
  5. The customer sees the storefront perfectly translated into their language.

  **Acceptance Criteria:**
  - Create the core data model required to store localized variants of catalog items (e.g., product names, descriptions) without duplicating the underlying inventory or pricing data.
  - Ensure strict multi-tenant isolation rules (data must be partitioned by `organization_id`).
  - Set up the background event trigger (e.g., NATS) that signals the AI agent department to perform translations when a catalog entity changes.
  - Implement the edge-friendly retrieval pattern to fetch the localized catalog based on the requester's locale.
  - Ensure the frontend design system supports RTL layouts out-of-the-box for languages like Arabic.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []