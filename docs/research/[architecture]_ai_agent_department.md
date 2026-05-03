# Issue Brief: AI Agent Department Architecture

## Title
AI Agent Department Architecture

## Problem Statement
Small business owners lack the time and expertise to manage all aspects of their business (marketing, customer support, legal, finance). Existing solutions treat AI as a reactive tool (e.g., a chatbot) that requires explicit prompting, which adds cognitive load. OHC needs a system where AI acts as proactive "Teammates" organized into functional departments that automatically handle background tasks, reducing the owner's workload invisibly.

## Research Report
- **Competitor Gaps**: Shopify's "Sidekick" and Wix's AI are reactive assistants. They do not operate autonomously in the background across different business functions.
- **User Needs**: Users want an "Operations Manager" to track inventory and an "Ambassador" to reply to DMs, not a generic "AI assistant".
- **OHC Approach**: Implement 7 distinct AI Agent Departments (Operations, Marketing, Sales, Customer Success, Finance, Legal, Advisory) that listen to system events, share context, and execute or draft actions autonomously.

## Design Doc

### Architecture Diagram
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

### Department Execution Triggers & Coordination
Departments are autonomous but interconnected:
- **Scheduled (Cron):** E.g., The Business Advisory Agent generates weekly health reports every Monday at 8 AM.
- **Event-Driven:** Triggered by system events. E.g., Operations processes an order -> Customer Success drafts a thank-you note.
- **On-Demand:** Direct user prompts via the dashboard UI.

Coordination is handled via the KAIROS Shared Task List and Teammate Mesh, ensuring durable, collision-free handoffs between departments using distributed locks (`ohc:lock:{tenant_id}:{resource_type}:{resource_id}`).

### Memory & Context
Agents utilize a unified memory model:
- **Short-Term Context:** Current session data and active task payload (e.g., the specific order details).
- **Long-Term Memory:** Embedded into `autodream_memories` using `pgvector`. This allows agents to recall past interactions, seasonal trends, and specific customer preferences.

### Approval Workflows
To maintain trust, actions are categorized by risk:
- **Auto-Execute:** Low-risk, reversible actions (e.g., updating internal tags).
- **Draft-for-Review:** High-risk, external actions (e.g., publishing social media posts, sending customer emails). The system presents a notification requiring a 1-tap approval via the mobile app.

### Mobile-First UX
All agent interactions (approving drafts, viewing advisory reports) are designed for a 375px mobile breakpoint. Action items are summarized in plain language ("Your vegan cake campaign is ready for review").

## Implementation Prompt
Implement the Draft-for-Review workflow engine within the KAIROS orchestrator. Agents must be able to submit high-risk actions (e.g., emails, social posts) into a pending state. Create the pending approval queue in the OHC-SIP DB, scoped to `tenant_id` via RLS. Implement the approval/rejection callback endpoints. Develop the mobile Flutter UI (375px first) for the "Agent Activity Feed" allowing explicit 1-tap approval from the tenant owner. Write E2E tests verifying an agent can draft an action, it appears in the feed, and the action executes only after owner approval.

## Priority
P1

## Estimated Scope
Large
