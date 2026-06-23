issue_title: "Architecture & Design: AI-Powered Autonomous Voice Dispatch & Scheduling Agent"
issue_description: |
  ## Problem Statement
  Field service owners like Carlos (Handyman) and Jun (Location Manager) operate on the move, frequently relying on Android/iOS devices while physically occupied. Traditional booking tools require them to navigate calendars, manually parse incoming customer calls, check inventory, and type out quotes or schedules—actions that are impossible while driving or on a job site. Missing a customer call directly translates to lost revenue. Existing market solutions (like Housecall Pro, ServiceTitan) are desktop-first, highly complex, and require the owner to act as a manual dispatcher.

  ## Research Report
  ### Market & Competitive Analysis
  - **Traditional SMB Platforms:** Wix, Squarespace, and GoDaddy lack native field-service scheduling capabilities tailored for offline/mobile-first operators.
  - **Vertical Giants:** ServiceTitan and Housecall Pro are too expensive ($100+/mo) and complex for micro-businesses and solopreneurs, functioning as heavy ERP systems rather than intelligent assistants.
  - **Voice AI Pioneers:** Tools like Bland AI are emerging but lack deep integration with the owner's booking calendar, quoting engine, and customer context.

  ### The Gap
  An AI-native Voice Dispatch Assistant that answers incoming service calls, parses the customer's intent (e.g., "I need a leaky pipe fixed"), checks the owner's calendar, quotes a deposit, and schedules the appointment, all while sending the owner a 375px mobile-friendly summary notification.

  ## Design Doc
  ### High-Level Architecture
  The Voice Dispatch Agent integrates with the existing OHC Teammate Mesh and Shared Task List.

  ```mermaid
  sequenceDiagram
      participant Customer
      participant OHC Voice Agent
      participant KAIROS
      participant Shared Task List
      participant Owner Mobile (375px)

      Customer->>OHC Voice Agent: Calls service number
      OHC Voice Agent->>KAIROS: Check calendar & pricing context
      KAIROS-->>OHC Voice Agent: Returns available slots
      OHC Voice Agent->>Customer: Negotiates time & sends SMS deposit link
      OHC Voice Agent->>Shared Task List: Creates `PENDING` booking task
      Shared Task List-->>Owner Mobile (375px): Push Notification: "New booking draft. Tap to approve."
  ```

  ### Mobile UX Flow (375px First)
  1. **Notification Card:** Translucent glass material card appears on the owner's lock screen: "Carlos, new pipe repair request from Sarah for tomorrow 2PM. $50 deposit collected."
  2. **Approval Screen:** Tapping opens a full-screen, unified inbox view.
     - **Top:** Audio player with 10-second AI summary of the customer call.
     - **Middle:** Proposed calendar block.
     - **Bottom:** Single massive button: "Approve Route & Send Confirmation".
  3. **Offline Tolerance:** If Carlos is in a basement with no signal, the approval is queued and synced via the standalone sync mesh once reconnected.

  ### AI Agent Integration Points
  - **Operations Assistant:** Consults `shared_tasks` to verify Carlos's existing route constraints.
  - **Customer & Relationship Assistant:** Maintains context on "Sarah" if she is a repeat customer.
  - **Finance Assistant:** Generates the Stripe Payment Link for the deposit and texts it via Twilio/MessageBird integration.

  ## Implementation Prompt
  **To the Implementer Agent:**
  Your task is to implement the Core Backend Logic and Mobile Layout for the Autonomous Voice Dispatch Agent.

  **Requirements:**
  1. **Backend Integration:** Create a new grpc/REST service endpoint to receive webhook payloads from a Voice Provider (e.g., Twilio). This service must parse the transcript, interact with the existing `shared_tasks` DAG, and generate a new booking proposal.
  2. **Mobile Screen (Flutter/PWA):** Design the "Booking Approval Card" conforming strictly to the OHC Premium Token library (translucent materials, Apple/Ubiquiti-style hierarchy, 375px viewport baseline, 44x44px touch targets). The UI must have ZERO mock data; it must consume the real `shared_tasks` backend state.
  3. **Verification:** You MUST verify the flow via Playwright E2E tests, simulating a voice webhook payload and confirming the UI state change for the owner. All tests must run and pass locally. DO NOT hardcode external API responses in the UI.

  ## Key Design Decisions
  - **Zero Trust/Multi-Tenant:** The voice webhook must securely resolve the `tenant_id` from the inbound phone number and apply row-level security before writing to the database.
  - **Asynchronous Execution:** Voice transcript processing utilizes the PG `SKIP LOCKED` job queue to ensure the webhook acknowledges the provider immediately without waiting for LLM generation.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
