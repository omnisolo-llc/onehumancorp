issue_title: "Architecting the Unified Omni-Channel Fulfillment Orchestrator"
issue_description: |
  ### Problem Statement
  Currently, OneHumanCorp (OHC) handles work intake in a fragmented manner. When a customer like "Sarah" messages Maya the baker on Instagram, the `MessageTriageWorker` might create a draft reply, but it doesn't automatically ensure the calendar is free or the price is optimized. The owner (Maya) is forced to perform manual context-switching between different dashboards. A real "Workbuddy" should present a single, coordinated decision: "Sarah wants a Vegan Cake on Friday for $55. Your calendar is clear and price includes peak surge. Send the deposit link?"

  ### Research Report
  Analysis of Tencent Workbuddy and WeCom reveals a "Workbench" philosophy where communication is inseparable from execution. Competitors like Shopify Sidekick are adding assistant layers, but OHC's "Unfair Advantage" is the deep integration between the agentic departments (Ops, Sales, Finance) and the multi-channel inbox.
  - **Competitive Gap**: Most assistants are "Read-only" or "Single-action". OHC needs a "Multi-agent Synthesis Loop" that coordinates internal state before bothering the owner.

  ### Design Doc

  #### Architecture Diagram (Mermaid.js)
  ```mermaid
  sequenceDiagram
      participant C as Customer (Instagram/WhatsApp)
      participant T as MessageTriageWorker
      participant O as FulfillmentOrchestrator
      participant OPS as Operations Agent (Calendar/Inventory)
      participant S as Sales Agent (Pricing/Ledger)
      participant OW as Owner App (375px)

      C->>T: "I want a vegan cake Friday"
      T->>O: Event: New Inquiry {type: cake, date: Fri}
      O->>OPS: Check Availability (Friday)
      OPS-->>O: Status: AVAILABLE
      O->>S: Calculate Price (Vegan Cake, Surge: Peak)
      S-->>O: Price: $55.00
      O->>O: Synthesize "Master Proposal"
      O->>OW: Push Action: "Maya, 1 order ready to confirm"
      OW->>OW: Show Translucent Glass Card (Summary + Pay Link)
      Maya->>OW: Tap "Approve & Send"
      OW->>O: Commit Action
      O->>C: Send "Confirmation + Deposit Link"
  ```

  #### Mobile UX Flow (375px First)
  1. **Notification**: "New Order Opportunity: Sarah (Instagram)".
  2. **Dashboard Card**: A premium macOS-style translucent card.
     - **Title**: "Fulfillment Draft: Custom Cake"
     - **Summary**: "$55.00 for Friday, July 4th"
     - **Logic Proofs**: "✅ Spot reserved in calendar. ✅ Surge pricing applied (+15%)."
     - **Interaction**: Single primary action "Approve & Send" (44x44px target).

  #### AI Agent Integration Points
  - **The Dispatcher**: `MessageTriageWorker` now routes to `FulfillmentOrchestrator` instead of writing directly to the feed.
  - **The Sub-Agents**: `OperationsAgent` and `SalesAgent` gain new `EvaluateProposal` capabilities.
  - **The Synthesizer**: A final LLM pass that combines departmental JSON into owner-friendly English.

  #### Key Design Decisions
  - **Centralized Orchestration**: Move from "Fire-and-forget" triage to a "Stateful Action Loop". This ensures the owner never sees a "hallucinated" proposal that conflicts with real business data (like a double booking).
  - **Fulfillment Drafts Table**: Introduce a persistence layer for multi-agent negotiation turns to ensure reliability on flaky networks.

  ### Implementation Prompt
  "Implement the `FulfillmentOrchestrator` service. This service must listen for `triage.inquiry` events. Upon a new inquiry, it should orchestrate a parallel check with the `Operations` department (via `BookingService`) and the `Sales` department (via `PricingEngine`). Synthesize these results into a single `agent_feed_item` with `lifecycle_state = 'PENDING_APPROVAL'`. The UI must render this as a unified fulfillment card. Acceptance criteria: A Playwright E2E test must prove that Maya can approve a coordinated order (Availability + Pricing) in exactly one tap from her mobile dashboard."

  ### Priority: P0
  ### Estimated Scope: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, mobile-first]
assignees: []
