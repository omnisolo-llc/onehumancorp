issue_title: "[Architecture] Autonomous AI Voice Receptionist Engine"
issue_description: |
  ## Problem Statement
  Small business owners like Carlos (Handyman) and Fatima (Food Cart Operator) often miss phone calls because their hands are full, they are driving to a job, or they are actively serving customers. Missed calls mean lost revenue. Traditional voicemails are inefficient, and existing answering services are expensive.

  ## Research Report
  **Competitor Analysis:**
  - **Google Voice:** Provides basic transcription but no intelligent conversational capabilities or action-taking.
  - **Twilio Voice AI:** Requires significant developer effort to build conversational flows.
  - **Bland AI / Vapi:** Powerful voice AI platforms, but they act as standalone services that don't natively integrate with a business's calendar, inventory, or quoting system.

  **Gaps Identified:**
  OHC needs an integrated "Autonomous AI Voice Receptionist" that can answer calls, converse naturally with customers, check calendar availability, provide quotes, and take orders—all while the business owner is busy. This must seamlessly integrate with the OHC omnichannel ledger.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Phone Call] --> B(Twilio Voice Webhook);
      B --> C[OHC Voice Gateway];

      C <-->|Real-time STT/TTS Streaming| D[Voice AI Engine];
      D <-->|Function Calling| E[KAIROS Orchestrator];

      E -->|Check Availability| F[(Calendar API)];
      E -->|Get Prices| G[(Inventory/Services DB)];

      D -->|Call Transcript & Summary| H[(Unified Inbox Ledger)];
      H --> I[Business Owner Mobile UI];
  ```

  ### Key Design Decisions
  - **Low Latency Streaming:** Utilize WebSocket connections between the OHC Voice Gateway and the Voice AI Engine (e.g., integrating with providers like ElevenLabs for TTS and Deepgram for STT) to ensure conversational latency under 800ms.
  - **Contextual Function Calling:** The Voice AI is equipped with tools specific to the business (e.g., `book_appointment`, `check_price`, `take_message`).
  - **Unified Inbox Integration:** Every call generates a transcript, an AI-generated summary, and any extracted actions (like a drafted calendar invite) directly in the owner's Omnichannel Inbox.

  ## Implementation Prompt
  **For Implementer Agent:**
  Implement the Autonomous AI Voice Receptionist architecture.
  - **Objective:** Create the voice gateway endpoints to handle incoming calls (via simulated Twilio TwiML), integrate a streaming STT/TTS/LLM loop, and connect it to the unified inbox.
  - **CUJ:** Customer calls Carlos's business number. The AI answers, asks how it can help, understands the customer needs a plumbing fix, quotes a standard hourly rate, and books a tentative slot on the calendar. Carlos sees the summary and the booked slot in his app.
  - **Acceptance Criteria:** Unit tests for the streaming voice loop. E2E test verifying a mock call flow results in a calendar event and an inbox transcript. Ensure strict tenant data isolation.

  ## Priority
  P1

  ## Estimated Scope
  Large

  ---
  ## ⚠️ Blocker Documentation
  **E2E Test Rate Limiting Issue**
  During task verification, the Playwright E2E tests (`//src/e2e:...`) consistently failed with `Exit 125`. The root cause is a Docker Hub unauthenticated pull rate limit specifically for `pgvector/pgvector:pg16` (`Error response from daemon: error from registry: You have reached your unauthenticated pull rate limit.`).

  Since all unit tests passed and this is a known infrastructure constraint outside the control of the agent environment, we are pushing the changes as requested by the user, skipping the E2E verification.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
