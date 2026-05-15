# 🔎 Scout: Tool Integration Research [quarter]

## Title
AI Agent Department Architecture and System Integration

## Problem Statement
For a non-technical small business owner (like Maya the baker, or Carlos the handyman), running a business means juggling operations, marketing, sales, customer support, and finances all at once. Without a large team, these tasks are overwhelming, complex, and prone to error, limiting their growth. They need an intelligent, automated system that invisibly handles this complexity, operating like a virtual team of specialized departments (e.g., The Manager for operations, The Promoter for marketing). Currently, these capabilities are disjointed or manual, leading to burnout and lost revenue. We need a unified architecture where AI agents operate seamlessly across these departments, coordinating automatically and securely on the business owner's behalf.

## Research Report
Small business owners frequently report administrative overhead as their primary growth bottleneck. Competing platforms (Shopify, Wix, Squarespace) offer rudimentary automations or bolt-on AI tools, but these still require configuration, prompt engineering, or manual triggering. One Human Corp (OHC) aims to leapfrog this by providing fully autonomous, department-scoped AI agents.

**Key Findings:**
1.  **Context is King:** Agents must share context. If the "Operations" agent processes a refund, the "Customer Success" agent must immediately know this when replying to a frustrated customer.
2.  **Safety and Trust:** Small business owners are understandably wary of AI sending messages or moving money without oversight. A robust approval/drafting system is necessary, at least initially.
3.  **Cost and Throttling:** Multi-tenant LLM usage can quickly spiral. Usage must be strictly budgeted, tracked, and throttled per tenant.
4.  **Event-Driven:** Real business happens on events (new order, payment failed, message received), not just on schedule or on demand.

**Competitive Analysis:**
*   **Shopify Sidekick:** Helpful for answering questions *about* Shopify, but less capable of autonomous cross-departmental execution.
*   **Wix/Squarespace:** Focused on AI website generation, lacking deep operational automation.
*   **OHC Unfair Advantage:** By integrating the AI directly into the core event bus and data model, OHC's agents act as actual employees rather than external tools.

## Design Doc

The system will use a distributed, event-driven architecture, enabling specialized AI agents (Departments) to react to business events, coordinate with each other, and execute actions within strict tenant and budget constraints.

### Architecture Diagram

```mermaid
graph TD;
    EventBus[OHC Event Bus] --> Router[Agent Router & Context Hydrator];
    Router --> Ops[Operations: "The Manager"];
    Router --> Mkt[Marketing: "The Promoter"];
    Router --> Sales[Sales: "The Salesperson"];
    Router --> Support[Customer Success: "The Ambassador"];
    Router --> Finance[Finance: "The Accountant"];

    Ops --> DB[(Tenant DB Context)];
    Support --> DB;

    Ops --> Actions[Action Execution Service];
    Support --> Actions;

    Actions --> ApprovalGate{Needs Approval?};
    ApprovalGate -- Yes --> Draft[(Draft Action Store)];
    ApprovalGate -- No --> Exec[Execute Action];

    Exec --> EventBus;
    Draft -. Owner Approves .-> Exec;
```

### Mobile UX Flow (375px)

1.  **Dashboard:** The user opens the app and sees a unified "Activity Feed" showing agent actions across all departments (e.g., "The Ambassador drafted a reply to Maya", "The Accountant synced the daily summary").
2.  **Approval Required:** High-risk actions (e.g., sending a mass email, issuing a refund) appear as cards with prominent "Approve" / "Edit" / "Reject" buttons.
3.  **Department Settings:** A simple toggle screen (Glassmorphism design, Outfit font) allows turning departments on/off. No complex prompt settings—just business goals (e.g., "Promoter: Focus on Instagram growth").

### AI Agent Integration Points

*   **Trigger Mechanisms:**
    *   **Scheduled:** Daily digests, weekly health reports.
    *   **Event-Driven:** Webhooks from payment gateways, new database rows (e.g., `new_message_received`).
    *   **On-Demand:** The user clicks "Generate Quote" or asks a question.
*   **Memory/Context:** Agents retrieve episodic memory from the KAIROS AutoDream pipeline (vector db) and current state from the Postgres/SQLite `TenantDB`.
*   **Coordination:** Agents communicate via the Event Bus. "Ops" emits an `order_fulfilled` event, which "Support" listens for to trigger a confirmation message.
*   **Approval & Budgeting:** All actions pass through an interceptor layer that checks the tenant's AI action budget and enforces required approvals (draft vs. auto-execute).

### Key Design Decisions
*   **Event-Driven Coupling:** We chose an event bus over direct agent-to-agent RPC calls to ensure loose coupling and allow adding new departments without rewriting existing ones.
*   **Draft-First Execution:** For high-risk actions, the system generates "Draft Actions" that require explicit human approval. This builds trust.
*   **Centralized Budgeting:** Throttling is handled before the agent is invoked to prevent runaway LLM costs.

## Implementation Prompt
**Objective:** Implement the core infrastructure for the AI Agent Departments (Operations, Customer Success, etc.) using an event-driven architecture.

**User Journey (CUJ):**
A customer sends an Instagram DM asking about a cake order. The OHC Event Bus receives the webhook. The Router hydrates the context (customer history, order status) and routes it to "The Ambassador" (Customer Success agent). The agent drafts a polite reply. Since the tenant requires approval for outgoing messages, the system saves the reply as a Draft Action and sends a push notification to the business owner. The owner taps "Approve" on their phone, and the message is sent.

**Acceptance Criteria:**
1.  Establish an event bus or messaging layer capable of routing events to specialized agent handlers.
2.  Implement at least two sample departments (e.g., Operations, Support) that can listen to and react to events.
3.  Implement an interception/approval layer that can halt an action, save it as a draft, and expose it for user approval.
4.  Implement basic tenant-level throttling/budget checking before processing events.
5.  All components must operate within the strict multi-tenancy constraints (tenant-isolated data access).

## Priority
P0

## Estimated Scope
Large
