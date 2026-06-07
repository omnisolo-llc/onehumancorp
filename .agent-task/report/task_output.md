issue_title: "Autonomous Multi-Language Edge Translation Architecture"
issue_description: |
  # Research Report: Autonomous Multi-Language Edge Translation Architecture

  ## Executive Summary
  This report investigates the architectural gap in multi-language support across the small business platform market. It defines a system for OneHumanCorp (OHC) to seamlessly serve multi-lingual communities (e.g., Arabic/English) through edge-cached, dynamically translated storefronts. The architecture removes the friction of manual translations by employing invisible AI agents to automatically translate inventory, marketing content, and customer communications.

  ## 1. Market Mapping & Competitor Discovery (Track 1)
  Competitors like Shopify and Wix offer multi-language capabilities but heavily rely on third-party plugins (e.g., Langify, Weglot) or require manual setup. These solutions add significant complexity, ongoing subscription costs, and often negatively impact page load times by processing translations client-side. The gap in the market is a platform where localization happens invisibly and instantaneously at the edge, requiring zero technical configuration from the merchant.

  ## 2. OHC Gap & Pain Point Identification (Track 3)
  - **Persona Focus:** Fatima (halal food cart operator).
  - **The Gap:** Fatima serves a diverse local community. She needs her storefront to display seamlessly in both Arabic and English. She does not have the technical knowledge to configure a translation plugin, define fallback languages, or manage translated SEO metadata. She requires a low-latency experience for her customers on varying network connections.

  ## 3. Deep Dive Architecture Design (Track 2 & Track 3)

  ### Architecture Diagram
  ```mermaid
  erDiagram
      MERCHANT ||--o{ PRODUCT : owns
      MERCHANT ||--o{ TRANSLATION_PREFERENCE : defines
      PRODUCT ||--o{ LOCALIZATION_REGISTRY : has_translations
      LOCALIZATION_REGISTRY {
          string tenant_id
          string resource_id
          string language_code
          string translated_text
      }
      AI_TRANSLATION_WORKER ||--|{ LOCALIZATION_REGISTRY : writes
      CDN_EDGE ||--|{ LOCALIZATION_REGISTRY : caches
  ```

  ```mermaid
  sequenceDiagram
      autonumber
      actor Fatima
      participant OHCMobile as OHC Mobile App
      participant DB as PostgreSQL (Central Ledger)
      participant AI as Operations Agent (Gemini Pro)
      participant CDN as Edge CDN
      actor Customer

      Fatima->>OHCMobile: Add new menu item "Spicy Chicken"
      OHCMobile->>DB: Save canonical English item
      DB-->>AI: Trigger translation job (SKIP LOCKED)
      AI->>AI: Translate to Arabic
      AI->>DB: Write to Central Localization Registry
      DB-->>CDN: Purge cache & push multi-language bundle
      Customer->>CDN: Request storefront (Accept-Language: ar)
      CDN-->>Customer: Return edge-cached Arabic storefront (< 100ms)
  ```

  ### Data Model & Delivery Protocol
  - **Central Localization Registry (PostgreSQL):** A multi-tenant translation ledger storing canonical strings and their translated variants. All table rows enforcing Row Level Security with `tenant_id`.
  - **Edge Distribution:** Translated payloads are compiled into static bundles and cached at the CDN Edge (Cloudflare/CloudFront). The edge router identifies the customer's `Accept-Language` header and delivers the appropriately localized static assets immediately, reducing latency.
  - **Fallback Chain:** If a specific dialect is missing (e.g., Arabic-Lebanese), the edge router falls back to the generic language code (e.g., Arabic), and finally to the tenant's primary language.

  ### AI Agent Coordination
  - **Operations Agent ("The Manager"):** Detects when a new product or service is added to the catalog and queues a translation job.
  - **Marketing Agent ("The Promoter"):** Translates product descriptions, SEO metadata, and generated social media posts into the active secondary languages.
  - **Customer Success Agent ("The Ambassador"):** Intercepts incoming multi-lingual customer DMs, translates them into the merchant's native language, drafts a response, translates the response back to the customer's language, and presents the bilingual action card for approval.

  ### Mobile-First & Edge Implementation
  - **Merchant UX:** A simple toggle in the 375px mobile UI: "Enable Arabic." No manual entry required. The Operations Agent performs the translation asynchronously.
  - **Customer UX:** The edge-delivered PWA loads instantly, defaulting to the phone's native language without flash-of-untranslated-text (FOUT).

  ## 4. Proposed Implementation Steps & Issue Prompt

  **Feature Name:** OHC Edge-Cached Multi-Language Engine

  **Target Persona:** Fatima the Food Cart Operator

  **Outcome:** Fatima adds a new menu item in English. The AI agents instantly translate the item name, description, and price formatting into Arabic. The edge cache updates, allowing her Arabic-speaking customers to view the fully localized menu instantly with zero configuration from Fatima.

  **Critical User Journey (CUJ):**
  1. Fatima logs into the OHC mobile app and adds "Spicy Chicken Over Rice" to her menu.
  2. The system triggers a Postgres `SKIP LOCKED` job in the AI Queue.
  3. The Operations Agent processes the job, employing Gemini Pro to translate the item details into Arabic.
  4. The translation is written to the Central Localization Registry.
  5. The cache invalidator purges the edge cache, and the new multi-language bundle is pushed to the CDN.
  6. An Arabic-speaking customer visits the link-in-bio on their low-end Android device. The CDN inspects the `Accept-Language` header and serves the Arabic storefront with < 100ms latency.

  **Next Actions for Engineering:**
  - **Step 1:** Implement the Central Localization Registry in PostgreSQL with `tenant_id` isolation.
  - **Step 2:** Create the AI Agent background worker capable of translating entire catalog schemas asynchronously.
  - **Step 3:** Design the edge routing logic to detect `Accept-Language` and serve cached localized payloads.
  - **Step 4:** Implement the mobile UI toggle for merchants to seamlessly enable new languages.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
