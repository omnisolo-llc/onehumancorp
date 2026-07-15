issue_title: "[Architecture] Real-Time Agent-to-Human Handoff & Hybrid Teammate Mesh Protocol"
issue_description: |
  ## Problem Statement
  Small business owners like Maya (baker) and Carlos (handyman) rely heavily on OneHumanCorp's AI agents to triage inquiries, draft quotes, and manage schedules. However, there are critical moments where the AI reaches a boundary of certainty or encounters a high-stakes, emotionally sensitive customer scenario (e.g., a furious customer complaining about a ruined wedding cake, or a complex custom build request requiring Carlos's physical inspection). Currently, if an AI agent fails or needs help, the handoff to the human owner is clunky, loses conversational context, or drops the thread entirely. Owners need a seamless, invisible "Teammate Mesh" where the AI acts like a real receptionist—warming up the customer, gathering all context, and smoothly handing the conversation to the human exactly when needed, without the customer experiencing friction.

  ## Research Report
  - **Market Context**:
    - Current solutions (Zendesk, Intercom) treat bot-to-human handoff as a ticket escalation, which feels transactional and corporate. Small businesses survive on personal relationships and warmth.
    - Platforms like WeCom or HubSpot provide shared inboxes, but the AI and the human operate in silos rather than as a collaborative mesh.
  - **Identified Gap in OHC**:
    - OHC lacks a unified "Hybrid Teammate Mesh Protocol" that defines how an agent identifies an escalation threshold (e.g., via sentiment analysis or confidence drop) and seamlessly transfers the context lock to a human via the mobile push notification architecture.
  - **Persona Impact**:
    - **Maya**: Needs the AI to handle standard cake quotes, but immediately ping her phone if a customer uses words like "allergic" or "ruined."
    - **Carlos**: Wants the AI to book standard repairs, but hand off complex multi-day renovation inquiries to his inbox with a summarized brief.

  ## Design Doc
  ### High-Level Architecture
  The solution introduces a `TeammateMesh` coordination layer built on top of our Redis distributed locks and PostgreSQL event sourcing. It manages conversation state machines and "Context Locks."

  ```mermaid
  sequenceDiagram
      participant C as Customer (WhatsApp/Web)
      participant Gateway as OHC API Gateway
      participant Agent as CS Agent (Ambassador)
      participant Mesh as Teammate Mesh (Redis/PG)
      participant Human as Owner Mobile App (375px)

      C->>Gateway: "My cake is ruined! Needs fixing now!"
      Gateway->>Agent: Route Message
      Agent->>Agent: Analyze Sentiment (High negative, urgency)
      Agent->>Mesh: Request Context Lock Transfer (Reason: Escalation)
      Mesh->>Human: Push Notification: "Urgent: Maya, Sarah needs you."
      Agent->>Gateway: "I'm so sorry to hear that. I'm getting Maya right now."
      Gateway->>C: Auto-reply (Warm Handoff)
      Human->>Mesh: Acknowledge & Take Lock
      Mesh->>Agent: Release Agent from Thread
      Human->>Gateway: "Hi Sarah, this is Maya. Let's fix this..."
      Gateway->>C: Direct Owner Message
  ```

  ### Core Mechanisms
  1. **Context Locks (Redis Redlock)**: Every active conversation has a lock owner (`agent_id` or `human_id`). Only the lock owner can mutate the conversation state or send messages.
  2. **Confidence & Sentiment Thresholds**: Agents continuously score their responses. If confidence drops below 85% or sentiment is strongly negative, the agent autonomously initiates a `WarmHandoff` event.
  3. **Summarized Briefs**: Before handoff, the agent compiles a 2-sentence summary (e.g., "Sarah's wedding cake arrived melted. She is very upset.") so the owner doesn't have to read 20 messages.

  ### Mobile UX Flow (375px)
  - **Push Notification**: "Urgent Handoff: Sarah (Cake Order #124)"
  - **Handoff Screen**: A sleek Glassmorphism card detailing the Agent's summary, the customer's mood (e.g., "Angry 🔴"), and a big "Take Over Conversation" button.
  - **Seamless Entry**: When Carlos taps "Take Over," the app focuses the chat input, showing the agent's last message to the customer.

  ### AI Agent Integration
  - The **Ambassador (CS)** and **Salesperson** agents are updated with a `HandoffCapability` plugin, giving them tool access to emit `EscalateToHuman` events.

  ## Implementation Prompt
  **Objective**: Architect and implement the Hybrid Teammate Mesh protocol for real-time agent-to-human handoff.

  **CUJ & Acceptance Criteria**:
  1. **Data Model**: Extend the conversation/thread PostgreSQL schema to support `lock_owner_id` and `lock_owner_type` (agent/human), ensuring strict multi-tenant RLS.
  2. **Mesh Service (Go Backend)**: Implement the gRPC/REST endpoints for `RequestHandoff`, `AcceptHandoff`, and `AgentSummarizeContext`. Use Redis for distributed lock coordination.
  3. **Agent Capability**: Update the core AI agent prompt/tooling (Gemini) to include the `EscalateToHuman` tool. Test it with a negative sentiment payload to ensure it triggers correctly.
  4. **Mobile Simulation**: Ensure the backend emits the proper WebSocket or Server-Sent Events (SSE) so the mobile app receives the real-time handoff push.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
