# 📏 Architect: Define AI Agent Department Architecture

## Problem Statement

Small business owners—whether they run a food cart or a boutique—often feel overwhelmed by the sheer number of hats they have to wear. They have to be the operations manager, marketing director, salesperson, customer support rep, accountant, and legal advisor, all at once. Maya, our baker, needs help answering Instagram DMs while she sleeps. Carlos, our handyman, needs help generating quotes. Priya, the boutique owner, needs help syncing inventory and managing newsletters. They don't have the time or expertise to manage all these functions efficiently, nor can they afford to hire dedicated staff for each. The complexity of running a business prevents them from focusing on what they love: their craft. We need a system that acts as an invisible team of experts, handling these tasks seamlessly in the background.

## Research Report

Our target users are non-technical individuals operating physical or digital businesses. They use platforms like Shopify, Wix, Squarespace, and GoDaddy, but these platforms often require significant manual effort to manage integrations, build workflows, and handle customer interactions. For example, Shopify requires third-party apps for robust customer support (like Gorgias) or advanced marketing (like Klaviyo), adding cost and complexity. Wix and Squarespace offer some integrated tools, but they still require manual configuration and active management.

The opportunity lies in completely abstracting these functions into "Departments" that operate autonomously. By framing AI capabilities as traditional business departments ("The Manager," "The Promoter," "The Accountant"), we reduce cognitive load and make the technology accessible. A baker doesn't need to configure a "multi-agent orchestrator pipeline"; she simply needs to tell "The Customer Success Team" to answer DMs about vegan cake options.

## Design Doc

### Core Philosophy

The AI Agent Department architecture is designed to mirror a real-world business structure. Each department has a specific mandate, operates autonomously based on business context, and communicates cross-departmentally when necessary.

### Departments

1.  **Operations ("The Manager"):** Order fulfillment, inventory alerts, booking scheduling, refund processing.
2.  **Marketing & Advertising ("The Promoter"):** Website content generation, SEO optimization, social media drafting, promotional email campaigns.
3.  **Sales & Acquisition ("The Salesperson"):** Quote generation, lead follow-up, referral program management.
4.  **Customer Success ("The Ambassador"):** Answering customer inquiries (DMs, emails, chat), order status updates, review solicitation.
5.  **Finance & Payments ("The Accountant"):** Payment processing monitoring, daily/weekly financial summaries, subscription billing management.
6.  **Legal & Compliance ("The Protector"):** Boilerplate policy generation, license expiration tracking, privacy compliance checks.
7.  **Business Advisory ("The Advisor"):** High-level business health monitoring, actionable suggestions (e.g., "Your vegan cakes are trending, consider a price increase"), trend analysis.

### Architecture Diagram

```mermaid
graph TD
    subgraph KAIROS Orchestration Engine
        EventBus[Event Bus / Message Queue]
        Memory[Shared Context & Memory Vector DB]
        Budget[Token & Usage Throttling Service]
        Approval[Draft vs. Auto-Execute Gateway]
    end

    subgraph Departments
        Ops[Operations "The Manager"]
        Mktg[Marketing "The Promoter"]
        Sales[Sales "The Salesperson"]
        CS[Customer Success "The Ambassador"]
        Fin[Finance "The Accountant"]
        Legal[Legal "The Protector"]
        Adv[Advisory "The Advisor"]
    end

    External[External Triggers: Webhooks, User Input, Scheduled] --> EventBus

    EventBus --> Ops
    EventBus --> Mktg
    EventBus --> Sales
    EventBus --> CS
    EventBus --> Fin
    EventBus --> Legal
    EventBus --> Adv

    Ops <--> Memory
    Mktg <--> Memory
    Sales <--> Memory
    CS <--> Memory
    Fin <--> Memory
    Legal <--> Memory
    Adv <--> Memory

    Ops --> Approval
    Mktg --> Approval
    Sales --> Approval
    CS --> Approval
    Fin --> Approval
    Legal --> Approval
    Adv --> Approval

    Approval --> Action[Execution / Draft Output]
    Budget --> EventBus
```

### Mobile UX Flow (375px)

The mobile experience focuses on simplicity and transparency.

1.  **Home Feed:** The primary interface is a unified feed (like a social media feed) showing updates from different departments.
    *   *Card:* "The Salesperson drafted a quote for Carlos. [Review & Send] [Edit]"
    *   *Card:* "The Manager restocked 'Vegan Chocolate Cake' inventory based on your recent supply purchase. [Acknowledge]"
2.  **Department Settings:** A simple list of departments. Tapping one allows the user to set broad instructions.
    *   *Customer Success Settings:* "Tone: Friendly and professional. Instruction: We only offer delivery on weekends."
3.  **Approval Queue:** A dedicated tab for actions requiring human review before execution (e.g., sending a refund, publishing a new website section).

### AI Agent Integration Points

*   **Trigger Types:**
    *   **Event-Driven:** A new order arrives (Triggers Ops & CS).
    *   **Scheduled:** Weekly financial summary (Triggers Finance & Advisory).
    *   **On-Demand:** User asks, "Draft a post about our new summer menu" (Triggers Marketing).
*   **Cross-Departmental Coordination:** Example: Operations flags an item as out-of-stock. Operations updates the database -> Event Bus -> Marketing pauses ads for that item -> Customer Success prepares an apology template for pending inquiries.
*   **Memory & Context:** All departments share a unified context vector database containing business facts, past interactions, product details, and user preferences.
*   **Approval Flow:** Agents can be configured to "Auto-Execute" (low risk, e.g., answering FAQ) or "Draft for Review" (high risk, e.g., issuing a refund).
*   **Budgeting:** Tenant usage is tracked globally. If a Free tier user exceeds their monthly AI action limit, the system degrades gracefully, requiring manual intervention for tasks previously automated.

### Key Design Decisions

1.  **Anthropomorphized Departments:** Naming agents "The Manager" or "The Promoter" rather than "Inventory Management Module" dramatically lowers the barrier to entry for non-technical users.
2.  **Shared Memory Core:** Prevents departments from giving conflicting information (e.g., Marketing promoting a product Operations knows is discontinued).
3.  **Explicit Approval Gateway:** Builds trust. Users can start with everything on "Draft for Review" and gradually move tasks to "Auto-Execute" as they gain confidence in the AI.

## Implementation Prompt

**Role:** Implementer
**Objective:** Build the core routing and coordination logic for the "Customer Success (The Ambassador)" department, specifically focusing on handling incoming customer messages (e.g., via a unified inbox).

**User-Facing Outcome:**
When a customer sends a message (e.g., "Do you do vegan cakes?"), the Customer Success agent should automatically receive the message, consult the business's shared memory (product catalog, past FAQs, business hours), and either draft a reply for the owner's review or auto-reply if the owner has configured it to do so. The business owner should see this activity in their mobile feed.

**Core User Journey (CUJ):**
1. A customer message arrives via an external channel (simulated via an API endpoint for now).
2. The system routes the message to the Customer Success department.
3. The agent retrieves relevant context from the business's memory.
4. The agent generates a response.
5. Based on tenant settings, the system either sends the response automatically or places it in a "Drafts" queue for the owner to approve.
6. The action is logged in the tenant's activity feed.

**Acceptance Criteria:**
- The Customer Success agent can receive incoming messages.
- The agent can access a simulated or real "shared memory" store to inform its response.
- A mechanism exists to toggle between "Auto-Execute" and "Draft for Review" for this specific agent.
- The outcome (draft or sent message) is visible in an activity log that the UI can consume.
- Ensure all data access is strictly isolated to the current tenant.

## Priority
P0 (Critical)

## Estimated Scope
Large
