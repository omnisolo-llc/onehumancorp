issue_title: "Implement Missing Omnichannel Voice Order Intake (Multi-Language) to Support Offline Operators"
issue_description: |
  # Research Report: Omnichannel Voice Order Intake (Multi-Language)

  ## 1. Problem Statement
  Operators working in high-friction environments (like Fatima in her food cart or Carlos doing repairs under a sink) often have their hands full, wear gloves, or work in poor lighting. Current order entry systems require navigating forms and tapping buttons on a glass screen, causing a disruption in workflow, slowing down service, and increasing the risk of missing orders. Additionally, users like Fatima operate with limited English proficiency. OHC is currently missing a low-friction, omnichannel voice interface that can capture orders and intents naturally in multiple languages, extract structured data, and feed directly into the unified inbox and agentic pipeline.

  ## 2. Research Findings
  - **Competitive Analysis:**
    - Square and Shopify rely strictly on UI taps for order intake.
    - WeCom and DingTalk support voice messages but mostly for communication, not direct structuration of commerce events.
    - Voice AI wrappers exist, but they are detached from the core operations/ledger system.
  - **Persona Fit:**
    - **Fatima (Food Cart):** Can speak an order ("2 chicken over rice, 1 with no white sauce") in Arabic or English directly into the OHC app, keeping the line moving.
    - **Carlos (Handyman):** Can dictate a material list or follow-up task while driving between jobs.
  - **Gap Analysis:**
    - OHC's unified inbox structure handles text and structured webhook data well, but lacks the specific architectural ingestion point for raw, unstructured voice audio that must be transcribed, translated, and parsed into a structured `OrderIntent` or `TaskIntent`.

  ## 3. Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      MobileApp[OHC Mobile App - 375px] -->|Voice Recording| EdgeGateway[API /gRPC]
      EdgeGateway --> AI_Whisper[Audio-to-Text & Translation Agent]
      AI_Whisper --> Transcript[Raw Transcript]
      Transcript --> AI_Operations[Operations Agent - Intent Parsing]
      AI_Operations -->|Generates| OrderIntent[Order / Task Intent]
      OrderIntent --> UnifiedInbox[Unified Inbox & Ledger]
      UnifiedInbox -->|Real-time update| MobileApp
  ```

  ### Mobile UX Flow (375px First)
  1. **Omnipresent Voice Button:** A prominent, floating (or bottom-bar integrated) microphone button with the translucent glass OHC design token.
  2. **Recording State:** Tapping/holding activates a visually pleasing recording state (subtle waveforms).
  3. **Immediate Feedback:** Upon release, a skeleton loader shows processing. Within ~1-2 seconds, the parsed intent (e.g., "Drafted Order: 2x Chicken Rice") appears as an actionable card in the Agent Feed.
  4. **One-Tap Approval:** Fatima taps "Confirm" to commit the order.

  ### AI Agent Integration Points
  - **Audio Pipeline:** Securely stream audio bytes to the backend. Route to a multimodal model (Gemini Pro/Whisper equivalent) to handle mixed languages and extract text.
  - **Intent Extraction:** The Operations Assistant receives the transcript, queries the tenant's current menu/services, and structures the JSON intent.

  ## 4. Implementation Prompt
  **To the Implementer:**
  Implement the Omnichannel Voice Order Intake feature.
  - Provide an API endpoint to accept audio data (or simulated text transcripts for MVP) tied to a tenant.
  - Build the corresponding Mobile-First (375px) UI component: a sleek voice recording button on the main dashboard that captures input and displays the resulting draft action.
  - Ensure the backend uses an LLM prompt/tool to extract structured order data (items, quantities, special requests) from the unstructured input.
  - The feature must strictly adhere to multi-tenant isolation, use the premium glassmorphism design tokens, and be fully covered by E2E Playwright tests simulating the voice-to-order flow.

  ## 5. Priority & Scope
  - **Priority:** P1
  - **Scope:** Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
