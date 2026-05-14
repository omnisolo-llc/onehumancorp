**Title**: AI Agent Department Architecture for OneHumanCorp

**Problem Statement**: Small business owners like Maya, Carlos, Priya, Leo, and Fatima operate without large teams. They face immense cognitive overhead managing operations, marketing, sales, customer success, finances, legal compliance, and strategic planning. Existing platforms provide tools but require the owner to execute the work. OHC needs a comprehensive architectural design for autonomous AI departments that operate invisibly, offloading this complexity and allowing anyone to run a business from their phone in under 10 minutes without manuals or code.

**Research Report**:
### Executive Summary
Our research indicates a massive gap in the SMB software market: small business owners don't want software; they want outcomes. By structuring AI agents as familiar business departments, OHC bridges the gap between complex orchestration and intuitive user experience. This report details the functional domains, triggers, and memory architectures for seven distinct AI departments.

### Competitive Analysis
1. **Shopify**: Offers AI features (Shopify Magic) but they are siloed tools (e.g., product description generation). There is no holistic, autonomous orchestration acting on the user's behalf.
2. **Wix/Squarespace**: AI is limited to initial site generation. Operational tasks still require manual intervention.
3. **GoDaddy**: Basic AI prompt-to-text. No proactive advisory or cross-departmental communication.
4. **OHC Advantage**: True autonomous agents that communicate with each other via a unified event mesh, operating continuously in the background.

### Persona Pain Points & AI Solutions
#### Maya (baker, 28)
- **Current Struggle**: Needs beautiful storefront with photo catalog, deposit-based custom orders, AI agent that replies to Instagram DMs like 'do you do vegan cakes?' while she sleeps. Runs everything from iPhone.
- **AI Solution**: The Marketing Agent creates a visually appealing catalog automatically. The Operations Agent handles custom orders and deposits. The Customer Success Agent handles Instagram DMs overnight, integrating with Operations to check vegan cake availability.

#### Carlos (handyman, 42)
- **Current Struggle**: Needs service listings with prices, booking calendar with deposit payments, customer inbox, AI quote generator. Android phone only.
- **AI Solution**: The Sales Agent generates comprehensive quotes based on simple inputs. The Operations Agent integrates bookings into his calendar, handling deposits autonomously. The UI provides a mobile-first, consolidated inbox.

#### Priya (boutique owner, 35)
- **Current Struggle**: Needs storefront + inventory sync, product variants (size/color), in-person tap-to-pay, email newsletter, daily mobile analytics.
- **AI Solution**: The Marketing Agent handles email newsletters and product variant management. The Advisory Agent provides actionable daily mobile analytics.

#### Leo (music tutor, 22)
- **Current Struggle**: Needs lesson booking with calendar sync, auto-generated meeting links, subscription lesson packages, AI follow-up for inactive students, portfolio page for TikTok link-in-bio.
- **AI Solution**: Operations handles scheduling and meeting links. Customer Success proactively follows up with inactive students based on retention schedules.

#### Fatima (food cart, 50)
- **Current Struggle**: Needs photo menu with sold-out toggles, pre-order/pickup with payment, phone notification on new order, printable daily order list, Arabic + English UI, works on low-end Android.
- **AI Solution**: The platform provides a low-bandwidth, accessible interface. Operations manages pre-orders and pickup notifications. The AI inherently understands and translates across languages.

### Deep Dive: Department Details
1. **Operations ('The Manager')**: Handles day-to-day execution. Triggered by order events and inventory changes. Remembers historical fulfillment times. Uses draft-for-review on refunds, auto-executes internal status updates.
2. **Marketing & Advertising ('The Promoter')**: Gets the business noticed. Triggered by on-demand updates or scheduled campaigns. Retains engagement data. Uses draft-for-review on all external publications.
3. **Sales & Acquisition ('The Salesperson')**: Turns interest into revenue. Triggered by new inquiries. Memorizes quote conversions. Uses draft-for-review on quotes.
4. **Customer Success ('The Ambassador')**: Keeps customers happy. Triggered by order shipments and reviews. Remembers customer preferences. Uses draft-for-review on sensitive replies, auto-executes simple confirmations.
5. **Finance & Payments ('The Accountant')**: Ensures correct money flow. Triggered by payments and end-of-month cycles. Tracks revenue trends. Auto-executes standard receipts, draft-for-review on tax reports.
6. **Legal & Compliance ('The Protector')**: Keeps the business safe. Triggered by new services or jurisdictions. Remembers past templates. Draft-for-review on all legal documents.
7. **Business Advisory ('The Advisor')**: Acts as a consultant. Triggered weekly or by anomalies. Remembers long-term goals. Auto-executes report generation.

**Design Doc**:
### High-Level Architecture Diagrams
```mermaid
sequenceDiagram
    participant O as KAIROS Orchestrator
    participant Hub as Teammate Mesh
    participant Op as Operations Agent
    participant CS as Customer Success Agent
    participant Fin as Finance Agent
    participant DB as Memory Store

    O->>Hub: New Order Event
    Hub->>Op: Trigger: Process Order
    Op->>DB: Fetch Context
    DB-->>Op: Context Valid
    Op->>Hub: Order Processed
    Hub->>Fin: Trigger: Track Payment
    Fin->>DB: Record Deposit
    Hub->>CS: Trigger: Send Confirmation
    CS->>DB: Fetch Customer Profile
    DB-->>CS: Profile (Preferences)
    CS->>Hub: Draft Email for Review

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class O,Hub,Op,CS,Fin,DB premium;
```

