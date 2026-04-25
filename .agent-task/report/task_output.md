# [Architecture] AI Agent Department Architecture

## Title
Implement the AI Agent Department System Architecture

## Problem Statement
Small business owners like Maya (the home baker) and Carlos (the freelance handyman) are overwhelmed by the complexity of managing operations, marketing, sales, and finances. They don't have the time or technical skills to integrate various software tools. Current platforms either offer basic chatbots or require manual configuration of complex workflows. OHC needs a system where AI agents work invisibly in the background, organized into understandable "departments" (Operations, Marketing, Sales, Customer Success, Finance, Legal, Advisory) that coordinate seamlessly, acting as a full team for the business owner.

## Research Report
### Competitive Analysis
- **Shopify**: Offers "Sidekick," which is primarily an assistant chatbot for the merchant. It doesn't function as a coordinated team of autonomous departments operating in the background. It answers questions and can perform some tasks on demand, but requires merchant prompting.
- **Wix**: Wix AI helps with website generation and text creation but lacks a continuous, background-running "department" model that handles day-to-day operations like following up on leads or tracking inventory asynchronously.
- **Squarespace**: Has limited AI integrations (mostly for copywriting). It is primarily a portfolio/storefront builder, not a business management AI suite.
- **GoDaddy**: Airo offers some AI setup tools (domain generation, logo creation) but lacks autonomous agents managing operations or customer success workflows post-setup.
- **OHC's Opportunity**: We are the first to treat AI as infrastructure rather than a bolt-on chatbot. Organizing AI into recognizable "departments" (e.g., "The Manager", "The Promoter") makes complex background operations accessible to non-technical users.

### Key Findings
- Non-technical users understand roles like "Accountant" or "Salesperson" better than "Automated Workflow" or "LLM Agent."
- Trust is the biggest barrier. Users need a system to approve actions ("draft-for-review" mode) before they let agents auto-execute.
- Inter-agent coordination is required (e.g., an order placed triggers Operations to manage fulfillment, which triggers Finance to log the revenue, and Customer Success to send a confirmation).

## Design Doc

### Key Design Decisions and Why
1. **Department Structure**: Organize agents into 7 functional departments (Operations, Marketing, Sales, Customer Success, Finance, Legal, Advisory). *Why: Mirrors a real business. Intuitive for non-technical owners.*
2. **Event-Driven Coordination**: Departments communicate via domain events (e.g., `OrderPlaced`, `InventoryLow`). *Why: Allows loose coupling. Operations processing an order can seamlessly trigger Customer Success without tight integration.*
3. **Approval Modes (Auto-Execute vs. Draft-for-Review)**: Each agent action type has a configurable approval mode. By default, high-risk actions (e.g., refunding money, sending a custom quote) are generated as "drafts" requiring the owner's 1-tap approval. *Why: Builds trust and prevents costly AI hallucinations from impacting the business.*
4. **Memory and Context**: Agents share a centralized semantic memory (pgvector embeddings of past interactions, orders, and business context). *Why: Ensures "The Promoter" knows about the latest products added by "The Manager" without the owner having to explain it.*
5. **Usage Throttling**: AI usage is budgeted per tenant with token tracking. *Why: Protects platform profitability and enforces tier limits (e.g., Free vs. Pro tiers).*

### Architecture Diagram

```mermaid
graph TD
    subgraph Event Bus / Message Broker
        EB[Domain Events: OrderPlaced, MessageReceived, etc.]
    end

    subgraph OHC AI Agent Departments
        OP[Operations - "The Manager"]
        MK[Marketing - "The Promoter"]
        SL[Sales - "The Salesperson"]
        CS[Customer Success - "The Ambassador"]
        FN[Finance - "The Accountant"]
        LG[Legal - "The Protector"]
        AD[Advisory - "The Advisor"]
    end

    subgraph Core Services
        DB[(Tenant PostgreSQL)]
        VDB[(pgvector Memory)]
        LK[Redis Locks]
        LLM[LLM Provider - Gemini/GPT]
    end

    EB --> OP
    EB --> MK
    EB --> SL
    EB --> CS
    EB --> FN
    EB --> LG
    EB --> AD

    OP <--> LLM
    CS <--> LLM
    FN <--> LLM

    OP <--> DB
    CS <--> VDB
    MK <--> LK
```

### AI Agent Integration Points
- **Triggers**:
  - *On Schedule*: Advisory runs weekly to generate health reports.
  - *On Event*: Customer Success reacts to `MessageReceived` or `OrderPlaced`.
  - *On Demand*: Owner asks Operations to "add a new vegan cake to the menu."
- **Shared Memory**: All agents query the pgvector database for tenant-specific context before generating responses or taking actions.
- **Coordination**: Agents publish events back to the Event Bus. E.g., Sales finalizes a quote -> publishes `QuoteAccepted` -> Finance initiates payment collection.

### UI Wireframes & Mobile UX Flow
**Screen 1: Department Dashboard (375px)**
- Header: "Your Team"
- Grid of 7 Cards (Operations, Marketing, Sales, etc.)
- Each card shows a status dot (Green = Active, Yellow = Action Required).
- Example: "The Ambassador (Customer Success) - 2 drafts to review"

**Screen 2: Draft Review Flow (375px)**
- Notification: "The Ambassador drafted a reply to Maya's Instagram DM."
- View message: "Hi Maya, yes we do vegan cakes! Would you like to order one?"
- Actions: [Approve & Send] [Edit] [Reject]

**Screen 3: Agent Settings (375px)**
- Title: "Marketing Department Settings"
- Toggle: "Auto-post to Instagram" (On/Off)
- Toggle: "Auto-reply to comments" (Draft for Review / Auto-Execute)

## Implementation Prompt
**For the Implementer Agent:**
Implement the foundational framework for the AI Agent Departments.
- Create the domain models for the 7 departments.
- Build the event-driven trigger system where an incoming event (like an order or a message) can be routed to the appropriate department's job queue.
- Implement the "Draft-for-Review" vs. "Auto-Execute" capability, including the UI for the business owner to review and approve pending agent actions on a mobile device.
- Ensure all agent contexts are isolated per tenant using the `tenant_id` and RL policies.
- Do not hardcode specific LLM APIs; use the existing provider interfaces.
- Acceptance Criteria: A user can simulate receiving a customer message, see the Customer Success agent generate a draft reply, review the draft in the mobile UI, and approve it for sending.

## Priority
P0

## Estimated Scope
Large
