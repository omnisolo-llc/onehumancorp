# Title
OHC AI Agent Department Architecture

# Problem Statement
Small business owners need an intuitive way to manage complex business operations without cognitive overhead. The AI Agent Department Architecture addresses this by organizing AI agents into 7 friendly functional areas (Operations, Marketing & Advertising, Sales & Acquisition, Customer Success, Finance & Payments, Legal & Compliance, and Business Advisory) that mirror real business departments, integrating seamlessly into daily workflows.

# Research Report
- **Functional Areas**: The platform organizes agents into Operations, Marketing & Advertising, Sales & Acquisition, Customer Success, Finance & Payments, Legal & Compliance, and Business Advisory departments.
- **Execution Triggers**: Departments are triggered via Scheduled (Cron), Event-Driven (system events), or On-Demand (direct user prompts).
- **Coordination**: Handled via the KAIROS Orchestrator and Teammate Mesh, enabling durable handoffs between departments.
- **Memory**: Agents utilize a unified memory model. Short-term context includes session data and task payloads. Long-term memory allows agents to recall past interactions.
- **Tier-Based Usage**: AI activity is gated by multi-tenant SaaS tiers. Usage is metered per tenant.
- **Cross-cutting Concerns**: All agent interactions are designed for a mobile-first UI with plain language summaries. Every agent query and action is scoped to guarantee complete isolation.

# Design Doc
## Architecture Diagram
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

## Key Design Decisions
- **Approval Workflows**: Actions are categorized by risk: Auto-Execute for low-risk actions, and Draft-for-Review for high-risk external actions requiring a 1-tap approval via the mobile app.

## UI Wireframes and Screen Flow (375px)
- **Home/Dashboard**: Displays a list of pending tasks requiring owner's attention, such as draft emails or social media posts, optimized for 375px breakpoint using OHC premium design tokens (Glassmorphism, Inter/Outfit).
- **Draft Review Screen**: Presents the draft content (e.g. email) with large, thumb-friendly "Approve" (green) and "Edit/Reject" (grey) buttons.

## Mobile UX Flow
1. **Push Notification**: The owner receives a native push notification (e.g., "The Ambassador drafted an email to Carlos").
2. **Action Tap**: Tapping the notification opens the app directly to the **Draft Review Screen**.
3. **1-Tap Approval**: The user reads the plain-language summary and taps "Approve."
4. **Optimistic Feedback**: The UI instantly shows a success state (confetti or checkmark), returning the user to their dashboard while the Orchestrator executes the task in the background.

# Implementation Prompt
**To Implementer Agent:**
Implement the Draft-for-Review workflow engine within the KAIROS orchestrator. Agents must be able to submit high-risk actions into a pending state, requiring explicit 1-tap approval from the tenant owner via the mobile dashboard before execution.

# Priority
P1

# Estimated Scope
Large
