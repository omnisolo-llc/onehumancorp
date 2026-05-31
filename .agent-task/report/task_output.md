issue_title: "Invisible Real-Time Omnilingual Localization and Communication Engine"
issue_description: |
  # [architecture] Invisible Real-Time Omnilingual Localization and Communication Engine

  ## Problem Statement
  Small business owners like **Fatima (the food cart operator)** operate in highly diverse, multilingual neighborhoods. She primarily speaks Arabic and limited English, yet her customers speak English, Spanish, and French. Currently, if she uses a platform like Shopify or Wix, setting up a multi-language menu requires manually translating every item, variant, and description using third-party plugins. Even worse, if a customer asks a question in Spanish via SMS, Fatima cannot easily reply, leading to lost sales and friction. The process is manual, requires technical add-ons, and creates a communication barrier that prevents her from growing her business effectively.

  She needs an engine that instantly and invisibly translates her entire business—storefront, checkout, and most importantly, omnichannel real-time messaging—into the language of the buyer, while allowing her to operate her entire management interface exclusively in Arabic.

  ## Research Report & Competitor Audit
  *   **Shopify:** Multi-language support exists but is bolted on via apps like Langify or built-in basic translation tools. It requires manual input or explicit triggering of bulk translations. Live customer chat translation is not natively integrated.
  *   **Wix:** Offers Wix Multilingual. Again, it is a setup step: the user must manually approve translations for their site. There is no real-time conversational translation for SMS/WhatsApp built into the core inbox.
  *   **Squarespace / GoDaddy:** Extremely rudimentary language support. Not built for real-time localized conversational commerce.
  *   **OneHumanCorp (OHC) Differentiation - "Invisible Omnilingual Autonomy":** OHC eliminates the "translation step." When Fatima creates a menu item in Arabic ("شاورما دجاج"), the AI instantly generates optimized descriptions, tags, and pricing contexts in all supported languages in the background. When a Spanish-speaking customer opens her web link, they see the site in Spanish natively based on browser headers. If they text "Hola, ¿tienes pollo?" to her business number, OHC's Ambassador Agent intercepts, translates the intent, checks inventory, and can automatically reply in Spanish, or translate the incoming text to Arabic for Fatima's mobile dashboard and translate her Arabic reply back to Spanish.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      MERCHANT_INPUT ||--o{ OMNILINGUAL_TRANSLATION_MESH : "Triggers (Create/Update)"
      CUSTOMER_REQUEST ||--o{ OMNILINGUAL_TRANSLATION_MESH : "Ingests (Browse/Message)"

      OMNILINGUAL_TRANSLATION_MESH {
          string spiffe_identity "Zero Trust Routing"
          string tenant_id "Multi-tenant Isolation"
          string source_language "Merchant's primary language"
      }

      OMNILINGUAL_TRANSLATION_MESH ||--o{ AI_LOCALIZATION_WORKERS : "Dispatches"

      AI_LOCALIZATION_WORKERS ||--o{ CACHE_LAYER : "Writes to (Redis/Edge)"
      AI_LOCALIZATION_WORKERS ||--o{ DATABASE : "Persists to (PostgreSQL vector)"

      OMNILINGUAL_TRANSLATION_MESH ||--o{ UNIFIED_INBOX : "Translates Live Messages"
      OMNILINGUAL_TRANSLATION_MESH ||--o{ STOREFRONT_RENDERER : "Serves localized content"
  ```

  ### End-to-End Business Journey (Fatima's CUJ)
  1.  **Onboarding:** Fatima signs up on her low-end Android phone. The app detects her locale/language preference (Arabic).
  2.  **Creation:** She snaps a picture of her Chicken Shawarma and types the price and name in Arabic.
  3.  **Invisible Localization:** The `OMNILINGUAL_TRANSLATION_MESH` takes the input, identifies the core entity, and the `Marketing & Advertising` agent generates localized marketing copy for English, Spanish, etc.
  4.  **Customer Interaction:** A customer scans a QR code. Their phone is set to Spanish. The edge cache instantly serves the Spanish variant of the menu.
  5.  **Conversational Commerce:** The customer sends a WhatsApp message in Spanish asking about allergens. The `Customer Success` Ambassador Agent understands the Spanish intent, checks the Arabic/canonical data model for ingredients, and replies instantly in Spanish: "Sí, es sin gluten."
  6.  **Human Escalation:** If the customer asks a highly complex question, the Spanish text is translated to Arabic in Fatima's unified mobile inbox. She replies in Arabic, and the system translates it back to Spanish for the customer.

  ### Mobile UX Flow (375px Baseline)
  *   **Translucent Glass Material:** Standard OHC premium design system.
  *   **No Translation Toggles:** There is no "Translate Site" button for the merchant. Localization is assumed and automatic.
  *   **Inbox Translation UI:** In the unified inbox, a message bubble from a customer shows the translated text (Arabic for Fatima) with a tiny, subtle spark icon and the original language code (e.g., "ES") indicating it was translated. Tapping the bubble toggles the original source text.

  ### Data Model & Invariants
  *   **Entity Canonicalization:** All user-generated content (Product, Service, Message) is stored with a canonical source language UUID.
  *   **Vector Search Support:** Product descriptions in all languages are embedded via pgvector to allow cross-lingual semantic search (a customer searching "pollo" finds the Arabic-named Shawarma).
  *   **Edge Caching:** Translated storefront views must be pushed to Edge cache with a strict TTL and cache invalidation driven by the core `Operations` agent to guarantee <50ms TTFB globally, even on slow networks.
  *   **Tenant Isolation:** Row-Level Security (RLS) via `tenant_id` on all translation and messaging tables to ensure absolute cross-tenant data privacy.

  ## Implementation Prompt
  Implement the Omnilingual Translation Mesh middleware and backend data model.
  1. Define the PostgreSQL schema for canonical content and localized variants, ensuring RLS.
  2. Implement the gRPC/API layer that intercepts incoming merchant input, triggers background `AI_LOCALIZATION_WORKERS` to generate translations, and updates the cache.
  3. Integrate the translation hook into the existing unified inbox messaging flow so that incoming customer messages are evaluated and automatically translated to the merchant's `source_language` if they differ.
  4. Acceptance Criteria: The system must successfully translate a new product listing into at least 3 target languages asynchronously and accurately translate a bi-directional chat message simulation. Unit tests must cover 100% of the new translation routing logic. Playwright E2E tests must demonstrate a localized checkout flow.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []