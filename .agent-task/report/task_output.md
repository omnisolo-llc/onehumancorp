issue_title: "Implement Unified Omnichannel Customer Support & Real-Time Inbox"
issue_description: |
  **Title**: Implement Unified Omnichannel Customer Support & Real-Time Inbox

  **Problem Statement**:
  Non-technical operators (like Maya the baker and Carlos the handyman) are overwhelmed by disjointed communication channels. They receive inquiries across Instagram DMs, WhatsApp, SMS, and email. Switching between apps causes them to lose track of leads, miss order details, and drop conversations. They need a single, unified "assistant-first" inbox where an AI can triage messages, remember context, and draft replies across all channels, so they never drop a lead or context-switch again.

  **Research Report**:
  - **Podium**: Consolidates messaging for local businesses. High utility but pricing is steep and onboarding can take days. They lack deep native AI drafting tailored for solopreneurs without complex setups. (Source: Podium product documentation - https://www.podium.com/)
  - **Front**: Excellent for team collaboration on email and chat, but overly complex for a single operator. The UI is dense and desktop-first, alienating users who run their business from a phone. (Source: Front official documentation - https://front.com/)
  - **GoHighLevel**: Powerful marketing and messaging suite, but notoriously difficult to configure. It caters to marketing agencies rather than direct small business operators. The mobile experience is clunky. (Source: GoHighLevel community forums - https://www.gohighlevel.com/)
  - **User Sentiment**: "I lose Instagram DM orders because I forget to check them after answering texts." - Maya persona equivalent.
  - **OHC Gap**: OHC currently lacks a real-time, unified messaging layer natively integrating WhatsApp, IG, SMS, and Email into a single queue with AI-assisted drafting and triage.

  **Comparative Feature Matrix**:
  | Feature | Podium | Front | GoHighLevel | OHC (Proposed) |
  | :--- | :---: | :---: | :---: | :---: |
  | Unified Multichannel Inbox | ✅ | ✅ | ✅ | ✅ |
  | Phone-First Operator UX | ❌ | ❌ | ❌ | ✅ |
  | Invisible Zero-Config Setup | ❌ | ❌ | ❌ | ✅ |
  | Native AI Draft Approvals | ❌ (Basic) | ❌ | ❌ | ✅ |
  | Solopreneur Pricing Fit | ❌ | ❌ | ❌ | ✅ |

  **Design Doc**:
  - **High-level System Architecture**:
    ```mermaid
    graph TD
      A[Customer Channels: IG, WhatsApp, SMS, Email] -->|Webhooks/APIs| B[Omnichannel Adapters]
      B --> C[Real-Time Engine & Inbox - Native Rust]
      C --> D[AI Triage & Drafting - Gemini Pro]
      C --> E[PostgreSQL - Multi-Tenant Message Store]
      E --> F[Flutter Mobile Client 375px]
    ```
  - **Mobile UX Flow (375px first)**:
    1. **Inbox View**: A simple list of conversations prioritized by AI triage, not just chronological. Urgent leads at the top.
    2. **Conversation View**: Unified timeline. Instagram DM and SMS from the same customer appear in one thread.
    3. **Action Bar**: Persistent bottom bar with "Approve AI Draft", "Write Reply", or "Create Task/Quote". Touch targets are 44x44px.
  - **Libraries**: Use established Rust crates (e.g., `axum` for webhooks, `tokio-tungstenite` for WebSockets, `sqlx` for database).

  **Implementation Prompt**:
  Build a native Rust multi-tenant omnichannel customer support engine. Implement adapters for at least two channels (e.g., Web Chat Widget and Email). Build a high-performance WebSocket real-time messaging layer that persists sessions and routes messages to a unified timeline per customer. Provide the AI assistant hooks to read new messages and generate draft replies. The mobile UI must display these unified threads perfectly on a 375px screen and offer one-tap approvals for AI drafts. Do not use complex setup screens; configuration must be invisible or guided by the assistant.

  **Priority**: P0

  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
