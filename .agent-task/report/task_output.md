issue_title: "Architecture: AI-Automated Multilingual Storefront & Edge Translation"
issue_description: |
  # Research Report: AI-Automated Multilingual Storefront & Menu Localization Architecture

  ## Executive Summary
  This report investigates the architectural gap in multi-language support for OneHumanCorp (OHC), focusing on micro-SME merchants with diverse customer bases or limited English proficiency. The goal is to design an automated, low-latency, and mobile-optimized localization system where AI agents handle real-time translation and cultural adaptation of menus and storefronts, specifically addressing the needs of personas like Fatima.

  ## 1. Market Mapping & Competitor Discovery (Track 1)
  - **Codebase Audit:** OHC currently lacks a native, robust multi-language string caching and on-the-fly translation mechanism for dynamic content like product menus.
  - **Competitor Systems:** Shopify and Wix rely on heavy third-party plugins (e.g., Langify) or manual translation input for multilingual storefronts, creating high friction for non-technical users. Square POS supports multiple languages but lacks automated, context-aware AI translation for menus and descriptions.
  - **The Gap:** There is no centralized, AI-driven capability that automatically translates user-entered menu items and descriptions into multiple languages while ensuring low-latency delivery to low-end mobile devices on slow connections.

  ## 2. OHC Gap & Pain Point Identification (Track 3)
  - **Persona Focus:** Fatima (50, food cart operator). She speaks Arabic and limited English. She needs to manage her menu in Arabic but offer a seamless English (and Arabic) storefront for her diverse customer base.
  - **The Gap:** Fatima would struggle with manual translation forms. She needs to input her menu once in Arabic, and have the system automatically generate, cache, and serve optimized English translations (including right-to-left layout adjustments for the Arabic UI). The storefront must load instantly on low-end smartphones.

  ## 3. Deep Dive Architecture Design (Track 2 & Track 3)

  ### Data Model & Delivery Protocol
  - **Multilingual Content Ledger (PostgreSQL):** A unified `LocalizableContent` schema linked via `tenant_id` and `resource_id`. It stores the original source string and AI-generated translated strings as JSONB or separate rows with language codes (e.g., `ar-SA`, `en-US`).
  - **Edge Caching (Redis/CDN):** Fully translated, static storefront views (JSON payloads and HTML) are aggressively cached at the edge (Cloudflare/CloudFront) and in Redis to guarantee ultra-low latency, crucial for slow network connections.
  - **Progressive Web App (PWA) / Mobile App Strategy:** The PWA detects the customer's browser language. On initial load, it fetches the appropriate language payload. The UI supports dynamic RTL (Right-to-Left) and LTR (Left-to-Right) switching. Image assets (menu photos) are aggressively compressed (WebP) to save bandwidth.

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile App - Arabic] -->|Adds Menu Item| B[OHC Backend]
      B --> C{PostgreSQL}
      B --> D[Marketing Agent]
      D --> E{LLM}
      E -->|Translates to English| D
      D --> C
      D --> F[Redis Edge Cache]
      G[Customer Browser - English] -->|Loads Storefront| F
  ```

  ### UI Wireframes & Screen Flow (375px)
  - **Screen 1 (Merchant View - Arabic):** Form with fields for "Item Name" and "Price", standard OHC "Save" button. Everything aligned RTL.
  - **Screen 2 (Customer View - English):** Product list with translated names and descriptions, lazy-loaded images, and "Add to Cart" button (LTR).

  ### AI Agent Coordination
  - **Marketing & Advertising Agent ("The Promoter"):** Detects when new menu items or descriptions are added. It automatically calls the LLM backend (Gemini/GPT-4o) to generate culturally accurate translations.
  - **Customer Success Agent ("The Ambassador"):** In-app customer inquiries in English are automatically translated to Arabic for Fatima's unified inbox. Fatima's replies in Arabic are translated back to English for the customer.
  - **Operations Agent ("The Manager"):** Manages localized printed daily order lists, ensuring the language format matches Fatima's preference (Arabic).

  ### Mobile-First Implementation
  - **Layout & Typography:** Full support for RTL layouts on 375px screens using OHC's Glassmorphism design tokens. Touch targets remain >= 44x44px.
  - **Low-Data Mode:** Skeleton screens and lazy-loading for menu images. The UI remains functional even when image loading is deferred.

  ## 4. Proposed Implementation Steps & Issue Prompt

  **Feature Name:** OHC Automated Multilingual Storefront & Edge Translation

  **Target Persona:** Fatima the Food Cart Operator

  **Outcome:** Fatima inputs her menu items and prices entirely in Arabic. The OHC system instantly generates an optimized English storefront. An English-speaking customer views the menu seamlessly, places an order, and Fatima receives a notification in Arabic.

  **Critical User Journey (CUJ):**
  1. Fatima opens her OHC app on her low-end Android phone. The UI is in Arabic.
  2. She adds a new menu item: "شاورما دجاج" (Chicken Shawarma) with a photo and price.
  3. In the background, the Marketing Agent translates the item name, generates a culturally appropriate description in English, and saves it to the Multilingual Content Ledger.
  4. The localized JSON payloads are immediately pushed to the Redis edge cache.
  5. An English-speaking customer scans Fatima's QR code on a 3G connection. The English version of the storefront loads instantly (cached at edge).
  6. The customer orders the Chicken Shawarma. Fatima receives an Arabic push notification for the new order.

  **Next Actions for Engineering:**
  - **Step 1:** Implement the `LocalizableContent` table in PostgreSQL with tenant-isolated Row Level Security.
  - **Step 2:** Create an asynchronous job worker (triggered by the Marketing Agent) to auto-translate and populate localized fields upon new product creation.
  - **Step 3:** Implement an edge-cacheable API endpoint that serves localized storefront JSON payloads based on the `Accept-Language` header.
  - **Step 4:** Update the Flutter/PWA frontend to dynamically support RTL/LTR transitions and lazy-load WebP images for low-bandwidth environments.

  **Priority:** P1
  **Estimated Scope:** Medium
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
