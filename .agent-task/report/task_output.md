issue_title: "Implement Autonomous Hybrid AI Scheduling & Operations Engine"
issue_description: |
  **Problem Statement**
  Small business owners—like Maya (The Home Baker), Carlos (The Freelance Handyman), and Leo (The Music Tutor)—are losing revenue and sleep because they act as their own receptionists.
  While traditional platforms (Shopify, Wix, Squarespace) offer booking modules, they require rigid manual configuration and passive behavior from the system. If Maya receives an Instagram DM at 2 AM asking, "Can you do a custom vegan cake for next Tuesday?", she has to wake up, check her calendar, calculate prep time, and manually respond.
  If Carlos is out on a job, he cannot reply to quotes fast enough, missing out on potential customers who want immediate booking.

  **The Gap**: Current solutions lack a proactive, offline-capable AI Agent that can negotiate scheduling, handle complex quoting constraints, reserve resources, and process deposits entirely invisibly across both local edge environments (mobile apps) and cloud sync points. We need a fully automated "Operations AI" that completely manages the top-of-funnel inquiry to confirmed booking flow, ensuring multi-tenant isolation, without any setup configuration required by the human owner.

  **Research Report**
  A deep dive into the OHC codebase (`docs/features/kairos/ai_automated_scheduling.md` and `premium_hybrid_os_design.md`) and industry standards reveals:
  1. **Competitor Shortfalls**: Shopify relies on third-party integrations which fragment UX. Wix and Squarespace (Acuity) offer integrated but heavily manual workflows. None offer an AI capable of context-aware negotiation out-of-the-box. GoDaddy's AI is superficial (mostly branding).
  2. **Current OHC Capability**: OHC's architecture robustly supports a "Shared Task List" utilizing centralized data store for cloud, gracefully degrading to local storage for offline edge processing. The `Teammate Mesh` enables fast cross-node agent communication. However, there is no high-level framework specifically orchestrating calendar availability sync, proactive AI customer interactions, and secure payment processing for services/bookings.
  3. **The Necessary Leap**: We need to bridge the Config Sync and Local Webhook Forwarding tooling to allow local edge agents on mobile devices to manage an AI-negotiated schedule seamlessly, securely syncing state when online, and continuing to handle logic locally when offline.

  **Design Doc**
  1. **Key Design Decisions**
  - **Zero Configuration**: The agent deduces service length, padding, and constraints strictly from the business's natural language description.
  - **Hybrid Synchronization**: Calendar states and bookings are CRDT-based. Agents operating locally (e.g., in a standalone mobile mode without cell service) can tentatively reserve slots, which securely sync to the Cloud Shared Task List via the Config Sync pipeline upon reconnection.
  - **Zero Trust Multi-Tenancy**: Every agent interaction and webhook payload is validated using secure workload IDs.

  2. **Architecture Diagram**
  ```mermaid
  graph TD
      Customer((Customer via DM/Web))
      AgentRouter[Agent Router / Tunnel]
      OpsAgent[Autonomous Ops Agent]
      LocalCalendar[(Local Edge DB)]
      CloudDB[(Cloud Shared DB)]
      PaymentService[Deposit Gateway]

      Customer -- "Inquiry: Can I book Tuesday?" --> AgentRouter
      AgentRouter -- "Route & Intent Parse" --> OpsAgent

      OpsAgent -- "Check Availability & Constraints" --> LocalCalendar
      OpsAgent -- "Propose Slot & Quote" --> Customer

      Customer -- "Accepts & Pays" --> PaymentService
      PaymentService -- "Webhook via Relay" --> AgentRouter
      AgentRouter -- "Confirm Payment" --> OpsAgent

      OpsAgent -- "Reserve Slot CRDT" --> LocalCalendar
      LocalCalendar -. "Background Sync via OHC-SIP" .-> CloudDB

      classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
      class AgentRouter,OpsAgent,LocalCalendar,CloudDB,PaymentService premium;
  ```

  3. **Data Model & Invariants (ER Diagram)**
  ```mermaid
  erDiagram
      TENANT ||--o{ CALENDAR_EVENT : manages
      TENANT ||--o{ AGENT_TASK : delegates
      TENANT {
          string identity_id PK
          string business_context
      }
      CALENDAR_EVENT {
          uuid event_id PK
          string tenant_id FK
          datetime start_time
          datetime end_time
          string status "TENTATIVE | CONFIRMED"
          string origin "LOCAL | CLOUD"
          string vector_crdt_hash
      }
      AGENT_TASK {
          uuid task_id PK
          string tenant_id FK
          string state "PENDING | NEGOTIATING | BOOKED"
          json context_memory
      }
  ```

  4. **UI Wireframes & Mobile UX Flow (375px First)**
  **The "Activity Feed" View (Grandmother Test Passable)**:
  - **Visuals**: macOS-style Translucent Glass materials on an iOS/Android device. Ubiquiti UniFi modular dashboard cards.
  - **Flow**:
    1. **Home Screen**: A clean, blurred glass card simply says, "Maya, you got 3 new custom orders while you slept! The Operations Agent secured deposits."
    2. **Interaction**: User taps the card.
    3. **Detail View**: A scrolling feed showing a brief summary of the conversation the AI had with the customer ("AI offered Tuesday 3 PM, customer accepted, paid $50 deposit.") with an option to manually "Cancel or Reschedule". No complex calendar grid unless the user clicks an "Advanced Calendar View" toggle.
    4. **Offline UX**: If offline, the top nav shows a subtle orange dot, and booking cards indicate "Saved locally. Will sync when online."

  **Implementation Prompt**
  "Implement the foundational schema and agent routing logic for the Autonomous Hybrid AI Scheduling Engine.
  Create the CRDT-based entities `CalendarEvent` and `AgentBookingTask` within the multi-tenant edge and cloud persistence layers.
  Ensure the `Operations` AI Agent can hook into incoming webhook events, parse scheduling intents, and output tentative booking CRDTs to local storage.
  Finally, construct the backend endpoints to serve the 'Activity Feed' timeline view to the frontend mobile interfaces, adhering to the macOS-style translucent glass component hierarchy. Acceptance criteria include successfully handling an offline booking creation and syncing it back to the cloud database once connectivity is restored, completely isolated via workload identity."

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
issue_scope: Large
