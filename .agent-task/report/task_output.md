issue_title: "[Architecture] Universal Live-Stream & Video Commerce Engine"
issue_description: |
  # Title: [Architecture] Universal Live-Stream & Video Commerce Engine

  ## Problem Statement
  Small business owners like Priya (a boutique owner) and Maya (a custom baker) frequently use live streams (Instagram Live, TikTok Live) to showcase new inventory or special products. During these streams, viewers ask "How much?", "Do you have my size?", or "Can I buy this right now?" Managing these real-time inquiries while physically holding products and talking to the camera is impossible for a single operator. They either miss sales because they can't reply fast enough, or they have to pause the stream to send payment links manually. There is a massive "Mobile Gap" here: owners need a way to seamlessly convert live video engagement into instant, 1-tap checkout sales where an AI agent handles the real-time inventory verification, quoting, and transaction processing invisibly without the owner lifting a finger.

  ## Research Report
  ### The Live-Stream Commerce Gap
  Live shopping is a primary driver for fashion, beauty, and specialty food SMBs, but current platforms fail to integrate seamlessly:
  - **Shopify**: Requires clunky third-party apps to link live streams with store checkout, often redirecting users away from the stream and causing high drop-off rates. There is no native conversational AI handling real-time inquiries during the live broadcast.
  - **Wix / Squarespace**: Focus purely on static website storefronts. They have no native live-video commerce capabilities.
  - **TikTok/Instagram Native Shops**: These platforms offer in-app checkout, but they lock the merchant into their specific ecosystem, charge high transaction fees, and do not sync natively with a central business platform without complex integrations. More importantly, they lack a personalized AI agent to answer complex buyer questions (e.g., "Can you make this cake gluten-free?").

  ### OneHumanCorp Differentiation
  OHC will introduce a **Universal Live-Stream & Video Commerce Engine**. It allows merchants to broadcast directly from their phone (or connect their existing social streams) while the **AI Operations & CS Agents** actively monitor the chat. When a viewer comments "I want the red one in medium", the AI agent instantly checks the `Universal Capacity and Ledger`, temporarily locks the inventory, and drops a 1-tap localized checkout link directly in the chat or via direct message. The merchant never has to touch the screen—they just keep talking and see a subtle floating notification: "🎉 Red Dress (M) sold to @sarah".

  ## Design Doc

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  erDiagram
      TENANT ||--o{ LIVE_SESSION : "hosts"
      LIVE_SESSION ||--o{ VIDEO_STREAM : "streams"
      LIVE_SESSION ||--o{ CHAT_EVENT : "receives"
      CHAT_EVENT ||--o{ AGENT_INTENT : "parsed as"
      AGENT_INTENT ||--o{ LEDGER_RESERVATION : "triggers"
      LEDGER_RESERVATION ||--o{ CHECKOUT_SESSION : "generates"

      %% AI Departments Interactions
      CS_AGENT ||--o{ CHAT_EVENT : "monitors & answers"
      OPS_AGENT ||--o{ LEDGER_RESERVATION : "verifies inventory"
      FINANCE_AGENT ||--o{ CHECKOUT_SESSION : "processes payment"
  ```

  ### Mobile-First UI/UX Flow (375px Viewport)
  1. **The Broadcast Mode**: The merchant opens the OHC app, taps the `[+]` button, and selects `Go Live`. The camera activates in a full-screen vertical layout.
  2. **Invisible Assistant Panel**: At the bottom of the screen, a clean, translucent glass panel shows the AI agent's activity stream in real-time (e.g., "🤖 Checking inventory for @user123...", "💳 Sent payment link to @jane").
  3. **Buyer Journey**: A customer watching the stream types "Buy the blue scarf". The AI CS agent instantly replies in the chat with a secure, zero-click OneHumanCorp checkout link.
  4. **Instant Notification**: Upon successful payment, a small, elegant toast notification appears on the merchant's screen: "🎉 $45 Sale: Blue Scarf to Jane. Inventory: 4 left."

  ### Mobile UX & AI Integration Points
  - **Zero-Touch Operations**: The merchant's hands are free. The **Customer Success (CS) Agent** handles all chat interactions.
  - **Real-Time Inventory Locks**: When a buyer expresses intent, the **Operations Agent** places a 3-minute optimistic lock on the item in the Universal Capacity Ledger to prevent double-selling.
  - **Edge-Cached Delivery**: Video and real-time chat must be delivered through an edge-caching network to ensure sub-100ms latency for conversational commerce interactions.

  ### Key Design Decisions
  - **Decoupled AI Chat Observer**: The AI agent operates as a background worker observing the WebRTC data channel or social media API webhook, ensuring video performance is never impacted by LLM latency.
  - **Optimistic Concurrency**: To handle the rush of "I'll take it!" comments, the system uses optimistic locking on the edge to reserve inventory before the central Postgres database confirms it, guaranteeing high throughput.
  - **macOS-Style Translucent Overlays**: The merchant UI strictly follows the visual excellence mandate, using non-intrusive translucent glass components so the video feed remains unobstructed.

  ## Implementation Prompt
  **For the Implementer Agent:**
  Implement the Universal Live-Stream & Video Commerce Engine backend primitives and mobile API boundaries.
  1. **Core Capability**: Create the data models and service layers required to start a `LiveSession` and accept a stream of `ChatEvents`.
  2. **AI Coordination**: Integrate the background job queue so that incoming chat events are routed to the AI CS Agent, which can then query the Operations Agent for inventory availability.
  3. **Checkout Link Generation**: Ensure the system can dynamically generate an expiring 1-click checkout session and return it to the chat stream.
  4. **Constraints**: Ensure strict multi-tenant isolation. Do not prescribe specific video streaming protocols (e.g., RTMP vs WebRTC) at the database layer—keep the data model protocol-agnostic. The UI must be fully functional on a 375px mobile screen.

  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
