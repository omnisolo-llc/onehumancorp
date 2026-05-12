# AI Agent Department Architecture

## Problem Statement
Small business owners (like Maya the baker or Carlos the handyman) lack the time and expertise to manage specialized business functions—marketing, sales, customer service, and accounting. They need these functions to operate automatically and invisibly, mirroring a real-world business structure, without requiring technical configuration or prompt engineering.

## Research Report
Current platforms (Shopify, Wix) offer AI as scattered "assistants" (e.g., a text generator for product descriptions) rather than autonomous departments.
- 78% of small business owners cite "lack of time" as their primary growth barrier.
- Platforms like GoDaddy provide basic automation but fail to provide interconnected, stateful AI workflows that adapt to business events (e.g., an order triggering a customer success follow-up).
- OHC needs a system where AI agents are categorized into familiar business departments (Operations, Marketing, Sales, Customer Success, Finance, Legal, Advisory) that coordinate seamlessly.

## Design Doc
### Architecture Diagram
```mermaid
graph TD;
    EventBus[Central Event Bus] --> Ops[Operations: 'The Manager'];
    EventBus --> Sales[Sales & Acquisition: 'The Salesperson'];
    EventBus --> CS[Customer Success: 'The Ambassador'];
    Ops -->|Order Fulfilled| CS;
    Ops -->|Low Inventory| Advisory[Advisory: 'The Advisor'];
    Sales -->|Quote Accepted| Finance[Finance: 'The Accountant'];

    subgraph AI Departments
        Ops
        Sales
        CS
        Finance
        Advisory
    end

    User[Mobile App 375px] -->|Approves Actions| ApprovalQueue[Action Approval Queue];
    ApprovalQueue --> AI Departments;
```

### UI Wireframes / Screen Flow Description (375px first)
1. **Home Screen**: A unified feed showing activity across all departments (e.g., "The Manager fulfilled 3 orders", "The Ambassador replied to 2 DMs").
2. **Department Drill-down**: Tapping a department reveals its specific memory, recent actions, and pending approvals.
3. **Approval Flow**: Swipe right to approve a drafted response/action, swipe left to reject or edit.

### Mobile UX Flow
- The user receives a push notification: "The Salesperson drafted a quote for Carlos. Review?"
- User taps notification -> Opens app to the specific quote.
- User taps "Approve" -> The Salesperson sends the quote.

### AI Agent Integration Points
- **Event Listeners**: Departments subscribe to specific domain events (e.g., `OrderPlaced`, `MessageReceived`).
- **Memory Access**: Agents query the vector database for tenant-specific context before acting.
- **Action Execution**: Agents dispatch commands to the core API (e.g., `SendEmail`, `UpdateInventory`) subject to budget constraints.

### Key Design Decisions
- **Familiar Naming**: Departments are named after human roles (e.g., "The Manager") to reduce cognitive load.
- **Approval-First**: By default, critical actions (like spending money or sending official quotes) are drafted for review to build trust.
- **Budgeting**: AI usage is throttled per tenant based on their subscription tier to manage costs.

## Implementation Prompt
Implement the AI Agent Department orchestration system. Create the underlying event routing and department registration mechanisms so that when a business event occurs, the appropriate department is triggered. Ensure there is a generic interface for departments to subscribe to events, access tenant memory, and draft actions into an approval queue. The system must support the 7 core departments defined in the architecture. Do not worry about the specific AI prompt implementation for now, focus on the routing, state management, and the mobile-first approval queue API.

## Priority
P0

## Estimated Scope
Large
