issue_title: "[Architecture] Omni-Context Sub-Agent Routing & Human-in-the-Loop (HitL) Handoff Engine"
issue_description: |
  # Omni-Context Sub-Agent Routing & HitL (Human-in-the-Loop) Handoff Engine

  ## Problem Statement
  Business owners like Maya (baker) and Jun (location manager) rely on OHC's AI agents to handle work triage and customer relationships (e.g., WhatsApp DMs). However, when a customer request is too bespoke, vague, or emotionally charged (e.g., "my cake arrived damaged"), fully autonomous AI risks making costly mistakes or providing poor customer service. The current system lacks a standardized architectural primitive for an AI agent to gracefully pause its workflow, escalate to the human owner, and either hand over control completely or request a "steering prompt" to resume autonomy.

  ## Research Report
  - **Competitor Systems Audit:**
    - **Shopify Inbox & Chatbots:** Features a binary handoff—once a human intervenes, the bot turns off completely. It does not allow for "human steering" where the human gives a brief instruction and the AI continues the conversation.
    - **Intercom / Zendesk:** Enterprise support tools have "bot to human" routing, but they are designed for large support teams, not a single busy operator on a mobile phone who just wants to say "Approve refund" and let the AI write the apology.
    - **Wix:** Basic auto-replies, lacks sophisticated context-aware handoffs.
  - **Identify Gaps:** OHC needs an `Escalation & HitL Engine`. Instead of binary takeovers, OHC must allow "Steering" (owner provides a 5-word directive, AI drafts the full response) and "Approval" (AI drafts high-risk actions, owner 1-tap approves on mobile).

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer WhatsApp/IG] -->|Message| B(The Ambassador Agent);
      B -->|Sentiment/Confidence Check| C{Confidence > Threshold?};
      C -- Yes --> D[Auto Reply];
      C -- No / High Risk --> E[HitL Handoff Engine];
      E -->|Mesh Event| F[State Machine / Postgres hitl_requests];
      F -->|Push Notification| G[Owner Mobile App 375px];
      G -->|Steering Prompt: 'Offer 20% off'| F;
      F -->|Wake| B;
      B -->|Draft & Send| A;
  ```

  ### Mobile UX Flow (375px)
  - **Notification:** Owner receives a push: "Action Required: Maya's Cakes - Damaged Order".
  - **Escalation Card:** A premium translucent glass card in the Assistant Shell. It summarizes the issue ("Customer received damaged cake. AI confidence low.").
  - **Interaction:** Owner sees three buttons: `Approve AI Refund Draft`, `Steer AI`, `Take Over Chat`.
  - **Steering:** Tapping "Steer" opens a native keyboard. Maya types: "Apologize and offer a free replacement tomorrow." The AI drafts the polished empathetic message and sends it.

  ### AI Agent Integration Points & Data Model
  - `hitl_requests` table: `id`, `tenant_id`, `agent_id`, `session_id`, `status` (PENDING, STEERED, TAKEN_OVER, RESOLVED), `context_summary`, `created_at`.
  - Strict tenant isolation using PostgreSQL row-level security (`tenant_id`).
  - Redis PubSub (Teammate Mesh) used to pause/wake the agent dynamically using the KAIROS state machine.

  ## Implementation Prompt
  **Role:** Implementer Agent
  **Goal:** Implement the HitL Handoff Engine within the KAIROS Orchestration layer and the `inbox` API.
  - **User-Facing Outcome:** When the Ambassador Agent is unsure, it creates a HitL request. The owner sees an escalation card on their mobile dashboard, can type a quick "steering prompt", and the agent resumes the conversation seamlessly.
  - **Acceptance Criteria:**
    - Create the `hitl_requests` PostgreSQL schema with RLS enabled.
    - Implement the backend APIs for agents to pause and request HitL (`POST /api/hitl/escalate`), and for owners to provide steering (`POST /api/hitl/{id}/steer`).
    - Integrate with the Teammate Mesh (`mesh.rs`) to suspend and wake agent tasks using the centralized state machine (`statemachine_v2.rs`).
    - Provide full E2E Playwright tests demonstrating an agent pausing, the owner steering via the UI, and the agent resuming.
    - Ensure 100% unit test coverage for the hitl backend logic.

  **Priority:** P1
  **Estimated Scope:** Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
