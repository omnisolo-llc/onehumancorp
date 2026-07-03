issue_title: "Implement Multilingual Voice Order Interceptor & Agentic KDS for Service Operators"
issue_description: |
  # Research Report: Multilingual Voice Order Interceptor & Agentic KDS

  ## Problem Statement
  Food and service operators, particularly those dealing with rapid point-of-sale pre-orders and phone call reservations (like Fatima, the food cart operator), face severe operational friction when taking orders manually. Fatima struggles to cook, handle a line of customers, and answer phone calls in a language that is not her native tongue. Missed calls mean lost revenue, and misunderstood orders lead to waste.
  Current POS systems provide simple order aggregation but lack real-time conversational AI to handle voice orders, translate them, and insert them directly into an operational queue (Kitchen Display System - KDS) without human intervention.

  ## Research Report & Market Findings
  - **Traditional Operators:** Systems like Square, Clover, and Toast provide strong KDS and POS hardware but require the human to manually input orders from a phone call or an external tablet.
  - **AI-Native Rivals:**
    - **11x.ai / Julian:** Strong at inbound sales but focused on B2B SaaS and real estate, not high-volume local commerce.
    - **Intercom Fin:** Primarily text-based support, not transactional voice-to-KDS ordering.
    - **PolyAI:** Excellent enterprise voice assistants for restaurants but out of reach for a solo food cart operator due to high setup costs and complexity.
  - **The OHC Opportunity:** By leveraging our internal KAIROS architecture and the existing `booking`, `quoting`, and `pos` modules, OHC can create a **Multilingual Order Interceptor**. This connects a unified SIP/Twilio voice endpoint directly to our `Operations Agent (The Manager)`. When a customer calls, the AI agent converses in the customer's language, transcribes and translates the intent into structured JSON, checks inventory/menu availability (via Postgres/Redis locks), and pushes the finalized order directly to the OHC mobile app feed as an actionable KDS card.

  ## Design Doc (Architecture)

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  graph TD
      A[Customer Phone Call] -->|SIP/Twilio Webhook| B(Omnichannel Voice Gateway)
      B --> C{Real-Time STT/TTS Streaming Engine}
      C --> D[The Operations Agent / Negotiator]
      D -->|Query Menu & Inventory| E[(PostgreSQL / Redis)]
      D -->|Confirm Order & Translate| C
      D -->|Publish Finalized Order| F[Event Mesh]
      F --> G[Action Required Queue]
      G --> H[Mobile App KDS Feed 375px]
  ```

  ### Mobile UX Flow (375px First)
  - **The "Agentic KDS" Feed:** The owner (Fatima) opens the OHC app. Instead of a traditional dense KDS, she sees a simplified feed of large, high-contrast cards.
  - **Incoming Order Card:** When a voice order completes, a new card appears at the top. It displays the order in her preferred language (e.g., Arabic), alongside the customer's original language (e.g., English) for reference if needed.
  - **Details:**
    - Card Title: "New Order: Pick-up at 12:30 PM"
    - Items: "2x Halal Chicken Over Rice (No White Sauce)"
    - Payment Status: "Pending at Pickup"
  - **Interactions (44x44px touch targets):** A primary green button `[Mark Ready]`, a secondary button `[Out of Stock/Cancel]`.
  - **Visual Design:** Dark Mode Translucent Glass `rgba(22, 22, 26, 0.7)` with `16px` border radius, ensuring high visibility even in harsh outdoor lighting.

  ### AI Agent Integration Points
  - **Operations Agent (The Manager):** Acts as the brains of the phone call. It uses a specific system prompt customized with the tenant's menu, pricing, and current availability.
  - **Identity Resolution:** If the phone number matches an existing customer, the agent greets them by name and asks if they want their "usual order".

  ### Key Design Decisions
  - **Asynchronous Processing:** The voice-to-text and LLM processing must happen in real-time, but the final KDS update is asynchronous via the event mesh to guarantee delivery even if Fatima's mobile connection is flaky.
  - **Language Agnostic KDS:** The data model must store the original raw text and the structured intent so that the UI can render the KDS view in the owner's preferred locale setting, independent of the caller's language.

  ## Implementation Prompt
  **User-Facing Outcome:** Fatima is busy cooking when a customer calls. She doesn't pick up. The OHC Multilingual Order Interceptor answers, takes an order for two chicken plates in English, confirms the 15-minute pickup time, and hangs up. Fatima's phone buzzes. She looks down to see a new KDS card showing the exact order translated into Arabic, with a large button to mark it ready.

  **Critical User Journey (CUJ) & Acceptance Criteria:**
  1. Set up a mock webhook endpoint to simulate an incoming Twilio voice transcript stream.
  2. Ensure the `Operations Agent` parses the transcript, cross-references a mock `Menu/Inventory` database, and generates a structured order JSON.
  3. Ensure the structured order includes translation metadata targeting the tenant's configured primary language.
  4. The order must be inserted into the `Action Required Queue` as a new KDS entry.
  5. Provide Playwright E2E tests: Simulate the incoming webhook, then have a user log in to the mobile UI (375px viewport), verify the translated order card appears in the feed, and successfully tap the "Mark Ready" button to dismiss it.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
