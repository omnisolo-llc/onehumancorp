issue_title: "Implement Multilingual Voice-to-Text Order Interceptor (The Receptionist Agent)"
issue_description: |
  **Title**: Implement Multilingual Voice-to-Text Order Interceptor (The Receptionist Agent)

  **Problem Statement**:
  Service and food operators like Fatima (Food Cart Operator) often face language barriers and cannot answer phone calls while actively cooking or serving customers. Currently, missed calls result in lost revenue, and language barriers cause order inaccuracies. Traditional phone systems or basic voicemails do not capture structured order data, leaving the business owner overwhelmed and customers frustrated.

  **Research Report**:
  - **Shopify/Wix/Squarespace**: These platforms rely heavily on web-based ordering and completely ignore the reality of phone-in orders, which remain a massive percentage of local food and service business revenue.
  - **GoDaddy**: Provides basic VoIP and call routing, but no real-time AI translation or structured order extraction.
  - **Modern AI Rivals (11x.ai, Bland AI)**: Focus on outbound sales or enterprise inbound support, which are too expensive and complex for a food cart operator.
  - **The OHC Differentiator**: OHC will provide a "Receptionist Agent" that intercepts phone calls, converses in the customer's language (e.g., English), extracts the structured order (e.g., 2 Halal chicken over rice, pickup in 15 mins), translates it into the owner's native language (e.g., Arabic), and pushes it as an actionable card to the owner's mobile device.

  **Design Doc**:
  - **Architecture diagram (Mermaid.js)**:
    ```mermaid
    graph TD
        A[Customer Calls OHC Number] -->|Twilio Voice Webhook| B(Voice Gateway)
        B --> C[Speech-to-Text Stream]
        C --> D{The Receptionist Agent}
        D -->|Query Catalog| E[Tenant Product DB]
        D -->|Extract Order Intent| F[Order Parsing Engine]
        F --> G[Translation Engine]
        G --> H[Action Required Queue]
        H --> I[Fatima's Mobile App Feed 375px]
        I -->|1-Tap Accept| J[Order Confirmed & TTS to Customer]
    ```
  - **Mobile UX flow (375px first)**:
    1. Fatima is in the OHC app.
    2. A high-priority card slides into her feed: "📞 New Voice Order: 2 Chicken over Rice, 1 Soda. Pickup: 15 mins. [Accept Order] [Decline]". The text is localized in Arabic for Fatima.
    3. Fatima taps "Accept Order".
    4. The card transitions to an active prep state, and the Agent plays a generated Voice response to the customer on hold: "Your order is confirmed and will be ready in 15 minutes."
  - **AI agent integration points**:
    - Uses a conversational voice LLM (e.g., via Twilio Media Streams + Gemini/OpenAI Realtime) to handle the call.
    - Uses an extraction prompt to match spoken items against the `Product` catalog.
    - Translates the final structured order into the owner's configured UI language.
  - **Key design decisions and why**:
    - *Voice-First, Not App-First for Customers*: Customers don't want to download an app to order from a food cart. They want to call.
    - *Approval-Based UX*: The agent puts the customer on a brief, polite hold while it pushes the structured card to the owner, ensuring the owner is never blindsided by an AI-accepted order they cannot fulfill.
    - *Twilio Voice Integration*: Provide an out-of-the-box business phone number for the tenant.

  **Implementation Prompt**:
  Build the "Receptionist Agent" inbound voice order flow. When a Twilio voice webhook hits the system, the agent should transcribe the user's order, match it against the active product catalog, and push a localized (translated) pending order card to the tenant's mobile-first feed. The UI must render on a 375px viewport with a clear "Accept" button that finalizes the order. Include Playwright E2E tests mocking the Twilio webhook and verifying the appearance and functionality of the order card in the mobile UI. Do not prescribe specific database schemas or API signatures; design them to fit the existing architecture.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
