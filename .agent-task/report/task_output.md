# OHC AI Agent Department Architecture

## Title
Architectural Design for OHC AI Agent Departments

## Problem Statement
The OHC platform aims to allow non-technical small business owners (e.g., Maya, Carlos) to operate their businesses with the help of AI agents. Currently, there is a lack of a clear architectural mapping of how these AI agents are categorized, triggered, and coordinated within the platform. The system needs a defined structure for "AI Departments" that mirror real-world business operations, ensuring they can seamlessly integrate into user workflows, respect multi-tenancy rules, and provide a 1-Tap Approval experience without overwhelming the user.

## Research Report
### Context and Departments
OHC's AI agents must be organized into functional departments that are easily understood by non-technical users. The target departments are:
1. **Operations ("The Manager")**: Handles order and booking processing, inventory tracking, fulfillment, and refunds.
2. **Marketing & Advertising ("The Promoter")**: Manages website design, SEO, social media posts, promotional content, QR codes, and link-in-bio pages.
3. **Sales & Acquisition ("The Salesperson")**: Generates quotes, follows up on leads, tracks referrals, and suggests upsells.
4. **Customer Success ("The Ambassador")**: Replies to messages, updates orders, requests reviews, and runs re-engagement campaigns.
5. **Finance & Payments ("The Accountant")**: Processes payments, generates financial reports, handles subscription billing, and creates tax summaries.
6. **Legal & Compliance ("The Protector")**: Manages terms/policies, contracts, GDPR compliance, license tracking, and liability disclaimers.
7. **Business Advisory ("The Advisor")**: Provides weekly health reports, next-action suggestions, seasonal trends, and pricing recommendations.

### Execution and Coordination
- **Triggers**: Departments can be triggered on a schedule (Cron), via events (Event-Driven), or on demand (Direct User Prompts).
- **Coordination**: Handled via the KAIROS Shared Task List and Teammate Mesh, using distributed locks (`ohc:lock:{tenant_id}:{resource_type}:{resource_id}`) for durable, collision-free handoffs.
- **Memory Model**: Utilizes a unified memory model with short-term context (session data) and long-term memory embedded into `autodream_memories` using `pgvector`.
- **Approval Workflows**: High-risk actions (e.g., sending emails) are put in a "Draft-for-Review" state, requiring a 1-tap approval from the user. Low-risk actions (e.g., updating tags) are auto-executed.
- **Tier Limits**: Agent activity is gated by the SaaS tier, with hard limits on monthly actions and rate limiting to prevent noisy-neighbor degradation.

## Design Doc

### Architecture Diagram (Mermaid.js)
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

### Key Design Decisions
- **1-Tap Handoff Triggers**: Strict coordination patterns are enforced for critical flows (e.g., Ops -> Success for fulfillment, Sales -> Ops for quotes, Advisor -> Promoter for growth).
- **Mobile-First UX**: All agent interactions (approving drafts, viewing advisory reports) are designed for a 375px mobile breakpoint with plain language summaries.
- **Security & Multi-Tenancy**: Every agent query and action must be scoped to the `tenant_id` via PostgreSQL Row Level Security (RLS) to ensure complete isolation.

## Implementation Prompt
**To Implementer Agent:**
Implement the Draft-for-Review workflow engine within the KAIROS orchestrator to support the AI Agent Departments. Agents must be able to submit high-risk actions (e.g., emails, social posts) into a pending state. Create the pending approval queue in the OHC-SIP DB, ensuring all tables have a `tenant_id` column and the corresponding PostgreSQL RLS policies. Implement the approval/rejection callback endpoints. Build the mobile-first (375px) UI to present these pending actions for a 1-tap approval from the tenant owner. Ensure that the interactions use OHC premium design tokens (Glassmorphism, correct typography) and handle state changes resiliently. Do not prescribe specific LLM inference engines or prompt tuning methodologies; focus on the unified API contract, workflow state machine, and the user-facing approval journey.

## Priority
P1

## Estimated Scope
Large
