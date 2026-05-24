issue_title: "Universal AI Translation & Localization Mesh"
issue_description: |
  # Universal AI Translation & Localization Mesh

  ## Problem Statement
  OneHumanCorp's vision is to empower any small business owner to run their business from their phone without touching code. One of our core personas, **Fatima (food cart, 50, limited English)**, needs an Arabic and English UI, but traditional localization strategies rely on static dictionaries, manual translation of content, and brittle i18n keys that fail when dealing with user-generated content, dynamic product menus, or real-time conversational interactions. Furthermore, her customers may speak multiple different languages. Currently, if Fatima uploads a menu item in Arabic, English-speaking customers cannot read it, and she struggles to navigate English-only dashboard metrics or support interactions. This language barrier introduces immense friction, preventing non-native English speakers from confidently operating or scaling their business.

  ## Research Report
  ### Competitive Landscape
  *   **Shopify:** Relies heavily on third-party translation apps. Does not offer deep, real-time native UI translation of dynamic inventory and customer messages natively without significant configuration and cost.
  *   **Wix:** Manual multi-lingual setups. Very rigid.
  *   **Stripe / Square:** Localized interfaces for the merchant, but poor real-time translation for custom business data (like menu items).

  ### Market Data
  *   A significant portion of SMB owners in urban centers are immigrants or non-native English speakers.
  *   Businesses with fully localized storefronts see up to a 70% increase in conversion rates for diverse demographics.
  *   Traditional localization (i18n keys) does not scale to user-generated catalog data and direct conversational commerce interactions (e.g., WhatsApp ordering).

  ### Opportunity
  We can implement a **Universal AI Translation & Localization Mesh** that sits transparently at the edge/persistence layer. Instead of static translation files, an AI Agent pipeline interceptes outgoing data (dashboard UI, storefront catalogs, customer messages) and translates it in real-time based on the viewer's locale, caching the results for performance. This ensures Fatima sees her entire dashboard, including customer orders, in Arabic, while her English-speaking customers see her store and receipts in English.

  ## Design Doc

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  sequenceDiagram
      participant Customer as Customer (English Locale)
      participant Edge as Edge Cache / API Gateway
      participant TranslationMesh as AI Translation Mesh (Edge Cache + LLM)
      participant CoreDB as Core Database (Stores canonical Arabic)
      participant Fatima as Fatima (Merchant, Arabic Locale)

      Customer->>Edge: Request Storefront / Menu (Locale: EN)
      Edge->>TranslationMesh: Check Cache for Menu (Source: AR, Target: EN)
      alt Cache Miss
          TranslationMesh->>CoreDB: Fetch Canonical Data (Arabic)
          TranslationMesh->>LLM: Translate Canonical Data to English
          TranslationMesh-->>TranslationMesh: Cache Translation
      end
      TranslationMesh-->>Edge: Return English Menu
      Edge-->>Customer: Display English Storefront

      Fatima->>Edge: View Dashboard Orders (Locale: AR)
      Edge->>TranslationMesh: Fetch Orders (Source: EN/Mixed, Target: AR)
      TranslationMesh-->>Edge: Return Arabic Dashboard
      Edge-->>Fatima: Display Arabic Dashboard
  ```

  ### Data Model & Invariants Diagram (Mermaid.js)
  ```mermaid
  erDiagram
      ORGANIZATION ||--o{ CANONICAL_CATALOG : owns
      ORGANIZATION ||--o{ AGENT_LOCALIZATION_PREF : configures
      CANONICAL_CATALOG ||--o{ LOCALIZED_EDGE_CACHE : generates

      ORGANIZATION {
          uuid org_id PK
          string primary_locale
          string display_name
      }
      CANONICAL_CATALOG {
          uuid item_id PK
          uuid org_id FK
          string content_hash
          json canonical_payload
          timestamp created_at
      }
      LOCALIZED_EDGE_CACHE {
          string cache_key PK "hash(item_id + target_locale)"
          uuid item_id FK
          string target_locale
          json localized_payload
          timestamp ttl
      }
      AGENT_LOCALIZATION_PREF {
          uuid pref_id PK
          uuid org_id FK
          string agent_role
          string persona_locale
      }
  ```

  ### UI Wireframes (375px Mobile-First) & Mobile UX Flow
  **Screen 1: Magic Language Setup (Onboarding)**
  *   Clean, Translucent Glass card on a 375px screen.
  *   Text: "What language do you prefer to manage your business?"
  *   Large, tap-friendly grid of languages (e.g., English, Español, العربية, 中文).
  *   *Grandmother Test:* Tap the language, and the UI immediately transitions to that language without a reload. No complicated dropdowns or "regional formatting" menus.

  **Screen 2: Multi-lingual Storefront Preview**
  *   Dashboard card showing a preview of a menu item (e.g., "Chicken Over Rice").
  *   A simple toggle at the top: `[ View as English Customer ] | [ View as Arabic Customer ]`.
  *   Shows the automatic translation applied. No manual input required by Fatima.

  ### AI Agent Integration Points
  *   **The Translator (Localization AI):** A background agent department responsible for real-time translation and cultural localization (e.g., formatting currency, dates, right-to-left layout adjustments).
  *   **The Cache Manager (Ops AI):** Manages the eviction and pre-warming of translated content in the edge cache whenever canonical data changes.

  ### Technical Integrity, Security, and Mobile-First Rules
  *   **Zero Trust & SPIFFE/SPIRE Isolation:**
      *   The Edge Cache / Gateway enforces multi-tenant boundary checks based on OIDC token signatures.
      *   Communication between the API server and the AI Translation Mesh agents is secured via mutual TLS (mTLS) backed by SPIFFE workload identities. Agents can only access data belonging to the `org_id` context bound to the current execution block.
      *   The Localized Edge Cache strictly segments cache keys by `tenant_id` to prevent any cross-tenant leakage of translated data.
  *   **Performance Targets:** The translation layer must not add more than 50ms to the 95th percentile response time for cached content. Cache hits bypass the LLM.
  *   **Offline-Capability Targets:** Local translations that were previously accessed must remain available offline via an on-device local cache. New translations while offline will fallback to canonical data if the translated cache is unpopulated.

  ## Implementation Prompt
  **To the Implementer:**
  Design and implement the Universal AI Translation & Localization Mesh. The CUJ involves a merchant (Fatima) entering catalog data in Arabic and managing her business via an Arabic dashboard, while her customers automatically view the storefront and receive receipts in English.

  **Acceptance Criteria:**
  *   **Seamless Translation:** Any string (static UI or dynamic database content) must be translatable dynamically.
  *   **Performance Target:** Cache hits under 50ms p95.
  *   **Mobile-First UX:** The language selection UI must be intuitive on a 375px display and follow the Translucent Glass UniFi design system. No complex settings menus.
  *   **Multi-tenant Isolation:** Translation caches must be strictly isolated per tenant to prevent cross-tenant data leakage. Secure communication using SPIFFE identity must be verified.
  *   **Right-to-Left (RTL) Support:** The UI must flawlessly handle layout flipping for RTL languages like Arabic on mobile.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
