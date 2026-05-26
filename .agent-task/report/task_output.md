issue_title: "Universal Multilingual Localization Mesh Architecture"
issue_description: |
  # [Architecture] Universal Multilingual Localization Mesh

  ## Problem Statement
  For users like **Fatima (food cart, 50, limited English)**, running a business often requires catering to a diverse customer base while navigating tools not in their native language. OHC currently lacks a cohesive architectural strategy for dynamic, multi-tenant multilingual support that spans the entire business journey (front-end UI, menus, order notifications, and customer interactions). Without an autonomous localization mesh, Fatima cannot effortlessly maintain a bilingual (Arabic + English) storefront, and AI agents lack the structured context to translate incoming pre-orders or customer messages accurately. This gap forces business owners into manual translation or alienates non-English-speaking customers, directly conflicting with OHC's mission of launching businesses with zero code and zero manuals.

  ## Research Report
  ### Competitive Analysis
  - **Shopify**: Provides robust localization through apps (e.g., Translate & Adapt), but it requires manual configuration, separate content entries for each locale, and often breaks with third-party themes. The AI translation is retroactive, not real-time.
  - **Wix/Squarespace**: Offers basic multilingual capabilities, but it often duplicates pages or requires complex site structures, creating friction for non-technical users.
  - **Square (POS)**: Supports multi-language hardware, but unified messaging and dynamic real-time catalog translation (e.g., updating a menu item in Arabic and automatically reflecting an English translation on the storefront) is absent.

  ### Findings & Opportunity
  The core issue is that localization is typically treated as a presentation layer problem (i18n files). In an Agentic OS, localization must be a **data layer and orchestration problem**. By introducing a Universal Multilingual Localization Mesh, OHC can automatically translate catalogs, synchronize agent context across languages, and dynamically render 375px mobile storefronts based on the buyer's browser locale—while ensuring the merchant (Fatima) only interacts with her native language in the app.

  ## Design Doc

  ### Key Design Decisions
  1. **Agentic Real-Time Translation**: Instead of static translation tables, we employ a hybrid approach: static UI elements are cached at the edge, while dynamic user-generated content (e.g., catalog items, customer DMs) is handled by the AI Operations Department in real-time, caching the result in the localized ledger.
  2. **Multi-Tenant Localization Boundary**: Each tenant defines a `PrimaryLocale` and an array of `SupportedLocales`. The backend automatically normalizes all incoming data to the `PrimaryLocale` for the merchant and localizes outbound data to the customer's detected locale.
  3. **Glassmorphism & RTL Support**: The UI components must inherently support Right-to-Left (RTL) layouts without breaking the macOS-style translucent glass aesthetic.

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  erDiagram
      TENANT ||--o{ LOCALE_CONFIG : configures
      TENANT ||--o{ CATALOG_ITEM : owns
      CATALOG_ITEM ||--o{ LOCALIZED_CONTENT : translates_to
      CUSTOMER_SESSION ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains

      TENANT {
          uuid id
          string primary_locale
      }
      LOCALE_CONFIG {
          uuid tenant_id
          json supported_locales
          boolean auto_translate
      }
      CATALOG_ITEM {
          uuid item_id
          string canonical_name
          string canonical_desc
      }
      LOCALIZED_CONTENT {
          uuid item_id
          string locale
          string localized_name
          string localized_desc
      }
      MESSAGE {
          uuid message_id
          string original_text
          string original_locale
          string translated_text
          string target_locale
      }
  ```

  ```mermaid
  sequenceDiagram
      participant Customer
      participant StorefrontEdge
      participant AI_Communications_Agent
      participant LocalizationMesh
      participant Fatima_App

      Customer->>StorefrontEdge: GET /menu (Locale: en-US)
      StorefrontEdge->>LocalizationMesh: Fetch Menu (target: en-US)
      LocalizationMesh-->>StorefrontEdge: Return English Content
      StorefrontEdge-->>Customer: Display English Menu

      Customer->>StorefrontEdge: Send Order Note ("No onions")
      StorefrontEdge->>AI_Communications_Agent: Ingest Message
      AI_Communications_Agent->>LocalizationMesh: Translate to Tenant Primary Locale (ar-SA)
      LocalizationMesh-->>AI_Communications_Agent: Translated Note ("بدون بصل")
      AI_Communications_Agent->>Fatima_App: Push Notification in Arabic
      Fatima_App-->>Fatima: Reads "بدون بصل"
  ```

  ### Mobile UX Flow (375px First)
  1. **Onboarding (Fatima)**: The app detects the device language (Arabic) and instantly renders the entire UI in RTL Arabic. It asks, "Would you like your customers to see your menu in English too?" (1-tap Yes).
  2. **Catalog Entry**: Fatima adds an item: "شاورما دجاج". She takes a photo. The AI automatically generates an English localized name ("Chicken Shawarma") and description in the background.
  3. **Storefront View (Customer)**: A customer scanning the QR code with an English iOS device sees the premium Glassmorphism menu fully in English.
  4. **Order Reception**: Fatima receives a push notification on her Android phone: "طلب جديد: شاورما دجاج" with the customer's note translated into Arabic.

  ### AI Agent Integration Points
  - **Operations Department**: Automatically translates and enriches catalog items upon creation.
  - **Communications/CS Department**: Intercepts real-time chat and order notes, translating them between the customer's locale and the merchant's primary locale.
  - **AutoDream Pipeline**: Analyzes multi-language interactions to improve future translation accuracy and cultural nuances (e.g., localized slang for food items).

  ## Implementation Prompt
  **For the Implementer Agent:**
  Implement the core data model and middleware for the Universal Multilingual Localization Mesh.
  1. Extend the Tenant provisioning flow to accept and store a `PrimaryLocale` and `SupportedLocales`.
  2. Create a generic `LocalizationMesh` module that can wrap catalog entities and conversation messages, providing automated AI translation fallback when a static localized string is missing.
  3. Update the Edge Storefront rendering pipeline to respect the `Accept-Language` header, serving localized content or requesting real-time translation from the backend if cached content is unavailable.
  4. Ensure all new components strictly enforce the existing multi-tenant isolation (SPIFFE/SPIRE) and fail gracefully to the `PrimaryLocale` if translation services timeout.
  5. Provide a basic RTL layout toggle in the central UI design token configuration.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []