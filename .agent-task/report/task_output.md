# 🏢 OneHumanCorp - AI Agent Department Architecture

## Problem Statement
Small business owners (SBOs) lack the time, expertise, and resources to manage the multifaceted operations of a modern business—from handling customer inquiries to managing finances, marketing, and legal compliance. Existing tools require manual intervention, context switching, and specialized knowledge. The OHC platform must orchestrate these complex, disparate tasks automatically and invisibly using AI agents, organized intuitively like a real business. We need an architectural foundation for how these "departments" operate, communicate, and scale without exposing complexity or breaking constraints (budgeting, compliance).

## Research Report

### Current State & Pain Points
Based on the provided personas (Maya, Carlos, Priya, Leo, Fatima) and platform goals:
*   **Context Fragmentation:** A business context (inventory, orders, customers) needs to be consistently shared across different operational areas.
*   **Asynchronous Complexity:** Actions happen at different times (e.g., an Instagram DM at 2 AM needs a response; a booked lesson needs a follow-up a week later).
*   **Trust and Verification:** AI actions must be trustworthy. High-stakes actions (refunds, large quotes) may require approval ("draft-for-review"), while low-stakes actions (answering FAQs) should be fully automated ("auto-execute").
*   **Resource Management:** AI execution is expensive. Usage must be tracked and limited per tenant based on their tier (Free, Starter, Pro, Business).

### Competitive Analysis
*   **Shopify:** Offers basic automation (Shopify Flow) and AI text generation (Shopify Magic), but fundamentally relies on the merchant to be the "manager." Agents are tools, not autonomous actors.
*   **Wix/Squarespace:** Provide AI site builders and basic CRM features, but lack continuous, background AI operational management across different domains (marketing, finance, legal).
*   **OHC Differentiation:** OHC's core value proposition is the *delegation* of management. The platform *is* the team. The AI departments must mirror real-world business structures (Operations, Marketing, Sales, Customer Success, Finance, Legal, Advisory).

## Design Doc: AI Agent Department Architecture

### Architectural Principles
1.  **Departmental Specialization:** Agents are specialized to specific domains (e.g., The Accountant, The Promoter). They have distinct scopes, toolsets, and guardrails.
2.  **Event-Driven Orchestration:** Departments are activated by events (external triggers like a webhook, internal schedules, or cross-department handoffs).
3.  **Shared Central Memory (Context):** All departments read from and write to a centralized Tenant Knowledge Base (TKB) to maintain a unified understanding of the business state and customer history.
4.  **Tier-Based Budget Enforcement:** Every agent action consumes a budget (tokens/compute). Execution is gated by the tenant's tier limits.
5.  **Approval Workflows (The "Draft vs. Do" Spectrum):** Actions are categorized by risk. Low-risk actions auto-execute. High-risk actions create "Drafts" requiring owner approval via mobile push notification.

### Architecture Diagram

```mermaid
graph TD
    %% External Inputs
    User[Business Owner Mobile App] --> |Approvals / Commands| API[OHC Gateway API]
    ExternalEvents[External Events: Webhooks, Social DMs, Emails, Scheduled Tasks] --> EventBus(Enterprise Event Bus)
    API --> EventBus

    %% Core Orchestrator
    EventBus --> Orchestrator[The Orchestrator / Dispatcher]

    %% Departments
    subgraph AI Departments [AI Agent Departments]
        Ops[The Manager<br>Operations]
        Mktg[The Promoter<br>Marketing]
        Sales[The Salesperson<br>Sales]
        CS[The Ambassador<br>Customer Success]
        Fin[The Accountant<br>Finance]
        Legal[The Protector<br>Legal & Compliance]
        Adv[The Advisor<br>Business Advisory]
    end

    %% Routing
    Orchestrator -->|Dispatch Event| Ops
    Orchestrator -->|Dispatch Event| Mktg
    Orchestrator -->|Dispatch Event| Sales
    Orchestrator -->|Dispatch Event| CS
    Orchestrator -->|Dispatch Event| Fin
    Orchestrator -->|Dispatch Event| Legal
    Orchestrator -->|Dispatch Event| Adv

    %% Shared Context & State
    subgraph Data Layer [Tenant Data & Memory Layer]
        TKB[(Tenant Knowledge Base<br>Vector + Relational)]
        Budget[(Budget & Limits<br>Tier Quotas)]
        ActionQueue[(Action & Draft Queue)]
    end

    %% Connections
    AI Departments <--> |Read/Write Context| TKB
    AI Departments --> |Check Quota| Budget
    AI Departments --> |Propose/Execute Action| ActionQueue

    ActionQueue --> |Push Notification for Approval| User
    ActionQueue --> |Execute Approved Action| ExternalSystems[External Systems: Stripe, Postmark, Instagram API]
```

### Department Definitions & Interactions