### Core Architectural Decisions and Why
#### Event-Driven Coordination
Agents react to domain events over a unified event bus rather than direct synchronous calls. This decoupling ensures the system can withstand failures and prevents cascading timeouts. It naturally enables the 'Draft-for-Review' workflow by allowing agents to emit pending actions as events.

#### 1-Tap Handoff Triggers
To deliver the '1-Tap Approval' experience, coordination patterns are enforced. For instance, when 'The Manager' fulfills an order, it emits an event. 'The Ambassador' intercepts this to draft a personalized notification. The business owner simply taps 'Approve' to finalize the chain.

### Memory & Context
Agents utilize a semantic, context-aware memory model. This ensures that historical interactions, seasonal trends, and specific customer preferences are automatically injected into the agent's decision-making process without requiring the user to constantly repeat instructions.

### Approval Workflows
Actions are categorized by risk. Low-risk actions (updating internal tags) are Auto-Execute. High-risk actions (sending customer emails, processing refunds) are Draft-for-Review, requiring explicit 1-tap approval via the mobile dashboard.

### Tier-Based Usage & Throttling
Agent activity is managed by the multi-tenant SaaS tier. Hard limits on monthly actions exist for lower tiers, and rate limiting is applied universally to prevent noisy-neighbor issues and ensure fair resource allocation.

### Mobile-First UX
All agent interactions are designed for a 375px mobile breakpoint. Action items are summarized in plain, jargon-free language. Optimistic UI updates are employed so the dashboard feels instantaneously responsive.

### Security & Multi-Tenancy
Strict data isolation boundaries guarantee that agents can only ever access context and perform actions within the scope of their assigned business entity.

**Implementation Prompt**:
You are an L5 Implementer agent. Your task is to design and implement the underlying Orchestrator logic to support the 7 distinct AI departments. Create the event routing and subscription logic to allow departments to react to domain events. Implement the 'Draft-for-Review' schema to manage pending high-risk actions awaiting user approval. Ensure all output models are structured for a 375px mobile UI consumption with plain-language summaries. Do not define specific database technologies or external provider APIs; focus purely on the robust trait definitions, domain events, and state transitions within the application layer.

**Priority**: P0

**Estimated Scope**: Large

### System Execution and Agent Lifecycle
To deliver on the promise of an invisible, autonomous business manager, the AI Agent Department Architecture operates on a highly optimized lifecycle.

#### The Agent Control Loop
1.  **Event Ingestion**: External stimuli (e.g., a Stripe webhook, a new email, a cron tick) enter the KAIROS Orchestrator via the API Gateway.
2.  **Context Assembly**: Before routing to an agent, the Orchestrator builds a comprehensive context object. This is critical for the "Grandmother Test," as it prevents the agent from asking the user for information the system already possesses. It queries the vector database for relevant past interactions and the relational database for current state (e.g., inventory levels).
3.  **Task Decomposition**: Complex events are broken down. An "Order Placed" event might spawn a fulfillment task for Operations and a thank-you draft task for Customer Success.
4.  **Agent Invocation**: The specific department agent is invoked. The system uses a specialized prompt structure that injects the assembled context and enforces the department's specific constraints and "personality."
5.  **Output Validation**: The agent's response is strictly validated against a protobuf schema. If an agent hallucinates a malformed action, it is rejected and retried with a higher temperature or routed to a fallback deterministic path.
6.  **Action Execution or Staging**: Validated outputs are either executed immediately (if low risk) or serialized into the `ApprovalDraft` table for user review.

#### Handling Asynchronous Long-Running Tasks
Some tasks, like generating a complex monthly financial report, exceed typical API timeout thresholds.
*   **Durable Queues**: These tasks are offloaded to a durable queue.
*   **Progress Indicators**: The mobile UI is updated with a non-blocking progress state ("The Accountant is crunching your numbers...").
*   **Completion Callback**: Upon completion, a server-sent event (SSE) or push notification alerts the business owner that the report is ready.

#### Resilience and Fault Tolerance
Small businesses operate on thin margins; system downtime translates directly to lost revenue.
*   **Idempotency**: All agent actions are designed to be idempotent. If the network drops while Carlos is approving a quote and the request is retried, the system guarantees the quote is only sent once.
*   **Circuit Breakers**: If the underlying LLM provider experiences an outage, the Orchestrator trips a circuit breaker. Non-critical tasks (like generating SEO tags) are paused, while critical tasks (like checkout processing) fall back to deterministic, non-AI logic to ensure business continuity.
*   **Data Consistency**: State mutations are wrapped in database transactions, ensuring that partial failures do not corrupt the business's operational state.

### Conclusion
The AI Agent Department Architecture transforms the complex orchestration of a small business into a series of simple, 1-tap decisions. By aligning technical boundaries with understandable human roles (The Manager, The Salesperson, The Accountant) and enforcing strict data isolation and approval workflows, OHC empowers non-technical users to leverage sophisticated automation safely and effectively. This design fulfills the primary product vision: allowing anyone to run a real business in under 10 minutes without manuals or code.
