issue_title: "Implement Unified Multi-Tenant Calendar & Booking Engine with AI Agentic Scheduling"
issue_description: |
  ## Problem Statement
  For service-based owners like Leo (music tutor) and Carlos (field service), scheduling is currently scattered across isolated channels, DMs, and manual calendar checks. The lack of a unified booking engine means double-booking risks are high, and manual coordination eats up their time. We need a native, multi-tenant Rust Calendar & Booking Engine that natively integrates with our AI Agents (e.g., Operations Assistant) to autonomously negotiate time slots, accept deposits via Stripe, and seamlessly update the owner's unified feed without manual intervention.

  ## Research Report
  - **Competitor Analysis:** Shopify (via apps), Wix Bookings, and specialized tools like Calendly/Acuity dominate this space. Wix Bookings tightly couples inventory and time, which is effective for small business operators but often breaks for multi-location or dynamic-agent scenarios.
  - **Market Gap:** Existing platforms treat booking as a static form. OHC can differentiate by treating booking as an "agentic negotiation" where the Operations Agent can converse with the customer (e.g., via IG DM or SMS) and provisionally hold time slots until a deposit is verified, entirely removing the owner from the triage phase.
  - **Codebase Context:** The current system lacks a `Bookings` or `TimeSlot` primitive in the multi-tenant PostgreSQL schema. We need to introduce a robust, time-zone-aware ledger for calendar events that integrates closely with the newly proposed Omnichannel Chat Engine.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      Tenant ||--o{ Calendar : owns
      Calendar ||--o{ TimeSlot : contains
      TimeSlot ||--o| Booking : reserves
      Booking }o--|| Contact : booked_by
      Booking ||--o| PaymentIntent : requires
      AgentBot ||--o{ Booking : manages
  ```

  ### Mobile UX Flow (375px)
  1. **Triage Feed:** The owner opens the app (375px) and sees an AI-generated summary card: *"Carlos, I scheduled 3 new repair visits for tomorrow and collected $150 in deposits."*
  2. **Booking Detail:** Tapping the card reveals a unified timeline: Customer inquiry (DM) -> Agent provided quote -> Deposit paid -> Time slot locked.
  3. **Calendar View:** A translucent, thumb-friendly day-agenda view showing confirmed slots and travel buffers.
  4. **Manual Override:** A single "Reschedule" button that commands the agent to re-negotiate with the customer.

  ### AI Agent Integration
  - **Operations Assistant (AgentBot):** Listens to `Conversation` webhook events. If intent is `scheduling`, it queries the `Calendar` API for available `TimeSlot`s.
  - **Provisional Holds:** The agent applies a Redis Redlock on a `TimeSlot` for 10 minutes while waiting for the customer's Stripe Checkout session to complete.
  - **Handoff:** If the customer asks a complex question out of bounds, the bot flags the `ConversationStatus::PendingOwner` and surfaces it in the Triage Feed.

  ### Key Design Decisions
  - **UTC Exclusively:** All `TimeSlot`s are stored in UTC with a separate `display_timezone` field at the `Calendar` level to prevent daylight saving edge cases.
  - **Agentic-First:** Bookings are mutated via intent actions (e.g., `ProposeTime`, `ConfirmHold`), not direct CRUD operations, ensuring the agent and owner have the same operational capabilities.

  ## Implementation Prompt
  **For the Implementer Agent:**
  Your task is to build the Backend primitives for the Unified Calendar & Booking Engine in Rust (`src/server/services/booking/`).
  1. Define the SeaORM models for `Calendar`, `TimeSlot`, and `Booking` enforcing row-level security by `tenant_id`.
  2. Create a booking gateway with methods to `query_availability`, `hold_slot`, and `confirm_booking`.
  3. Integrate the gateway with the Operations Agent prompt context so the LLM can output structured JSON to call `query_availability`.
  4. Ensure a Playwright E2E test covers the flow: Customer asks for time -> Agent responds with slot -> Booking confirmed in UI.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
