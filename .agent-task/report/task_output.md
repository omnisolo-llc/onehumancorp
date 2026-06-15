issue_title: "[Architecture Design] Offline-First Mutative Sync & AI Conflict Resolution for Mobile Workers"
issue_description: |
  ## Mission Queue Protocol: Offline-First Mutative Sync & AI Conflict Resolution

  ### Problem Statement
  Non-technical owner/operators like **Carlos (Field Service Owner)** and **Fatima (Food Cart Operator)** frequently operate in environments with poor or nonexistent cellular data—basements, rural areas, or crowded events. When OHC requires a continuous network connection to capture leads, record payments, update order statuses, or assign tasks, the tool becomes a liability instead of an assistant. If offline writes fail, data is lost, trust is broken, and operations stall. They need a system that feels fast and reliable regardless of network conditions, automatically syncing changes and resolving conflicts (like double-bookings or oversells) via an AI agent when the connection is restored, without needing to understand "sync errors" or "offline mode."

  ### Research Report
  - **Market Context**: Platforms like Shopify POS and Square handle offline payments but often struggle with complex offline mutative state (e.g., changing an order, updating inventory, and rescheduling simultaneously). Specialized field-service apps (Jobber, ServiceTitan) support offline but have rigid, non-agentic conflict resolution that forces the user to manually review merge conflicts.
  - **User Need**: Carlos needs to complete a job, add a custom line item, and mark it paid while in a basement. Fatima needs to mark items out of stock instantly on her device even if her cell connection drops, queuing the update to the central storefront.
  - **The OHC Differentiator**: Instead of showing complex "Sync Error - Conflict" screens, OHC uses an **Operations AI Agent** to automatically handle conflict resolution. If Carlos and his assistant both update a job offline, the AI agent reviews both intents, intelligently merges non-conflicting fields, and only surfaces a simple, natural-language question if there's an unresolvable business conflict (e.g., "You and John both scheduled different appointments for 2 PM on Tuesday. Should we ask one of them to move to 3 PM?").

  ### Design Doc
  #### 1. Mobile UX Flow (375px First)
  - **Seamless Offline Transition**: No jarring "You are offline" modal. A subtle, translucent glass indicator at the top of the UI indicates "Working offline — changes saved securely."
  - **Action Execution**: User performs actions normally (e.g., Tap "Complete Job", Add "$50 Parts", Tap "Collect Payment"). The UI updates instantly (optimistic UI) using local device storage.
  - **Reconnection & Sync**: Upon regaining signal, a subtle background animation shows syncing. If the Operations Agent identifies a conflict, a unified Work Triage notification appears: "⚠️ Schedule conflict detected. Review options."
  - **Agent Interaction**: Tapping the notification opens a simple chat-like interface where the Operations Agent explains the issue and offers 1-tap resolution buttons.

  #### 2. Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Mobile as Flutter PWA (Local DB/Queue)
      participant Gateway as OHC API Gateway
      participant SyncEngine as Mutative Sync Engine
      participant DB as PostgreSQL (Tenant Data)
      participant OpsAgent as Operations AI Agent

      Mobile->>Mobile: User updates data (Offline)
      Mobile->>Mobile: Optimistic UI Update & Queue Event
      Note over Mobile,Gateway: Network Restored
      Mobile->>Gateway: Push Sync Queue (Event Batch)
      Gateway->>SyncEngine: Process Events
      SyncEngine->>DB: Check Latest Server State
      alt No Conflict
          SyncEngine->>DB: Apply Mutations
          SyncEngine-->>Mobile: Sync Success
      else Conflict Detected
          SyncEngine->>OpsAgent: Send conflicting intents
          OpsAgent->>OpsAgent: Analyze business rules & context
          alt Auto-resolvable
              OpsAgent->>DB: Apply Merged State
              OpsAgent-->>Mobile: Sync Success (with silent note)
          else Requires Owner Input
              OpsAgent->>DB: Save Conflict State
              OpsAgent-->>Mobile: Push Triage Task (Requires Decision)
          end
      end
  ```

  #### 3. AI Agent Integration Points
  - **Operations Agent (Conflict Resolver)**: Subscribes to the dead-letter/conflict queue of the Sync Engine. Uses a specific prompt template injected with tenant business rules to decide if an offline mutation can be merged safely or needs owner escalation.
  - **Work Triage Agent**: Receives escalations from the Operations Agent and formats them into bite-sized, plain-language actionable cards for the owner's feed.

  #### 4. Key Design Decisions
  - **Event Sourcing / CRDTs for Mobile Queue**: Instead of sending final states, the mobile app queues *intent events* (e.g., `JobCompleted`, `ItemAdded`). This gives the Sync Engine and AI Agent maximum context to resolve conflicts.
  - **Local-First PWA Storage**: Utilize IndexedDB (via Flutter plugins) for a robust local cache and mutation queue.
  - **Agent-in-the-Loop Resolution**: Never show technical merge conflict UI. Always use the AI to either fix it or ask a simple business question.

  ### Implementation Prompt
  **To the Implementer:**
  Implement the backend Sync Engine and the Flutter offline-mutation queue to support offline-first operations.
  1. Define the Protobuf/gRPC contracts for the Sync Event Queue.
  2. Implement a local storage queue in the Flutter PWA that intercepts critical mutations when offline and applies optimistic UI updates.
  3. Build the backend Sync Engine that processes these event batches.
  4. Integrate the Operations Agent to handle write conflicts. When a conflict occurs (e.g., optimistic version mismatch), pass the conflicting events to the LLM with the context of the current state.
  5. If the LLM cannot safely merge, create a Work Triage Task for the owner.
  **Acceptance Criteria:** A user can turn off their network, complete a task in the UI, turn the network back on, and see the task successfully synced to the backend without error popups. A forced conflict must result in an Operations Agent triage task, not a raw database error.

  ### Priority
  P0

  ### Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, mobile-first]
assignees: []
