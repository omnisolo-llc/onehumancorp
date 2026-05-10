# AI Agent Department Architecture

## Problem Statement

Small business owners — like Maya the baker or Carlos the handyman — lack the time, expertise, and resources to manage every operational aspect of their business. They need an affordable, automated solution that invisibly handles customer inquiries, sales generation, marketing, financial tracking, and legal compliance. Current platforms (Shopify, Wix, Squarespace) offer tools but require the owner to do the work. OHC needs AI "employees" that do the work for them.

## Research Report

Our target users (Maya, Carlos, Priya, Leo, Fatima) operate in diverse verticals (physical goods, services, retail, subscriptions, food). Their common pain point is the "second shift" — managing operations after business hours.

**Competitive Analysis:**
- **Shopify:** Excellent e-commerce, but relies heavily on third-party apps for marketing and customer service. High cognitive load to setup.
- **Wix/Squarespace:** Good website builders, but lack deep operational automation.
- **GoDaddy:** Basic tools, but no proactive AI management.

**OHC Differentiation:**
OHC provides "Departments" — pre-configured, context-aware AI agents that act autonomously based on the business's data (inventory, calendar, orders).

## Design Doc

### Architecture Diagram

```mermaid
graph TD;
    User[Business Owner] --> |Configures/Approves| Hub[OHC Orchestration Hub];

    Hub --> |Routes Events| EventBus[Event Bus];

    EventBus --> |Trigger| Operations[Operations Dept (The Manager)];
    EventBus --> |Trigger| Marketing[Marketing Dept (The Promoter)];
    EventBus --> |Trigger| Sales[Sales Dept (The Salesperson)];
    EventBus --> |Trigger| CustomerSuccess[Customer Success Dept (The Ambassador)];
    EventBus --> |Trigger| Finance[Finance Dept (The Accountant)];
    EventBus --> |Trigger| Legal[Legal Dept (The Protector)];
    EventBus --> |Trigger| Advisory[Advisory Dept (The Advisor)];

    Operations --> |Updates| Database[(Tenant DB)];
    Marketing --> |Updates| Database;
    Sales --> |Updates| Database;
    CustomerSuccess --> |Updates| Database;
    Finance --> |Updates| Database;
    Legal --> |Updates| Database;
    Advisory --> |Updates| Database;

    Database --> |Context/Memory| Hub;

    Operations -.-> |Inter-Dept Event| CustomerSuccess;
    Sales -.-> |Inter-Dept Event| Operations;
```

### Mobile UX Flow (375px)

1. **Dashboard:** User sees a clean dashboard with "Department Updates" (e.g., "The Promoter created 3 Instagram posts").
2. **Department View:** Tapping a department shows its recent activity, pending approvals, and settings.
3. **Approval Flow:** If an agent drafts a response or action (e.g., a refund), it appears in a "Needs Review" queue. User can tap "Approve", "Edit", or "Decline".
4. **Settings:** Simple toggles for agent autonomy (e.g., "Auto-reply to common questions", "Draft only for complex inquiries").

### AI Agent Integration Points

- **Operations:** Listens to order/booking events. Triggers inventory updates or fulfillment workflows.
- **Marketing:** Listens to new product additions. Generates SEO descriptions and social media drafts.
- **Sales:** Monitors abandoned carts or incomplete inquiries. Sends follow-up messages.
- **Customer Success:** Intercepts incoming messages (email/SMS/chat). Uses RAG against order history and FAQs to reply.
- **Finance:** Runs scheduled jobs (daily/weekly) to summarize revenue and expenses.
- **Legal:** Monitors content for compliance (e.g., ensuring allergy warnings are on food items).
- **Advisory:** Analyzes weekly performance metrics to generate actionable suggestions.

### Inter-Department Coordination

Departments communicate asynchronously by emitting and consuming standardized events on the Event Bus. This ensures decoupled, reliable workflows:
- **Order Fulfillment to Notification:** When the *Operations Department* successfully processes a new order or updates fulfillment status (e.g., shipped), it emits an `OrderFulfilled` event. The *Customer Success Department* intercepts this and automatically sends a personalized confirmation or tracking update to the customer.
- **Sales to Operations:** When the *Sales Department* successfully closes a custom quote, it emits a `QuoteAccepted` event. The *Operations Department* catches this to automatically block time on the calendar and generate an invoice.

### Usage Budgeting & Throttling (Per Tenant)

To protect platform resources and align with the multi-tenant SaaS tier limits, AI usage is metered at the tenant level:
- **Token/Action Counters:** Every agent action (LLM call, API request) is recorded against a monthly tenant quota.
- **Graceful Degradation:** When a tenant reaches 90% of their limit, the system alerts the owner with an upgrade CTA. At 100%, non-critical proactive agents (e.g., Marketing, Advisory) pause. Critical reactive agents (e.g., Customer Success handling inbound messages) fall back to simple auto-responders or strict "Draft Mode" without full LLM generation.
- **Throttling:** High-frequency events from a single tenant (e.g., a sudden viral spike in messages) trigger backoff queues to prevent noisy-neighbor issues on the shared AI orchestration hub.

### Key Design Decisions

1. **Department Metaphor:** Using friendly, relatable names (The Manager, The Promoter) instead of technical terms ("Order Processing Agent", "Marketing Agent") to align with the "Grandmother Test".
2. **Event-Driven Triggers:** Agents react to business events (new order, new message) rather than relying solely on scheduled polling, ensuring timely responses.
3. **Approval Queue:** Crucial for building trust. Users can start with agents in "Draft Mode" and switch to "Auto-Execute" as they gain confidence.
4. **Shared Context (Tenant DB):** All agents access the same underlying data, preventing contradictory actions (e.g., Sales offering a discount on an out-of-stock item).

## Implementation Prompt

**Task for Implementer:**
Implement the "Customer Success (The Ambassador)" department workflow.

**User-Facing Outcome:**
When a customer sends a message (e.g., "Where is my order?"), the Ambassador agent should automatically retrieve the customer's order history, draft a polite, context-aware reply, and either send it automatically or place it in the owner's "Needs Review" queue, based on the owner's autonomy settings.

**CUJ (Critical User Journey):**
1. Customer submits an inquiry via the OHC storefront contact form.
2. The Ambassador agent receives the inquiry.
3. The agent queries the order database for the customer's recent activity.
4. The agent drafts a response.
5. If autonomy is set to "Draft Mode", the owner sees a notification, reviews the draft, and approves it.
6. If autonomy is "Auto-Execute", the response is sent immediately.

**Acceptance Criteria:**
- Agent correctly identifies the customer and retrieves relevant order data.
- Response tone is professional and aligns with the business's profile.
- Autonomy settings (Draft vs. Auto) are respected.
- All actions are logged for the business owner to review.
- The UI for reviewing drafts meets the "Grandmother Test" and is fully functional on mobile (375px).

## Priority
P0 (Critical path for the OHC vision)

## Estimated Scope
Large
