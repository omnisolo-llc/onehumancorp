issue_title: "AI-to-Human Seamless Handoff & Escalation Engine Design"
issue_description: |
  # AI-to-Human Seamless Handoff & Escalation Engine

  ## Problem Statement
  When OneHumanCorp's autonomous agents operate invisibly on behalf of our users (Maya, Carlos, Priya, Leo, Fatima), they inevitably encounter boundary conditions or complex edge cases. For example, Maya receives an Instagram DM with a photo of an extremely intricate wedding cake that the AI cannot confidently quote, or Carlos's client requests a non-standard service outside his listed catalog. Currently, an unhandled scenario creates friction for the customer (awkward "I am just an AI" responses) and anxiety for the business owner who might miss a high-value lead. We need a unified, mobile-first architectural pattern that allows AI agents to gracefully escalate conversations to the business owner, capture the human's manual response seamlessly from their 375px mobile device, and use that response to enrich the AI's contextual memory—all without breaking the illusion of seamless service for the end customer.

  ## Research Report
  **Market Landscape & Competitor Audit**
  - **Shopify & Wix**: Rely almost exclusively on basic static chatbots or third-party integrations (like Gorgias) which clearly demarcate "bot" vs. "human" phases. They lack built-in context-preserving handoffs that blend invisibly.
  - **Intercom / Zendesk**: Heavy, enterprise-focused tooling that overwhelms our non-technical small business owners. They are not built for a solo operator using only a smartphone on the go.
  - **The Gap**: No existing SMB platform provides a *Zero-Friction Handoff* that looks indistinguishable from a single responsive human, where the business owner is notified dynamically and can interject into the ongoing stream directly via a unified mobile feed, while training the AI simultaneously.

  ## Design Doc

  ### Architecture Overview

  ```mermaid
  erDiagram
      CONVERSATION {
          string id
          string channel "e.g., IG_DM, WhatsApp"
          string state "AI_ACTIVE, PENDING_HUMAN, HUMAN_ACTIVE"
          datetime last_escalated
      }
      MESSAGE {
          string id
          string sender "AI, HUMAN, CUSTOMER"
          text content
          boolean is_escalation_trigger
      }
      ESCALATION_CONTEXT {
          string id
          string reason "CONFIDENCE_LOW, EXPLICIT_REQUEST, UNKNOWN_INTENT"
          json ai_summary
      }
      MEMORY_UPDATE {
          string id
          text newly_learned_fact
      }

      CONVERSATION ||--o{ MESSAGE : contains
      MESSAGE ||--o| ESCALATION_CONTEXT : triggers
      ESCALATION_CONTEXT ||--o| MEMORY_UPDATE : generates
  ```

  ```mermaid
  sequenceDiagram
      participant C as Customer (IG DM)
      participant AI as OHC AI Agent
      participant N as Notification Service
      participant O as Owner (Mobile UI)
      participant M as Context Memory

      C->>AI: Sends complex request (e.g., photo of custom cake)
      AI->>AI: Confidence score < Threshold
      AI->>C: "That sounds amazing! Let me double-check the specifics for you really quick."
      AI->>N: Trigger High-Priority Handoff Event (includes context summary)
      N->>O: Push Notification: "New custom request requires your input."
      O->>N: Opens Mobile Feed (375px)
      N->>O: Displays AI Summary + Proposed Draft (if any)
      O->>AI: Edits draft and taps 'Send'
      AI->>C: Owner's customized response
      AI->>M: Consolidates human response into business context for future
  ```

  ### Mobile-First UX Flow (375px Viewport)
  1. **Push Notification Alert**: "Action Needed: Maya, a customer asked about vegan fondant pricing."
  2. **The "Handoff Card" UI**: Utilizing macOS-style Translucent Glass and UniFi modular dashboard design.
     - **Header**: Customer Name + Source Icon (Instagram).
     - **Context Block**: A brief 2-sentence AI-generated summary of the conversation so far.
     - **Visuals**: Any images attached by the customer clearly displayed.
     - **Action Area**: A pre-filled text area with the AI's *best guess* response, which the owner can either tap to send immediately, or tap to edit.
     - **Fallback Actions**: Quick reply buttons (e.g., "I'm busy, tell them I'll reply tonight", "Politely decline").
  3. **Seamless Resumption**: Once the human sends the message, the UI confirms "Message sent. AI has learned this preference." The AI resumes listening and answering standard follow-ups.

  ### Key Design Decisions
  - **Invisible Handoff**: The end customer never sees a "Transferring you to a human" message unless explicitly configured. The AI uses natural stalling tactics ("Let me pull up my schedule...") to buy time for the human.
  - **Asynchronous Safe Fallbacks**: If the business owner doesn't respond within 5 minutes, the AI is programmed to send a polite expectation-setting message ("We're checking the kitchen right now, we'll have an answer for you shortly!").
  - **Auto-Learning Loop**: Every time the owner overrides or sends a manual message during a handoff, the agent's contextual memory is updated asynchronously, ensuring the next identical edge-case can be handled autonomously.
  - **Strict Isolation**: Escalation events and memory updates are strictly isolated per tenant using Zero Trust multi-tenant patterns.

  ## Implementation Prompt
  **Role**: Engineering Swarm Implementer
  **Objective**: Build the AI-to-Human Seamless Handoff & Escalation Engine.
  **User-Facing Outcome**: When an AI agent encounters a low-confidence scenario, it gracefully stalls the customer while instantly surfacing a rich "Action Required" card in the business owner's mobile feed. The owner can review a summary, edit a proposed response, and send it—seamlessly returning control to the AI and updating its knowledge base for next time.
  **Acceptance Criteria**:
  1. Implement the core Escalation Event emitter within the AI agent flow that triggers when confidence is below the threshold or the user explicitly asks for the owner.
  2. Build the unified mobile-first UI component (Handoff Card) that displays the conversation summary, context, and editable AI-proposed response.
  3. Implement the background worker that updates the specific tenant's AI context memory based on the human's manual response.
  4. Ensure the end customer receives natural, non-robotic placeholder messages while waiting for the human.
  5. All data must adhere strictly to multi-tenant isolation boundaries.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
