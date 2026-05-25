issue_title: "Autonomous Multilingual Localization & Cultural Context Mesh"
issue_description: |
  ## Title
  Autonomous Multilingual Localization & Cultural Context Mesh

  ## Problem Statement
  Fatima, a 50-year-old food cart owner with limited English, relies on community members to take pre-orders for her halal cart. Her customers speak English, but she manages her business in Arabic. Existing platforms force her to either use an interface in English (which she struggles with) or manually translate her menu, orders, and customer messages. This barrier creates extreme friction, leading to missed orders, confusion on pickup times, and an inability to operate a digital storefront effectively. She needs a system that invisibly translates everything—from the backend UI she uses to the messages she receives—while presenting a perfect, localized English experience to her customers.

  ## Research Report

  ### Competitive Landscape
  *   **Shopify:** Offers multilingual storefronts via third-party apps (e.g., Langify, Weglot), but the admin interface is rigidly set to the user's primary language. Translation is a manual, batch process rather than real-time, conversational, and context-aware.
  *   **Wix / Squarespace:** Similar to Shopify, they allow multiple languages for the customer-facing site, but require manual setup and do not translate inbound customer communications dynamically.
  *   **GoDaddy:** Basic site translation, lacks any deep integration for real-time order translation or admin localization.

  ### The OHC Gap
  Reviewing the current architecture docs, OHC has robust structures for Omnichannel Unified Inbox, Instant Localized Invoicing, and Realtime Multilingual KDS Preorder Engine. However, there is a missing foundational layer: the "Mesh" that sits between the Tenant (Fatima), the System (OHC APIs), and the Customer. We lack an autonomous engine that intercepts all text (UI, catalog, inbound messages, outbound notifications) and translates it seamlessly, maintaining cultural nuances and formatting (e.g., RTL support, currency, date formats) without the user ever explicitly managing a "translation" tool.

  ## Design Doc

  ### Architecture Diagram

  ```mermaid
  erDiagram
      TENANT ||--o{ LOCALIZATION_PROFILE : defines
      LOCALIZATION_PROFILE {
          string tenant_id PK
          string primary_language
          string target_customer_languages
          boolean rtl_enabled
      }
      TENANT ||--o{ CATALOG_ITEM : manages
      CATALOG_ITEM ||--|{ LOCALIZED_TEXT : contains
      LOCALIZED_TEXT {
          string entity_id PK
          string entity_type "Catalog | Message | UI"
          string language_code
          string translated_content
      }
      INBOUND_MESSAGE ||--o| TRANSLATION_EVENT : triggers
      TRANSLATION_EVENT {
          string source_text
          string source_lang
          string target_lang
          string translated_text
      }
  ```

  ```mermaid
  sequenceDiagram
      participant Customer
      participant Storefront (English)
      participant Mesh as Multilingual Localization Mesh
      participant Agent as AI Translation Agent
      participant Dashboard (Arabic)
      participant Fatima

      Customer->>Storefront: Views menu (English)
      Storefront->>Mesh: Fetch Catalog (Target: English)
      Mesh->>Agent: Translate if not cached
      Mesh-->>Storefront: Returns English Catalog
      Customer->>Storefront: Submits pre-order "No onions, extra spicy"
      Storefront->>Mesh: Inbound Order Note
      Mesh->>Agent: Translate "No onions..." to Arabic
      Mesh-->>Dashboard: Pushes order in Arabic
      Dashboard->>Fatima: Sees "بدون بصل، حار جداً"
      Fatima->>Dashboard: "سيكون جاهزاً خلال 10 دقائق" (Ready in 10 mins)
      Dashboard->>Mesh: Outbound notification
      Mesh->>Agent: Translate to English
      Mesh-->>Customer: SMS: "Your order will be ready in 10 minutes."
  ```

  ### Mobile UX Flow (375px First)
  1. **The Onboarding Flow:** When Fatima first creates her account, she selects her primary language (Arabic). The entire UI immediately switches to RTL (Right-to-Left) and Arabic text.
  2. **The Catalog Creation:** She snaps a photo of her menu. The AI extracts the items in Arabic. A small, elegant "Globe" icon appears, indicating the items have been automatically translated into her local region's dominant languages (e.g., English, Spanish).
  3. **The Inbox Experience:** When an English-speaking customer sends a message ("Can I pick up at 5?"), the message appears in Fatima's inbox in Arabic. A subtle dual-language toggle allows her to see the original English if desired. Her Arabic reply is instantly sent back to the customer in English.
  4. **The Customer View:** The customer interacts with the storefront, cart, and SMS notifications entirely in English. They have no idea the backend is operating in Arabic.

  ### AI Agent Integration Points
  *   **AI Translation Department (Core):** A dedicated background agent that intercepts all textual data mutations. It handles real-time translation using context-aware LLMs, ensuring culinary terms ("shawarma", "white sauce") are translated correctly.
  *   **AI Operations Department:** Utilizes the translated text for printing KDS tickets and routing orders correctly.
  *   **AI Customer Success Department:** Reads translated inbound messages to draft suggested replies in the tenant's primary language.

  ### Key Design Decisions & Integrity
  *   **Zero-Config Translation:** Translation happens implicitly on read/write boundaries. The user never "submits a batch for translation."
  *   **Edge-Cached Localization:** Storefront catalog translations are heavily cached at the edge (CDN) to maintain the < 1s load time mandate.
  *   **RTL and Layout Integrity:** The system must enforce strong layout primitives that seamlessly handle Left-to-Right and Right-to-Left text flipping without breaking the Translucent Glass UI or unified dashboard cards.
  *   **Cultural Context:** The AI doesn't just translate words; it localizes formats (e.g., 24-hour vs 12-hour time, currency symbols).

  ## Implementation Prompt
  **To Implementer Agent:**
  Implement the core Multilingual Localization Mesh. You must design the data layer and interception middleware that automatically translates content between the Tenant's primary language and the Customer's preferred language.

  The system must:
  1. Define a `LocalizationProfile` for each tenant, specifying their primary language and UI layout preference (RTL/LTR).
  2. Create middleware that intercepts all reads/writes to `CatalogItem` and `Message` entities. If the requested language differs from the stored language, the middleware should invoke the AI Translation service.
  3. Ensure translations for static entities (like catalogs) are stored and heavily cached (e.g., using a `LocalizedText` table/cache) to prevent redundant AI calls and guarantee low latency.
  4. Provide a mechanism for real-time translation of ephemeral data (like chat messages) that operates asynchronously, delivering the translated payload via WebSocket/Server-Sent Events to the mobile dashboard.
  5. All database access must be strictly scoped by `tenant_id`.

  Do not implement the actual LLM integration or the frontend UI. Focus on the middleware architecture, the data schema for storing translated variants, and the caching strategy.

  **Acceptance Criteria:**
  * A Tenant can set their primary language.
  * Fetching a catalog item with an `Accept-Language` header different from the tenant's primary language returns translated content (mocked AI response).
  * Repeated fetches for the same translated item hit a cache and do not invoke the translation service.
  * Inbound chat messages are intercepted and a translated event is emitted.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
