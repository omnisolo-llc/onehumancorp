issue_title: Invisible Real-time Translation and Localization Engine
issue_description: "## Problem Statement\nFatima (Food Cart Operator, 50) relies on\
  \ taking pre-orders efficiently but struggles with existing platforms that default\
  \ to complex English interfaces. Her customers primarily order in English, but she\
  \ needs to operate her business\u2014managing incoming orders, marking items out\
  \ of stock, and printing daily summaries\u2014in Arabic. Current solutions like\
  \ Shopify and Wix require convoluted multi-language app setups and fail to provide\
  \ real-time translation for live order tickets (KDS) or push notifications. Fatima\
  \ needs an invisible translation layer that instantly bridges the language gap between\
  \ her English-speaking customers and her Arabic-centric daily operations.\n\n##\
  \ Research Report\n- **Competitor Analysis:** Shopify Markets and Wix Multilingual\
  \ focus heavily on static storefront localization but lack seamless, real-time bi-directional\
  \ translation for backend operational tools and KDS displays. They require merchants\
  \ to manually input translations or use costly third-party plugins.\n- **OHC Opportunity:**\
  \ By leveraging the OHC Hybrid Event Mesh and our AI Orchestrator (KAIROS), we can\
  \ embed real-time translation directly into the event stream. This ensures that\
  \ an order placed in English appears instantly in Arabic on Fatima\u2019s low-end\
  \ Android device, while her local operational updates (e.g., \"Sold Out\") are translated\
  \ back to English for the customer-facing storefront.\n\n## Design Doc\n### Architecture\
  \ Diagram\n```mermaid\ngraph TD;\n    Customer[Customer Storefront - English] -->|Places\
  \ Order| Gateway[Zero-Trust Gateway];\n    Gateway --> KAIROS[KAIROS Orchestrator];\n\
  \    KAIROS --> TranslationAgent[AI Translation Agent];\n    TranslationAgent -->|Translated\
  \ Payload| EventMesh[Hybrid Event Mesh];\n    EventMesh --> SyncDaemon[Local Sync\
  \ Daemon];\n    SyncDaemon --> KDS_UI[Fatima's KDS - Arabic];\n```\n\n### Mobile\
  \ UX Flow (375px First)\n1. **Customer Checkout:** The customer places an order\
  \ on the mobile web storefront in English.\n2. **Push Notification:** Fatima receives\
  \ an immediate, loud push notification on her Android device, natively translated\
  \ into Arabic: \"\u0637\u0644\u0628 \u062C\u062F\u064A\u062F: \u062F\u062C\u0627\
  \u062C \u0645\u0634\u0648\u064A \u0645\u0639 \u0623\u0631\u0632\" (New Order: Grilled\
  \ Chicken with Rice).\n3. **Operational Screen:** The KDS view displays large, high-contrast\
  \ order cards in Arabic (RTL layout).\n4. **State Change:** Fatima taps the green\
  \ \"\u062C\u0627\u0631\u064A \u0627\u0644\u062A\u062C\u0647\u064A\u0632\" (Preparing)\
  \ button. The event is pushed back through the Translation Agent, updating the customer's\
  \ tracking link in English.\n\n### Key Design Decisions\n- **Bi-directional Stream\
  \ Translation:** Translation occurs in transit via the Translation Agent, ensuring\
  \ neither the customer nor the merchant ever sees the \"wrong\" language.\n- **RTL\
  \ Native:** The UI layer dynamically shifts to Right-to-Left layouts when Arabic\
  \ is the active operational language, maintaining 44x44px minimum touch targets\
  \ and premium design tokens.\n- **Zero Trust:** Localization profiles and translated\
  \ data streams are strictly isolated per tenant using SPIFFE/SPIRE authentication.\n\
  \n## Implementation Prompt\n**Objective:** Implement the real-time Translation Agent\
  \ middleware within the KAIROS event pipeline.\n**Critical User Journey (CUJ):**\
  \ \n- A customer orders in Language A.\n- The business owner receives the order\
  \ notification and KDS entry in Language B.\n- The business owner updates the order\
  \ status in Language B, and the customer is notified in Language A.\n**Acceptance\
  \ Criteria:**\n1. The translation step must add no more than 200ms latency to the\
  \ end-to-end event delivery.\n2. The system must support dynamic RTL rendering for\
  \ operational UI based on the localized payload.\n3. All data models must maintain\
  \ a single source of truth, storing both the original payload and the localized\
  \ view efficiently without schema fragmentation.\n\n## Priority\n`P0`\n\n## Estimated\
  \ Scope\nLarge\n"
issue_priority: P0
issue_category: research
issue_type: task
issue_label:
- agent-report
assignees: []
