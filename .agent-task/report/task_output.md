# Issue Brief: AI Agent Department Architecture

## Title
AI Agent Department Architecture: Invisible, Event-Driven Teammates

## Problem Statement
Small business owners (Maya, Carlos, Priya, Leo, Fatima) juggle too many operational hats: marketing, customer service, inventory management, and finance. Competitor tools treat AI as an add-on "tool" that requires prompting, formatting, and manual effort—creating *more* work. OHC must provide "teammates": AI agents organized into intuitive departments that operate autonomously in the background, triggered by events, sharing a unified memory, and relying on simple 1-tap approvals for critical actions. The technical challenge is to architect these departments so they interact seamlessly via the KAIROS Triad (Shared Task List, Teammate Mesh, AutoDream Memory) without complex setup.

## Research Report
- **Goal**: Design how AI agents run invisibly in the background, organized into understandable functional "departments," operating autonomously on the Teammate Mesh, and storing context in the long-term memory.
- **Findings**:
  - Small business owners lack time for complex configuration and prompt engineering. They need "done-for-you" functionality.
  - The OHC AI Differentiation Manifesto contrasts "Tool AI" with "Teammate AI." OHC agents must proactively monitor the event mesh, draft actions, and surface them via a 1-tap approval feed.
  - The KAIROS Triad (Shared Task List, Teammate Mesh using Redis/Centrifugo, AutoDream Pipeline with pgvector) forms the perfect substrate for inter-agent coordination.
- **Key Departments**:
  1.  **Operations ("The Manager")**: Monitors velocity, flags low stock, processes orders.
  2.  **Marketing & Advertising ("The Promoter")**: Generates social calendars, runs campaigns, updates storefront vibe.
  3.  **Sales & Acquisition ("The Salesperson")**: Generates quotes, follows up on leads, tracks referrals.
  4.  **Customer Success ("The Ambassador")**: Drafts 1-tap responses to DMs and emails, re-engages inactive users.
  5.  **Finance & Payments ("The Accountant")**: Reconciles payments, warns about recurring billing, tracks deposits.
  6.  **Legal & Compliance ("The Protector")**: Manages terms, tracks licenses, monitors GDPR constraints.
  7.  **Business Advisory ("The Advisor")**: Generates human-language daily briefings ("Your vegan cake is trending. Boost spend by $5").
- **Competitive Analysis**: Shopify Magic and Wix ADI act on direct prompts. OHC agents act on system events (e.g., an Instagram DM triggering The Ambassador to draft a reply, pulling inventory context from The Manager).

## Design Doc

### Key Design Decisions
1.  **Event-Driven Autonomous Execution**: Agents do not wait for prompts; they subscribe to the Teammate Mesh. For instance, `tenant.order.created` triggers both Operations and Customer Success.
2.  **The Draft-for-Review Queue**: High-risk actions (e.g., publishing a post, sending an email) are marked as `ActionRisk: High`. They pause execution and push a draft to the user's dashboard for a 1-tap approval from their mobile device. Low-risk actions (e.g., tagging a customer) auto-execute.
3.  **Unified Memory (AutoDream)**: All departments access the same `pgvector` memory store (`autodream_memories`). When "The Salesperson" generates a quote, it uses context about the customer gathered by "The Ambassador."
4.  **Tiered Resource Allocation**: Agent activity limits are enforced via the `TierService` (Free = 1 Dept/100 actions; Business = Unlimited). Throttling happens at the KAIROS Orchestrator level to protect the Shared Task List.

### Architecture Diagram (Mermaid.js)
```mermaid
sequenceDiagram
    participant Event as Teammate Mesh (Hub)
    participant O as KAIROS Orchestrator
    participant Ops as The Manager (Ops)
    participant CS as The Ambassador (Success)
    participant Adv as The Advisor
    participant Mem as AutoDream (pgvector)
    participant Owner as Business Owner (Mobile)

    Note over Event: "New Instagram DM: Vegan Cakes?"
    Event->>O: Route Event
    O->>CS: Claim Task (Draft Reply)
    CS->>Mem: Query "vegan cakes availability"
    Mem-->>CS: Context (In Stock)
    CS->>O: Push Draft Reply (High Risk)
    O->>Owner: Push Notification: "1-Tap Approval"
    Owner->>O: Taps "Approve"
    O->>Event: Execute Reply

    Note over Event: "End of Day"
    Event->>Adv: Trigger Daily Briefing
    Adv->>Mem: Aggregate Daily Activity
    Adv->>Owner: "Vegan cake inquiries up 20% today."

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class Event,O,Ops,CS,Adv,Mem,Owner premium;
```

### UI Wireframes & Mobile UX
- **Action Feed (375px)**: A simple stack of cards on the home screen. "The Ambassador drafted a reply to Maya's DM. [Approve] [Edit]."
- **Department Settings**: A toggle list. "Turn on The Promoter to automate Instagram posts." No complex configuration; turning it on grants it access to the event mesh.
- **Visual Design**: Uses OHC premium design tokens (Glassmorphism cards, Outfit/Inter typography, > 44px touch targets).

## Implementation Prompt
**To Implementer Agent:**
Implement the "Draft-for-Review" workflow engine within the KAIROS Orchestrator.
1. Define the `ActionRisk` enum (Low, High) within the core agent task payload structure.
2. Create the `pending_approvals` table or queue mechanism in the OHC-SIP database to store high-risk agent outputs.
3. Implement the API endpoints for the mobile dashboard to fetch pending approvals and submit a 1-tap `Approve` or `Reject` decision.
4. Modify the core event dispatcher so that when an agent yields a High-risk action, it enters the pending state and triggers a mobile push notification payload instead of executing immediately.
Do not prescribe specific LLMs or underlying database driver details. Focus on exposing the required surface area for the 1-Tap Approval flow. Ensure E2E tests navigate from task generation to approval.

## Priority
P0

## Estimated Scope
Large
