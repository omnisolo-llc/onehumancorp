issue_title: "Autonomous Voice Order Interceptor & Multilingual KDS Engine"
issue_description: |
  # Research Report: Autonomous Voice Order Interceptor & Multilingual KDS Engine

  ## Problem Statement
  Small business operators with high-volume, hands-on tasks, such as Fatima (food cart operator), struggle to manage incoming phone orders while actively working. Language barriers further complicate this process, leading to misunderstood orders, lost sales, and poor customer experience. Existing platforms like Shopify or Wix do not offer native phone answering services, let alone multilingual real-time translation integrated directly into a Kitchen Display System (KDS) or point-of-sale system.

  ## Research Report
  - **Market Context:** Current restaurant tech relies on tablet-based apps (e.g., Square KDS, Toast) where orders come from online forms or delivery apps. Phone orders still require manual input.
  - **Competitor Gaps:**
    - **Shopify / Wix / Squarespace:** E-commerce focused. They have POS solutions but do not handle incoming phone calls autonomously.
    - **11x.ai / Specialized Voice Bots:** Can handle inbound calls but are not integrated seamlessly into an SMB's inventory and POS ecosystem. Setting them up requires technical integrations.
  - **The OHC Opportunity:** OHC can leapfrog competitors by providing an AI agent that acts as a front-of-house worker. It intercepts phone calls, handles multi-turn voice conversations (e.g., in English), translates the intent into structured order data, and presents it on a 375px mobile KDS in the owner's native language (e.g., Arabic).

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Phone Call] -->|Twilio/SIP SIP| B(Voice Gateway)
      B --> C{Real-Time Speech-to-Text & Intent Classifier}
      C -->|English Text| D[The Order Taker Agent]
      D -->|Query| E[Tenant Product Catalog]
      D -->|Confirm Order & Total| C
      C -->|Text-to-Speech| A
      D -->|Finalized Order JSON| F[Order Ingestion Queue]
      F --> G[Translation Engine]
      G -->|Translated to Native Language| H[PostgreSQL Order Ledger]
      H --> I[Mobile KDS UI 375px]
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **KDS Feed (Mobile):** A simple queue of current orders. The screen auto-refreshes.
  - **Order Card:** Displays the order number, items requested (in the owner's preferred language), total amount, and customer phone number.
  - **Interaction:** A single large "Complete" button (min 44x44px) that the owner taps when the food is ready. This triggers an automated SMS to the customer.
  - **Visual Design:** High contrast, large fonts for readability at a glance. Translucent glass styling but optimized for outdoor/bright light viewing.

  ### AI Agent Integration Points
  - **The Order Taker Agent (Voice):** An LLM connected to a streaming speech pipeline. It has access to the current menu and sold-out states. It can negotiate variations (e.g., "no onions").
  - **The Ambassador (SMS):** Handles the "Order Ready" SMS notification automatically.

  ### Key Design Decisions
  - **Asynchronous Handoff:** The AI handles the entire call and only pushes the structured data to the KDS once the order is finalized.
  - **Multilingual UI:** The system translates customer input into the owner's configured language automatically.

  ## Implementation Prompt
  **Feature Name:** Multilingual Autonomous Voice Order Interceptor
  **Target Persona:** Fatima (Food Cart Operator)
  **Outcome:** An AI voice agent answers inbound calls, takes food orders in English, and displays the structured orders on Fatima's mobile screen in Arabic. Fatima just cooks the food and taps "Complete" to text the customer.

  **Next Actions:**
  1. Create the `VoiceOrderSession` data model in PostgreSQL to track active and completed AI phone calls.
  2. Implement the `Voice Gateway` service that connects to Twilio/SIP and streams audio to a Speech-to-Text service, then to an LLM, and back to Text-to-Speech.
  3. Develop the Mobile KDS UI (375px view) that displays incoming orders in real-time, pulling from the `VoiceOrderSession` and translating content into the owner's locale.
  4. Build automated E2E Playwright tests verifying the KDS UI updates when a mocked voice order is finalized.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
