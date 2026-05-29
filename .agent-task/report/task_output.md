issue_title: "[Architecture] Autonomous Multilingual Localization Mesh"
issue_description: |
  # Autonomous Multilingual Localization Mesh

  ## Problem Statement
  Small business owners serving diverse communities often struggle with language barriers. For example, Fatima (50), runs a halal food cart and is most comfortable operating her business in Arabic. However, her customers often speak English or Spanish. She needs to manage her menu, read incoming orders, and reply to customer inquiries in Arabic, while the customers need to see the storefront, menu items, and communications in their preferred languages. Existing platforms require manual translation of every product, variant, and UI string, which is overwhelming, time-consuming, and prone to error for a non-technical user. OneHumanCorp (OHC) needs a seamless, zero-touch Multilingual Localization Mesh that invisibly handles translation and cultural adaptation across the entire platform.

  ## Research Report
  ### Current Landscape
  - **Shopify / Wix / Squarespace**: Localization relies on third-party apps (e.g., Weglot, Langify) or manual input of multiple language variants. These solutions often focus on the storefront but fail to localize the back-office dashboard, leaving merchants like Fatima struggling to operate the platform itself.
  - **Pain Points**: Manual data entry for every product in multiple languages, disjointed customer communication (e.g., getting an English DM and not knowing how to reply), and lack of localized notifications.

  ### The OHC Opportunity
  By leveraging LLMs dynamically, OHC can create an "Autonomous Multilingual Localization Mesh". This system sits as a transparent proxy layer between the core data model and the user interfaces (both merchant dashboard and customer storefront). It automatically detects the user's preferred language, translates catalog items, descriptions, and UI elements on the fly (with intelligent caching), and seamlessly translates bidirectional communication (e.g., an English customer DM is translated to Arabic for Fatima; her Arabic reply is translated back to English for the customer).

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      subgraph Merchant_Device
          M[Merchant Dashboard UI - Arabic]
      end

      subgraph Customer_Device
          C[Customer Storefront UI - English/Spanish]
      end

      subgraph OHC_Multilingual_Localization_Mesh
          API[API Gateway / Router]
          Cache[(Localization Cache Redis)]
          AIT[AI Translation Engine Provider]
          Context[Context & Tone Memory]
      end

      subgraph OHC_Core
          DB[(Core Database Ledger)]
          Agent[Operations AI Agent]
      end

      M <--> API
      C <--> API
      API <--> Cache
      Cache -- Miss --> AIT
      AIT <--> Context
      API <--> Core
      AIT --> Cache
      API <--> Agent
  ```

  ### UI Wireframes & Screen Flow (375px Mobile First)
  **Merchant Experience (Fatima's Arabic View):**
  - **Dashboard**: All cards (Sales, Orders, Notifications) use RTL (Right-to-Left) layout natively and display Arabic text. The macOS-style translucent glass UI adapts elegantly to RTL.
  - **Product Edit**: Fatima enters a new menu item "شاورما دجاج" (Chicken Shawarma). She doesn't need to specify English.
  - **Order Notification**: "New Order: Chicken Shawarma - No Onions" arrives. The mesh translates the customer's "No Onions" request into Arabic instantly on her screen.

  **Customer Experience (English View):**
  - **Storefront**: The menu dynamically displays "Chicken Shawarma" with an AI-generated appetizing description in English.
  - **Checkout**: All flow elements are in the customer's browser locale.
  - **Chat**: Customer asks, "Is the chicken halal?" The mesh routes this, translates it to Arabic for Fatima, and translates her "Yes, certified" response back to English.

  ### Mobile UX Flow
  1. **Implicit Language Detection**: The merchant app detects the device locale (or allows a 1-tap override during onboarding).
  2. **Zero-Touch Catalog**: The merchant adds an item in their native language. Background agents immediately generate cached translations for the top 5 local languages based on geo-AI discovery.
  3. **Unified Inbox**: Chat bubbles show the native language text prominently, with an optional subtle "Translated from English" badge.

  ### AI Agent Integration Points
  - **Translation Operations Department**: Background agent responsible for continuous localization of new catalog items, updating the Redis cache.
  - **Customer Service Agent**: Intercepts DMs/chats, detects language, translates, and drafts suggested replies in the merchant's native language.
  - **Context & Tone Memory**: Ensures that "shawarma" isn't mistranslated awkwardly, preserving the culinary and cultural context.

  ### Key Design Decisions
  - **Edge-Caching for Performance**: Translations for static/catalog content are aggressively cached at the edge (Redis) to ensure latency targets are met. The AI is only invoked on cache misses or dynamic conversational text.
  - **Native RTL Support**: The UI component library must structurally support Right-to-Left layouts dynamically based on the detected locale.
  - **Zero-Trust Isolation**: Translation requests are tenant-scoped to ensure cross-tenant data leakage does not occur via the AI translation provider.

  ## Implementation Prompt
  **Task**: Implement the Autonomous Multilingual Localization Mesh.
  **User Journey**: Fatima, a food cart owner, sets her app language to Arabic. She creates a new menu item in Arabic. An English-speaking customer visits her OHC storefront, sees the menu item perfectly translated to English, places an order with special instructions in English. Fatima receives the order and instructions translated into Arabic on her device. She replies to a question in Arabic, and the customer receives it in English.
  **Acceptance Criteria**:
  1. Implement a transparent middleware/layer that intercepts requests and translates payloads based on the requester's locale (merchant or customer).
  2. Implement robust edge caching to ensure catalog and storefront loads meet strict <200ms latency targets.
  3. Ensure the UI components dynamically adapt to RTL languages seamlessly on a 375px viewport.
  4. Integrate the Customer Service Agent to handle real-time bidirectional chat translation.
  5. Provide an invisible experience; neither the merchant nor the customer should have to press a "Translate" button.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []