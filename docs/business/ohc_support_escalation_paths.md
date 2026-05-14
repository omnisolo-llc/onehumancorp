# OHC Support Escalation Paths

This document details the escalation paths for users who require assistance beyond the automated in-app help systems. It ensures a smooth transition from self-service to human support.

## 1. The Support Funnel

Our goal is to resolve issues as early in the funnel as possible.

### Level 1: Proactive Help (Tooltips & Walkthroughs)
*   **Goal**: Prevent questions before they arise.
*   **Mechanism**: Contextual tooltips explain non-obvious UI elements. Interactive walkthroughs guide users through complex initial setups.

### Level 2: Self-Service (Help Center)
*   **Goal**: Empower users to find answers quickly.
*   **Mechanism**: The searchable, slide-out Help Center containing plain-language articles and short video tutorials.

### Level 3: AI Assistance (Help Agent)
*   **Goal**: Provide conversational answers based on Help Center content.
*   **Mechanism**: The floating AI Chat button. The agent synthesizes answers and provides links to full articles.

### Level 4: Human Support (Escalation)
*   **Goal**: Resolve complex, account-specific, or highly technical issues.
*   **Mechanism**: Ticketing system or live chat with a human agent.

## 2. Escalation Triggers

The transition from Level 3 to Level 4 (Human Support) should be seamless and triggered by specific conditions.

### Trigger 1: Explicit User Request
*   If a user types "I need a human," "talk to a person," or similar phrases, the AI agent must immediately offer an escalation link.

### Trigger 2: AI Failure
*   If the AI agent cannot find a relevant answer in the `HelpContent.ts` store (RAG failure), it must gracefully state its limitation and offer escalation.
*   *Example*: "I couldn't find a specific guide for that. Would you like me to connect you with our support team?"

### Trigger 3: Negative Feedback Loop
*   If a user downvotes multiple AI responses or indicates that the provided article did not solve their problem, the system should proactively prompt for escalation.

## 3. Escalation Handoff Process

When an escalation occurs, context is critical. The human agent must not ask the user to repeat themselves.

*   **Context Transfer**: The entire chat history with the AI agent, the user's current page URL, and their recent Help Center search queries must be bundled and attached to the support ticket.
*   **User Expectation Setting**: The system must clearly communicate the expected response time (e.g., "A support specialist will review your request and reply within 2 hours").
