issue_title: "Unified Agentic Staff Scheduling & Task Dispatch Architecture"
issue_description: |
  # Research Report: Unified Agentic Staff Scheduling & Task Dispatch Architecture

  ## 1. Problem Statement
  Small business owners and operators (e.g., Jun the Location Manager, Carlos the Field Service Owner, Nora the Agency Principal) struggle with coordinating staff, assigning tasks, and managing schedules. Traditional scheduling tools (like Homebase or When I Work) are disconnected from the core business operations (sales, bookings, inventory) and require manual configuration. Owners need an assistant that can autonomously generate schedules, assign tasks based on real-time business demand, and handle staff shift swaps seamlessly on a mobile device.

  ## 2. Research Report (Track 1)
  - **Market Context**: Existing platforms like Square or Shopify require third-party apps for robust staff scheduling, which creates data silos. Standalone apps like Sling or Deputy do not know about the business's real-time service bookings or inventory shipments, resulting in understaffing or overstaffing.
  - **The OHC Opportunity**: By natively integrating staff scheduling with the Booking and E-commerce systems, OHC can use the Operations Agent to autonomously forecast staffing needs, automatically assign tasks (e.g., "Fulfill 5 cake orders today" to a specific baker), and handle employee shift swaps via natural language.
  - **Competitor Gaps**:
    - *Shopify*: Basic staff permissions, but no native shift scheduling or task dispatching tied to orders.
    - *Square*: Has team management, but lacks AI-driven autonomous scheduling and natural language shift swapping.
    - *Standalone (Homebase/When I Work)*: Detached from actual business demand (bookings, orders, client projects).

  ## 3. Design Doc (Track 2 & 3)
  ### Architecture Diagram & Data Model
  ```mermaid
  erDiagram
    StaffMember ||--o{ Shift : "assigned to"
    Shift ||--o{ Task : "includes"
    StaffMember ||--o{ ShiftSwapRequest : "requests"
    ShiftSwapRequest }|--|| Shift : "swaps"
    Order ||--o{ Task : "generates"
    Booking ||--o{ Task : "generates"
    OperationsAgent ||--o{ Shift : "schedules automatically"
    OperationsAgent ||--o{ ShiftSwapRequest : "approves/manages"
  ```
  - **`StaffMember`**: Represents an employee or contractor, with roles, skills, and availability preferences.
  - **`Shift`**: A designated working block for a `StaffMember` at a specific location or route.
  - **`Task`**: A specific action (e.g., fulfill order, repair HVAC) assigned to a `Shift` or `StaffMember`, linked to `Order` or `Booking`.
  - **`ShiftSwapRequest`**: State machine for shift swaps, managed autonomously by the Operations Agent.

  ### Mobile UX Flow (375px)
  1. **Owner View**: The owner sees a unified Operations feed. A card suggests: "Demand is high this Saturday due to 15 cake orders. Should I add an extra shift for Baker A? [Approve & Notify]".
  2. **Staff View**: Staff members log in to a simplified worker view on their phone. They see their upcoming shifts and assigned tasks.
  3. **Natural Language Swap**: A staff member messages the AI: "I'm sick, can someone cover my Tuesday morning shift?". The Operations Agent automatically finds available staff, asks for coverage, and upon agreement, updates the schedule and notifies the owner.

  ### AI Agent Integration
  - **Operations Agent**: Monitors bookings, orders, and historical data to forecast staffing needs. Drafts the weekly schedule for owner approval.
  - **Internal Comms Agent**: Handles back-and-forth communication for shift swaps and task updates via SMS or native app notifications.

  ### Mobile-First & Security Integrity
  - All interfaces must be perfectly usable on a 375px screen with large >44px touch targets.
  - **Multi-Tenant Isolation**: Strict PostgreSQL RLS on `tenant_id` for all `StaffMember`, `Shift`, and `Task` tables to ensure cross-tenant data privacy. Redis keys for scheduling locks will use `ohc:lock:{tenant_id}:schedule:{shift_id}`.

  ## 4. Implementation Prompt (Track 4)
  **Feature Name**: OHC Agentic Staff Scheduling & Task Dispatch
  **Target Persona**: Jun the Location Manager & Carlos the Field Service Owner
  **Outcome**: Jun can approve an AI-generated weekly staff schedule that automatically accounts for upcoming orders and bookings. Staff can view their tasks and request shift swaps directly through the OHC mobile interface.

  **Next Actions for Engineering**:
  1. Implement the core PostgreSQL data models (`StaffMember`, `Shift`, `Task`, `ShiftSwapRequest`) with strict RLS multi-tenant isolation.
  2. Build the 375px mobile-first UI for the Owner (Schedule Approval Card) and Staff (My Shifts & Tasks view).
  3. Create the Operations Agent capability to parse natural language for shift swaps and automatically reassign associated tasks.
  4. Integrate the task dispatch system with the existing `Order` and `Booking` models to auto-generate tasks based on new demand.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
