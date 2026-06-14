issue_title: "Unified Multi-Agent Orchestrator Architecture"
issue_description: |
  **Problem Statement**
  The core promise of OHC is that an owner can "Ask one assistant; it coordinates messages, customers, tasks, calendar... and agent work behind the scenes." Currently, OHC lacks a unified, multi-agent orchestrator architecture capable of receiving a vague user input (e.g., "I need to set up my cake shop and take deposits") and dynamically planning, delegating, and executing the cross-functional tasks needed to fulfill it. Without this, the system behaves like a collection of siloed bots rather than a single coordinated work assistant.

  **Research Report**
  - **Competitor Analysis:**
    - **AutoGPT / BabyAGI:** Demonstrated the power of autonomous task planning and execution, but often get stuck in loops or require technical knowledge to steer.
    - **LangGraph:** Provides a robust framework for building stateful, multi-actor applications with LLMs. It excels at cyclic graphs, which is essential for agentic workflows where agents need to collaborate, critique, and iterate.
    - **Claude Code:** Shows how an agent can be deeply integrated into a specific environment (a codebase) to autonomously execute tasks with user oversight.
  - **OHC Gap:** OHC needs a centralized orchestration layer (the "KAIROS Orchestrator") that sits between the user interface and the specialized AI departments (Operations, Finance, CS, etc.). This orchestrator must act as the unified brain, capable of understanding complex, multi-step requests, breaking them down into an execution graph, and routing sub-tasks to the appropriate specialized agents.

  **Design Doc**

  *Architecture Diagram:*
  ```mermaid
  graph TD
      User([User Request:<br>"Set up cake shop<br>& take deposits"]) --> Orchestrator

      subgraph KAIROS Orchestrator
          Orchestrator[Unified Orchestrator] --> Planner[Task Planner]
          Planner --> Router[Task Router]
          Router --> Memory[Shared Context Memory]
      end

      subgraph Specialist Agents
          Router --> Operations[Operations Agent<br>(Catalog & Inventory)]
          Router --> Finance[Finance Agent<br>(Stripe Deposits)]
          Router --> Ambassador[CS Agent<br>(Auto-replies)]
      end

      Operations --> ResultAggregator
      Finance --> ResultAggregator
      Ambassador --> ResultAggregator

      ResultAggregator[Result Aggregation] --> Orchestrator
      Orchestrator --> UserResponse([Unified Feed Update:<br>"Shop ready. Deposit link created."])
  ```

  *Mobile UX Flow (375px):*
  1. **Input:** The user opens the OHC app and types or speaks into the main assistant input: "I want to start selling vegan cakes, require a 50% deposit, and auto-reply to Instagram DMs."
  2. **Thinking State:** The feed shows a sleek, translucent glass card indicating "Coordinating your setup..." with subtle pulsing indicators for "Updating catalog," "Configuring Stripe," and "Setting up auto-replies."
  3. **Execution & Approval:** If a sub-task requires approval (e.g., confirming the deposit amount), a prioritized Action Card appears in the feed.
  4. **Resolution:** Once complete, a single, plain-language summary card appears: "Your vegan cake shop is live. Deposits are set to 50%. I've drafted an auto-reply for Instagram. Tap to review."

  *AI Agent Integration:*
  - **The Orchestrator (KAIROS):** Acts as the primary interface. Uses a complex LLM prompt to decompose the user's intent into a Directed Acyclic Graph (DAG) of tasks.
  - **Specialist Agents:** Register their capabilities (tools) with the Orchestrator. They execute isolated tasks and report status back to the shared memory state.

  *Key Design Decisions:*
  - Implement the orchestrator using a stateful graph architecture (inspired by LangGraph) to allow for cyclic reasoning (e.g., an agent failing a task and asking the user for clarification before retrying).
  - Use a Redis-backed shared memory context to maintain state across different agent executions, ensuring the Finance agent knows what the Operations agent just created.

  **Implementation Prompt**
  Implement the core `UnifiedOrchestrator` service and the baseline state graph for multi-agent coordination.
  - **User Outcome:** The user can give a single, complex command that requires the coordination of at least two distinct specialized agents, and the system executes it cohesically.
  - **CUJ:** As Maya (Baker), I tell the assistant "Set up a new product for Wedding Cakes and make sure I get a $100 deposit." The Orchestrator routes the product creation to the Operations Agent, the deposit configuration to the Finance Agent, and returns a single success summary to my feed.
  - **Acceptance Criteria:**
    - Create the `UnifiedOrchestrator` class that can parse intent and generate a task execution plan.
    - Implement a basic state machine or DAG execution engine to run the tasks.
    - Define interfaces for `SpecialistAgent` registration.
    - Ensure all inter-agent context is stored and retrieved from a shared data structure.

  **Priority:** P0
  **Estimated Scope:** Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
