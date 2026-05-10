# OHC AI Agent Department Architecture

## 1. Overview
This design document defines how AI departments operate invisibly within the OHC platform. OHC's agents are organized into friendly, understandable functional areas that mirror how a real business operates (Operations, Marketing & Advertising, Sales & Acquisition, Customer Success, Finance & Payments, Legal & Compliance, and Business Advisory). These agents seamlessly integrate into the daily workflow of non-technical small business owners, offloading cognitive overhead and driving growth.

## 2. Goals & Non-Goals
### 2.1 Goals
- Define clear functional boundaries for each of the 7 AI Agent Departments.
- Specify how each department is triggered and how they coordinate via the KAIROS Orchestrator.
- Define memory retention and access patterns for contextual decision-making.
- Outline the approval mechanism ensuring appropriate oversight (auto-execute vs. draft-for-review).
- Establish usage limits and budgeting based on tenant tiers.

### 2.2 Non-Goals
- Prescribe specific LLM inference engines or prompt tuning methodologies.
- Define explicit SQL DDL schemas for the database.
- Specify exact queueing mechanisms or worker node provisioning.

## 3. Detailed Design

### 3.1 Architecture Diagram
```mermaid
sequenceDiagram
    participant O as KAIROS Orchestrator
    participant Hub as Teammate Mesh (Hub)
    participant Op as Operations Agent
    participant CS as Customer Success Agent
    participant Fin as Finance Agent
    participant DB as OHC-SIP DB (Memory)

    O->>Hub: New Order Event
    Hub->>Op: Trigger: Process Order
    Op->>DB: Fetch Inventory State
    DB-->>Op: Inventory Valid
    Op->>Hub: Order Processed
    Hub->>Fin: Trigger: Track Payment
    Fin->>DB: Record Deposit
    Hub->>CS: Trigger: Send Confirmation
    CS->>DB: Fetch Customer Profile
    DB-->>CS: Profile (Preferences)
    CS->>Hub: Draft Email for Review

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class O,Hub,Op,CS,Fin,DB premium;
```

### 3.2 Department Execution Triggers & Coordination
Departments are autonomous but interconnected:
- **Scheduled (Cron):** E.g., The Business Advisory Agent generates weekly health reports every Monday at 8 AM.
- **Event-Driven:** Triggered by system events. E.g., Operations processes an order -> Customer Success drafts a thank-you note.
- **On-Demand:** Direct user prompts via the dashboard UI.

Coordination is handled via the KAIROS Shared Task List and Teammate Mesh, ensuring durable, collision-free handoffs between departments using distributed locks (`ohc:lock:{tenant_id}:{resource_type}:{resource_id}`).

#### Critical 1-Tap Handoff Triggers
To ensure the "1-Tap Approval" experience, the following coordination patterns are strictly enforced:
1.  **Ops -> Success (The Fulfillment Flow)**: When "The Manager" marks an order as `SHIPPED` or `READY_FOR_PICKUP`, a `tenant.order.fulfillment_ready` event is emitted. "The Ambassador" immediately drafts a personalized notification for the customer.
2.  **Sales -> Ops (The Quote flow)**: When a customer accepts a quote, "The Salesperson" emits `tenant.quote.accepted`. "The Manager" automatically creates an `ORDER` and a `BOOKING` in the shared task list, pending the owner's confirmation.
3.  **Advisor -> Promoter (The Growth Flow)**: When "The Advisor" identifies a high-velocity product (e.g., Maya's vegan cake), it emits `tenant.insight.trending`. "The Promoter" then drafts a social media campaign or a website banner to capitalize on the trend.

### 3.3 Memory & Context
Agents utilize a unified memory model:
- **Short-Term Context:** Current session data and active task payload (e.g., the specific order details).
- **Long-Term Memory:** Embedded into `autodream_memories` using `pgvector`. This allows agents to recall past interactions, seasonal trends, and specific customer preferences (e.g., "Customer X always asks for vegan options").

### 3.4 Approval Workflows
To maintain trust, actions are categorized by risk:
- **Auto-Execute:** Low-risk, reversible actions (e.g., updating internal inventory tags, parsing analytics).
- **Draft-for-Review:** High-risk, external actions (e.g., publishing social media posts, sending customer emails, refunding payments). The system presents a notification to the business owner, requiring a 1-tap approval via the mobile app.

### 3.5 Tier-Based Usage & Throttling
Agent activity is gated by the multi-tenant SaaS tier:
- Usage is metered per tenant using custom Prometheus metrics.
- Hard limits on monthly AI actions (e.g., Free: 100, Starter: 1,000, Pro: Unlimited).
- Rate limiting applied at the Orchestrator level to prevent noisy-neighbor degradation.

## 4. Cross-cutting Concerns
### 4.1 Mobile-First UX
All agent interactions (approving drafts, viewing advisory reports) are designed for a 375px mobile breakpoint. Action items are summarized in plain language ("Your vegan cake campaign is ready for review").

### 4.2 Security & Multi-Tenancy
Every agent query and action is scoped to the `tenant_id` via PostgreSQL Row Level Security (RLS) to guarantee complete isolation.

## 5. Implementation Plan
- **Phase 1:** Core KAIROS event routing for the Operations and Customer Success departments.
- **Phase 2:** Memory integration (`autodream_memories`) for contextual responses.
- **Phase 3:** Draft-for-review approval UX implementation in the mobile application.

---

## Issue Brief: Implement AI Agent Approval Workflow Engine

**Title**: Implement AI Agent Approval Workflow Engine
**Problem Statement**: The Orchestrator lacks a mechanism for "Draft-for-Review" workflows. Business owners like Maya need absolute autonomy and trust that high-risk actions (like sending an email or publishing a social media post) won't happen without their explicit consent. Without this, agents cannot safely perform external tasks.
**Research Report**:
- OHC's mandate demands Absolute Autonomy, but trust must be earned.
- Competitors force either all-manual or all-automatic execution. OHC's "1-Tap Approval" is a key differentiator.
- Actions must be categorized by risk level (Auto-Execute vs. Draft-for-Review).
**Design Doc**:
- The KAIROS Orchestrator requires an `ActionRisk` evaluation step.
- High-risk actions are persisted in a pending state in the OHC-SIP DB.
- A notification event is pushed to the mobile dashboard (375px optimized).
- The dashboard presents the drafted action clearly in plain language with a 1-tap "Approve" or "Reject" interaction.
- See architecture diagram in section 3.1 for the broader context.
**Implementation Prompt**:
To Implementer Agent: Implement the Draft-for-Review workflow engine within the KAIROS orchestrator. Define `ActionRisk` levels in the agent mission payload. Create a pending approval queue mechanism in the backend. Implement the approval/rejection callback endpoints that transition the task from pending to executed. Ensure the UI for the 1-tap approval adheres to the Visual Excellence Mandate (Glassmorphism, mobile-first). Do NOT prescribe specific SQL schema changes, focus on the API contracts and workflow orchestration.
**Priority**: P1
**Estimated Scope**: Medium