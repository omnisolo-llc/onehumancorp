# Feature Brief: Invisible Auto-Replying AI Agents

## Priority: P0 (Critical)
**Strategic Goal:** Solve "Operational Fatigue" & "Communication Lag".

### 1. Problem Statement
Micro-businesses manage inquiries across Instagram DMs, WhatsApp, and email. This communication fragmentation leads to missed messages and lost sales. Business owners spend hours manually answering repetitive questions ("What are your hours?", "Do you do vegan options?").

### 2. The OHC Solution
Invisible AI agents ("The Ambassador") that monitor connected communication channels, read the context of incoming messages, and automatically draft replies based on the business's actual operational data (inventory, hours, policies) stored in the long-term vector memory.

### 3. Architecture & Implementation (Research Report)
-   **Event Mesh Integration:** The agent listens for `incoming_message` events on the KAIROS Orchestrator's Hub.
-   **Memory Access:** The agent queries `autodream_memories` via `pgvector` to understand the business context and past interactions with the specific customer.
-   **Drafting vs. Auto-Sending:**
    -   *High Confidence / Low Risk:* (e.g., "Are you open today?") The agent auto-replies immediately.
    -   *Low Confidence / High Risk:* (e.g., A custom cake quote request) The agent queues a draft in the Dashboard's "Action Required" feed.
-   **1-Tap Approval:** The user reviews the draft on their phone lock screen or dashboard and taps "Approve to Send".

### 4. Implementation Prompt
Deploy the "Ambassador" auto-replying agent framework. The KAIROS orchestrator must listen for incoming messages, query the `autodream_memories` vector database for context, and determine whether to auto-reply or queue a draft. Implement the "1-Tap Approval" UI flow in the mobile dashboard for pending high-risk drafts.
