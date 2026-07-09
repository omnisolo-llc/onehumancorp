issue_title: "[Architecture] Omni-Channel AI Inbox for Unified Customer Communications"
issue_description: |
  ## Problem Statement
  Small business owners and operators (like Maya the baker or Nora the agency principal) are overwhelmed by fragmented communication channels. They receive inquiries via Instagram DMs, WhatsApp, email, SMS, and web chat. Currently, tracking these conversations requires constantly switching between apps, leading to missed leads, delayed responses, and lost context. They need a single, unified "Omni-Channel AI Inbox" where all customer communications are aggregated, triaged by AI, and actioned within the same workspace where they manage operations and payments.

  ## Research Report
  **Competitive Analysis:**
  - **Shopify:** Relies on third-party apps like Gorgias or Zendesk, which are expensive, complex to set up, and not deeply integrated with the core platform's agentic workflows.
  - **Wix/Square:** Offer basic unified inboxes but lack autonomous AI drafting and intelligent triage based on business context (e.g., matching an Instagram DM to an active order).
  - **OHC Market Gap:** By integrating a unified inbox natively with our AI Swarm, OHC can not only aggregate messages but actively assist. The AI can draft responses based on inventory, past orders, and policies, providing a massive "unfair advantage" over traditional platforms.

  ## Design Doc
  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  graph TD;
      subgraph External Channels
          IG[Instagram DMs] --> WebhookGateway;
          WA[WhatsApp] --> WebhookGateway;
          Email[Email Inbound] --> WebhookGateway;
          WebChat[Website Chat] --> WebhookGateway;
      end

      subgraph OHC Backend
          WebhookGateway[Ingress Gateway] --> TriageQueue[(Event Queue)];
          TriageQueue --> Triager[AI Triage Agent];
          Triager --> ConversationDB[(Unified Conversation CRDT)];
          Triager --> Routing[Agent Routing];

          Routing --> SalesAgent[Sales / Quoting];
          Routing --> SupportAgent[Customer Support];
      end

      subgraph Frontend (Flutter/Web)
          ConversationDB --> UI[Omni-Channel Inbox UI];
      end
  ```

  ### Mobile UX Flow (375px First)
  1. **Unified Feed:** The user opens the OHC mobile app. The primary screen shows a unified inbox. Each thread clearly indicates its source (e.g., a small Instagram or WhatsApp icon) and the customer's name.
  2. **AI Triage Labels:** Threads are automatically tagged by the AI (e.g., "Urgent," "Quote Request," "Issue").
  3. **Thread View:** Tapping a thread opens the chat. The UI uses macOS-style Translucent Glass materials for message bubbles.
  4. **AI Drafts:** At the bottom, instead of just a keyboard, the AI presents a pre-drafted response based on context (e.g., "Hi, yes we can do vegan cakes! Should I send a quote?"). The user can tap "Approve & Send" or edit the draft.
  5. **Quick Actions:** A "+" menu allows the user to instantly generate an invoice or booking link directly into the chat flow.

  ### AI Agent Integration Points
  - **Triage Agent:** Analyzes incoming messages to determine intent, urgency, and the appropriate sub-agent.
  - **Drafting Agent (Customer Success):** Uses the tenant's context (inventory, past interactions, tone guidelines) to draft responses.
  - **Operations Agent:** Can be invoked from the chat to check availability or stock before drafting a reply.

  ## Implementation Prompt
  **Goal:** Implement the backend data model and the mobile-first (375px) UI for the Omni-Channel AI Inbox.
  **CUJ (Critical User Journey):**
  1. Maya receives a mock incoming Instagram DM asking about a custom cake order.
  2. The message appears in the new unified inbox UI with an "Instagram" source indicator.
  3. Maya taps the message and sees an AI-generated draft response proposing a next step.
  4. She taps "Send", and the UI optimistically updates the thread.
  **Acceptance Criteria:**
  - Create the `Conversation` and `Message` entities with tenant isolation and channel source tracking.
  - Implement the 375px mobile UI for the inbox list and thread view, adhering to the premium translucent glass design system.
  - Integrate a mock or basic agent call to generate a draft response upon opening the thread.
  - Provide full Playwright E2E test coverage for the inbox navigation, draft approval, and sending flow.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
