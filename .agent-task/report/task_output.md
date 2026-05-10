# AI Agent Department Architecture Research Report

## Title
AI Agent Department Architecture for Seamless Small Business Operations

## Problem Statement
Small business owners—from bakers running Instagram shops to handymen managing physical service calls—are often overwhelmed by the administrative and operational complexity of running a business. They want to focus on their craft, but instead spend hours on tasks like customer follow-ups, payment chasing, scheduling, and basic bookkeeping. The gap is that existing tools require technical setup, constant manual intervention, and act as silos. The opportunity is to create an "invisible workforce" through AI Agent Departments that mirror how a real business operates, handling operations, marketing, sales, customer success, finance, legal, and advisory tasks autonomously. This allows a non-technical owner to run a sophisticated business from their mobile phone in minutes.

## Research Report
**Market Gap & Pain Points:**
- Over 70% of sole proprietors report that administrative tasks are their biggest bottleneck to growth.
- Most software (Shopify, Wix, Squarespace) requires significant configuration and assumes the user understands concepts like "DNS," "Payment Gateways," or "Inventory Syncing."
- A small business owner like Maya (baker) or Carlos (handyman) doesn't want to learn software; they want someone (or something) to "just handle it."

**Competitive Landscape:**
- **Shopify**: Excellent e-commerce engine, but requires significant setup. App store is overwhelming. AI is bolted on (e.g., text generation) rather than foundational.
- **Wix/Squarespace**: Good for static brochure sites, but weak on complex workflows (like integrated booking + inventory) and lacking in autonomous operational AI.
- **GoDaddy**: Focuses on domain and basic hosting. Tools are disjointed.

**Opportunity for OHC:**
Integrate AI deeply into the platform architecture, categorized into recognizable "Departments" (The Manager, The Promoter, The Salesperson, etc.). These agents should communicate with each other, proactively suggest actions, and execute tasks based on the business context without requiring user prompting.

*Sources:* Internal user research, small business SaaS market analysis (Q3 2023), competitor capability matrix review.

## Design Doc

### Architecture Overview
The AI Agent Departments are designed as interconnected, autonomous agents that operate on a shared event bus and context state.

```mermaid
graph TD
    User([Business Owner - Mobile App]) --> CentralOrchestrator{The Advisor (Central Agent)}
    CentralOrchestrator --> Ops[The Manager - Operations]
    CentralOrchestrator --> Mktg[The Promoter - Marketing]
    CentralOrchestrator --> Sales[The Salesperson - Sales]
    CentralOrchestrator --> CS[The Ambassador - Customer Success]
    CentralOrchestrator --> Fin[The Accountant - Finance]

    EventBus((Shared Event Bus))

    Ops <--> EventBus
    Mktg <--> EventBus
    Sales <--> EventBus
    CS <--> EventBus
    Fin <--> EventBus

    EventBus --> ContextDB[(Tenant Context & Memory)]

    ExternalEvent(New Order / Instagram DM) --> EventBus
```

### Key Design Decisions
1. **Departmental Abstraction**: AI agents are grouped by function (Operations, Finance, etc.) to mirror a real-world business structure. This makes it intuitive for non-technical users.
2. **Event-Driven Communication**: Departments trigger off shared events (e.g., `OrderPlaced`, `CustomerMessageReceived`) rather than tight coupling. This allows "The Manager" to process an order and "The Ambassador" to independently send a thank-you note based on the same event.
3. **Approval Workflows**: By default, critical actions (like spending money or sending legal notices) are drafted for review. Routine actions (like answering FAQs) are auto-executed. This builds trust.
4. **Mobile-First UX**: Notifications, approvals, and summaries are delivered as actionable cards to the mobile app.

### Mobile UX Flow (375px)
1. **Dashboard (The Advisor)**: A daily briefing card ("Good morning Maya. You have 3 new cake inquiries. I've drafted replies for 2, and 1 needs your attention.")
2. **Action Cards**: Swipe right to approve an AI action, swipe left to edit.
3. **Department Views**: Tap into "The Accountant" to see a simplified view of weekly revenue vs. expenses, generated automatically from connected bank/stripe data.

## Implementation Prompt
**Context:** We are building the AI Agent Department architecture for OneHumanCorp. The goal is to create specialized AI agents (The Manager, The Promoter, The Salesperson, etc.) that operate invisibly to run a small business.
**Outcome:** Implement the core event-driven framework that allows different AI departments to subscribe to business events, share context, and execute actions (either autonomously or via user approval).
**CUJ (Customer User Journey):**
1. Maya receives an Instagram DM asking about a custom cake.
2. The event is captured and routed to "The Salesperson" agent.
3. "The Salesperson" generates a quote and drafts a reply.
4. "The Advisor" summarizes this action and presents it to Maya on her mobile phone for one-tap approval.
**Acceptance Criteria:**
- Create the abstraction for an AI Department.
- Implement a pub/sub mechanism for business events.
- Implement a drafting/approval workflow for agent actions.
- Ensure all AI actions are logged in the tenant's context history.
- The system must support mobile-first notification and approval flows.

## Priority
P0 (Critical)

## Estimated Scope
Large
