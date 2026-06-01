# Issue Brief: Auto-Replying AI Agents

## Title
[AI Differentiation] Auto-Replying Agents

## Problem Statement
Manual Repetition & Lost Sales: Small business owners (like Maya the Baker) spend hours answering repetitive questions ("Do you do vegan cakes?", "What are your hours?") across various social media platforms (Instagram DMs, WhatsApp). This constant context switching leads to delayed responses, missed sales opportunities, and severe operational burnout.

## Research Report
- **Competitive Landscape:** Shopify offers a generic "Inbox" with manual FAQ setup. Wix has basic chat. None provide deep context-aware, autonomous response capabilities out of the box that require zero configuration.
- **User Pain Points:**
  - Answering the same question 15 times a day.
  - Waking up to 10 missed messages from interested customers in different time zones.
  - The inability to afford dedicated customer support staff.
- **AI Differentiation:** The "Customer Success Agent" (The Ambassador) operates invisibly in the background. It reads the business's existing data (store policies, product catalog, calendar) and drafts or auto-sends accurate replies without the owner needing to configure complex rule sets or decision trees.

## Design Doc
### High-Level Architecture
- **Trigger:** An incoming message via a connected channel (Instagram DM, WhatsApp, or Web Chat) is received by the backend webhook.
- **Agent Action (Customer Success Department):**
  - **Ingestion & Context Retrieval:** The incoming message is embedded and used to query the tenant's context database (pgvector). This retrieves relevant products, policies, and past conversation history for that specific customer.
  - **Draft Generation:** The LLM (Gemini Pro/GPT-4o) generates a contextual reply based on the system prompt and retrieved context.
  - **Action Decision:** Based on a confidence score and user settings, the agent either:
    1.  **Drafts:** Places the generated response in the owner's inbox for manual approval/editing.
    2.  **Auto-Replies:** Sends the response directly to the customer.
- **Data Architecture:**
  - Implement a `message_threads` table to store conversation history.
  - Ensure robust, row-level tenant isolation for all message data.
  - Store vector embeddings of store data in a dedicated schema for fast retrieval.

### Mobile UX Flow
- **Inbox View:** Display incoming messages. Messages with an AI-generated draft are clearly marked (e.g., "✨ AI Suggested Reply").
- **Draft Interaction:** User taps a message thread. The AI draft is pre-populated in the text input field. The user can hit send immediately, edit the text, or discard the draft and write their own.
- **Settings:** A simple toggle switch in the department settings: "Auto-reply to common questions when I'm away" (On/Off).

## Implementation Prompt
Deploy invisible AI agents capable of answering customer inquiries on social channels using the merchant's real-time data. Implement the backend webhook handlers for incoming messages, the context retrieval logic using pgvector, and the LLM generation step. On the frontend, build the UI to display these messages and surface the AI-generated drafts for 1-tap approval or editing, seamlessly integrating into a unified inbox experience.

## Priority
P0

## Estimated Scope
Large
