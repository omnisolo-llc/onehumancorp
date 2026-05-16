# [architecture] AI Agent Department Architecture

## Title
Implement KAIROS AI Agent Department Architecture for Invisible Business Management

## Problem Statement
Small business owners like Maya (baker), Carlos (handyman), and Fatima (food cart) are overwhelmed. They don't just need a website builder; they need a team. When a customer messages Maya on Instagram at 2 AM asking "do you do vegan cakes?", she is asleep. When Carlos finishes a job, he forgets to ask for a review. They spend 40% of their time on admin tasks rather than their craft. Existing tools treat AI as a "copilot" that requires constant prompting, which creates more work. Business owners need "colleagues"—autonomous departments (Operations, Marketing, Sales, Customer Success, Finance) that handle tasks invisibly in the background, only asking for 1-tap approval when necessary.

### Persona Pain Points
- **Maya (Baker):** Overwhelmed by DMs and order tracking. Needs automatic replies and order status updates.
- **Carlos (Handyman):** Bad at follow-ups and quoting. Needs automatic quote generation and review requests after jobs.
- **Priya (Boutique):** Struggles with marketing consistency. Needs automatic social media posts when new inventory arrives.
- **Fatima (Food Cart):** Needs automatic stock out notifications and simple daily summaries.

## Research Report
The current SMB platform landscape treats AI as a reactive tool rather than a proactive teammate.

### Competitive Landscape

| Platform | AI Approach | Proactive Actions | SMB Usability | Pricing Strategy |
|----------|-------------|-------------------|---------------|------------------|
| **Shopify** | "Magic" Text Gen | Low (Reactive) | High complexity | Expensive add-ons |
| **Wix** | AI Site Gen | None | Medium | Tiered |
| **Squarespace**| Basic Copywriting | None | High (Design focus)| Standard |
| **GoDaddy** | AI Prompts | Low | Low | Cheap intro, high renewal |
| **OHC (Target)**| **Autonomous Teammates**| **High (Event-driven)**| **Frictionless (1-tap)**| **Value-based Tiers** |

### Findings
1. **The Prompting Fatigue:** Non-technical owners do not want to write prompts. They want the system to notice an event (e.g., "new product added") and offer a solution (e.g., "Here is a 7-day social media plan for your new product. Approve?").
2. **Invisible Operations:** Automation must run invisibly. The "Action Feed" model is superior to the "Chat" model for daily operations.
3. **Trust & Control:** Users fear AI hallucinating to customers. A hybrid "Draft for Review" (1-tap approval) vs "Auto-Execute" model builds trust progressively.

## Design Doc

### Architecture Diagram

```mermaid
graph TD
    %% Define Styles (Visual Excellence Mandate)
    classDef user fill:#E8F0FE,stroke:#1A73E8,stroke-width:2px,color:#1A73E8,font-family:Outfit;
    classDef event fill:#FCE8E6,stroke:#D93025,stroke-width:2px,color:#D93025,font-family:Inter;
    classDef agent fill:#E6F4EA,stroke:#1E8E3E,stroke-width:2px,color:#1E8E3E,font-family:Outfit;
    classDef queue fill:#FFF7E1,stroke:#F9AB00,stroke-width:2px,color:#F9AB00,font-family:Inter;
    classDef outcome fill:#F3E8FD,stroke:#9334E6,stroke-width:2px,color:#9334E6,font-family:Outfit;

    subgraph Business Events
        E1(Customer DM) :::event
        E2(Product Added) :::event
        E3(Job Completed) :::event
    end

    subgraph KAIROS Event Router
        Router{Event Mesh} :::queue
    end

    subgraph AI Departments
        CS[The Ambassador\nCustomer Success] :::agent
        MKT[The Promoter\nMarketing] :::agent
        OPS[The Manager\nOperations] :::agent
        ADV[The Advisor\nBusiness Advisory] :::agent
    end

    subgraph Action & Approval
        Feed[Action Feed\nDashboard UI] :::queue
        Auto[Auto-Execute] :::outcome
    end

    E1 --> Router
    E2 --> Router
    E3 --> Router

    Router --> CS
    Router --> MKT
    Router --> OPS

    CS -->|Drafts Reply| Feed
    MKT -->|Drafts Social Post| Feed
    OPS -->|Flags Low Stock| Feed
    CS -->|Answers FAQ| Auto

    Feed -->|1-Tap Approve| User((Business Owner)) :::user
    User --> Auto

    Auto --> ADV
    ADV -->|Weekly Briefing| Feed
```

### Key Design Decisions
- **Event-Driven, Not Prompt-Driven:** Departments are subscribed to specific business events via an event mesh, removing the need for manual triggering.
- **The "Action Feed" UI:** Instead of a chat window, the core interface is a chronological feed of actionable cards (e.g., "Drafted a response to Maya. [Send] [Edit] [Discard]").
- **Progressive Autonomy:** By default, external-facing actions (like emailing a quote) are queued for "1-Tap Approval". As the user builds trust, they can toggle specific actions to "Auto-Execute".
- **Department Personas:** We use humanized, functional names ("The Promoter" instead of "Marketing Agent") so a baker or handyman immediately understands the value.

### Mobile UX Flow (375px First)
1. **Lock Screen:** Push Notification: *"The Ambassador drafted a reply to an Instagram DM. Tap to review."*
2. **Action Feed Screen (App Open):**
   - Clean, glassmorphic card interface (Outfit heading, Inter body).
   - Entrance animation (slide up, <= 300ms, cubic-bezier).
   - Card content: "Customer asked: 'Do you do vegan cakes?' | Draft: 'Yes, we have 3 vegan options! I can send you the menu. - Maya's Bakery'"
   - Touch targets (44x44px minimum): `[Approve & Send]` `[Edit]` `[Dismiss]`.
3. **Approval:** User taps `[Approve & Send]`. Card collapses with a subtle exit animation (<= 200ms). Success toast appears.

### AI Agent Integration Points
- **Operations:** Triggers on order creation/fulfillment.
- **Marketing:** Triggers on inventory update or seasonal schedules.
- **Customer Success:** Triggers on inbound webhooks (Instagram/Facebook API, email).
- **Business Advisory:** Scheduled chron cron job for end-of-week analysis.

## Implementation Prompt
**Context for Implementer:**
Implement the "AI Agent Department Architecture" feature. This is the core mechanism that transitions OHC from a passive tool to an active teammate.

**User Journey (CUJ):**
1. The business owner opens their mobile dashboard and sees the "Action Feed".
2. The system simulates an incoming business event (e.g., a customer DM or a low inventory alert).
3. The KAIROS backend routes this event to the correct AI Department.
4. The AI Department generates a proposed action and queues it in the Action Feed as a "Draft".
5. The business owner views the card and taps "Approve" (1-tap).
6. The action is executed and moved to history.

**Acceptance Criteria:**
- Create the visual Action Feed UI component for the dashboard (mobile-first, using OHC design tokens: glassmorphism, Outfit/Inter fonts, correct animation timings).
- Wire up the event routing so that specific simulated events trigger specific departments ("The Ambassador" for messages, "The Manager" for stock).
- Implement the 1-tap approval flow that transitions an action from "draft" to "executed".
- Do not prescribe specific database schemas or API endpoints—design those to support the CUJ securely and performantly.
- Ensure the feature is fully usable on a 375px mobile viewport.

## Priority
`P0` (Critical - Defines the platform's core differentiation)

## Estimated Scope
Large
