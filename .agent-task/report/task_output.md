# [Architecture] AI Agent Department System: Orchestrating the Invisible Business Engine

## Problem Statement
Small business owners—whether it's Maya the baker or Carlos the handyman—often spend more time running their business than actually practicing their craft. They have to play the role of manager, promoter, salesperson, customer support, and accountant. Traditional platforms (Shopify, Wix) give them software tools to do these jobs, but *still require the owner to do the work*. When a customer DMs Maya on Instagram asking for a vegan cake quote while she's sleeping, she loses the sale if she doesn't reply fast enough. The core gap in the market is that owners don't just need software; they need *staff*. They need a completely invisible, intelligent, and coordinated workforce that runs the complexity of the business autonomously, entirely manageable from a smartphone.

## Research Report
### Competitive Analysis
- **Shopify**: Offers "Shopify Magic" (AI copy generation) and basic automation rules (Shopify Flow). However, it requires technical setup, and AI is a "feature," not a unified autonomous workforce.
- **Wix/Squarespace**: AI website generation during onboarding and basic AI text generation. Lacks autonomous operations, customer support agents, or proactive advisory.
- **GoDaddy**: Basic AI prompt-to-website features, but stops there. Zero proactive AI engagement with customers.
- **OneHumanCorp (OHC) Opportunity**: Instead of building "AI features," OHC organizes AI into distinct, understandable "Departments" (like "The Manager" or "The Promoter"). This mental model mirrors a real business, passing the grandmother test.

### Key Findings
1. **Coordination is Key**: AI agents cannot work in silos. If "The Salesperson" closes a deal, "The Manager" needs to fulfill it, and "The Accountant" needs to log the payment.
2. **Trust & Autonomy**: Owners are scared of AI making mistakes (e.g., promising a 90% discount). We need a clear mode-switch between "Draft for Review" (owner approves) and "Auto-Execute" (AI takes action).
3. **Budget Control**: LLM inference is expensive. Multi-tenant AI usage needs clear budgeting and throttling based on the tenant's tier.

## Design Doc

### Key Design Decisions
1. **Department Mental Model**: Structure agents into seven understandable departments: Operations, Marketing, Sales, Customer Success, Finance, Legal, and Advisory. No technical jargon.
2. **Event-Driven Coordination**: Departments communicate via a central event bus (invisible to the user). An event like `OrderFulfilled` triggers actions across multiple departments.
3. **Approval Workflows**: Every AI action defaults to "Draft for Review" until the user explicitly toggles "Auto-Execute" for that specific department/task.
4. **Shared Tenant Memory**: All departments access a shared "Tenant Context" (memory) containing business rules, brand voice, inventory state, and customer interaction history.
5. **Tier-Based Budgeting**: AI usage is metered. Actions are throttled based on the SaaS tier (Free, Starter, Pro, Business) using a token/action budget system.

### Architecture Diagram

```mermaid
flowchart TD
    %% User and External Triggers
    User[Business Owner (Mobile)]
    Cust[Customer / External]
    Cron[Time/Schedule]

    %% Core Event Bus
    Bus{Core Business Event Bus}

    %% AI Departments
    subgraph AI Workforce [AI Departments]
        Ops["Operations\n(The Manager)"]
        Mktg["Marketing\n(The Promoter)"]
        Sales["Sales\n(The Salesperson)"]
        CS["Customer Success\n(The Ambassador)"]
        Fin["Finance\n(The Accountant)"]
        Legal["Legal\n(The Protector)"]
        Adv["Advisory\n(The Advisor)"]
    end

    %% State & Memory
    Mem[(Shared Tenant Memory & Context)]
    Budget[(Usage & Budget Throttling)]
    Approve[(Approval Queue / Inbox)]

    %% Connections
    User -->|Approves actions, Configures| Approve
    Cust -->|DMs, Orders, Books| Bus
    Cron -->|Daily/Weekly Triggers| Bus

    Bus -->|Routes Events| Ops & Mktg & Sales & CS & Fin & Legal & Adv
    Ops & Mktg & Sales & CS & Fin & Legal & Adv -->|Reads/Writes| Mem
    Ops & Mktg & Sales & CS & Fin & Legal & Adv -->|Checks Limits| Budget

    %% Department Coordination Example
    Ops -->|Publishes 'Order Shipped'| Bus
    Bus -->|Triggers| CS
    CS -->|Drafts 'Thank You' Msg| Approve
```

### Mobile UX Flow (375px First)
1. **Home Screen ("The Hub")**: A clean, Glassmorphism dashboard showing a unified "Action Inbox."
2. **The Inbox**: A Tinder-like swipe interface for AI drafts.
   - *Card*: "The Ambassador drafted a reply to Sarah's Instagram DM about vegan cakes. [View Message]"
   - *Action*: Swipe right to approve and send, swipe left to discard, tap to edit.
3. **Department Settings**:
   - Tap on "The Promoter" icon.
   - See recent activity: "Generated 2 Instagram posts this week."
   - Toggle switches: "Auto-post to Instagram" (Off - needs approval), "Auto-reply to comments" (On).
4. **Advisory Briefing**: A daily morning push notification: "The Advisor: You have 3 pending orders and a new review. Tap to view today's brief."

### AI Integration Points
- **Triggers**:
  - *On Demand*: User taps "Draft Newsletter" in Marketing.
  - *On Event*: Webhook from payment gateway triggers Finance and Operations.
  - *On Schedule*: Advisory runs every Monday at 8 AM to generate a weekly health report.
- **Memory Storage**: Interleaved interaction histories categorized by Customer ID and Business ID, enabling continuity (e.g., CS agent knows the customer asked Sales for a discount yesterday).

## Implementation Prompt
**To the Implementer Agent:**
Implement the core orchestration layer for the AI Department System.
- **Outcome**: A functional event-driven framework where distinct AI "Departments" (Ops, Sales, CS, etc.) can subscribe to business events, read from a shared tenant memory, and propose actions.
- **CUJ (Critical User Journey)**: A customer places an order. The system must route this event to Operations (which deducts inventory) and Customer Success (which drafts a thank-you message). The thank-you message must land in the business owner's "Action Inbox" for review, rather than sending automatically.
- **Acceptance Criteria**:
  - Departments must be modular and independently triggerable via events, schedules, or manual requests.
  - All drafted actions must be intercepted by an Approval Queue if the department is not set to "Auto-Execute."
  - Tenant memory must be accessible by all departments to ensure consistent context.
  - The system must check and deduct from the tenant's AI action budget before executing.
  - Provide full test coverage ensuring events are correctly routed and budget limits are enforced.

**Priority**: P0
**Estimated Scope**: Large
