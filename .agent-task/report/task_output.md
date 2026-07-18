issue_title: "Multilingual Voice Order Interceptor for Non-Technical Operators"
issue_description: |
  ## Problem Statement
  Food cart operators and similar service owners (e.g., the Fatima persona) often struggle with language barriers when taking phone orders in a fast-paced environment. They rely on limited English, which leads to order errors and lost revenue. They require a system that intercepts voice orders, translates them accurately, and presents them in their native language directly on their Kitchen Display System (KDS) or mobile device.

  ## Research Report
  - **Market Gap**: Existing KDS and POS systems assume the operator is fluent in the customer's language. Voice AI solutions like 11x.ai or Bland AI are enterprise-focused and too complex for a food cart owner to configure. OHC has an opportunity to provide a zero-setup, fully agentic voice interceptor.
  - **Persona Evidence**: Fatima needs to focus on cooking, not struggling to understand a phone call in a noisy environment.
  - **Competitive Analysis**: Shopify and Wix do not offer native voice interception. Specialized AI agents exist but lack deep, native integration into a POS/inventory system that a small operator uses daily.

  ## Design Doc
  - **Architecture**:
    - **Ingestion**: A Twilio (or similar) SIP trunk integrated into OHC to receive inbound calls to a designated business number.
    - **Speech-to-Text & LLM**: Real-time streaming STT (e.g., Deepgram) feeding into a Gemini-powered intent and translation engine. The agent identifies order items based on the OHC menu catalog.
    - **Translation & Normalization**: The agent maps spoken items to catalog IDs and translates any special instructions into the operator's preferred language (e.g., Arabic for Fatima).
    - **Presentation**: The order is pushed via WebSocket/Redis to the OHC Mobile POS (acting as a KDS) in the operator's native language.

  - **Architecture Diagram**:
    ```mermaid
    sequenceDiagram
        autonumber
        participant Customer
        participant SIP Trunk
        participant STT
        participant LLM
        participant OHC POS
        Customer->>SIP Trunk: Calls to place order
        SIP Trunk->>STT: Streams audio
        STT->>LLM: Transcribed text
        LLM->>LLM: Identify intent & translate to native language
        LLM->>OHC POS: Push translated order to KDS
        OHC POS-->>Customer: Order Confirmation
    ```

  - **Mobile UX Flow (375px)**:
    - The operator sees a live "Incoming Call" card that automatically transitions to "Order Being Placed...".
    - Once finished, an "Approve Order" card appears with the translated items and a one-tap "Accept" button (touch target > 44px).
  - **AI Agent Integration**: The Operations Agent acts as the orchestrator, communicating with the Voice Agent to process the call and the Inventory Agent to reserve stock.

  ## Implementation Prompt
  - Implement a Voice Order Interceptor service that receives inbound calls.
  - Integrate a real-time speech-to-text engine and an LLM to parse the customer's voice order against the tenant's menu catalog.
  - Translate the parsed order and special instructions into the tenant's configured primary language.
  - Build the mobile-first (375px) KDS approval card UX where the operator can view the translated order and tap "Accept".
  - Do NOT prescribe specific database schema modifications; focus on the service layer, LLM integration, and the mobile UX flow.

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []