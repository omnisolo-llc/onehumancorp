# Issue Brief: Omnichannel AI Inbox (The Ambassador)

## 1. Context & Problem
**Pain Point Addressed:** #1 Communication Overload.
Small business owners (Maya the Baker, Carlos the Handyman) are suffering from severe operational fatigue. They spend hours manually responding to repetitive DMs (Instagram/WhatsApp) and emails ("Do you have vegan options?", "When are you open?"). This manual work takes away from their craft and leads to lost revenue when messages are missed while they are working or sleeping.

## 2. Objective
Build an inbox that unifies DMs, SMS, and Emails into a single thread view, and uses "The Ambassador" AI agent to automatically draft replies based on the business's embedded memory (`autodream_memories`).

## 3. Scope & Requirements

### 3.1. Unified Thread Model
*   **Aggregation:** Normalize incoming messages from at least two distinct sources (e.g., Instagram DMs, SMS, Email) into a single "Customer Profile" and "Thread."
*   **Mobile-First UI:** The interface must be optimized for a 375px viewport. No horizontal scrolling. The design should utilize translucent glass aesthetics.

### 3.2. The Ambassador Agent
*   **Invisible Autonomy:** The AI acts as a middleware interceptor. It analyzes incoming message sentiment and intent without manual invocation.
*   **Contextual Drafting:** The agent queries the business's knowledge base (menu, calendar, pricing, `autodream_memories`) to draft accurate replies.
*   **Human-in-the-Loop:** Generates push notifications for new drafts with a 1-click "Approve" button or allows the user to edit the text directly before sending.
*   **Escalation:** Identifies complex requests that require human input (e.g., highly custom orders) and flags them for the owner.

### 3.3. Multi-Tenant Safety & Architecture
*   **Zero-Trust Routing:** The `INBOX_ROUTER` must strictly enforce multi-tenant isolation using SPIFFE identities.
*   **Data Boundary:** A tenant must only ever be able to read threads associated with their `organization_id`. Cross-tenant leakage is a catastrophic failure mode.

## 4. Expected Impact
*   **Time Saved:** Reduce manual communication time by 80%.
*   **Increased Conversion:** Capture leads immediately, 24/7.
*   **Reduced Friction:** Provide a unified view, eliminating the need to toggle between multiple apps.

## 5. Implementation Notes
*   Hook into the background AI Orchestration engine to trigger response drafts on new message events.
*   Hide complex configuration (webhooks, API keys) behind an "Advanced Settings" toggle.
*   Ensure the sent messages are clearly distinguished between human-sent and AI-generated (e.g., marked with "✨ Ambassador").
