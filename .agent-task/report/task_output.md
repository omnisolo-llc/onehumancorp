# Architecture Brief: AI Agent Department Architecture

## Title
OHC AI Agent Department Architecture: Invisible Complexity for Small Business Owners

## Problem Statement
Small business owners (Maya, Carlos, Priya) do not have the time or expertise to manage complex software, hire employees for discrete administrative tasks, or integrate multiple different AI tools. They need an "invisible workforce" that operates reliably in the background, organized into intuitive "departments" that handle distinct business functions (operations, marketing, sales, customer success) without requiring technical oversight or prompting.

## Research Report
- **The "Blank Canvas" Problem**: Current AI solutions (ChatGPT, general copilots) require the user to act as a prompt engineer and project manager. Small business owners suffer from cognitive overload when presented with a blank chat box.
- **Departmental Model**: A real-world business operates in functional silos (e.g., Marketing, Sales, Operations). Aligning AI agents to these familiar concepts reduces friction.
- **Proactive vs. Reactive AI**: OHC's unique advantage is proactive AI. Instead of waiting for a command, the agents monitor events (e.g., a new order, a missed booking) and act autonomously or prepare drafts for 1-tap approval.

## Design Doc

### Key Design Decisions
- **Unified Departmental Registry**: Define 7 core departments: Operations ("The Manager"), Marketing ("The Promoter"), Sales ("The Salesperson"), Customer Success ("The Ambassador"), Finance ("The Accountant"), Legal ("The Protector"), and Business Advisory ("The Advisor").
- **Event-Driven Execution**: Agents are triggered primarily by KAIROS orchestrator events (e.g., `order.created`, `inventory.low`), not just direct user prompts.
- **1-Tap Approval Flow**: High-risk external actions (emailing a customer, posting to Instagram) are drafted by the agent and placed in an approval queue, requiring a single tap from the owner to execute. Low-risk internal actions (updating a dashboard counter) are auto-executed.
- **Shared Context Memory**: All agents within a tenant's workspace share access to a central vector database, ensuring "The Salesperson" knows what "The Manager" just fulfilled.
- **Usage & Budgeting limits**: Agent activities are tightly scoped by the tenant's current SaaS tier. Hard limits and throttles exist on monthly AI actions (e.g., Starter: 1000/mo) at the orchestrator layer to prevent costly runaway execution loops or multi-tenant noisy-neighbor starvation.

### Architecture Diagram (Mermaid.js)

```mermaid
graph TD
    subgraph KAIROS Orchestrator
    EventBus[Hybrid Event Mesh]
    TaskQueue[Shared Task Queue]
    ApprovalQueue[1-Tap Approval Queue]
    end

    subgraph AI Agent Departments
    Ops[Operations: "The Manager"]
    Mktg[Marketing: "The Promoter"]
    Sales[Sales: "The Salesperson"]
    CS[Customer Success: "The Ambassador"]
    Fin[Finance: "The Accountant"]
    Adv[Business Advisory: "The Advisor"]
    end

    subgraph OHC-SIP DB
    StateDB[Relational DB]
    VectorMem[Vector Memory]
    end

    UserApp[Mobile Dashboard App]

    %% Flow
    EventBus -->|Trigger| Ops
    EventBus -->|Trigger| Sales
    EventBus -->|Trigger| CS

    Ops -->|Read/Write| StateDB
    Ops -->|Store Memory| VectorMem

    CS -->|Query Context| VectorMem
    CS -->|Draft Message| ApprovalQueue

    Adv -->|Analyze Trends| VectorMem
    Adv -->|Generate Report| ApprovalQueue

    ApprovalQueue -->|Notification| UserApp
    UserApp -->|1-Tap Approve| TaskQueue
    TaskQueue -->|Execute| EventBus
```

### Mobile UX Flow
1. **The "Activity Feed"**: The primary interface for AI interaction is a unified feed on the mobile dashboard.
2. **Draft Cards**: High-risk actions appear as cards in the feed (e.g., "Draft Email to Carlos: Your order has shipped").
3. **1-Tap Interaction**: The card has two large touch targets: "Approve" (Green, >44px) and "Edit/Reject" (Red).
4. **Optimistic Updates**: Tapping "Approve" immediately dismisses the card with a shimmer effect, while KAIROS handles execution in the background.

## Implementation Prompt
**To Implementer Agent:**
Implement the core KAIROS event routing and execution framework for the AI Agent Departments. Define the 7 department personas and their permitted event triggers. Implement the `ApprovalQueue` mechanism, where actions marked with a high `RiskLevel` are intercepted and stored in a pending state until a `1_tap_approve` event is received from the UI. Create the corresponding mobile-first UI component (the "Activity Feed" card) to display these pending actions. Ensure that all agents read from and write to the shared memory store to maintain context. Implement the rate-limiting and monthly budget enforcement middleware based on the tenant's tier. Focus on the internal wiring and event flow; do not prescribe specific LLM API calls or vector embedding formats.

## Priority
P0

## Estimated Scope
Large
