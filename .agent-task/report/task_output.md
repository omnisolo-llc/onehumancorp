issue_title: "Research: Autonomous Operations Manager Agent Protocol"
issue_description: |
  # Architectural Gap & Scaling Discovery: Autonomous Operations Manager Agent Protocol

  ## Problem Statement
  While OHC has a foundational "Agent Feed" concept where AI agents draft responses (e.g., the Ambassador agent for DMs), it currently lacks a robust execution layer—the **Autonomous Operations Manager Agent Protocol**.
  Currently, the system acts as an advisory AI (like Shopify's Sidekick), prompting owners to review and manually execute actions. For non-technical SMB owners (like Maya the Baker or Carlos the Handyman), the true value of OHC lies in the AI securely and autonomously executing state changes (e.g., reserving inventory, scheduling a booking, dispatching an invoice) based on intent, rather than just suggesting them. Without this protocol, owners suffer from "App Tax Fatigue" and setup paralysis, and remain trapped in operational bottlenecks.

  ## Research Report
  ### Competitive Landscape
  - **Shopify & Wix**: Powerful but rely heavily on complex app ecosystems. Shopify's Sidekick acts as a chatbot but rarely executes complex multi-step cross-domain tasks autonomously.
  - **AI-Native Builders (Durable, Mixo, 10Web)**: Excellent at zero-click generation and rapid onboarding but lack deep operational capabilities post-launch (like managing dynamic service bookings and integrated POS inventory).

  ### OHC Gap Identification
  - OHC currently focuses on advisory intelligence (drafting replies).
  - A critical gap exists between identifying user intent (e.g., "I want a cake on Tuesday") and executing the necessary multi-tenant database CRUD operations safely.
  - **Pain Point**: If Maya approves a drafted reply accepting an order, she still has to manually adjust inventory and schedule the pickup.

  ## Design Doc

  ### High-Level Architecture Design
  The Operations Manager Agent Protocol serves as a secure middleware layer between the Agent Feed (Intent & Context Resolution) and the OHC Core Services (Booking, Commerce, Ledger).

  #### Key Design Decisions
  - **Zero-Trust Execution**: Agents cannot execute direct SQL. They interact with the protocol via strictly typed, schema-validated tool calls protected by SPIFFE/SPIRE workload identities.
  - **Tenant Isolation**: Every protocol action is hard-scoped to the `tenant_id` of the owner.
  - **Human-in-the-Loop (HITL) Tiering**: Actions are tiered by risk. Low-risk (e.g., tagging a customer) auto-executes; high-risk (e.g., issuing a refund or modifying core pricing) requires explicitly sending an "Action Card" to the Agent Feed for 1-tap owner approval.

  #### AI Agent Integration Points
  - **Ambassador Agent**: Hands off structured intents to the Operations Protocol instead of just generating text.
  - **Operations Manager Agent**: The core LLM worker configured to orchestrate multi-step business logic and commit state changes.

  #### Mobile UX Flow (375px First)
  1. **The Feed**: Owner opens OHC app to the unified feed.
  2. **The Action Card**: A card appears: "Maya, I received a DM from Sarah for a custom vegan cake. I have checked inventory (available) and drafted a quote for $45. [Approve & Send Quote] [Edit] [Decline]"
  3. **Execution**: Owner taps "Approve". The UI immediately transitions the card to a "Processing" state (shimmer effect), then a "Sent" state (translucent green checkmark).
  4. **Offline Resilience**: If the network drops, the "Approve" action is queued locally (optimistic UI) and synced when connectivity returns.

  #### Architecture Diagram

  ```mermaid
  sequenceDiagram
      participant Customer
      participant AgentFeed as Agent Feed (UX)
      participant OMAgent as Ops Manager Agent
      participant OpsProtocol as Ops Manager Protocol
      participant Services as OHC Services (DB)

      Customer->>AgentFeed: "Can I book a repair for Friday?"
      AgentFeed->>OMAgent: Analyze Intent (Booking Request)
      OMAgent->>OpsProtocol: CheckAvailability(tenant_id, date: Friday)
      OpsProtocol->>Services: Query Schedule
      Services-->>OpsProtocol: Available slots
      OpsProtocol-->>OMAgent: Return slots
      OMAgent->>AgentFeed: Push Action Card: "Draft reply with Friday slots?"
      Owner->>AgentFeed: Taps "Approve & Hold Slot"
      AgentFeed->>OMAgent: Owner Approved
      OMAgent->>OpsProtocol: ExecuteBooking(tenant_id, details)
      OpsProtocol->>Services: Commit state (Row Level Security)
      Services-->>OpsProtocol: Success
      OpsProtocol-->>OMAgent: Confirm
      OMAgent->>AgentFeed: Update Card UI (Success)
      AgentFeed->>Customer: "You are booked for Friday!"
  ```

  ## Implementation Prompt
  **Task for Implementer**: Implement the `Operations Manager Agent Protocol` core service.
  - **User-Facing Outcome**: When an owner approves an action in the Agent Feed, the system securely and autonomously performs the underlying state changes (e.g., inventory deduction, booking creation) without the owner needing to open a separate settings page.
  - **Critical User Journey (CUJ)**:
    1. Login as a business owner (e.g., Maya).
    2. Navigate to the Agent Feed.
    3. Locate a pending "Action Card" proposing a multi-step operation (e.g., accepting an order which updates inventory and sends a Stripe payment link).
    4. Tap "Approve".
    5. The system performs the state change and updates the UI to reflect the successful outcome.
  - **Acceptance Criteria**:
    - Implement the service layer protocol handling authorized agent tool calls.
    - Define strictly typed interfaces for at least two operational domains (e.g., Booking, Inventory).
    - Ensure all database writes respect PostgreSQL Row-Level Security (RLS) by `tenant_id`.
    - Add full E2E Playwright test coverage verifying the owner can approve an action card and the database state changes correctly.
    - UI components must follow the OHC Premium Token library (macOS Translucent Glass styling, mobile-first 375px responsive constraints).

  ## Priority
  P0 (Critical)

  ## Estimated Scope
  Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
