issue_title: "Implement Multilingual Voice-to-KDS Order Interceptor Engine"
issue_description: |
  # Research & Design Report: Multilingual Voice-to-KDS Order Interceptor Engine

  ## 1. Problem Statement
  **The Pain Point:** Fatima (The Food Cart Operator) receives phone orders during peak rush hours. Often, these orders are in English, while she prefers reading and organizing orders in Arabic. Answering the phone while cooking disrupts her flow, leading to lost orders, miscommunication, and a bottleneck in fulfilling demand. She needs a system that can intercept calls, converse with the customer in multiple languages, process the order, take payment, and display it accurately on a Kitchen Display System (KDS) in her native language.

  **The Goal:** Empower the "Operations Assistant" AI Agent to autonomously intercept incoming phone orders via a voice interface. The agent must handle the conversation, translate the intent, check the menu and stock, process pre-payment, and seamlessly push the order to a localized KDS interface.

  ## 2. Research Report
  - **Competitor Systems Audit:**
    - **Square KDS:** Good for in-person orders but lacks an autonomous voice AI interceptor. It relies on the customer ordering through an online link or in person.
    - **11x.ai (Alice/Julian):** Strong outbound sales AI voice agents, but not optimized for real-time menu browsing and local food cart operations.
    - **Intercom Fin:** Text-based and geared toward customer support, not transactional voice ordering with POS integration.
    - **Toast:** Robust POS but lacks an AI voice agent taking real-time multilingual phone orders and translating them directly to the KDS.
  - **Identify Gaps:** OHC needs a specialized "Multilingual Voice-to-KDS Order Interceptor Engine" that bridges the Voice/Telephony layer, the `Unified Capacity Mesh` (for menu/inventory), and the Operations KDS. The AI must construct an order state, process payment or generate a payment link, and translate the final order for the operator.

  ## 3. Design Doc

  ### 3.1 Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Phone Call] -->|Twilio/WebRTC Voice Stream| B(Voice Gateway & STT)
      B --> C[Operations AI Agent]
      C -->|Check Menu & Stock| D[Unified Capacity Mesh]
      C -->|Conversational Ordering| B
      B --> A
      C -->|Extract Order Intent| E[Order Translation Engine]
      E -->|Translate to Operator Language| F[Ledger & Reconciliation]
      F -->|Commit Order| D
      F -->|Push to KDS| G[Kitchen Display System UI]
  ```

  ### 3.2 Mobile UX Flow
  - **375px Flow:**
    - The customer calls and interacts with the AI voice agent.
    - On Fatima's end (the operator), her 375px tablet/phone displays the KDS view.
    - A new order card pops up automatically, written in Arabic (her native language). The card shows: "New Voice Order: 2x Chicken Shawarma, No Onions - Pickup in 15 mins. Status: Paid."
    - She taps "In Progress" and then "Ready" to move the order along the pipeline.

  ### 3.3 AI Agent Integration Points
  - **Operations Agent (The Interceptor):** Triggered by an incoming voice call. It uses LLM capabilities to manage the conversation flow, referencing the available menu items and dynamically updating the customer on stock ("Sorry, we're out of falafel today.").
  - **Translation Engine:** Processes the structured order payload and translates the item names and special instructions to the operator's configured language preference before persisting to the KDS feed.

  ### 3.4 Key Design Decisions
  - **Voice Integration:** Utilize a low-latency WebRTC or Twilio integration for real-time voice streaming to the LLM backend.
  - **Multilingual KDS:** The data model must store both the original intent and the translated display text to maintain an accurate audit log while providing a localized UX.
  - **Asynchronous Processing:** The KDS feed must update via WebSockets/SSE so the operator sees the order the moment the voice call concludes, without needing to refresh the page.

  ## 4. Implementation Prompt
  **For Implementer Agent:**
  Implement the Multilingual Voice-to-KDS Order Interceptor Engine.
  - **User-Facing Outcome:** The AI Operations Agent intercepts an incoming customer phone call, takes a food order conversationally in English, processes it, and instantly displays the structured order on the operator's mobile KDS screen in their native language (e.g., Arabic).
  - **Critical User Journey (CUJ) & Acceptance Criteria:**
    - Create backend data models for `VoiceOrderSession` and translated KDS items.
    - Integrate the Operations AI Agent to handle voice order intents (using mocked audio/text for the test).
    - Provide an E2E test verifying a mock flow: Simulated incoming voice intent -> AI processes order -> Order appears on the 375px KDS UI localized to the operator's language.
    - Ensure strict tenant isolation and support for slow/offline-tolerant networks for the KDS.
  - **Priority:** P1
  - **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
