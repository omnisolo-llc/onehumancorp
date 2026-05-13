**Title**: Proactive AI Omnichannel Assistant

**Problem Statement**:
From the perspective of Carlos (the handyman) or Maya (the baker), keeping up with customer messages across Instagram, WhatsApp, email, and SMS is impossible. They lose 30% of potential sales simply because they are busy working and cannot reply within the "golden 5 minutes" that a customer expects. Existing solutions (like Shopify Inbox or ManyChat) require complex rule-building, logic trees, or a desktop interface to manage. Small business owners don't want to build a chatbot; they want an assistant that just knows what to say and handles the inbox for them.

**Research Report**:
Based on our Top 10 SMB Pain Points analysis, "Communication Lag" (losing sales due to slow responses) is a top 8 pain point, and "Operational Fatigue" (the never-ending inbox) is the #2 pain point. Competitors like Shopify offer "Sidekick," but it is a reactive tool that requires prompting. Platforms like Durable and 10Web focus heavily on website generation but offer very thin post-launch operational support. OHC's opportunity is to provide an "Autonomous Teammate."

### Market Feature Gap
| Feature | **Shopify** | **Wix** | **ManyChat** | **OHC (Target)** |
| :--- | :--- | :--- | :--- | :--- |
| **Omnichannel Inbox** | Yes | Yes | Social Only | **Yes (All channels)** |
| **Auto-Drafting** | No | No | Template-based | **Context-Aware AI** |
| **Approval Flow** | N/A | N/A | Immediate | **1-Tap Lock Screen** |
| **Business Memory** | Low | Low | Low | **High (RAG-based)** |

**Design Doc**:
*   **High-Level Architecture**:
    *   **Omnichannel Gateway**: Ingests messages from Meta Graph API (IG/WhatsApp), Twilio (SMS), and email.
    *   **Context Engine**: A RAG (Retrieval-Augmented Generation) layer that pulls from the `business_memory` store (inventory levels, business hours, FAQs, pricing).
    *   **Agent Worker**: Processes the incoming message and the retrieved context to draft a response.
    *   **Notification Service**: Pushes the drafted response to the mobile client via push notification or WebSocket.
*   **UI/UX Flow (Mobile First - 375px)**:
    1.  User receives a push notification: "New IG DM from Sarah: 'Are you open tomorrow?'"
    2.  User taps notification, opening the OHC app to the "Action Feed".
    3.  The screen shows Sarah's message and an AI-drafted reply: "Hi Sarah! Yes, we are open tomorrow from 8 AM to 5 PM. Can I help you book a slot?"
    4.  The user sees three large buttons: [Approve & Send] [Edit] [Reject].
    5.  User taps [Approve & Send]. The message is dispatched.
*   **Mermaid Diagram**:
    ```mermaid
    graph TD
        A[Customer Message (IG/SMS)] --> B[Omnichannel Gateway]
        B --> C[Context Engine (RAG)]
        C --> D[Business Memory (Postgres/Vector)]
        D --> C
        C --> E[AI Draft Agent]
        E --> F[Mobile Action Feed]
        F --> G{User Action}
        G -->|1-Tap Approve| H[Send Reply to Customer]
        G -->|Edit| I[Update Draft & Send]
    ```

**Implementation Prompt**:
Build a mobile-first UI for an "Action Feed" that displays incoming customer messages alongside context-aware, AI-drafted responses. The interface must allow the business owner to review the drafted reply and send it with a single tap, or easily edit it before sending. The backend system should ingest messages, use the business's stored knowledge to draft accurate replies, and push these drafts to the user's feed. The core Critical User Journey (CUJ) is: A customer asks a question -> the AI drafts a perfect reply -> the owner approves it from their lock screen or feed in < 3 seconds -> the customer receives the response. Acceptance criteria include a 375px optimized layout, < 300ms entrance animations, and seamless 1-tap approval functionality.

**Priority**: P0

**Estimated Scope**: Large
