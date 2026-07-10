issue_title: "Implement Autonomous Multilingual Order Interceptor Agent"
issue_description: |
  ## Research Report

  ### Problem Statement
  Small business owners with language barriers (like Fatima the food cart owner) struggle to process real-time phone orders and walk-in requests effectively while doing operational work. During manual product verification in the live environment (`docker compose up`), OHC's current system allows standard widget-based bookings and text-based intakes, but lacks an intelligent agentic layer that can handle phone or multilingual orders, parse intent automatically, and route it to an accessible operational display (like a tablet KDS). Fatima needs an "Assistant" that takes voice or text orders in any language and seamlessly adds them to a daily work list in her native language.

  ### Competitive Analysis
  - **Shopify / Square:** Require manual or rigid structured input (POS interfaces). They do not easily handle unstructured multilingual voice or chat in real-time.
  - **Wix / GoDaddy:** Great for setup but rely entirely on standard web forms. No dynamic agentic interceptor for walk-ups or live translated communication.
  - **Intercom Fin / 11x:** Excellent at generalized chat/support routing, but too generic and expensive for a food-cart operator.
  - **The OHC Gap:** OHC can leapfrog these solutions by providing an inherently multilingual voice/text listener that instantly structures messy input into verified operations tasks for the owner.

  ### Proposed Architecture
  1. **Omni-Channel Entrypoint:** Extend `Work Triage` (the unified inbox feed) to accept text/voice events in arbitrary languages.
  2. **Translation & Intent Routing:** A specialized `Multilingual Order Interceptor` agent that receives raw customer input, auto-detects the language, translates to the owner's language, and identifies it as an "Order", "Query", or "Status Check".
  3. **Operational Output:** Translates the intent into a concrete daily task/order inside the existing backend job/task queues, creating an organized, pre-translated list for the owner.

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Customer
      participant OHC_Unified_Inbox
      participant Multilingual_Agent
      participant Order_Queue
      participant Owner_KDS

      Customer->>OHC_Unified_Inbox: "Quiero 3 tacos de pollo" (Audio/Text)
      OHC_Unified_Inbox->>Multilingual_Agent: Process Raw Input
      Multilingual_Agent-->>Multilingual_Agent: Detect Lang (es), Translate to (en/ar)
      Multilingual_Agent-->>Multilingual_Agent: Extract Intent (Order) & Entities (3x Chicken Tacos)
      Multilingual_Agent->>Order_Queue: Create Structured Order Task
      Order_Queue->>Owner_KDS: Display: "3x Chicken Tacos" in Owner's preferred lang
  ```

  ### Mobile UX Flow (375px First)
  1. **The Walk-up Screen:** A large, prominent, microphone icon or single text box on the OHC mobile app for the owner to hold up to the customer.
  2. **The "Listening" State:** A clean, Apple/Ubiquiti-style translucent material overlay indicating it is processing audio or translating.
  3. **The Result Card:** A simple, high-contrast UniFi-style card showing the structured order ("3x Chicken Tacos") in the owner's language, with a massive "Confirm & Add to List" button.

  ### AI Agent Integration Points
  - **Agent Department:** This falls under the "Customer & Relationship Assistant" combined with "Operations Assistant".
  - **LLM Prompting:** A strict system prompt for the Multilingual Interceptor to return only structured JSON (Intent, Items, Quantities) from messy multilingual input.
  - **Memory:** The agent uses Tenant context to know what menu items are actually available to avoid hallucinating fake menu items.

  ### Implementation Prompt
  1. Ensure the `Tenant` model supports a `language_preference` (e.g., "en", "ar", "es").
  2. Implement the `Multilingual Order Interceptor` agent service in Rust using the LLM provider interface. It must accept raw text/audio strings, detect language, and output structured order data based on the tenant's menu/catalog.
  3. Create the 375px mobile-first UI for the "Walk-up / Listen" mode that routes the audio transcript to this agent.
  4. Prove the end-to-end CUJ via a Playwright E2E UI test demonstrating an incoming Spanish string successfully appearing in the owner's feed in English.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
