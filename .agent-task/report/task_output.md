# [AI Agent] AI Agent Department Architecture

## Problem Statement

Small business owners—whether a baker like Maya or a handyman like Carlos—often juggle multiple roles simultaneously. They are the CEO, the marketer, the customer support rep, and the accountant. This context switching causes fatigue, missed opportunities, and limits the growth of their business. The gap lies in the fact that while large corporations have dedicated departments and teams, small businesses rely entirely on the owner's limited time. OHC needs a way to invisibly offload these operational, marketing, and sales tasks to an AI workforce that operates exactly like a well-staffed small business team, communicating in plain language that the owner inherently understands.

## Research Report

- **Current State:** Many SaaS platforms offer fragmented "AI features" (e.g., Shopify's Magic text generation or Wix's ADI website builder), but these are tools the user must actively operate.
- **The Gap:** There is no autonomous background workforce that coordinates cross-functionally. For instance, if an order is placed, an owner shouldn't have to trigger the "send confirmation" or "update inventory" actions manually.
- **Competitive Landscape:**
  - *Shopify / Squarespace:* Offer AI tools for content creation but lack autonomous background workers.
  - *GoDaddy / Wix:* Focus on initial setup automation but require manual daily operation.
  - *OHC:* The platform aims to provide invisible AI agents organized into familiar "departments" that coordinate to run the business.
- **Proposed Solution:** Implement an AI Agent Department Architecture that mimics a real business structure (Operations, Marketing, Sales, Customer Success, Finance, Legal, Advisory). These agents operate autonomously, either executing tasks or drafting them for approval based on the owner's preferences and confidence thresholds.

## Design Doc

### Architecture Overview

The AI Agent Department system is designed around event-driven autonomous workers that subscribe to business events, retrieve necessary context, and execute actions within their domain.

```mermaid
graph TD;
    UserAction[User/Customer Action] --> API[Rust API Server];
    SystemEvent[Scheduled System Event] --> API;

    API --> EventBus[Event Bus / KAIROS Sub-Agent Queue];

    EventBus --> DeptOps[Operations Agent 'The Manager'];
    EventBus --> DeptMktg[Marketing Agent 'The Promoter'];
    EventBus --> DeptSales[Sales Agent 'The Salesperson'];
    EventBus --> DeptCS[Customer Success Agent 'The Ambassador'];
    EventBus --> DeptFin[Finance Agent 'The Accountant'];
    EventBus --> DeptLegal[Legal Agent 'The Protector'];
    EventBus --> DeptAdv[Advisory Agent 'The Advisor'];

    DeptOps --> Memory[(Long-Term Memory)];
    DeptMktg --> Memory;
    DeptSales --> Memory;
    DeptCS --> Memory;
    DeptFin --> Memory;
    DeptLegal --> Memory;
    DeptAdv --> Memory;

    Memory --> StateMachine[Distributed State Machine];

    StateMachine --> ActionApproval{Action Approval Required?};
    ActionApproval -- Yes --> DraftForReview[Draft Action for Owner Review];
    ActionApproval -- No --> ExecuteAction[Auto-Execute Action];

    DraftForReview --> NotifyOwner[Notify Owner (Mobile Push/UI)];
    ExecuteAction --> AuditLog[Record to Audit Log];
```

### Mobile UX Flow (375px First)

1. **Dashboard Home:** The owner sees a unified "Agent Activity" feed on their mobile dashboard. E.g., "The Promoter scheduled 3 Instagram posts for this week."
2. **Review Screen:** A notification badge indicates tasks awaiting approval. Tapping it opens a simple list: "The Ambassador drafted a reply to Maya. [Approve] [Edit]."
3. **Department Settings:** Tapping into a specific department (e.g., "Sales") allows the owner to set autonomy levels ("Always auto-send quotes under $500" vs. "Review all quotes").

### AI Agent Integration Points

- **Trigger Mechanisms:**
  - *Event-Driven:* Webhooks from payment gateways (e.g., successful payment triggers Operations and Customer Success).
  - *Schedule-Driven:* Weekly crons trigger Finance (weekly report) and Advisory (weekly health check).
  - *On-Demand:* Owner explicitly asks an agent to perform a task via natural language in the UI.
- **Coordination:** Agents communicate via the Event Bus. When Operations fulfills an order, it emits an `OrderFulfilled` event, which Customer Success listens to in order to send the tracking email.
- **Memory & Context:** Agents query the `PersistentMemoryStore` and `VectorRepository` to retrieve historical customer interactions and business rules before acting.
- **Approval & Throttling:** Actions that mutate public state or involve money are gated by the `confidence_threshold` defined in `src/agents/builtin/departments.rs`. If confidence is below the threshold or the user configured manual review, the action is saved as a draft. AI usage is throttled per tenant based on their SaaS tier limits.

### Key Design Decisions

- **Familiar Naming:** Departments are named using real-world roles (e.g., 'The Promoter', 'The Accountant') so non-technical users immediately understand their purpose.
- **Event-Driven Architecture:** Decoupling agents via an event bus prevents deadlocks and allows departments to operate asynchronously without blocking user-facing API requests.
- **Draft-First Approach:** To build trust, critical actions default to drafting for review until the system gains confidence or the owner explicitly grants full autonomy.

## Implementation Prompt

**Task:** Implement the core event routing and draft-approval workflow for the AI Agent Departments.
**User-Facing Outcome:** When a customer completes a purchase, the Operations agent should automatically process the order, and the Customer Success agent should automatically draft a personalized thank-you email for the owner to approve with one tap on their mobile device.
**Acceptance Criteria:**
- The system correctly routes the `OrderCompleted` event to the relevant agent departments.
- The Customer Success agent generates a draft response and stores it in a pending state rather than sending it immediately.
- The mobile UI displays the draft response with "Approve" and "Edit" options.
- Upon approval, the response is sent to the customer and the action is logged.

## Priority
P0 (Critical)

## Estimated Scope
Large