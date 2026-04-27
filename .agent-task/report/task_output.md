# [architecture] AI Agent Department Architecture

## 1. Problem Statement
Non-technical small business owners lack the resources to handle every aspect of their operations (e.g., fulfilling orders, posting to social media, interpreting analytics). Existing AI solutions act as bolted-on chatbots rather than integrated infrastructure. OHC needs a comprehensive architectural model that categorizes AI agents into familiar departments, allowing them to autonomously manage tasks securely, reliably, and within the context of a business's historical data, without overwhelming the user.

## 2. Research Report
### Competitor Analysis
*   **Shopify:** Uses "Sidekick," which functions primarily as a conversational chatbot rather than autonomous departments. It lacks proactive event-driven workflows for marketing and legal tasks.
*   **Wix:** Features "Wix AI" for site generation and some SEO, but it is not modeled as a continuous, operational workforce handling day-to-day fulfillment.
*   **Squarespace / GoDaddy:** Offer limited, disparate AI tools (e.g., text generation) but no cohesive, orchestrator-driven agentic platform.

### Market Gap
The market lacks a platform where AI acts as invisible infrastructure organized into recognizable business functions (departments) that proactively coordinate and execute workflows, rather than simply responding to chat prompts.

## 3. Design Doc

### 3.1 Department Overview
The AI Agent Departments are:
1.  **Operations ("The Manager"):** Order and booking processing, inventory tracking, fulfillment, refunds.
2.  **Marketing & Advertising ("The Promoter"):** Website design, SEO, social media posts, promotional content, QR codes, link-in-bio pages.
3.  **Sales & Acquisition ("The Salesperson"):** Quote generation, lead follow-up, referral tracking, upsell suggestions.
4.  **Customer Success ("The Ambassador"):** Message replies, order updates, review requests, re-engagement campaigns.
5.  **Finance & Payments ("The Accountant"):** Payment processing, financial reports, subscription billing, tax summaries.
6.  **Legal & Compliance ("The Protector"):** Terms/policies, contracts, GDPR compliance, license tracking, liability disclaimers.
7.  **Business Advisory ("The Advisor"):** Weekly health reports, next-action suggestions, seasonal trends, pricing recommendations.

### 3.2 Trigger Mechanisms
Departments operate using three trigger modalities:
-   **Scheduled (Cron):** Routine tasks (e.g., Business Advisory running weekly health analysis).
-   **Event-Driven:** Reactive actions triggered by system events (e.g., New Order -> Operations processes -> Customer Success drafts confirmation).
-   **On-Demand:** Explicit actions requested by the user via the mobile UI.

### 3.3 Coordination & Concurrency
Agents coordinate via the KAIROS Orchestrator and Teammate Mesh:
-   **Message Bus/Pub-Sub:** Events are routed to relevant departments via production Redis Pub/Sub channels.
-   **Distributed Locking:** To prevent conflicting actions (e.g., Operations modifying inventory while Sales generates a quote), distributed locks (`ohc:lock:{tenant_id}:{resource_type}:{resource_id}`) via Redis Redlock must be acquired before mutating shared state.

### 3.4 Context & Memory
Agents require a unified memory model to make intelligent decisions:
-   **Short-Term Context:** Current session payload, active task details.
-   **Long-Term Memory (`autodream_memories`):** Episodic memories embedded using pgvector. This allows agents to retrieve historical context (e.g., previous customer preferences, past successful marketing campaigns) to inform their actions.

### 3.5 Security & Data Isolation
-   **Multi-Tenancy:** All agent queries and actions must be strictly scoped using `tenant_id` enforced via PostgreSQL Row Level Security (RLS).

### 3.6 Approval Workflows (Trust Model)
Actions are categorized by risk to maintain owner control:
-   **Auto-Execute:** Low-risk, reversible, or internal actions (e.g., updating inventory numbers, parsing analytics).
-   **Draft-for-Review:** High-risk, external-facing actions (e.g., sending emails, publishing social media, issuing refunds). The agent prepares the payload and places it in a pending state. A push notification is sent to the mobile app, requiring a 1-tap approval from the owner before execution.

### 3.7 Tier-Based Throttling
Usage is metered to manage infrastructure costs:
-   **Hard Limits:** Free (100 actions/mo), Starter (1,000 actions/mo), Pro (Unlimited).
-   **Enforcement:** KAIROS Orchestrator enforces rate limits and budget caps per tenant.

### 3.8 UI/UX Flow (Mobile-First)
All interactions are optimized for a 375px display:
1.  **Notification:** "Your Promoter agent drafted an Instagram post for the new vegan cakes."
2.  **Review Screen:** Displays the image, caption, and schedule in a clean, glassmorphic card.
3.  **Action:** Two prominent touch targets (>= 44x44px): [Approve & Schedule] or [Edit Draft].
4.  **Plain Language:** All reports and actions avoid technical jargon.

### 3.9 Architecture Diagram

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

## 4. Implementation Prompt
**Task for Implementer:** Implement the basic framework for the AI Agent Department architecture.
1.  **Goal:** Establish the KAIROS routing infrastructure to support the 7 departments and the Draft-for-Review workflow.
2.  **CUJ:** A simulated order event is emitted. The system should route the event to the "Operations" department, which successfully updates the internal state. It should then trigger the "Customer Success" department, which generates a communication payload flagged as "Draft-for-Review", pending user approval.
3.  **Acceptance Criteria:**
    -   Routing logic correctly dispatches events to designated department handlers based on the event type.
    -   Distributed locking is utilized during simulated state mutation.
    -   A payload categorized as high-risk correctly enters a "pending" state rather than executing immediately.
    -   Do not define specific database schemas or API endpoints; focus on the orchestrator routing, locking, and the state machine transition for pending approvals.

## 5. Priority
`P1` (High Priority - Foundational Architecture)

## 6. Estimated Scope
Large
