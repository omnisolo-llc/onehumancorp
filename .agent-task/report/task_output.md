issue_title: "Architect Autonomous Inter-Agent Negotiation & Consensus Mesh"
issue_description: |
  ## Mission Queue Protocol: The "Boardroom" Engine

  ### Title: Autonomous Inter-Agent Negotiation & Consensus Mesh

  ### Problem Statement
  Small business owners like **Maya (baker)** and **Carlos (handyman)** are often paralyzed by conflicting business priorities. For example, Maya's **Marketing Agent** might suggest a "Buy One Get One Free" sale to drive growth, while her **Finance Agent** flags that this would result in a net loss on every cake. Currently, such conflicts either result in the owner receiving two contradictory notifications or the first agent to "claim" the task winning by default. This creates "Decision Fatigue" and erodes trust in the assistant. Owners need their AI departments to debate, negotiate, and reach a consensus *before* presenting a single, unified recommendation.

  ### Research Report
  - **The "Chatty" Problem**: Most multi-agent frameworks (AutoGen, CrewAI) allow agents to talk to each other but lack a formal "Constraint-Based Negotiation" protocol. They often loop endlessly or produce "groupthink" without respecting department-specific utility functions (e.g., Finance must prioritize margins).
  - **Competitive Analysis**:
    - **Shopify/Wix**: No multi-agent coordination. Each AI tool (description generator, ad buyer) operates in a silo.
    - **Enterprise ERPs**: Use complex rule-based workflow engines, which fail the "Grandmother Test" due to configuration overhead.
  - **The OHC Leap**: We will implement a **Consensus Mesh** where each agent has a "Departmental Mandate" (Utility Function). When a proposed action impacts multiple departments, the KAIROS Orchestrator triggers a "Deliberation Session."

  ### Design Doc

  #### Architecture Diagram (Mermaid.js)
  ```mermaid
  sequenceDiagram
      participant Hub as KAIROS Orchestrator
      participant Mkt as Marketing Agent (Goal: Volume)
      participant Fin as Finance Agent (Goal: Margin)
      participant Adv as Business Advisory (Arbitrator)
      participant User as Maya (Mobile 375px)

      Mkt->>Hub: Propose Action: "50% Discount on Cupcakes"
      Hub->>Hub: Detect Conflict: Finance Impact High
      Hub->>Fin: Deliberate: Review Discount Proposal
      Fin->>Hub: Counter-Offer: "Max 15% discount to keep 30% margin"
      Hub->>Mkt: Review Counter-Offer
      Mkt->>Hub: Consensus: "Agreed on 20% discount + Free Delivery"
      Hub->>Adv: Validate Consensus Rationale
      Adv-->>Hub: Approved: Balanced Growth & Profit
      Hub->>User: 1-Tap Card: "Balanced Growth Plan: 20% Discount + Free Delivery"
  ```

  #### Data Model & Invariants
  ```mermaid
  erDiagram
      DELIBERATION_SESSION ||--o{ AGENT_STANCE : contains
      DELIBERATION_SESSION {
          uuid id PK
          uuid original_task_id FK
          string status "OPEN | RESOLVED | STALEMATE"
          jsonb consensus_result
      }
      AGENT_STANCE {
          uuid session_id FK
          string agent_id
          string argument
          float utility_score "0.0 - 1.0"
          boolean approved
      }
  ```
  **Key Invariants:**
  1. **Mandate Conflict Detection**: Any task tagged with `risk: HIGH_FINANCIAL` or `impact: CROSS_DEPT` MUST trigger a deliberation if more than one agent has registered interest in those tags.
  2. **The "Advocate" Rule**: Each agent must argue *strictly* for its department's primary goal (e.g., Marketing cannot concede margin without a documented "Growth Multiplier" justification).
  3. **Zero-Jargon Transparency**: The owner never sees the word "Utility Function." They see "The Accountant and The Promoter have agreed on a plan that grows your sales while keeping your profits safe."

  #### Mobile UX Flow (375px First)
  1. **The Pulse**: A shimmering card on the dashboard says: *"Your agents are discussing a new strategy to fill your calendar next week..."* (macOS Glassmorphism styling).
  2. **The Result**: A notification appears. Tapping it opens a **Consensus Card**:
     - **Recommendation**: "The 20% 'Early Bird' Special."
     - **Why**: "Promoter says it will fill 5 slots; Accountant says it covers your costs."
     - **Action**: `[ Approve Strategy ]` or `[ View Details ]`.
  3. **The "Grandmother Test"**: Maya doesn't need to know how they negotiated. She just needs to know they both "signed off" on the plan.

  ### Implementation Prompt
  **To the Implementer:**
  Implement the `DeliberationService` within the `src/server/orchestration/` module.
  1. Update the `DepartmentOrchestrator` to intercept `execute_action` calls that flag cross-departmental impact.
  2. Implement a "Round-Robin Negotiation" loop where the proposing agent and the conflicting agent exchange "Stances" (Stance = Argument + Utility Score).
  3. Integrate the LLM provider to evaluate if a "Consensus" has been reached based on the mandates defined in `departments.rs`.
  4. Build the backend API to surface active deliberations to the mobile dashboard.
  5. Ensure strict multi-tenant isolation so Maya's agents never "hear" the debate happening in Carlos's business.

  ### Priority: P1
  ### Estimated Scope: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