| Department | Triggers | Key Responsibilities | Example Output (Action) | Tool Access |
| :--- | :--- | :--- | :--- | :--- |
| **The Manager (Ops)** | New Order, Inventory Change, Schedule Event | Order fulfillment routing, inventory sync across channels, booking conflict resolution. | Update stock level; Dispatch prep notification to owner. | DB Write, Notification API |
| **The Promoter (Mktg)** | Schedule (Weekly), New Product Added | SEO updates, social media draft generation, email newsletter creation. | Draft an Instagram post about a new vegan cake. | Vector DB (Assets), Email API |
| **The Salesperson (Sales)** | Abandoned Cart, Request for Quote | Lead scoring, custom quote generation, up-sell recommendations. | Send a custom quote for a kitchen remodel via SMS. | Pricing Engine, SMS API |
| **The Ambassador (CS)** | Incoming DM, Support Ticket | Answering FAQs, handling refund requests, collecting reviews. | Reply to DM: "Yes, we offer gluten-free options!" | DM API, Ticketing System |
| **The Accountant (Fin)** | Payment Received, End of Month | Reconciling payments, generating tax summaries, handling failed subscriptions. | Generate monthly P&L PDF; Retry failed card. | Stripe API, Reporting Engine |
| **The Protector (Legal)** | New Booking, Product Creation | Ensuring terms are attached, managing GDPR consent, flag liability risks. | Append liability waiver to a kayak rental booking. | Policy Engine, Document Gen |
| **The Advisor (Advisory)** | Schedule (Weekly), Anomaly Detection | Health check reports, growth suggestions. | Push Notification: "You're selling out of X quickly. Raise price?" | Analytics Engine |

### Mobile UX Flows

#### 1. The "Action Required" Approval Flow (Draft-for-Review)
*   **Trigger:** "The Ambassador" receives a complex refund request via email.
*   **Evaluation:** Agent determines risk > threshold. Creates a `DraftAction`.
*   **Mobile Push:** Owner receives notification: *"Review requested: Refund $150 to John Doe? [Approve] [Edit] [Deny]"*
*   **Screen:** Details of the request, agent's rationale ("John is a top 5% customer"), and suggested reply.
*   **Action:** Owner taps "Approve". Action moves from Queue -> Execution.

#### 2. The "Daily Briefing" Flow (Advisory)
*   **Trigger:** Morning schedule. "The Advisor" synthesizes data from Ops, Sales, and Fin.
*   **Mobile View:** App opens to a personalized "Good Morning Maya" dashboard.
*   **Content:**
    *   "You have 3 cake orders due today." (Ops)
    *   "Your new Instagram post drove $200 in sales yesterday." (Mktg/Sales)
    *   "Suggestion: Your booking calendar is empty next Tuesday. Run a 10% promo?" (Advisor)

### AI Integration Points & Context Strategy
*   **Event Handling:** When a webhook arrives (e.g., Stripe Payment Intent), the API Gateway publishes an event to the Event Bus. The Orchestrator determines which department needs to handle it (e.g., The Accountant).
*   **Memory Retrieval:** Before generating a response, an agent queries the Tenant Knowledge Base (TKB). The TKB combines relational data (Orders, Products) with Vector data (past conversations, brand voice documents).
*   **Budgeting:** Every LLM call goes through a central `BudgetTracker`. If a "Free" tier user exhausts their 100 actions/month, the Orchestrator pauses non-critical background tasks (Mktg/Advisory) and queues them, notifying the user to upgrade. Core operations (Ops/CS) might run in a degraded, template-based mode to avoid breaking the business.
*   **Handoffs:** An agent can yield to another. E.g., The Ambassador receives a DM about a complex custom order -> Creates an internal event -> Handoff to The Salesperson to generate the quote.

### Key Design Decisions
*   **Decision:** Asynchronous, Event-Driven Orchestration.
    *   *Why:* External APIs (Instagram, Stripe) and LLM inference have high latency. Synchronous execution would block the platform and break mobile UX.
*   **Decision:** Strict "Draft vs. Execute" boundary based on action type.
    *   *Why:* To build trust. SBOs will not adopt an AI that might accidentally refund $1000 without permission. Money-movement always requires approval initially.
*   **Decision:** Unified Tenant Knowledge Base.
    *   *Why:* To prevent agents from contradicting each other. "The Promoter" shouldn't advertise a product that "The Manager" knows is out of stock.

## Implementation Prompt
**Task:** Implement the foundational Agent Orchestrator and Department Routing logic for the OHC platform.
**User-Facing Outcome:** The platform can receive simulated external events (e.g., "New Customer Message", "Payment Failed") and route them to the correct AI "Department" (e.g., CS, Finance) for processing. High-risk actions must generate an approval request (draft) rather than executing immediately.
**Acceptance Criteria:**
1.  Define the core interfaces/traits for an `AgentDepartment` and the `Orchestrator`.
2.  Implement at least two sample departments (e.g., `CustomerSuccessDepartment`, `FinanceDepartment`) that can receive an event and produce an `AgentAction` (either `Execute` or `DraftForApproval`).
3.  Implement a central event routing mechanism that inspects incoming events and dispatches them to the appropriate department.
4.  Implement a mock `TenantKnowledgeBase` that departments can query for context.
5.  Implement a mock `BudgetTracker` that denies execution if the tenant's tier quota is exceeded.
6.  Ensure the system logs transitions clearly (e.g., "Event Received -> Routed to Finance -> Action Drafted -> Awaiting Approval").

**Priority:** P0 (Critical Path)
**Estimated Scope:** Large
