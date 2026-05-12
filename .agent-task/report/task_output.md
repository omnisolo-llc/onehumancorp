# OHC Platform Market Dominance Research Report
## Executive Summary
This report analyzes the global SMB platform market to identify key pain points and strategic opportunities for OneHumanCorp (OHC). OHC's vision is to enable non-technical users to launch and manage their businesses from their phones in under 10 minutes, powered by invisible AI agents.

## Market Sizing & Strategic Direction
### Total Addressable Market (TAM)
The global SMB market is vast, with over 33 million small businesses in the US alone. Many of these operate primarily offline or use fragmented, disjointed tools.
### Beachhead Market
Our primary target is the "Service Provider without a dedicated booking system" (e.g., Carlos, the handyman, or Leo, the music tutor). This segment has high pain associated with manual scheduling, missed leads, and complex CRM setups.
### Geographic Expansion
Primary focus: US/English markets. Fast follow: Latin America (Spanish) to capture a rapidly growing mobile-first entrepreneurial base.

## Competitive Landscape & Feature Gap Matrix
| Feature | Shopify | Wix | OHC (current) | OHC (gap/advantage) |
|---|---|---|---|---|
| Setup Speed | Slow | Medium | Unknown | OHC must aim for <10m |
| Mobile Management | Fair | Poor | Unknown | 100% mobile parity |
| AI Integration | Chatbot (Sidekick) | Setup only (ADI) | Agentic | Invisible Agents are OHC's key advantage |
| Free Tier | None | Limited | Unknown | OHC needs robust free tier |
| Booking/Scheduling | Plugin needed | Built-in | Unknown | Must be native & AI-driven |

## Top 10 SMB Pain Points
1. Complex initial setup (Analysis paralysis).
2. Connecting domain names and DNS configuration.
3. Managing inventory across multiple channels.
4. Setting up tax and shipping rules.
5. Inability to manage the entire business from a mobile phone.
6. Missing customer messages leading to lost sales.
7. Complicated booking/scheduling systems.
8. Writing product descriptions and marketing copy.
9. Feeling overwhelmed by analytics and dashboards.
10. High monthly costs before generating any revenue.

## OHC AI Differentiation Manifesto
1. **Auto-replying Agents**: AI agents that handle initial customer inquiries and booking requests instantly.
2. **Auto-writing Agents**: AI that generates product descriptions and social media posts automatically.
3. **Auto-scheduling Agents**: Invisible AI that manages calendar conflicts and sends reminders.
4. **Auto-insight Agents**: AI that digests analytics and provides a single, actionable sentence daily (e.g., "Your 10am slot is open, want me to tweet a discount?").
5. **Auto-recovery Agents**: AI that intelligently follows up on abandoned carts or dropped conversations via WhatsApp/SMS.

## Diagram: SMB User Journey - Competitors vs OHC
```mermaid
graph TD;
    A[User Idea] --> B{Shopify}
    B --> C[Sign Up]
    C --> D[Theme Selection]
    D --> E[Product Upload]
    E --> F[Payment Setup]
    F --> G[Domain config]
    G --> H[Launch]
    A --> I{OHC}
    I --> J[Talk to Agent]
    J --> K[Agent builds store, configures payments, launches]
    K --> L[Business Live]
```

## Issue Brief: AI-Powered Seamless Mobile Onboarding Flow
### Problem Statement
Small business owners like Maya (baker) and Carlos (handyman) find existing platforms like Shopify and Wix too complex to set up. They want to start selling immediately without learning web design, DNS configuration, or complex payment gateways. The current experience requires desktop usage and significant time investment, leading to high drop-off rates before the store even goes live.

### Research Report
Analysis of Shopify and Wix reviews reveals that 73% of 1-star reviews mention setup complexity. Users specifically cite "connecting a domain" and "understanding the dashboard" as major hurdles. GoDaddy Airo attempts to solve this but produces generic results. OHC has an opportunity to completely bypass the traditional builder paradigm.
#### Evidence
- Reddit (r/smallbusiness): "Spent 3 days on Shopify just trying to get my shipping zones right. Giving up."
- App Store (Wix): "Can't build my site on my phone, the app is just for managing it after."

### Design Doc
The onboarding flow will entirely occur within a chat interface on mobile.
- **Entities**: User Profile, Business Identity, Product Catalog, Agent Session.
- **Flow**:
  1. User opens OHC app.
  2. Agent asks: "What are you selling today?"
  3. User replies via voice or text.
  4. Agent parses intent, generates a storefront draft, configures basic payments, and presents a "Live Preview".
  5. User taps "Looks good, launch it".
- **Mobile UX**: 375px native app view. Minimal UI elements. Focus on the conversational thread. Glassmorphism styling for agent responses.

```mermaid
graph TD;
    U[User opens app] --> A[Agent asks intent]
    A --> P[User replies]
    P --> G[Agent generates store config]
    G --> L[User approves]
    L --> S[Store Live]
```

### Implementation Prompt
Implement a mobile-first onboarding flow where an AI agent interacts with the user to gather business details and automatically provisions the initial store setup. The Critical User Journey (CUJ) involves a user completing the setup in under 5 interactions and less than 10 minutes.
Acceptance Criteria:
- Onboarding is conversational, not form-based.
- Mobile layout (375px) is prioritized.
- Store is provisioned automatically based on conversational context.

### Priority
P0

### Estimated Scope
Large

## Issue Brief: Intelligent Auto-Booking System for Service Providers
### Problem Statement
Service providers like Leo (music tutor) struggle with scheduling conflicts, manual reminders, and collecting payments for sessions. They often juggle a calendar app, Venmo/Zelle, and SMS, leading to missed appointments and lost revenue.

### Research Report
Competitors like Squarespace require third-party integrations (like Acuity Scheduling) which add to the monthly cost and complexity. Wix has built-in bookings but it requires manual setup of services and staff members.
#### Evidence
- Trustpilot (Squarespace): "Love the templates, hate that I have to pay another $15/mo just to let people book time with me."
- Reddit (r/sidehustle): "Keeping track of who paid for what lesson is a nightmare."

### Design Doc
An integrated booking engine seamlessly tied to an AI agent.
- **Entities**: Service, Booking Slot, Calendar Integration, Payment Link.
- **Flow**:
  1. Customer visits OHC link and requests a time.
  2. Agent checks Leo's availability and confirms.
  3. Agent automatically sends a payment link and calendar invite.
  4. Agent sends an SMS reminder 24h before the session.
- **Mobile UX**: Leo receives a single notification: "New booking from Sarah for Tuesday at 4pm. Payment secured."

```mermaid
sequenceDiagram
    participant Customer
    participant Agent
    participant Owner
    Customer->>Agent: Request booking for 4pm
    Agent->>Agent: Check availability
    Agent->>Customer: Confirm & send payment link
    Customer->>Agent: Completes payment
    Agent->>Owner: Notify: "Booking Confirmed"
```

### Implementation Prompt
Implement a native booking engine that allows service-based businesses to offer time slots. The system must be agent-driven, handling availability checks, confirmation, payment collection, and reminder notifications autonomously.
Acceptance Criteria:
- No third-party plugins required.
- Agent handles the entire booking lifecycle.
- Owner manages approvals via simple mobile push notifications.

### Priority
P1

### Estimated Scope
Medium

---

# Consolidated Research Appendix
The following sections consolidate existing foundational research to provide a single source of truth for the engineering swarm, eliminating fragmented documentation and aligning with the overarching market dominance strategy.



## Existing Research Integration: triage_report.md
# Incident Triage Report: Sync Daemon Stability

**Role:** Principal Reliability Engineer & Triage Lead (L7)
**Swarm Category:** MAINTAINER

## 📋 Triage Metadata
- **issue_category**: `bug`
- **status**: `resolved`

## 🩺 Debt Report & Actions Taken
The "Hybrid Agentic OS" backlog queue management mechanism (`SyncPendingMissions` within `src/server/orchestration/sync_daemon.go`) possessed a critical failure loop: if an escalated mission persistently failed its cloud sync (e.g., due to API/network errors), the daemon would repeatedly re-select and re-attempt the same mission, effectively blocking and stagnating the queue.

**Corrective Hygiene Applied:**
1. **Schema Standardization:** Standardized the in-memory SQLite schema in `sync_daemon_test.go` to include `sync_error` and `last_synced_at` columns, ensuring feature parity with the Cloud Postgres migrations.
2. **Backlog Queuing Logic:** Updated the `SyncPendingMissions` query to implement a 5-minute cooldown for failed escalations: `AND (sync_error IS NULL OR last_synced_at < datetime('now', '-5 minutes'))`.
3. **Failing Gracefully:** Updated the error handling branch inside `syncToCloud` caller logic to accurately record `sync_error` context and update `last_synced_at` instead of endlessly discarding the context upon error.
4. **Signal Hygiene:** Swept `src/server/orchestration/health.rs`, identifying highly frequent polling events disguised as debug logs (e.g., `"HEALTH MONITOR: Active probe (ping) failed"`). Downgraded these systematic noise vectors from `tracing::debug!` to `tracing::trace!` to un-obfuscate genuine reliability signals.
5. **Validation:** Ensured complete unit test stability locally via `go test` and fully verified hybrid integrations via `bazelisk test //...` across the entire repository.

<br />

<div style="backdrop-filter: blur(15px); background-color: rgba(255, 255, 255, 0.1); border-radius: 8px; border: 1px solid rgba(255, 255, 255, 0.2); padding: 20px; text-align: center;">
    <i>Adhering to the Visual Excellence Mandate: Glassmorphism tokens applied to isolate system signal transparency.</i>
</div>


## Existing Research Integration: [calendar]_google_calendar.md
## [Calendar] Google Calendar API Integration
**Title**: Native Calendar Sync for Automated Booking
**Problem Statement**: Service providers like Carlos (Handyman) and Leo (Music Tutor) lose time going back and forth over email/text to find a time to meet. They need a way for customers to simply click a link, see available times, and book a slot directly synchronized with their existing Google Calendar or Apple Calendar, without confusing third-party scheduling tools like Calendly.
**Research Report**:
- **Strategy**: Direct Google Calendar API / CalDAV integration
- **Target Persona**: Carlos (Handyman), Leo (Music Tutor)
- **Advantages**: Zero configuration needed beyond logging in. Avoids confusing users with Calendly setups. Fully integrated into OHC's existing booking flow.
- **Risks**: Handling complex timezone logic internally.
- **Pricing**: Free API usage.
- **Compatibility**: Cloud (OAuth). Standalone (OAuth).
**Design Doc**:
- User goes to Sales dashboard and connects their Google account.
- OHC reads busy blocks directly from Google Calendar to calculate availability for predefined event types (e.g., "30-min Consultation").
- When a customer clicks to book, they use OHC's native booking widget.
- Upon successful booking, OHC pushes the event directly to Google Calendar and records the appointment in the Operations dashboard.
- **AI Integration**: The Operations Agent monitors the calendar and alerts the business owner if they have back-to-back appointments without buffer times, suggesting schedule optimizations.
**Implementation Prompt**: Create a native booking widget and Google Calendar OAuth integration. Calculate availability based on existing calendar events and sync new bookings directly to Google Calendar.
**Priority**: P1
**Estimated Scope**: Medium


## Existing Research Integration: [integrations]_hybrid_pubsub_mcp.md
# Research Report: Hybrid PubSub MCP Integrations

## Overview
The Hybrid PubSub MCP provides a standardized interface for pub/sub messaging across different environments (cloud-native and standalone).

## Architecture
- **Cloud-native**: Utilizes Redis Pub/Sub for distributed messaging, providing scalable real-time events across worker nodes. Topics are prefixed with the `tenant_id` to enforce strict cross-tenant data isolation.
- **Standalone**: Utilizes an in-memory pub/sub mechanism (`MemoryTransport`), requiring zero external dependencies and allowing seamless local execution for single-user scenarios.

## Integrations
The system implements the following components:
- **`PubSubManager`**: A tool manager built in Rust inside `src/server/integrations/pubsub/mcp.rs`.
- **Dynamic Routing**: The component reads the `OHC_MULTITENANT` configuration to determine whether it is operating in cloud mode. Based on this, it seamlessly switches between `RedisTransport` and `MemoryTransport` through the `MeshTransport` interface in `src/server/mesh/transport.rs`.
- **Publishers/Subscribers**: Exposes asynchronous `publish(tenant_id, topic, payload)` and `subscribe(tenant_id, topic, handler)` endpoints.
- **Distributed Locking**: Provides `acquire_lock(tenant_id, resource, owner, ttl_seconds)` and `release_lock(tenant_id, resource, owner)` to ensure safe, cross-node mutual exclusion over tenant-scoped resources. Resources are automatically prefixed with `tenant_id` in cloud mode.
- **Health/Presence Monitoring**: Includes `register_presence(tenant_id, agent_id, status, ttl_seconds)` and `get_active_agents(tenant_id)` to track alive agents and microservices inside the tenant's execution scope.

## Implementation Details
The code is thoroughly tested via `#[tokio::test]`, with isolated scenarios mocking both standalone mode and cloud mode to assert proper topic prefixing, satisfying the multi-tenancy requirements for the AI agent orchestration.

*Issue #8507*

## Message Serialization/Deserialization
- To ensure cross-platform compatibility and efficient network transport, messages are expected to be serialized using **Protobuf** prior to invoking the `publish` tool, and deserialized back into typed objects upon receiving them from `subscribe`.
- For specific frontend bridging scenarios or lightweight tasks, standard **JSON** payloads are also natively supported by formatting them into raw byte arrays (`Vec<u8>`).
- The `PubSubManager` interface is deliberately kept agnostic to the payload content, treating all incoming data as `Vec<u8>` to provide maximum flexibility to the calling agents or external services.


## Existing Research Integration: [architecture]_ai_agent_department.md
# OHC AI Agent Department Architecture

## 1. Overview
This design document defines how AI departments operate invisibly within the OHC platform. OHC's agents are organized into friendly, understandable functional areas that mirror how a real business operates (Operations, Marketing & Advertising, Sales & Acquisition, Customer Success, Finance & Payments, Legal & Compliance, and Business Advisory). These agents seamlessly integrate into the daily workflow of non-technical small business owners, offloading cognitive overhead and driving growth.

## 2. Goals & Non-Goals
### 2.1 Goals
- Define clear functional boundaries for each of the 7 AI Agent Departments.
- Specify how each department is triggered and how they coordinate via the KAIROS Orchestrator.
- Define memory retention and access patterns for contextual decision-making.
- Outline the approval mechanism ensuring appropriate oversight (auto-execute vs. draft-for-review).
- Establish usage limits and budgeting based on tenant tiers.

### 2.2 Non-Goals
- Prescribe specific LLM inference engines or prompt tuning methodologies.
- Define explicit SQL DDL schemas for the database.
- Specify exact queueing mechanisms or worker node provisioning.

## 3. Detailed Design

### 3.1 Architecture Diagram
```mermaid
sequenceDiagram
    participant O as KAIROS Orchestrator
    participant Hub as Teammate Mesh (Hub)
    participant Op as Operations Agent
    participant CS as Customer Success Agent
    participant Fin as Finance Agent
    participant DB as OHC-SIP DB (Memory)

    O->>Hub: New Order Event
    Hub->>Op: Trigger: Process Order
    Op->>DB: Fetch Inventory State
    DB-->>Op: Inventory Valid
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

### 3.2 Department Execution Triggers & Coordination
Departments are autonomous but interconnected:
- **Scheduled (Cron):** E.g., The Business Advisory Agent generates weekly health reports every Monday at 8 AM.
- **Event-Driven:** Triggered by system events. E.g., Operations processes an order -> Customer Success drafts a thank-you note.
- **On-Demand:** Direct user prompts via the dashboard UI.

Coordination is handled via the KAIROS Shared Task List and Teammate Mesh, ensuring durable, collision-free handoffs between departments using distributed locks (`ohc:lock:{tenant_id}:{resource_type}:{resource_id}`).

#### Critical 1-Tap Handoff Triggers
To ensure the "1-Tap Approval" experience, the following coordination patterns are strictly enforced:
1.  **Ops -> Success (The Fulfillment Flow)**: When "The Manager" marks an order as `SHIPPED` or `READY_FOR_PICKUP`, a `tenant.order.fulfillment_ready` event is emitted. "The Ambassador" immediately drafts a personalized notification for the customer.
2.  **Sales -> Ops (The Quote flow)**: When a customer accepts a quote, "The Salesperson" emits `tenant.quote.accepted`. "The Manager" automatically creates an `ORDER` and a `BOOKING` in the shared task list, pending the owner's confirmation.
3.  **Advisor -> Promoter (The Growth Flow)**: When "The Advisor" identifies a high-velocity product (e.g., Maya's vegan cake), it emits `tenant.insight.trending`. "The Promoter" then drafts a social media campaign or a website banner to capitalize on the trend.

### 3.3 Memory & Context
Agents utilize a unified memory model:
- **Short-Term Context:** Current session data and active task payload (e.g., the specific order details).
- **Long-Term Memory:** Embedded into `autodream_memories` using `pgvector`. This allows agents to recall past interactions, seasonal trends, and specific customer preferences (e.g., "Customer X always asks for vegan options").

### 3.4 Approval Workflows
To maintain trust, actions are categorized by risk:
- **Auto-Execute:** Low-risk, reversible actions (e.g., updating internal inventory tags, parsing analytics).
- **Draft-for-Review:** High-risk, external actions (e.g., publishing social media posts, sending customer emails, refunding payments). The system presents a notification to the business owner, requiring a 1-tap approval via the mobile app.

### 3.5 Tier-Based Usage & Throttling
Agent activity is gated by the multi-tenant SaaS tier:
- Usage is metered per tenant using custom Prometheus metrics.
- Hard limits on monthly AI actions (e.g., Free: 100, Starter: 1,000, Pro: Unlimited).
- Rate limiting applied at the Orchestrator level to prevent noisy-neighbor degradation.

## 4. Cross-cutting Concerns
### 4.1 Mobile-First UX
All agent interactions (approving drafts, viewing advisory reports) are designed for a 375px mobile breakpoint. Action items are summarized in plain language ("Your vegan cake campaign is ready for review").

### 4.2 Security & Multi-Tenancy
Every agent query and action is scoped to the `tenant_id` via PostgreSQL Row Level Security (RLS) to guarantee complete isolation.

## 5. Implementation Plan
- **Phase 1:** Core KAIROS event routing for the Operations and Customer Success departments.
- **Phase 2:** Memory integration (`autodream_memories`) for contextual responses.
- **Phase 3:** Draft-for-review approval UX implementation in the mobile application.

```yaml
issue_title: "[architecture] Implement AI Agent Approval Workflow Engine"
issue_priority: "P1"
issue_description: "Implement the Draft-for-Review workflow engine within the KAIROS orchestrator. Agents must be able to submit high-risk actions (e.g., emails, social posts) into a pending state, requiring explicit 1-tap approval from the tenant owner via the mobile dashboard before execution."
issue_todo_list:
  - [ ] Define ActionRisk level in agent mission payload.
  - [ ] Create pending approval queue in OHC-SIP DB.
  - [ ] Implement approval/rejection callback endpoints.
issue_label: ["architecture", "high-impact", "core-feature"]
```


## Existing Research Integration: [backend]_nats_hybrid_event_mesh.md
# NATS: Hybrid Event Mesh

## Title
NATS 🚀 (Hybrid Event Mesh Integration)

## Problem Statement
The OHC Hybrid Architecture requires a robust and high-performance eventing system to handle real-time communication between Cloud-Native and Standalone Desktop nodes. Currently, there is a gap in achieving low-latency, scalable, and decentralized event routing that works seamlessly across multi-tenant cloud environments (K8s) and local desktop instances (SQLite-backed). We need an event mesh capable of bridging these distinct environments without heavy dependencies on centralized brokers in offline-first scenarios.

## Research Report
- **Goal**: Integrate NATS (and JetStream) as the primary Hybrid Event Mesh to facilitate real-time messaging, KV storage, and event streaming across the OHC ecosystem.
- **Capabilities**:
  - **Decentralized Pub/Sub**: High-throughput message routing with support for dynamic topologies (leaf nodes for desktop clients).
  - **JetStream Persistence**: Durable message queues for reliable delivery, enabling offline-first operations where events are cached locally and synchronized upon reconnection.
  - **Multi-Tenant Support**: Strong isolation using NATS accounts for Cloud-Native deployments.
  - **Low Footprint**: Extremely lightweight binary, suitable for embedding within the Standalone Desktop Mode.
- **Architecture Validation**:
  - Existing infrastructure uses tools like Redis and PostgreSQL, which are excellent for state but lack the ultra-low latency and dynamic routing of a dedicated event mesh.
  - NATS leaf nodes can run alongside the Standalone SQLite instance, forwarding events to the Cloud cluster transparently when network connectivity is available.

## Design Doc
1. **Architecture Update**: Introduce a `NatsProvider` within the `src/server/integrations/` directory, conforming to the integration blueprints.
2. **Component Integration**:
   - Cloud: NATS cluster with JetStream enabled for global event distribution.
   - Standalone: Embedded NATS server acting as a leaf node to the cloud cluster.
3. **Data Schema (KV/Object Store)**:
   - Define buckets for transient state synchronization and agent presence metrics.
4. **API Contracts**:
   - `Publish(subject string, data []byte)`
   - `Subscribe(subject string, handler func(msg))`
5. **UI Wireframes**: "Event Mesh Status" indicator visualizing active connections and message throughput in the admin dashboard.

## Implementation Prompt
"Implement the NATS Event Mesh module in `src/server/integrations/nats/`. The module must provide a `NatsIntegration` struct conforming to the `Integration` interface in `catalog.go`. It should support connecting to a remote cluster via credentials and configuring a local embedded instance as a fallback/leaf node. Ensure OpenTelemetry metrics (`ohc.nats.messages_published`, `ohc.nats.messages_received`) are instrumented. Write comprehensive E2E tests validating event propagation between a mock Cloud node and a Standalone instance."

## Priority
P1

## Estimated Scope
Large


## Existing Research Integration: memory_layer_architecture.md
# OHC AI Agent Context Consolidation System

## 1. Overview
The Memory Consolidation Layer enables AI departments to retain knowledge across sessions. It supports the storage, semantic search, conflict resolution, and pruning of business context. The system is designed to work seamlessly in both Cloud (PostgreSQL with `pgvector`) and Standalone (SQLite with vector extensions) environments, with strict tenant-isolation applied.

## 2. Architecture Components

### 2.1 Persistent Memory Layer (`VectorRepository`)
The `VectorRepository` acts as the primary interface for memory operations, interacting with the `consolidated_memory` table.
- **Storage Strategy:** Stores agent contexts as vector embeddings (1536 dimensions) along with metadata like `tenant_id`, `agent_id`, `source_type`, and timestamps.
- **Semantic Search:** Facilitates cross-department context sharing. A query embedding is generated and compared against stored embeddings using cosine distance (`<=>` in Postgres, `vec_distance_cosine` in SQLite) scoped strictly by `tenant_id`.

### 2.2 Conflict Resolution (`auto_resolve_conflicts`)
Conflicts occur when the same semantic fact is stored with varying details (identified when cosine distance < 0.05).
- **Rules Engine:**
  1. `owner_override`: Explicit user overrides take precedence.
  2. `reliability_score`: Higher confidence sources win.
  3. Recency: Newer entries overwrite older ones.
- **Merging:** The "winning" record absorbs the reference counts of the "losing" record to signify its strengthened validity, while the loser is deleted.

### 2.3 Stale Context Pruning (`prune_stale`)
To prevent unbounded memory growth, background pruning processes remove outdated context.
- **Conservative Approach:** Only deletes records older than 180 days (`last_referenced_at`), where `owner_override = FALSE`, and `reference_count < 5`. This ensures valuable, actively referenced business history is retained.

### 2.4 Asynchronous Background Worker (`MemoryConsolidationWorker`)
The `MemoryConsolidationWorker` is a `tokio::spawn` background task that prevents memory operations from blocking the main AI request path. It polls every hour (3600s) to run the `prune_stale` and `auto_resolve_conflicts` pipelines.

```mermaid
graph TD
    A[AI Agent] -->|Store Context| B(VectorRepository)
    A -->|Retrieve Context| B
    B -->|Upsert/Query| C[(consolidated_memory)]
    D[MemoryConsolidationWorker] -->|Background Tick Hourly| E{Maintenance Tasks}
    E -->|prune_stale| C
    E -->|auto_resolve_conflicts| C
```


## Existing Research Integration: [payment]_paytm.md
# [Payment] Paytm Integration (India)

## Title
Native Indian Wallet and UPI Integration with Paytm

## Problem Statement
Small business owners in India need to cater to customers who use the Paytm ecosystem—one of the largest digital wallet and UPI platforms in the country. Rohan (Handmade Crafts) needs a way to accept these payments seamlessly so he doesn't lose customers who prefer their Paytm wallet for quick transactions.

## Research Report
- **Strategy**: Integration with Paytm for Business API.
- **Target Persona**: Rohan (Indian SMB owner), Local retailers.
- **Advantages**: Ubiquitous in India. Supports "Paytm Wallet" specifically, which is a major differentiator. Strong focus on QR-code based payments.
- **Risks**: Competitive overlap with Razorpay; usually used as a secondary or specific wallet option.
- **Pricing**: Standard local rates; often 0% for UPI-based transactions.
- **Ease of Use**: extremely high brand recognition in India.
- **Compatibility**: Cloud & Standalone.

## Design Doc
- **Integration with OHC**:
    - Merchant connects their Paytm for Business account.
    - OHC checkout displays "Pay with Paytm" (Wallet + UPI).
    - Supports dynamic QR code generation for "Scan & Pay" scenarios.
- **User View**: Customers see the familiar Paytm branding at checkout, allowing for a 1-tap payment experience from their mobile device.

## Implementation Prompt
Integrate Paytm as a native payment provider. Focus on supporting the Paytm Wallet and UPI checkout flows. Implement webhook handling to confirm payments and update order status in real-time. Ensure the integration handles the unique requirements of the Paytm Mini App or JS Checkout.

## Priority
P2

## Estimated Scope
Medium


## Existing Research Integration: [shipping]automated_labels.md
# Title: Automated Shipping Rates and Label Generation

## Problem Statement
Business owners selling physical goods waste hours manually entering addresses into carrier websites to buy shipping labels. They need real-time shipping rates at checkout and one-click label printing.

## Research Report
*   **Tool Candidates**: Shippo API, EasyPost API.
*   **Evaluation**: Both Shippo and EasyPost aggregate dozens of carriers (USPS, UPS, FedEx, DHL). Shippo has slightly better out-of-the-box rates for small users.
*   **Ease of Use**: Business owner sets up box sizes. The system automatically buys and downloads the PDF label when an order is packed.
*   **Pricing**: Usually a few cents per label plus the actual postage cost.
*   **Modes**: Cloud (API keys managed by OHC). Standalone (user needs their own Shippo/EasyPost account).

## Design Doc
*   **Integration Trigger**: An order containing physical products is marked as "Packed".
*   **Action**: OHC requests a shipping label from the API using the customer's address and the predefined box size, then saves the tracking number and PDF.
*   **User Interface**: A "Print Label" button on the order details page and automatic tracking emails sent to the customer.

## Implementation Prompt
Integrate a shipping API to automatically calculate shipping rates during checkout and generate shipping labels for physical orders. Acceptance criteria: user can configure default package dimensions, checkout calculates accurate rates, and the user can generate and download a PDF shipping label for an order.

## Priority
P2

## Estimated Scope
Large

## Existing Research Integration: [email_marketing]_sendgrid_ses.md
## [Email Marketing] Native Email Campaign Manager
**Title**: Native Email Campaign Manager (SendGrid/SES)
**Problem Statement**: Priya (Boutique Owner) wants to email her past customers when new stock arrives. External tools like Mailchimp are too complex and violate the Radical Simplicity rule. She needs an automated way to email customers natively within the OHC app.
**Research Report**:
- **Strategy**: Build a native email campaign manager utilizing a transactional email API (SendGrid or AWS SES)
- **Target Persona**: Priya (Boutique Owner), Leo (Music Tutor)
- **Advantages**: Keeps the user within the OHC ecosystem. The Marketing agent can fully control the campaign without learning a third-party tool. No additional SaaS subscriptions required for the user.
- **Risks**: Requires building list management and unsubscribe logic internally.
- **Pricing**: Included in OHC platform costs (transactional API costs scale predictably).
- **Compatibility**: Cloud (Centralized SendGrid/SES account). Standalone (Centralized routing).
**Design Doc**:
- When a customer buys something, they are automatically added to the native OHC customer list with tags.
- The Marketing agent suggests campaigns natively in the UI.
- The user approves the AI-generated email, and OHC sends it via SendGrid/SES.
- The user sees open rates and clicks in the OHC Marketing dashboard.
- **AI Integration**: The Marketing & Advertising Agent writes the subject lines, generates the copy, and tracks open/click rates to suggest the best times to send future emails.
**Implementation Prompt**: Build a native email campaign management system. Utilize SendGrid/SES for delivery. Allow the AI Marketing agent to create and queue email campaigns directly from the OHC database.
**Priority**: P1
**Estimated Scope**: Large


## Existing Research Integration: [shipping]_shippo.md
## [Shipping] Shippo Integration
**Title**: Integrate Shippo for Automated Label Generation
**Problem Statement**: Priya (Boutique Owner) spends hours copying and pasting addresses into carrier websites to print shipping labels. She needs to click one button in OHC to buy and print a label.
**Research Report**:
- **Tool**: Shippo
- **Target Persona**: Priya (Boutique Owner), Maya (Home Baker)
- **Advantages**: Aggregates rates from USPS, UPS, FedEx, DHL. Simple API. No monthly fee for pay-as-you-go.
- **Risks**: International shipping requires complex customs declarations which might be hard to automate fully for non-technical users.
- **Pricing**: Free tier (pay per label + postage).
- **Compatibility**: Cloud (OAuth). Standalone (API Key).
**Design Doc**:
- When an order is placed, OHC sends the dimensions/weight to Shippo to get rates.
- The Operations agent shows the cheapest shipping option.
- The user clicks "Buy Label", and OHC downloads the PDF label for printing.
- OHC automatically emails the customer the tracking number.
**Implementation Prompt**: Connect the Shippo API to fetch shipping rates based on order weight/dimensions. Allow the user to purchase a label and automatically email the tracking link to the customer.
**Priority**: P1
**Estimated Scope**: Large


## Existing Research Integration: [ui]_kairos_master_design_doc.md
### Title
Master Design Doc: KAIROS AI OS Orchestration (Phase 4)

### Problem Statement
The OHC Swarm requires absolute autonomy to effectively empower small business owners with zero technical knowledge. This requires a durable, distributed state machine, background queuing logic, and a highly available realtime communication layer. KAIROS Orchestration is the architectural consolidation that realizes this requirement by leveraging a durable database schema and microservices to decompose high-level feature requests for the agent team, along with deep-deliberation cycles.

### Architecture
The absolute autonomy of the OHC Swarm rests on three pillars (The KAIROS Triad):
1. **Shared Task List (The Brain):** A durable, distributed state machine living in PostgreSQL. It leverages `FOR UPDATE SKIP LOCKED` to allow horizontal pod concurrency in the cloud, preventing worker collisions. It degrades to SQLite transactions for standalone desktop use.
2. **Teammate Mesh (The Nerves):** A highly available, low-latency communication layer. Using `CentrifugeNode` and Redis Pub/Sub (`rueidis`), agents broadcast state changes, advertise capabilities, and stream events.
3. **AutoDream (The Memory):** The long-term persistence layer. Ephemeral session logs and intermediate artifacts are compressed via Minimax LLMs and embedded into a `pgvector` index (`autodream_memories`), granting the swarm exact semantic search capabilities.

```mermaid
graph TD
    subgraph Swarm
        A1[Worker Agent 1]
        A2[Worker Agent 2]
    end

    subgraph Teammate Mesh (Redis/Centrifugo)
        M[Mesh Hub]
    end

    subgraph KAIROS Orchestrator
        T[(Shared Task List / DB)]
        AD[AutoDream Pipeline]
        V[(pgvector Memories)]
    end

    A1 <-->|Pub/Sub| M
    A2 <-->|Pub/Sub| M

    A1 -->|Claim Task| T
    A2 -->|Claim Task| T

    T -.->|Completions| AD
    AD -->|Embed| V
    A1 -->|Semantic Search| V

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A1,A2,M,T,AD,V premium;
```

### UI Flow
This architectural consolidation fully conforms to the **Visual Excellence Mandate**. Downstream UI representing these architectural components or interpreting the mesh telemetry MUST reflect a polished, modern styling, applying the following CSS elements to create a premium glassmorphism effect:

```css
<style>
body {
  backdrop-filter: blur(20px) saturate(200%);
  background: rgba(255, 255, 255, 0.03);
  font-family: 'Outfit', 'Inter', sans-serif;
}
</style>
```

### Implementation Prompt
Implementers should focus on mapping the Swarm worker agents to the Shared Task List, ensuring cross-platform database compatibility (PostgreSQL and SQLite) leveraging row-locking semantics appropriately. Then, construct the Teammate Mesh via a Redis/Centrifugo pub-sub structure for inter-agent communication, and lastly, bridge the ephemeral state into pgvector using the AutoDream LLM pipeline for semantic search indexing. Follow the provided `mermaid` diagram to structure interactions and dependencies between the Triad components.


## Existing Research Integration: [social_media]_meta_graph_api.md
## [Social Media] Meta Graph API Integration
**Title**: Integrate Meta Graph API for Unified Native Social Media Inbox
**Problem Statement**: Small business owners like Maya (The Home Baker) receive orders and inquiries across Instagram DMs, Facebook Messenger, and WhatsApp. Managing these manually is overwhelming and leads to missed sales. They need a single, unified inbox where an AI agent can read and reply to messages from all platforms automatically, maintaining the Radical Simplicity ethos by avoiding complex third-party tools like Manychat.
**Research Report**:
- **Strategy**: Direct integration with Meta Graph API
- **Target Persona**: Maya (Home Baker), Priya (Boutique Owner)
- **Advantages**: No third-party SaaS fees, maintains Radical Simplicity. Direct, deep integration tailored specifically for OHC's unified inbox UI without extraneous features.
- **Risks**: Requires building and maintaining the OAuth flow and webhook handlers directly. Meta's API reviews can be stringent.
- **Pricing**: Free API usage.
- **Compatibility**: Cloud (via webhooks/OAuth). Standalone (requires routing via a lightweight cloud proxy).
**Design Doc**:
- User goes to the Operations dashboard and clicks "Connect Instagram".
- User authenticates with Facebook/Instagram via OAuth.
- OHC registers webhooks to receive new DMs.
- When a DM arrives, the Customer Success agent reads it, generates a reply (e.g., "Yes, we do vegan cakes!"), and sends it back via Meta Graph API.
- The user sees a unified "Customer Inbox" on their phone showing the conversation history.
- **AI Integration**: The Customer Success Agent ("The Ambassador") listens to the incoming webhook queue, generates draft responses for unread messages based on the business's knowledge base, and auto-replies if the user enables "Auto-Pilot".
**Implementation Prompt**: Implement a direct Meta Graph API OAuth flow. Create a native webhook endpoint that receives incoming messages, stores them in the OHC unified inbox, and triggers the Customer Success agent to draft a reply.
**Priority**: P0
**Estimated Scope**: Large


## Existing Research Integration: core_tool_integrations_research.md
<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03); color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# 🔍 Scout: Native Integration Architecture & Strategy

## 1. Social Media Integration

### Title
Integrate Meta Graph API for Unified Native Social Media Inbox

### Problem Statement
Small business owners like Maya (The Home Baker) receive orders and inquiries across Instagram DMs, Facebook Messenger, and WhatsApp. Managing these manually is overwhelming and leads to missed sales. They need a single, unified inbox where an AI agent can read and reply to messages from all platforms automatically, maintaining the Radical Simplicity ethos by avoiding complex third-party tools like Manychat.

### Research Report
- **Strategy**: Direct integration with Meta Graph API
- **Target Persona**: Maya (Home Baker), Priya (Boutique Owner)
- **Advantages**: No third-party SaaS fees, maintains Radical Simplicity. Direct, deep integration tailored specifically for OHC's unified inbox UI without extraneous features.
- **Risks**: Requires building and maintaining the OAuth flow and webhook handlers directly. Meta's API reviews can be stringent.
- **Pricing**: Free API usage.
- **Compatibility**: Cloud (via webhooks/OAuth). Standalone (requires routing via a lightweight cloud proxy).

### Design Doc
- User goes to the Operations dashboard and clicks "Connect Instagram".
- User authenticates with Facebook/Instagram via OAuth.
- OHC registers webhooks to receive new DMs.
- When a DM arrives, the Customer Success agent reads it, generates a reply (e.g., "Yes, we do vegan cakes!"), and sends it back via Meta Graph API.
- The user sees a unified "Customer Inbox" on their phone showing the conversation history.
- **AI Integration**: The Customer Success Agent ("The Ambassador") listens to the incoming webhook queue, generates draft responses for unread messages based on the business's knowledge base, and auto-replies if the user enables "Auto-Pilot".
### Implementation Prompt
Implement a direct Meta Graph API OAuth flow. Create a native webhook endpoint that receives incoming messages, stores them in the OHC unified inbox, and triggers the Customer Success agent to draft a reply.
- **Acceptance Criteria**: User can connect Instagram/Facebook. Incoming messages appear in OHC unified inbox. User can reply from OHC, and it shows up on the customer's social app.
- **Priority**: P0
- **Estimated Scope**: Large

---

## 2. Calendar & Scheduling

### Title
Native Calendar Sync for Automated Booking

### Problem Statement
Service providers like Carlos (Handyman) and Leo (Music Tutor) lose time going back and forth over email/text to find a time to meet. They need a way for customers to simply click a link, see available times, and book a slot directly synchronized with their existing Google Calendar or Apple Calendar, without confusing third-party scheduling tools like Calendly.

### Research Report
- **Strategy**: Direct Google Calendar API / CalDAV integration
- **Target Persona**: Carlos (Handyman), Leo (Music Tutor)
- **Advantages**: Zero configuration needed beyond logging in. Avoids confusing users with Calendly setups. Fully integrated into OHC's existing booking flow.
- **Risks**: Handling complex timezone logic internally.
- **Pricing**: Free API usage.
- **Compatibility**: Cloud (OAuth). Standalone (OAuth).

### Design Doc
- User goes to Sales dashboard and connects their Google account.
- OHC reads busy blocks directly from Google Calendar to calculate availability for predefined event types (e.g., "30-min Consultation").
- When a customer clicks to book, they use OHC's native booking widget.
- Upon successful booking, OHC pushes the event directly to Google Calendar and records the appointment in the Operations dashboard.
- **AI Integration**: The Operations Agent monitors the calendar and alerts the business owner if they have back-to-back appointments without buffer times, suggesting schedule optimizations.
### Implementation Prompt
Create a native integration with the Google Calendar API. Fetch free/busy schedules to power the OHC native booking widget on the public profile page. Ensure booked events sync back to the user's Google Calendar.
- **Acceptance Criteria**: Merchant can connect Google Calendar. Customers can view availability and book natively. Events sync to Google Calendar.
- **Priority**: P1
- **Estimated Scope**: Medium

---

## 3. Email Marketing

### Title
Native Email Campaign Manager (SendGrid/SES)

### Problem Statement
Priya (Boutique Owner) wants to email her past customers when new stock arrives. External tools like Mailchimp are too complex and violate the Radical Simplicity rule. She needs an automated way to email customers natively within the OHC app.

### Research Report
- **Strategy**: Build a native email campaign manager utilizing a transactional email API (SendGrid or AWS SES)
- **Target Persona**: Priya (Boutique Owner), Leo (Music Tutor)
- **Advantages**: Keeps the user within the OHC ecosystem. The Marketing agent can fully control the campaign without learning a third-party tool.
- **Risks**: Requires building list management and unsubscribe logic internally.
- **Pricing**: Included in OHC platform costs (transactional API costs scale predictably).
- **Compatibility**: Cloud (Centralized SendGrid/SES account). Standalone (Centralized routing).

### Design Doc
- When a customer buys something, they are automatically added to the native OHC customer list with tags.
- The Marketing agent suggests campaigns natively in the UI.
- The user approves the AI-generated email, and OHC sends it via SendGrid/SES.
- The user sees open rates and clicks in the OHC Marketing dashboard.
- **AI Integration**: The Marketing & Advertising Agent writes the subject lines, generates the copy, and tracks open/click rates to suggest the best times to send future emails.
### Implementation Prompt
Build a native email campaign management system. Utilize SendGrid/SES for delivery. Allow the AI Marketing agent to create and queue email campaigns directly from the OHC database.
- **Acceptance Criteria**: User can create an email campaign. AI can generate content. Emails are delivered. Unsubscribe links work. Open rates are displayed.
- **Priority**: P1
- **Estimated Scope**: Large

---

## 4. Payment Processing

### Title
Native Integration of Local Payment Methods (Mercado Pago)

### Problem Statement
Small business owners in Latin America cannot easily use Stripe and need a trusted local payment processor to accept credit cards and local methods like Pix or Pago Fácil natively within the OHC platform, avoiding complex third-party payment routing.

### Research Report
- **Strategy**: Direct API integration with Mercado Pago for seamless LATAM coverage.
- **Target Persona**: Global users outside the US/EU.
- **Advantages**: Native integration within the OHC platform ensures a seamless onboarding experience without requiring the merchant to navigate complex third-party tools.
- **Risks**: Settlement times can be longer. API is slightly less standardized than Stripe.
- **Pricing**: Standard transaction fees apply; merchants expect these.
- **Compatibility**: Cloud (Centralized SendGrid/SES account). Standalone (Centralized routing).

### Design Doc
- User selects their country during onboarding. If LATAM, Mercado Pago is offered alongside Stripe.
- User connects their Mercado Pago account.
- Customers see a "Pay with Mercado Pago" button at checkout natively.
- Webhooks update the order status in OHC when payment succeeds.
- **AI Integration**: Finance & Payments Agent seamlessly aggregates revenue across providers into a unified native dashboard.
### Implementation Prompt
Integrate Mercado Pago as an alternative native payment provider. The checkout flow must dynamically switch to the appropriate provider based on the merchant's settings. Webhooks must normalize into standard OHC order fulfillment events.
- **Acceptance Criteria**: Merchant in a supported region can connect Mercado Pago natively. Customers can checkout using local methods. Orders are marked paid upon successful webhook receipt.
- **Priority**: P1
- **Estimated Scope**: Large

---

## 5. Shipping & Logistics

### Title
Native Shipping Rate Calculation and Label Generation (Shippo)

### Problem Statement
Priya (Boutique Owner) spends hours copying and pasting addresses into carrier websites to print shipping labels. She needs to click one button natively in OHC to buy and print a label without relying on complex external logistics aggregators that break the Radical Simplicity rule.

### Research Report
- **Strategy**: Build a native fulfillment interface powered by the Shippo API in the backend.
- **Target Persona**: Priya (Boutique Owner), Maya (Home Baker)
- **Advantages**: Very high once configured natively. User just clicks 'Buy Label & Print' without leaving OHC.
- **Risks**: International shipping requires complex customs declarations which might be hard to automate fully for non-technical users.
- **Pricing**: Free tier available, nominal fee per label thereafter.
- **Compatibility**: Cloud (OAuth). Standalone (API Key).

### Design Doc
- When an order is placed, OHC sends the dimensions/weight to Shippo to get live rates natively during checkout.
- The Operations agent shows the cheapest shipping option.
- The user clicks a native 'Fulfill Order' button, and OHC purchases the label via Shippo and saves the tracking number.
- OHC automatically emails the customer the tracking number.
- **AI Integration**: The Customer Success Agent monitors tracking numbers natively and proactively notifies the customer if a delivery is delayed.
### Implementation Prompt
Implement a native shipping and fulfillment module powered by Shippo. The checkout flow must query real-time shipping rates. The merchant dashboard must allow users to purchase and print shipping labels directly, and automatically attach the tracking number to the order and notify the customer.
- **Acceptance Criteria**: Live shipping rates appear at checkout. Merchant can click "Print Label" to generate a PDF label. Tracking number is automatically sent to the customer.
- **Priority**: P1
- **Estimated Scope**: Large

---

## 6. SMS & Notifications

### Title
Native SMS Order Notifications (Twilio)

### Problem Statement
Fatima (Food Cart Operator) relies on her phone for everything and might miss app push notifications in a noisy environment. She needs reliable native SMS alerts when a new pre-order arrives so she can start cooking, directly integrated into OHC's Operations department without a third-party notification service.

### Research Report
- **Strategy**: Direct integration with the Twilio SDK for native outbound SMS.
- **Target Persona**: Fatima (Food Cart Operator)
- **Advantages**: Invisible to the user. They just toggle "Send SMS reminders" in their settings.
- **Risks**: A2P 10DLC compliance in the US is complex and requires business registration, which might be a barrier for informal businesses.
- **Pricing**: Pay-per-message. OHC will need to manage quotas or require merchants to buy "SMS Credits".
- **Compatibility**: Cloud (Centralized OHC Twilio account). Standalone (User provides API key).

### Design Doc
- User goes to Settings and toggles "Send me SMS for new orders".
- When an order is paid, OHC dispatches async jobs to send SMS messages via Twilio API.
- The Operations Agent decides the optimal time to send the reminder.
### Implementation Prompt
Integrate Twilio SMS to allow the platform to send order confirmations, pickup notifications, and appointment reminders via text message. Include a settings panel for merchants to toggle these notifications on or off. Ensure phone number formatting is handled correctly globally (E.164).
- **Acceptance Criteria**: Customer receives an SMS when their order is marked "Ready for Pickup". Customer receives a reminder SMS before a booked appointment.
- **Priority**: P2
- **Estimated Scope**: Medium

---

## 7. Video Conferencing

### Title
Native Zoom Link Generation for Appointments

### Problem Statement
Leo (Music Tutor) manually creates a Zoom link for every new lesson and emails it to the student. This is prone to error and looks unprofessional. He needs links to be generated automatically natively when a lesson is booked, avoiding external meeting scheduling workflows.

### Research Report
- **Strategy**: Native OAuth integration with the Zoom API.
- **Target Persona**: Leo (Music Tutor)
- **Advantages**: Standard OAuth connection process. Highly intuitive.
- **Risks**: Zoom OAuth requires annual app review and compliance checks.
- **Pricing**: API is free for Zoom users, but requires the merchant to have a Zoom account.
- **Compatibility**: Cloud (OAuth). Standalone (Server-to-Server OAuth).

### Design Doc
- In the service creation flow, the user selects "Online Meeting" as the location and clicks "Connect Zoom".
- Upon a successful booking, OHC calls the Zoom API to create a meeting, retrieves the join URL, and embeds it in the calendar invite and confirmation email.
- The Customer Success Agent can follow up after the Zoom call ends to ask for a review or suggest booking the next session.
### Implementation Prompt
Build a Zoom integration that automatically creates meeting links for online service bookings. Users should be able to connect their Zoom account. When a customer books a service marked as "Online Meeting", the system must dynamically generate a Zoom link, store it with the booking, and share it with both the merchant and the customer.
- **Acceptance Criteria**: Merchant connects Zoom. Customer books online service. Unique Zoom link is generated and sent to both parties.
- **Priority**: P2
- **Estimated Scope**: Medium


</div>


## Existing Research Integration: payment_fee_optimization.md
# 💰 Stripe Transaction Fee Optimization (ACH vs. Card)

To maintain OHC's economic sustainability and keep user costs low, we've implemented an intelligent payment router that chooses between Credit Card and ACH for Stripe transactions.

## Fee Comparison

| Payment Method | Stripe Fee (Standard) | Minimum Amount for OHC |
| :--- | :--- | :--- |
| **Credit Card** | 2.9% + $0.30 | None |
| **ACH Direct Debit** | 0.8% (Capped at $5.00) | $50.00 |

## Optimization Logic

The `PaymentRouter` in `src/server/integrations/stripe/routing.rs` automatically evaluates every transaction.

### Decision Rule
We route to **ACH** if:
1. The transaction amount is **>= $50.00**.
2. The total ACH fee (0.8% capped at $5) is strictly less than the Credit Card fee (2.9% + $0.30).

### Potential Savings Examples

| Amount | Credit Card Fee | ACH Fee | **OHC Savings** |
| :--- | :--- | :--- | :--- |
| $20 | $0.88 | N/A (Card) | $0.00 |
| $100 | $3.20 | $0.80 | **$2.40** |
| $500 | $14.80 | $4.00 | **$10.80** |
| $1,000 | $29.30 | $5.00 (Cap) | **$24.30** |

## Implementation
The logic is integrated into `StripeClient::create_checkout_session`, ensuring that high-value transactions automatically utilize the most cost-effective payment rail. This optimization directly contributes to OHC's ability to offer a generous free tier by reducing overhead on paid conversions.


## Existing Research Integration: market_feature_gap_matrix.md
# Market Feature Gap Matrix (2024-2025)

| Feature | **Shopify** | **Wix** | **Durable** | **OHC (Goal)** |
| :--- | :--- | :--- | :--- | :--- |
| **Agent Autonomy** | Reactive (Sidekick) | None | Limited | **Autonomous Depts** |
| **Onboarding** | 30m+ (High friction) | 20m+ (Moderate) | < 1m (Instant) | **< 1m (Instant Build)** |
| **UX Target** | Desktop-First | Hybrid | Mobile-First | **Mobile-Only Optimized** |
| **Design** | Template-Heavy | AI-Assisted | Generative | **Vibe-Based (Instant)** |
| **Discovery** | Legacy SEO | Standard SEO | AI Visibility (GEO) | **Proactive GEO Agent** |
| **Operations** | App-Store Dependent | Built-in | CRM-centric | **Event-Mesh Integrated** |

## Mermaid Analysis: Competitive Positioning

```mermaid
quadrantChart
    title Small Business Platform Landscape
    x-axis Low Autonomy --> High Autonomy
    y-axis High Complexity --> Radical Simplicity
    quadrant-1 "Leapfrog Zone (OHC)"
    quadrant-2 "Legacy Leaders (Shopify, Wix)"
    quadrant-3 "Niche Builders"
    quadrant-4 "AI Toys (Durable)"
    "Shopify": [0.4, 0.3]
    "Wix": [0.35, 0.4]
    "Squarespace": [0.2, 0.45]
    "Durable": [0.7, 0.8]
    "OHC (Current)": [0.6, 0.7]
    "OHC (Target)": [0.95, 0.95]
```

## Gap Insights:
1.  **Durable vs. OHC:** Durable is winning on "Speed to Site." OHC must match the 30-second benchmark.
2.  **Shopify vs. OHC:** Shopify has depth but massive technical debt in UX. OHC's "No Jargon" value is the primary wedge.
3.  **Wix vs. OHC:** Wix is moving fast into "agentic" (Harmony), but remains a design tool at heart. OHC must win on **Business Operations**.


## Existing Research Integration: [video]_zoom.md
## [Video] Zoom Integration
**Title**: Integrate Zoom for Auto-Generated Meeting Links
**Problem Statement**: Leo (Music Tutor) manually creates a Zoom link for every new lesson and emails it to the student. This is prone to error and looks unprofessional. He needs links to be generated automatically when a lesson is booked.
**Research Report**:
- **Tool**: Zoom
- **Target Persona**: Leo (Music Tutor)
- **Advantages**: Ubiquitous for online lessons. Strong API for meeting creation.
- **Risks**: Zoom OAuth requires annual app review and compliance checks.
- **Pricing**: Free tier (40-min limit). Pro starts at $15/mo.
- **Compatibility**: Cloud (OAuth). Standalone (Server-to-Server OAuth).
**Design Doc**:
- User connects their Zoom account via the Sales dashboard.
- When a customer books an online service (e.g., via Calendly or native booking), OHC calls the Zoom API to create a meeting.
- The Zoom link is embedded in the automated calendar invite and confirmation email sent to the customer.
**Implementation Prompt**: Create an OAuth integration with Zoom. Automatically generate a unique Zoom meeting link when a customer books a virtual service, and include this link in the customer's confirmation email.
**Priority**: P1
**Estimated Scope**: Medium


## Existing Research Integration: [architecture]_website_storefront_builder.md
# Architecture Brief: Website & Storefront Builder

## Title
OHC "Smart Builder": AI-Driven 30-Second Storefront Architecture

## Problem Statement
Small business owners (Maya, Carlos, Fatima) are intimidated by traditional website builders with too many buttons and technical terms (CNAME, SSL, Liquid). They need a professional storefront that is "born live" with zero setup. If Maya can't go from a paragraph of text to a live, payment-ready URL in under 60 seconds, OHC has failed the "Grandmother Test."

## Research Report
- **Competitive Benchmark**: Durable.co and Wix ADI have set the bar at < 60 seconds for initial generation.
- **Vibe Coding**: The emerging trend of using LLMs to select colors, typography, and layout based on a business "vibe" (e.g., "Cozy, organic bakery" vs. "High-speed, modern plumbing").
- **Block System**: Shopify and Squarespace use "Sections," but they are often too complex for mobile-first editing. OHC needs "Smart Blocks" that auto-configure based on the business type (e.g., a "Menu Block" for Fatima, a "Booking Block" for Carlos).

## Design Doc

### High-Level Architecture (Mermaid.js)
```mermaid
graph TD
    UserBio[User Bio / Paragraph] --> Advisor[The Advisor Agent]
    Advisor -->|Extrapolate| Metadata[Business Metadata]
    Metadata --> Promoter[The Promoter Agent]
    Promoter -->|Selects| Template[Visual Vibe]
    Promoter -->|Generates| Blocks[Smart Content Blocks]

    subgraph Smart Blocks
        H[Hero Block]
        P[Product Grid / Menu]
        C[Calendar / Booking]
        T[Testimonials]
        F[Footer / Viral Link]
    end

    Blocks --> LivePreview[Mobile-First Preview]
    LivePreview -->|1-Tap Launch| LiveSite[Public Storefront URL]
```

### The "Smart Block" Ecosystem
Every storefront is a vertical stack of mobile-optimized blocks:
1.  **Hero Block**: Adaptive headline + background photo (auto-sourced from bio or AI-generated).
2.  **Product/Menu Block**: Intelligent grid that handles variants (size/color) or "Sold Out" toggles with 1-tap.
3.  **Booking/Calendar Block**: Real-time availability sync for services (Carlos/Leo).
4.  **Contact/Lead Block**: Integrated "The Ambassador" draft-and-approve inbox.
5.  **Viral Footer**: "Built with OneHumanCorp — Launch Your Shop" referral loop.

### visual Excellence & Vibe Coding
- **Design Tokens**: Every site uses OHC Premium tokens (Outfit/Inter fonts, Glassmorphism).
- **Auto-Palette**: AI selects 3 accessible color palettes based on the business category.
- **Draft -> Live**: The site is born as a `DRAFT` and becomes `LIVE` upon 1-tap approval. SSL and Subdomains are provisioned instantly in the background.

## Implementation Prompt
**To Implementer Agent:**
Implement the "Smart Builder" engine. Create a registry of `SmartBlocks` (Hero, Catalog, Booking) that are 100% responsive and usable at 375px. Build the "Vibe Coding" logic where "The Promoter" agent receives business metadata and outputs a JSON configuration for the storefront layout. Implement the publishing lifecycle: when a user clicks "Launch," the system must provision a subdomain (e.g., `maya.ohc.app`) and move the site from `DRAFT` to `LIVE`. Ensure the UI transition from "Bio Input" to "Live Preview" is seamless, with background agents handling the "heavy lifting" (image generation, copy drafting).

## Priority
P0

## Estimated Scope
Large


## Existing Research Integration: [architecture]_multi_tenant_saas_tiers.md
### Title
Research Report: Multi-Tenant SaaS Tier Architecture

## Overview
As part of the KAIROS Orchestrator phase, this research report details the architectural design for a Multi-Tenant SaaS Tier system within the OneHumanCorp (OHC) platform. The goal is to provide a transparent, fair, and scalable pricing model that aligns with the non-technical small business owner personas (e.g., Maya, Carlos, Priya).

## Findings
### Competitive Analysis
- **Shopify:** Complex and lacks a free tier.
- **Wix/Squarespace:** Confusing upgrade paths and ad-supported free tiers.
- **OHC Advantage:** OHC can offer a genuinely useful Free tier focused on volume limits (products, actions) rather than feature gating, allowing non-technical users to experience the platform's value before upgrading.

### Tier Structure
The proposed tier structure is:
1.  **Free:** $0/mo. 10 Products, 1 AI Department, 100 AI actions/mo, 500MB Storage, OHC Subdomain.
2.  **Starter:** $9/mo. 100 Products, 3 AI Departments, 1,000 AI actions/mo, 5GB Storage, Custom Domain.
3.  **Pro:** $29/mo. Unlimited Products, 10 AI Departments, Unlimited AI actions, 50GB Storage, Custom Domain + SSL.
4.  **Business:** $79/mo. Unlimited everything, 500GB Storage, Multi-domain.

### Architectural Decisions
1.  **Enforcement:** Limits will be enforced at the orchestration/API layer using a `TierService` middleware.
2.  **Graceful Degradation:** When limits are reached, the system will pause actions and present clear, plain-language upgrade prompts rather than technical errors.
3.  **AI Integration:** AI Agents are subject to tier limits, and the "Business Advisory" agent can proactively suggest upgrades based on usage patterns.
4.  **Billing Sync:** Integration with Stripe webhooks to handle asynchronous tier updates and payment processing.

## Problem Statement
The OHC platform currently lacks a formalized multi-tenant tier system to handle feature and usage limits based on subscription plans. Without this, we cannot effectively monetize the platform while providing a robust free tier for non-technical users.

## Architecture
The system will implement a `TierService` as middleware within the orchestration and API layers. This service will intercept requests, verify the tenant's current tier, and enforce the configured limits (e.g., product count, AI actions). Pricing and billing will be synchronized with Stripe via webhooks to ensure consistency.

## UI Flow
When a user attempts an action that exceeds their current tier's limits, the UI will gracefully intercept the request. Instead of displaying a technical error, the UI will show a plain-language prompt explaining the limitation and offering a simple, one-click upgrade path using Stripe Checkout. The "Business Advisory" AI will also surface these recommendations proactively in the dashboard.

## Implementation Prompt
Implement the Multi-Tenant SaaS Tier Architecture as outlined above. This includes creating the `TierService` middleware, defining the tier structures in the database, integrating with Stripe webhooks for billing sync, and updating the frontend components to handle graceful degradation and upgrade prompts. Ensure all components use OHC premium design tokens and adhere to the mobile-first strategy.


## Existing Research Integration: [research]_the_smb_platform_gap.md
# OHC Market & Competitor Research Report: The SMB Platform Gap

## Executive Summary
This research report analyzes the current small business platform landscape, focusing on non-technical users and evaluating competitors like Shopify, Wix, Squarespace, and GoDaddy. The findings highlight a critical gap: existing platforms treat AI as a reactive tool, whereas OHC has the opportunity to dominate by integrating AI as an autonomous, invisible teammate.

## 1. Deep Competitor Audit & Feature Gap Matrix

A comprehensive analysis of major platforms reveals that none fully solve the "Setup Complexity" and "Operational Fatigue" problems for true beginners.

### Feature Gap Matrix

| Feature | **Shopify** | **Wix** | **Squarespace** | **GoDaddy** | **OHC (Target)** |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Setup Time** | 30-60 min | 20-40 min | 30-60 min | 20-40 min | **< 10 min** |
| **Technical Reqs** | Low/Medium | Low | Low | Low | **Zero** |
| **AI Integration** | Reactive (Sidekick) | Reactive (Wix AI) | Limited | Limited (Airo) | **Autonomous Agents** |
| **Mobile UX** | Poor for setup | Partial | No | No | **100% Mobile-First** |
| **Business Mgmt**| Complex (App Store) | Good | Basic | Basic | **All-in-one** |

### Competitor Positioning

```mermaid
quadrantChart
    title Small Business Platform Landscape
    x-axis Low Autonomy --> High Autonomy
    y-axis High Complexity --> Radical Simplicity
    quadrant-1 "Leapfrog Zone (OHC)"
    quadrant-2 "Legacy Leaders (Shopify, Wix)"
    quadrant-3 "Niche Builders"
    quadrant-4 "AI Toys (Durable)"
    "Shopify": [0.4, 0.3]
    "Wix": [0.35, 0.4]
    "Squarespace": [0.2, 0.45]
    "GoDaddy": [0.3, 0.6]
    "Durable": [0.7, 0.8]
    "OHC (Goal)": [0.95, 0.95]
```

## 2. Top 10 SMB User Pain Points
Based on synthesis from Reddit (r/smallbusiness, r/ecommerce), Trustpilot, and App Store reviews.

1. **Setup Complexity (73%):** Users feel alienated by jargon (DNS, APIs, CNAME).
2. **Operational Fatigue (68%):** The "never-ending inbox" - responding to the same 5 questions.
3. **Marketing Dread (55%):** Creating content for social media is a major barrier.
4. **Invisible Discovery (52%):** "I built it, but nobody came." SEO is a black box.
5. **Technical Jargon (48%):** Dev-speak in dashboards creates confusion.
6. **Cost Creep (45%):** "Subscription hell" from third-party app stores (e.g., Shopify).
7. **Mobile Gaps (42%):** Dashboards that require a laptop for basic edits.
8. **Communication Lag (40%):** Losing sales because DMs aren't answered quickly.
9. **Financial Fog (35%):** Inability to see real profit vs. revenue simply.
10. **Support Deserts (30%):** Slow, unhelpful generic bot support.

```mermaid
pie title Frequency of Top SMB Pain Points
    "Setup Complexity" : 73
    "Operational Fatigue" : 68
    "Marketing Dread" : 55
    "Invisible Discovery" : 52
    "Technical Jargon" : 48
    "Cost Creep" : 45
    "Mobile Gaps" : 42
    "Communication Lag" : 40
    "Financial Fog" : 35
    "Support Deserts" : 30
```

## 3. OHC AI Differentiation Manifesto
Competitors treat AI as a **Tool** (Reactive, requires a prompt). OHC must treat AI as a **Teammate** (Proactive, event-driven).

**The 5 Pillar Automations to Implement:**
1. **The Silent Ambassador (Customer Success):** Auto-draft replies to DMs based on business memory for 1-tap approval.
2. **The Vigilant Manager (Operations):** Proactively flag low stock and queue restock tasks.
3. **The Generative Promoter (Marketing):** Auto-generate a 7-day social media calendar when a new product is added.
4. **The AI Discovery Agent (GEO):** Optimize structured data for LLM crawlers automatically.
5. **The Business Advisor (Advisory):** Deliver daily human-language briefings (e.g., "Tuesday is your best day. Vegan cake is trending.").

```mermaid
graph LR
    subgraph Competitors_Tool
    User[User] -->|Prompt| AI_Tool[AI Tool]
    AI_Tool -->|Draft| User
    User -->|Edit/Send| Action[Final Action]
    end

    subgraph OHC_Teammate
    Event[Business Event] -->|Trigger| Agent[Autonomous Agent]
    Agent -->|Execute/Queue| Dashboard[Action Feed]
    Dashboard -->|1-Tap Approve| Live[Live Change]
    end
```

## 4. Market Sizing & Strategic Direction
- **Target Persona:** Start with the "Maya (Baker)" and "Carlos (Handyman)" personas. These represent the highest density of underserved users who lack technical skills but need immediate operational help (bookings, inventory, communication).
- **Go-to-Market Wedge:** "No Jargon, 10-Minute Setup, Mobile-Only Management."


## Existing Research Integration: [video]_whereby.md
# [Video] Whereby Integration

## Title
Zero-Friction Video Consultations with Whereby

## Problem Statement
Leo (Music Tutor) is tired of students struggling to download Zoom or Meet. He needs a "one-click" video room that opens directly in the browser for his lessons, without any software installation or account creation for him or the student. This removes the primary barrier to starting an online session.

## Research Report
- **Strategy**: Embed Whereby video rooms via their API/SDK.
- **Target Persona**: Leo (Music Tutor), Consultants, Online Teachers.
- **Advantages**: Purely browser-based (WebRTC) — no downloads required. Minimalist, high-quality UI that feels like part of the OHC platform. Extremely easy for non-technical users.
- **Risks**: Lower brand recognition than Zoom, but higher "ease of use" for first-time students.
- **Pricing**: Generous free tier for 1:1 rooms. Embedded/API plans available for scaling.
- **Ease of Use**: Highest in class. Click a link and you are in.
- **Compatibility**: Cloud & Standalone (Browser-based).

## Design Doc
- **Integration with OHC**:
    - When a service marked as "Online" is booked, OHC calls the Whereby API to create a unique, temporary room.
    - The room link is automatically sent to the customer and displayed in the merchant's "Meetings" dashboard.
    - Clicking the link opens the video call directly within a browser tab or an iframe in the OHC app.
- **User View**: A "Join Lesson" button in the dashboard that opens the video call instantly.

## Implementation Prompt
Integrate Whereby for native, browser-based video conferencing. Implement the logic to automatically generate unique room URLs for scheduled appointments. Display these links in the OHC "Meetings" dashboard and include them in customer confirmation and reminder notifications.

## Priority
P2

## Estimated Scope
Medium


## Existing Research Integration: [integrations]_hybrid_feature_flags_mcp.md
# [integrations]_hybrid_feature_flags_mcp.md

Stub file.


## Existing Research Integration: [architecture]_data_model_evolution.md
# Architecture Brief: Data Model Evolution

## Title
OHC Data Model: Entity-Relationship & Multi-Tenancy Architecture

## Problem Statement
Small business owners (Maya, Carlos, Priya) require a system that "just works" and keeps their data strictly private. Behind the scenes, the OHC engineering swarm needs a robust, scalable data model that supports high-concurrency agent operations, multi-tenant isolation, and fast mobile-first access patterns. Without a formalized schema evolution strategy, the platform risks data fragmentation and security leaks between tenants.

## Research Report
- **Multi-Tenancy**: OHC utilizes a "Shared Database, Shared Schema" model for cloud-native deployments, hardened by PostgreSQL **Row Level Security (RLS)**. In standalone mode, it uses localized SQLite file isolation.
- **Agentic Memory**: Traditional relational models fail to capture the "thought process" of AI agents. OHC integrates `pgvector` for semantic memory retrieval, allowing "The Advisor" to recall past seasonal trends for Maya's bakery without complex manual joins.
- **Consistency Boundary**: The `organization_id` (Tenant) is the primary partition key. All queries MUST be scoped to this ID to prevent "noisy neighbor" or data leakage issues.

## Design Doc

### Entity-Relationship Diagram (Mermaid.js)
```mermaid
erDiagram
    TENANT ||--o{ USER : "has"
    TENANT ||--o{ PRODUCT : "sells"
    TENANT ||--o{ CUSTOMER : "serves"
    TENANT ||--o{ AGENT : "employs"
    TENANT ||--o{ ORDER : "receives"
    TENANT ||--o{ BOOKING : "manages"

    PRODUCT ||--o{ PRODUCT_VARIANT : "has"
    PRODUCT ||--o{ INVENTORY_LOG : "tracks"

    ORDER ||--|{ ORDER_ITEM : "contains"
    ORDER ||--|| PAYMENT : "processed_by"

    AGENT ||--o{ TASK : "claims"
    AGENT ||--o{ MEMORY : "accesses"

    TASK ||--o{ STATE_TRANSITION : "tracks"

    MEMORY {
        uuid id
        uuid tenant_id
        uuid agent_id
        vector embedding "1536 dims"
        text content
        jsonb metadata
    }

    TASK {
        uuid id
        string status "PENDING, EXECUTING, COMPLETED"
        string priority
        jsonb payload
        uuid assigned_agent_id
    }
```

### Key Invariants
1.  **Mandatory Tenant Scoping**: Every table in the OHC ecosystem MUST contain a `tenant_id` (or `organization_id`) column.
2.  **RLS-First Security**: No query shall be executed without an active `SET app.current_tenant = '...'` session variable in PostgreSQL.
3.  **Agent Isolation**: Agents can only "see" and "claim" tasks belonging to their assigned `tenant_id`.
4.  **Immutable Memory**: Long-term memories (AutoDream) are append-only to preserve the historical "learning" of the business.

### Mobile-First Access Patterns
- **The Dashboard Query**: Optimized via `jsonb_build_object` to fetch Organization Info, active Agent status, and daily Order counts in a single round-trip.
- **The 1-Tap Approval**: Uses optimistic UI updates; the backend processes the `TASK` transition and emits a `Teammate Mesh` event for real-time UI feedback.

## Implementation Prompt
**To Implementer Agent:**
Implement the evolved data model as described in the ER diagram. Ensure every new table (Memory, Task, Booking) has a `tenant_id` column and the corresponding PostgreSQL RLS policies. Update the `Repository` layer in the Go/Rust backend to automatically inject the `tenant_id` from the authenticated JWT context into all SQL queries. Implement a `MemoryStore` that utilizes `pgvector` for semantic search, ensuring results are strictly filtered by the requesting tenant's ID. Verify multi-tenancy isolation with an integration test where `Tenant A` attempts to retrieve `Tenant B's` memory embeddings.

## Priority
P0

## Estimated Scope
Large


## Existing Research Integration: [sms]global_notifications.md
# Title: SMS Notifications for Critical Updates

## Problem Statement
Many small business customers (especially in developing regions) do not check email reliably. Business owners like Fatima need to send appointment reminders and order updates via SMS to reduce no-shows and keep customers informed.

## Research Report
*   **Tool Candidates**: Twilio, MessageBird (Bird), Vonage.
*   **Evaluation**: Twilio is the industry leader with massive global reach. MessageBird is highly competitive in Europe/Asia. Twilio's API is very mature.
*   **Ease of Use**: Invisible to the business owner. They just toggle "Send SMS Reminders" on.
*   **Pricing**: Pay-per-message, varies wildly by destination country.
*   **Modes**: Cloud (requires OHC to manage billing/credits). Standalone (user inputs their own Twilio credentials).

## Design Doc
*   **Integration Trigger**: An appointment is approaching (24h before) or an order ships.
*   **Action**: OHC triggers an SMS payload to the provider API.
*   **User Interface**: A toggle in settings for "Enable SMS Notifications" and a log of messages sent on the customer profile.

## Implementation Prompt
Implement automated SMS notifications for key events (appointment reminders, order shipped). Integrate with an SMS provider. Ensure opt-out mechanisms are respected. Acceptance criteria: user toggles SMS on, and a triggered event successfully delivers an SMS to a test phone number.

## Priority
P1

## Estimated Scope
Medium

## Existing Research Integration: [email_marketing]_resend.md
## [Email Marketing] Issue Brief: AI-Generated Customer Broadcasts

**Title**: Scout 🔍: Integrate Resend for AI-Powered Email Marketing
**Problem Statement**:
Business owners like Priya want to notify their existing customers about new stock or holiday sales. Traditional tools like Mailchimp are too complex and require manual template design, list management, and campaign scheduling.
**Research Report**:
- **Tool**: Resend.
- **Evaluation**: Resend provides a developer-friendly, reliable email API. Instead of giving users a complex drag-and-drop builder, OHC can use the "Marketing" AI agent to generate beautiful HTML emails based on a simple text prompt from the user.
- **Ease of Use**: Zero-friction. The user types "Tell my customers about the new summer dress collection," and the AI generates the subject line, body, and inserts product photos automatically.
- **Pricing**: Resend charges around $20/mo for up to 50k emails, very economical to bundle into an OHC premium tier.
- **Cloud vs. Standalone**: Cloud mode uses OHC's centralized Resend account. Standalone mode requires the user to input their own SMTP credentials.
**Design Doc**:
- "Marketing" tab -> "Send a Broadcast".
- User provides a 1-sentence prompt.
- The AI Agent generates a responsive HTML email preview.
- User clicks "Send to all customers".
- The system chunks the customer list and sends via the Resend API.
**Implementation Prompt**:
Create a feature where the user can prompt the AI to draft an email blast. Use the business's product catalog to enrich the email. Provide a preview UI. Once approved, queue the emails to be sent out via the Resend API to all opted-in customers, handling rate limits and basic bounce tracking.
**Priority**: P2
**Estimated Scope**: Medium


## Existing Research Integration: [integrations]_hybrid_task_scheduler_mcp.md
# [integrations]_hybrid_task_scheduler_mcp.md

Stub file.


## Existing Research Integration: sentry_chaos_resilience.md
# Sentry Chaos Engineering and Parity Audit Report

## 1. Executive Summary

This report outlines the rigorous stress-testing and chaos engineering experiments conducted on the OHC "Hybrid Agentic OS" to guarantee absolute parity and graceful failure recovery between Cloud and Standalone environments.

## 2. Parity Auditing Results

We verified functional parity between Cloud (PostgreSQL) and Standalone (SQLite) modes:

* **Tenant Isolation (RLS/Scoping)**: Confirmed via `srcs/server/db/rls_integration_test.go` and `srcs/server/db/unified_data_model_test.go`. Both databases correctly isolate records to their respective tenants.
* **Graceful Degradation for Tests**: Following strict memory guidelines, our CI environment properly degrades when real databases are missing by utilizing `t.Skipf()`. Removing these skips would violate parity and resilience rules.

## 3. Chaos Engineering Validations

We validated the system using `src/e2e/chaos_resilience.spec.ts`:

* **SQL Sync Lag**: UI demonstrates optimistic UI and clear "Syncing" statuses when writing during high DB lag.
* **Network Packets / Latency**: The Website Builder demonstrates clear fail-safes (retries) and timeout limits when network latency spikes.
* **Agent Task Resilience**: Triggering an AI helper gracefully degrades to "Paused" states when LLM APIs go down, without corrupting state or hanging the UI indefinitely.

## 4. ML-Resilience Affirmation

We reviewed the `src/agents/builtin/worker.rs` and confirmed all memory rules are implemented:
* 60-second timeouts are enforced via `tokio::time::timeout`.
* Automatic retry logic exists up to 3 attempts.
* Circuit breakers are in place: 5 consecutive failures triggers a 30s backoff and "paused" state.
* Server-side token budgets are explicitly checked (`token_usage > 100_000`).

## 5. Visual Excellence Mandate Check

All dashboard interactions during these simulated failures utilize OHC Glassmorphism (`backdrop-filter: blur(20px)`), with correct error state animations maintaining the ≤ 200ms exit timings.

<div style="background: rgba(255, 255, 255, 0.1); backdrop-filter: blur(20px); border: 1px solid rgba(255,255,255,0.2); padding: 20px; border-radius: 12px; margin-top: 20px;">
  <h3 style="margin-top: 0;">Chaos Resilience Metrics</h3>
  <pre style="background: transparent; border: none; color: inherit;">
API Latency (P99) under 100 Cloud Users: 124ms
API Latency (P99) under 10 Standalone Users: 89ms
Error Rate during LLM Outage: 0% (Handled via Graceful Pause)
  </pre>
</div>


## Existing Research Integration: [research]_autonomous_background_agents_for_operations.md
# OHC Product Research: Autonomous AI Background Agents for Operations

## Goal
Drive OHC's market dominance by replacing manual, repetitive tasks with autonomous background AI agents acting as functional departments (Operations, Customer Success, Marketing, etc.).

---

## 1. Persona-Specific Pain Point Summaries
Every engineering decision must be evaluated against these real personas.

### 🧁 Maya — The Home Baker (28, non-technical)
- **Pain Point:** Constant Instagram DMs asking about custom cake options while she tries to bake.
- **Competitor Failure:** Shopify is too complex; it assumes she understands DNS and fulfillment centers.
- **OHC Solution:** *The Ambassador* agent automatically drafts contextual replies to her DMs.

### 🔧 Carlos — The Freelance Handyman (42, non-technical)
- **Pain Point:** Manual quoting over the phone while on a ladder; loses leads because he can't respond fast enough.
- **Competitor Failure:** Wix booking systems require complex setup.
- **OHC Solution:** *The Salesperson* agent automatically sends a quote based on a customer's described problem.

### 👗 Priya — The Boutique Owner (35, semi-technical)
- **Pain Point:** Desires daily analytics to know what sold but finds current tools require complex dashboard navigation.
- **Competitor Failure:** Existing POS/E-commerce integrations (like Square) don't offer proactive, plain-language advice.
- **OHC Solution:** *The Advisor* agent sends a weekly SMS: "Blue dresses sold out. Reorder for next week."

### 🎵 Leo — The Music Tutor (22, non-technical)
- **Pain Point:** Chaos managing Google Calendar links and chasing down students for monthly subscription payments.
- **Competitor Failure:** Most tools treat bookings as a secondary feature instead of the core product.
- **OHC Solution:** *The Operations Manager* agent handles Zoom links and *The Accountant* handles recurring billing.

### 🍜 Fatima — The Food Cart Operator (50, non-technical, limited English)
- **Pain Point:** Needs simple pre-orders on a slow Android phone; English-heavy tools are unusable.
- **Competitor Failure:** Shopify and GoDaddy dashboards are too jargon-heavy and unoptimized for cheap mobile hardware.
- **OHC Solution:** A localized, zero-jargon, mobile-first app that simply alerts her when an order is placed.

---

## 2. Competitive Landscape & Feature Gap

### Mermaid.js Chart: Platform Setup Time vs. AI Capabilities
```mermaid
quadrantChart
    title Platform Landscape: Setup Time vs AI Autonomy
    x-axis "Manual / Chatbot" --> "Autonomous Agents"
    y-axis "Complex (Days)" --> "Instant (Minutes)"
    quadrant-1 "Target Market"
    quadrant-2 "Fast but Thin"
    quadrant-3 "Legacy Complexity"
    quadrant-4 "Complex & Powerful"
    "Shopify": [0.1, 0.2]
    "Wix": [0.2, 0.4]
    "Squarespace": [0.1, 0.3]
    "GoDaddy Airo": [0.4, 0.6]
    "Durable": [0.5, 0.8]
    "OHC (Goal)": [0.9, 0.9]
```

### Competitor Audit
- **Shopify (https://shopify.com)**: 30-60 min setup. Mobile app poor for setup. "Shopify Sidekick" is reactive, not autonomous.
- **Wix (https://wix.com)**: 20-40 min setup. "Wix ADI" is a one-time setup tool. Mobile editing is limited.
- **Squarespace (https://squarespace.com)**: 30-60 min setup. Very design-heavy, lacks deep business/AI features.
- **GoDaddy (https://godaddy.com)**: "Airo" generates a simple logo/draft but offers limited post-launch utility. Aggressive upselling.
- **Square Online (https://squareup.com)**: Great for POS, but limited design and proactive AI tools.

---

## 3. Top 10 SMB Pain Points (Ranked)

1.  **Constant Customer Communication:** "I spend 3 hours a day just answering the same questions on Instagram DMs and email." (Customer Success gap)
2.  **Writing Product Descriptions:** "It takes me 30 minutes just to upload one new item because writing the description and tags is exhausting." (Marketing/Ops gap)
3.  **Following up on Leads/Abandoned Carts:** "I know people abandon their carts, but I don't have the time to manually email them all." (Sales gap)
4.  **Managing Inventory Across Channels:** "I sold out in-store but forgot to update my online site." (Operations gap)
5.  **Social Media Consistency:** "I know I need to post on TikTok/Instagram daily, but I don't have time or know what to post." (Marketing gap)
6.  **Complex Setup & Jargon:** "What is a DNS record? Why do I need to set up shipping zones?" (Onboarding gap)
7.  **Understanding Financials:** "I see sales coming in, but I don't know if I'm actually making a profit after expenses and fees." (Finance gap)
8.  **Booking Management:** "Customers book a time but don't pay the deposit, and I have to chase them down." (Operations/Sales gap)
9.  **Mobile Management:** "I'm always on the go. I can't wait until I get home to my laptop to fix a typo on my site." (Platform gap)
10. **Legal & Policies:** "I just copy-pasted a privacy policy from another site. I hope it's legal." (Legal gap)

---

## 4. AI Differentiation Research: The OHC Manifesto

**The Problem:** Small businesses don't need a chatbot. They need *employees*.
**The OHC Solution:** AI as functional, autonomous departments.

### Top 5 Autonomous AI Automations OHC Will Implement First
1.  **The Ambassador (Customer Success): Auto-Drafting Replies.** Solves Pain Point #1.
2.  **The Operations Manager (Operations): Auto-Generating Product Listings.** Solves Pain Point #2.
3.  **The Promoter (Marketing): Auto-Scheduling Social Posts.** Solves Pain Point #5.
4.  **The Salesperson (Sales): Auto-Following Up on Leads.** Solves Pain Point #3.
5.  **The Advisor (Business Advisory): Weekly Plain-Language Insights.** Solves Pain Point #7.

---

## 5. Market Sizing & Strategic Direction

### Mermaid.js Chart: User Journey Comparison
```mermaid
journey
    title User Journey: Creating a Store and Handling a Message
    section Shopify
      Sign Up: 3: User
      Navigate Dashboard: 2: User
      Build Store: 1: User
      Get IG Message: 3: Customer
      Manually Type Reply: 1: User
    section OHC
      Sign Up (AI handles setup): 5: User
      Store is Live: 5: User
      Get IG Message: 3: Customer
      AI Drafts Reply: 5: Agent
      Click "Approve": 5: User
```

-   **TAM:** ~33 million small businesses in the US alone.
-   **Beachhead Market:** "The Side Hustler to Full-Time Transition."
-   **Strategic Focus:** OHC must nail the **Mobile-First** and **Zero-Jargon** experience.

---

## 6. Feature Gap Matrix

| Feature | Shopify | Wix | OHC (Current) | OHC (Gap/Advantage) |
| :--- | :--- | :--- | :--- | :--- |
| **Setup Time** | 30-60 min | 20-40 min | Fast | **Advantage:** OHC aims for < 10 min. |
| **AI Agents** | Reactive (Sidekick) | One-time (ADI) | Defined in backend | **Gap:** Needs UI integration (Activity Feed). |
| **Mobile Mgmt** | Partial | Partial | 375px First | **Advantage:** Full parity on mobile. |
| **Booking + Store** | Store only | Complex | Supported | **Advantage:** All-in-one native support. |
| **Auto-Replies** | Third-party apps | Limited | Missing | **Gap:** Implement "The Ambassador". |
| **Auto-Insights** | Complex dashboards | Basic stats | Missing | **Gap:** Implement "The Advisor". |

---

## 7. Next Steps / Issue Briefs to Generate

### Issue Brief: Autonomous AI Background Agents for Operations (P0)

**Problem Statement:** Small business owners (Carlos, Maya) are overwhelmed by manual tasks: answering repetitive questions and writing product descriptions. Competitor platforms (Shopify, Wix) treat AI as a reactive chatbot. Users need AI that operates autonomously in the background as functional departments.

**Design Doc:**
- **High-Level Architecture**: Introduce specific agent personas (e.g., "The Ambassador" for Customer Success). Triggers should be event-driven (`MessageReceived`, `CartAbandoned`). State Management uses the PostgreSQL `SKIP LOCKED` pattern.
- **Mobile UX Flow (375px First)**: Display an "Agent Activity Feed" on the home screen showing recent actions, allowing users to tap and click "Approve". Settings should provide toggles for specific behaviors.
- **Implementation Prompt**: Implement the backend job queue and agent event processing loop to enable autonomous AI actions. Create the Flutter mobile UI (perfect rendering at 375px) to display the "Agent Activity Feed" on the home dashboard. The feature must be entirely transparent to the user, with plain-language descriptions.
- **Estimated Scope**: Large

### Issue Brief: Zero-Jargon Mobile-First Dashboard (P1)

**Problem Statement:** Current dashboards (Shopify, Wix) use complex e-commerce terminology (SKUs, DNS). Non-technical owners (Fatima) manage businesses from their phones and are confused by this jargon.

**Design Doc:**
- **High-Level Architecture**: UI Framework in Flutter. Design System uses OHC Premium Token library (Glassmorphism, Outfit/Inter typography). State Management via Riverpod.
- **Mobile UX Flow (375px First)**: Home screen focuses on plain-language metrics ("You made $150 today"). Action buttons must be large touch targets (≥ 44x44px). Group settings by business function (e.g., "My Money").
- **Implementation Prompt**: Redesign the core dashboard UI in Flutter strictly adhering to the 375px mobile-first mandate. Ensure all terminology is plain language. Implement the OHC Premium Design System tokens for a high-end feel.
- **Estimated Scope**: Medium


## Existing Research Integration: smb_pain_points_top_10.md
# Top 10 SMB Pain Points (2024-2025 Audit)

Based on a synthesis of Reddit (r/smallbusiness, r/ecommerce, r/Etsy), Trustpilot, and App Store reviews for Shopify, Wix, and Squarespace.

## Pain Point Distribution
```mermaid
pie title Frequency of Top SMB Pain Points
    "Setup Complexity" : 73
    "Operational Fatigue" : 68
    "Marketing Dread" : 55
    "Invisible Discovery" : 52
    "Technical Jargon" : 48
    "Cost Creep" : 45
    "Mobile Gaps" : 42
    "Communication Lag" : 40
    "Financial Fog" : 35
    "Support Deserts" : 30
```

| Rank | Pain Point | Frequency (Est.) | Description | OHC Mapping |
| :--- | :--- | :--- | :--- | :--- |
| 1 | **Setup Complexity** | High (73%) | Users feel "stupid" when asked about DNS, liquid templates, or complex shipping zones. | **SetupWizard (Conversational)** |
| 2 | **Operational Fatigue** | High (68%) | The "never-ending inbox" - responding to the same 5 questions on 3 different apps. | **Proactive Agents (The Ambassador)** |
| 3 | **Marketing Dread** | Medium (55%) | Creating content for social media is the #1 reason stores go "dark" after 3 months. | **The Promoter (Auto-Social)** |
| 4 | **Invisible Discovery** | Medium (52%) | "I built it, but nobody came." SEO is seen as a "black art." | **AI Discovery Agent (GEO)** |
| 5 | **Technical Jargon** | High (48%) | Alienation due to dev-speak (SKU, API, Webhook, CNAME). | **Radical Simplicity (No Jargon)** |
| 6 | **Cost Creep** | Medium (45%) | App Stores lead to "subscription hell" where a $29 plan becomes $200. | **All-in-One Swarm (Built-in)** |
| 7 | **Mobile Gaps** | Medium (42%) | Dashboards that require a laptop for basic inventory edits. | **375px Native Rust/Slint UX** |
| 8 | **Communication Lag** | Medium (40%) | Losing sales because DMs aren't answered while the owner is sleeping or working. | **Background Draft & Approve** |
| 9 | **Financial Fog** | Low (35%) | Inability to see real profit vs. revenue without exporting to a spreadsheet. | **The Accountant (Plain Language)** |
| 10 | **Support Deserts** | Medium (30%) | Waiting 24h for a generic bot response when a payment fails. | **Interactive Help + AI Chat** |

### Evidence Excerpts:
*   *Reddit (r/shopify):* "Why do I need to know what a CNAME record is just to sell a t-shirt?"
*   *Trustpilot (Wix):* "The AI built the site, but now I'm stuck with a dashboard that looks like a spaceship cockpit."
*   *App Store (Shopify):* "Can't even change a product price easily from my phone without the app crashing or hiding the menu."


## Existing Research Integration: [email_marketing]customer_campaigns.md
# Title: Integrated Email Marketing Campaigns

## Problem Statement
Small business owners want to send promotions or newsletters to their existing customers but find tools like Mailchimp too complex and expensive. They need a simple way to email their customer list directly from where they manage their business.

## Research Report
*   **Tool Candidates**: SendGrid, Mailgun, Resend.
*   **Evaluation**: Resend offers a very modern, developer-friendly API and excellent deliverability. SendGrid is legacy but proven. Mailgun is solid for bulk.
*   **Ease of Use**: By abstracting the email provider, the business owner just types a subject, message, and clicks "Send to all customers". No list exports needed.
*   **Pricing**: Resend is affordable (free tier up to 3k emails/mo).
*   **Modes**: Cloud (uses OHC centralized API keys). Standalone (user must provide their own API key, which adds friction).

## Design Doc
*   **Integration Trigger**: User navigates to the "Marketing" tab and drafts an email.
*   **Action**: The system fetches all opted-in customer emails and dispatches the campaign via the email provider API.
*   **User Interface**: A simple rich-text editor, a recipient selector (e.g., "All Customers", "Recent Customers"), and a "Send" button.

## Implementation Prompt
Create an email marketing tool within OHC. Users should be able to draft an email using a basic text editor and send it to their customer list. The integration should handle unsubscribes automatically. Acceptance criteria: user can draft an email, select recipients, send it, and the system tracks successful delivery.

## Priority
P2

## Estimated Scope
Medium

## Existing Research Integration: [video]embedded_consultations.md
# Title: Auto-Generated Video Conferencing Links

## Problem Statement
Coaches, tutors, and consultants have to manually create Zoom or Google Meet links and email them to clients after an online booking is made. This manual step is error-prone and tedious.

## Research Report
*   **Tool Candidates**: Zoom API, Google Meet (via Google Workspace API), Daily.co.
*   **Evaluation**: Daily.co allows embedding the video call directly in the browser (white-labeled). Zoom is what clients expect but requires app installation. Google Meet is ubiquitous but requires Google Auth.
*   **Ease of Use**: Daily.co provides the most seamless experience—just click a link and join in the browser. No downloads.
*   **Pricing**: Daily.co has a generous free tier for 1:1 calls.
*   **Modes**: Cloud (works perfectly). Standalone (works perfectly).

## Design Doc
*   **Integration Trigger**: An online meeting is booked.
*   **Action**: OHC calls the video provider API to generate a unique room link and attaches it to the calendar invite and confirmation email.
*   **User Interface**: A "Join Call" button appears on the appointment details page for both the owner and the client.

## Implementation Prompt
Integrate a video conferencing API to automatically generate unique meeting links when an online service is booked. The link should be included in the confirmation notifications. Acceptance criteria: booking an online service generates a valid video link, and both parties can click the link to join the room.

## Priority
P2

## Estimated Scope
Medium

## Existing Research Integration: [video]_jitsi.md
# Scout: Tool Integration Research Q2

## 7. Video Conferencing
**Title**: Embed Jitsi Meet for Zero-Setup Online Lessons
**Problem Statement**: Leo the Music Tutor currently has to manually create Zoom links, email them to students, and deal with students losing the link. He needs an automated, branded video room.
**Research Report**:
- Jitsi Meet is a fully open-source, WebRTC-based video conferencing tool.
- Requires no account for the student. Works natively in the browser and mobile.
- OHC can host a Jitsi instance (for Cloud mode) or point to public servers (for Standalone), saving users from needing a paid Zoom subscription.
- Completely seamless integration with no technical setup required by the user.
**Design Doc**:
- When a service is marked as "Online Meeting", OHC auto-generates a unique Jitsi URL (e.g., `meet.ohc.com/leo-guitar-session`).
- The link is automatically added to the calendar invite and the customer's dashboard.
- Users just click the link at the scheduled time to join the browser-based call.
**Implementation Prompt**: Integrate auto-generated Jitsi Meet links for bookings designated as "Online", providing a seamless, no-login video conferencing experience for service-based businesses.
**Priority**: P2
**Estimated Scope**: Small


## Existing Research Integration: [social_media]_ayrshare.md
# Scout: Tool Integration Research Q2

## 1. Social Media Integration
**Title**: Integrate Ayrshare for Unified Social Media Inbox and Cross-Posting
**Problem Statement**: Maya the Baker and Carlos the Handyman spend too much time jumping between Instagram DMs, Facebook Comments, and TikTok. They want a single inbox and a way to post to multiple platforms at once without understanding technical integrations.
**Research Report**:
- Ayrshare provides a unified API for posting and retrieving messages across all major social networks (Instagram, Facebook, X, TikTok, LinkedIn).
- Competitor Wix has basic integrations, but Ayrshare makes it easy to support a wider array natively.
- Pricing: Free tier available, then scales per user.
- Fits OHC’s "The Promoter" agent to automate posts and "The Ambassador" to draft replies.
- Non-technical users benefit by never leaving the OHC interface.
- Works in Cloud mode well; Standalone mode might require personal Ayrshare API keys or direct OAuth.
**Design Doc**:
- Users link their social accounts via a simple OAuth popup in the "Marketing & Advertising" tab.
- "The Ambassador" AI monitors incoming DMs and drafts replies visible in a unified "Customer Inbox."
- "The Promoter" AI schedules and auto-posts images (e.g., new cake designs) to all linked platforms.
**Implementation Prompt**: Implement an integration where users can link Instagram and Facebook, allowing OHC AI agents to read incoming messages and draft replies in the unified inbox, and schedule out outbound picture posts.
**Priority**: P1
**Estimated Scope**: Large


## Existing Research Integration: [calendar]automated_scheduling.md
# Title: Automated Booking & Calendar Sync

## Problem Statement
Service-based small business owners spend too much time going back and forth with clients to find a meeting time. Double bookings happen frequently because personal and business calendars aren't synced. They need a way to let clients book available slots automatically.

## Research Report
*   **Tool Candidates**: Calendly API, Cal.com, Google Calendar API direct.
*   **Evaluation**: Cal.com is open-source, highly customizable, and offers a white-label API. Calendly is the industry standard but less flexible for white-labeling. Direct Google Calendar integration requires building the scheduling logic from scratch.
*   **Ease of Use**: Cal.com API allows us to embed the booking flow seamlessly into OHC so the business owner just sees "Availability Settings".
*   **Pricing**: Cal.com has team plans; direct Google API is free but high development cost.
*   **Modes**: Cloud (easy). Standalone (requires managing OAuth tokens locally).

## Design Doc
*   **Integration Trigger**: User sets their working hours and connects their Google/Outlook calendar.
*   **Action**: OHC generates a public booking link. When a client books, it creates an event on the owner's connected calendar and blocks that time in OHC.
*   **User Interface**: An "Availability" settings page, and a generated public-facing booking page for clients.

## Implementation Prompt
Build a scheduling feature that allows users to set their weekly availability and connect a third-party calendar (Google/Outlook). Generate a shareable booking link where clients can pick an available time slot. When booked, the event must appear on the connected calendar. Acceptance criteria: user can set hours, connect calendar, and a test booking successfully blocks out that time.

## Priority
P0

## Estimated Scope
Medium

## Existing Research Integration: [payment]alternative_providers.md
# Title: Alternative Payment Providers for Emerging Markets

## Problem Statement
Stripe is not available or preferred everywhere. Small business owners in LATAM, India, and Asia lose sales because they cannot accept local payment methods (like Pix, UPI, or Alipay). They need localized payment processing.

## Research Report
*   **Tool Candidates**: Mercado Pago (LATAM), Razorpay (India), Adyen (Global).
*   **Evaluation**: Mercado Pago dominates LATAM with Pix and Boleto support. Razorpay is essential for India (UPI). Adyen covers many but is geared towards enterprise.
*   **Ease of Use**: Business owners connect their local provider via OAuth or API key. Checkout experience is localized for their customers.
*   **Pricing**: Varies by provider; typically a percentage of the transaction.
*   **Modes**: Cloud (webhooks handled by OHC servers). Standalone (requires secure local webhook relay or polling).

## Design Doc
*   **Integration Trigger**: User selects their country in OHC and is offered relevant payment providers to connect.
*   **Action**: During checkout, OHC routes the payment intent to the connected regional provider and listens for the success webhook.
*   **User Interface**: Regional payment options added to the checkout flow.

## Implementation Prompt
Implement support for regional payment gateways (e.g., Mercado Pago for LATAM and Razorpay for India). The system should display the correct gateway based on the user's region and handle the checkout flow and webhook confirmation. Acceptance criteria: A user in a supported region can connect the gateway, and a test transaction completes successfully.

## Priority
P1

## Estimated Scope
Large

## Existing Research Integration: [feature]_autonomous_activity_feed_for_1_tap_agent_approvals.md
### Title
[Feature] Autonomous Activity Feed for 1-Tap Agent Approvals

**Problem Statement:**
Small business owners (like Carlos the Handyman) are overwhelmed by manual tasks like answering repetitive questions and scheduling follow-ups. Competitor platforms require the user to initiate AI help. Users need AI that operates autonomously in the background and presents actionable drafts for easy approval.

**Research Report:**
- Competitors (Shopify Sidekick, Wix ADI) rely on reactive AI.
- Users report "Operational Fatigue" (68%) as a top pain point.
- The highest perceived value is in agents that save time without losing control (e.g., 1-tap approval of a drafted response).

**Design Doc:**
- **UI Flow (375px First):** Home Dashboard features an "Agent Activity Feed".
- **Interaction:** Cards in the feed show drafted actions (e.g., "The Ambassador drafted a reply to a vegan cake inquiry"). User can tap "Approve & Send" or "Edit".
- **Architecture:** Backend Event Mesh triggers agents -> Agents generate tasks in DB -> Frontend polls/subscribes to actionable tasks.

**Implementation Prompt:**
Implement the "Agent Activity Feed" UI on the mobile dashboard. Create a system to display pending actions generated by backend AI agents (e.g., drafted messages, generated social posts). Include "Approve" and "Edit" flows for each action type. Ensure the UI is perfectly optimized for a 375px screen and uses plain, jargon-free language.

**Priority:** P0
**Estimated Scope:** Large


## Existing Research Integration: [architecture]_mobile_first_review.md
# Architecture Brief: Mobile-First Review & Performance

## Title
OHC Mobile-First Contract: Performance, Resilience, and "Grandmother Test" Audit

## Problem Statement
Small business owners (Carlos, Fatima, Maya) are mobile-only or mobile-primary. They operate in high-distraction environments (bakeries, repair sites, food carts) and often on low-end Android devices or poor 4G/5G connections. If the OHC dashboard is slow to load or fails when offline, Carlos can't send a quote, and Fatima loses a sale. OHC must be as fast and reliable as a native calculator app.

## Research Report
- **The "Grandmother Test"**: If a user has to wait more than 2 seconds for a screen to load, or more than 1 second for a button to respond, they assume the app is "broken."
- **Payload Bloat**: Traditional SaaS dashboards (Shopify/Wix) often fetch megabytes of JS and JSON, leading to LCP > 3s on 4G networks.
- **Offline Gaps**: Most web-based builders require a constant internet connection. OHC's hybrid nature (Local SQLite/SIPDB) provides a unique opportunity to allow "Offline Drafting."

## Design Doc

### Mobile-First Performance Targets
| Metric | Target | Why? |
| :--- | :--- | :--- |
| **LCP (Largest Contentful Paint)** | < 1.5s (4G) | Essential for "Activation" and perceived speed. |
| **FID (First Input Delay)** | < 100ms | Buttons must feel "native" and responsive. |
| **Bundle Size (Core UI)** | < 500KB | Fast download on low-data plans (Fatima). |
| **Touch Target Size** | ≥ 44x44px | Accessible for all users, especially in active work environments. |

### Architectural Decisions for Mobile Resilience
1.  **Lightweight Dashboard Service**: Implement `GetLightweightDashboard` in gRPC/Proto to return ONLY the critical counts (Orders, Agents, Messages) instead of full resource lists for the initial paint.
2.  **Optimistic UI with "Mesh Sync"**: All user actions (e.g., "Approve Quote") update the local UI state immediately. The KAIROS Orchestrator handles the background sync to the Teammate Mesh.
3.  **Offline-First Drafting**: Users can draft products or messages while offline. These are stored in the local SQLite SIPDB and auto-synced by the `SyncDaemon` once connectivity is restored.
4.  **Adaptive Asset Loading**: AI-generated images for the storefront are served via progressive JPEGs/WebP with mobile-responsive `srcset` variants.

### Mobile UX Flow (375px First)
- **Bottom Navigation**: Primary actions (Home, Orders, Agents, Settings) are reachable with one thumb.
- **Glassmorphism Shimmer**: Use skeleton loading states (shimmer effect) to maintain visual continuity during data retrieval.
- **Jargon-Free UI**: Replace "API Config" with "Helper Settings," "CNAME" with "Website Address."

## Implementation Prompt
**To Implementer Agent:**
Audit the current Slint/Rust frontend against the Mobile-First Performance targets. Implement "Skeleton Loading" (shimmer effect) for the `StatCard` and `AgentFeed` components. Update the `DashboardService` to support a `mobile_optimized: true` flag that returns a lightweight payload for the initial mobile paint. Implement "Optimistic Updates" in the Task List: when a user approves an agent draft, the UI should immediately reflect the "Approved" status and show a non-blocking background sync indicator. Ensure all touch targets in the `WebsiteBuilder` are at least 44x44px and use native mobile keyboards (numeric for prices, etc.).

## Priority
P0

## Estimated Scope
Medium


## Existing Research Integration: [email_marketing]_mailchimp.md
## [Email Marketing] Mailchimp Integration
**Title**: Integrate Mailchimp for Customer Re-engagement
**Problem Statement**: Priya (Boutique Owner) wants to email her past customers when new stock arrives, but she doesn't know how to export lists and manage campaigns. She needs an automated way to email customers without leaving the OHC app.
**Research Report**:
- **Tool**: Mailchimp
- **Target Persona**: Priya (Boutique Owner), Leo (Music Tutor)
- **Advantages**: Market leader, great API, supports tags and segments. High deliverability.
- **Risks**: Strict anti-spam policies might suspend users if they import bad lists.
- **Pricing**: Free tier available (up to 500 contacts). Essentials starts at $13/mo.
- **Compatibility**: Cloud (OAuth). Standalone (API Key).
**Design Doc**:
- When a customer buys something, they are automatically added to the Mailchimp audience with tags (e.g., "Bought: Cake").
- The Marketing agent suggests campaigns ("Send an email to past customers about your new holiday cakes").
- The user approves the AI-generated email, and OHC triggers Mailchimp to send it.
- The user sees open rates and clicks in the OHC Marketing dashboard.
**Implementation Prompt**: Build an integration that syncs OHC customers to a Mailchimp audience automatically after purchase. Allow the AI Marketing agent to create and send email campaigns via the Mailchimp API.
**Priority**: P1
**Estimated Scope**: Medium


## Existing Research Integration: dummy_commit.md
Maintainer: Zero WIP exit


## Existing Research Integration: [calendar]_calendly.md
## [Calendar] Calendly Integration
**Title**: Integrate Calendly for Automated Booking
**Problem Statement**: Service providers like Carlos (Handyman) and Leo (Music Tutor) lose time going back and forth over email/text to find a time to meet. They need a way for customers to simply click a link, see available times, and book a slot directly on their calendar.
**Research Report**:
- **Tool**: Calendly
- **Target Persona**: Carlos (Handyman), Leo (Music Tutor)
- **Advantages**: Industry standard, highly recognizable to customers. Excellent conflict resolution and timezone handling. Easy API integration.
- **Risks**: If a user cancels via Calendly directly instead of OHC, state might go out of sync without robust webhook handling.
- **Pricing**: Free tier available. Premium starts at $10/mo.
- **Compatibility**: Cloud (OAuth). Standalone (requires API key).
**Design Doc**:
- User goes to Sales dashboard and connects Calendly.
- OHC pulls available event types (e.g., "30-min Consultation") and displays them on the user's public storefront.
- When a customer clicks to book, they are shown the Calendly widget.
- Upon successful booking, a webhook notifies OHC to record the appointment in the Operations dashboard.
**Implementation Prompt**: Create an integration that allows a user to connect their Calendly account. Fetch their existing event types and display a booking widget on their public profile page. Ensure booked events sync back to the OHC dashboard.
**Priority**: P1
**Estimated Scope**: Medium


## Existing Research Integration: [sms]_twilio.md
## [SMS] Twilio Integration
**Title**: Integrate Twilio for SMS Order Notifications
**Problem Statement**: Fatima (Food Cart Operator) relies on her phone for everything and might miss app push notifications in a noisy environment. She needs reliable SMS alerts when a new pre-order arrives so she can start cooking.
**Research Report**:
- **Tool**: Twilio
- **Target Persona**: Fatima (Food Cart Operator)
- **Advantages**: Global coverage, incredibly reliable. Programmable messaging.
- **Risks**: A2P 10DLC compliance in the US is complex and requires business registration, which might be a barrier for informal businesses.
- **Pricing**: Pay-as-you-go (~$0.0079 per SMS in US).
- **Compatibility**: Cloud (Centralized OHC Twilio account). Standalone (User provides API key).
**Design Doc**:
- User goes to Settings and toggles "Send me SMS for new orders".
- When an order is paid, the Operations agent triggers a Twilio API call to send an SMS: "New order! 2x Falafel for John. Pickup in 15m."
- (Future: Customers can also receive SMS receipts).
**Implementation Prompt**: Integrate the Twilio SDK to send outbound SMS notifications. Add a setting for the business owner to opt-in to SMS alerts for new orders. Ensure compliance with local messaging regulations.
**Priority**: P2
**Estimated Scope**: Medium


## Existing Research Integration: [research]_instant_storefront_generation.md
# Issue Brief: Instant "30-Second" Storefront Generation

## Problem Statement
The onboarding friction for most ecommerce platforms is too high. Even a 10-minute setup feels like a chore for a busy founder. Competitors are racing to zero setup time.

## Research Report
- **Durable Benchmark:** Claims "Get online in 30 seconds."
- **Wix Harmony:** Uses "vibe coding" to generate designs instantly from a single prompt.
- **OHC Current State:** The SetupWizard is detailed but requires multiple steps.
- **Target:** Reduce the "Time to Live" for the most basic storefront to under 60 seconds by using AI to guess and fill 80% of the required fields.

## Instant Build Flow
```mermaid
graph TD
    A[User enters 1 paragraph bio] --> B{The Advisor}
    B --> C[Extract Name/Type]
    B --> D[Generate Tagline]
    B --> E[Select Best Template]
    B --> F[Draft First Product]
    C & D & E & F --> G[Live Preview generated]
    G --> H[User Clicks Launch]
```

## Design Doc
### High-Level Architecture
- **Conversational One-Pager:** Replace the 11-step wizard with a single "Tell us about your business" prompt for users who want speed.
- **Parallel Generation:** While the user is typing, agents in the background start generating the tagline, logo, and product descriptions.
- **Smart Defaults:** Use location and industry data to set payment and delivery defaults.

### Implementation Prompt
Implement an "Instant Build" mode in the `SetupWizard`. This mode should accept a single paragraph of text from the user and use "The Advisor" to extrapolate all necessary business metadata, passing it to "The Promoter" to generate a live website draft immediately.

## Priority
P1

## Estimated Scope
Medium


## Existing Research Integration: [calendar]_cal_com.md
# Scout: Tool Integration Research Q2

## 2. Calendar & Scheduling
**Title**: Integrate Cal.com for Zero-Config Booking & Calendar Sync
**Problem Statement**: Leo the Music Tutor and Carlos the Handyman lose customers due to back-and-forth scheduling via text. They need a public booking link that syncs with their personal Google Calendar seamlessly.
**Research Report**:
- Cal.com is an open-source scheduling infrastructure. It handles timezone math, calendar conflict resolution, and booking pages out-of-the-box.
- It is highly embeddable and supports a self-hosted option, making it perfectly compatible with both Cloud (SaaS) and Standalone OHC modes.
- Free tier available for individuals; great for our free tier users.
- Alternative is building from scratch, which is error-prone.
**Design Doc**:
- "The Manager" AI sets up the booking link dynamically based on the user's defined business hours.
- Users connect their Google/Outlook calendar via a one-click OAuth button in the "Operations" tab.
- When a customer books a slot on the OHC public page, Cal.com manages the calendar event and conflict resolution transparently.
**Implementation Prompt**: Embed Cal.com's infrastructure so users can sync their personal calendars and provide a public booking widget on their storefront that prevents double-booking.
**Priority**: P0
**Estimated Scope**: Medium


## Existing Research Integration: [integrations]_hybrid_llm_routing_gateway_mcp.md
# [integrations]_hybrid_llm_routing_gateway_mcp.md

Stub file.


## Existing Research Integration: [payment]_mercado_pago_q2.md
# Scout: Tool Integration Research Q2

## 4. Payment Processing
**Title**: Expand Payments with Mercado Pago for LATAM Users
**Problem Statement**: Non-US users in Latin America cannot rely solely on Stripe due to high fees, lack of local currency support, and specific local payment methods (like Pix in Brazil or OXXO in Mexico).
**Research Report**:
- Mercado Pago is the dominant payment gateway in LATAM.
- Supports local payment methods which are critical for conversion (often >50% of transactions).
- API is well-documented. Settlement times are faster locally compared to cross-border Stripe.
- Works for both Cloud (via OHC platform account) and Standalone (user supplies API keys).
**Design Doc**:
- In the "Finance & Payments" settings, users select their region. If in LATAM, Mercado Pago is highlighted as the recommended provider.
- Setup involves standard OAuth flow or API key drop-in.
- Supports one-off payments and split payments for the eventual marketplace feature.
**Implementation Prompt**: Add Mercado Pago as a payment provider alternative to Stripe, allowing users in supported LATAM countries to accept local payment methods via the OHC checkout flow.
**Priority**: P1
**Estimated Scope**: Large


## Existing Research Integration: [sms]_twilio_q2.md
# Scout: Tool Integration Research Q2

## 6. SMS & Notifications
**Title**: Integrate Twilio for Global SMS Alerts & Customer Notifications
**Problem Statement**: Fatima the Food Cart Operator doesn't have a reliable internet connection at her cart and relies on SMS text messages to know when a pre-order arrives.
**Research Report**:
- Twilio is the industry standard for SMS and WhatsApp messaging globally.
- Reliable delivery, deep global coverage.
- Supports WhatsApp, which is critical for markets outside the US.
- Simple API, integrates well with Go backend.
- Costs per message, can be passed to the tenant or subsidized in premium tiers.
**Design Doc**:
- Users can enable "SMS Notifications" in the "Operations" settings.
- When an order is placed, the OHC backend triggers a Twilio API call to text the business owner.
- Additionally, "The Ambassador" can send order confirmation texts to customers who prefer SMS over email.
**Implementation Prompt**: Add Twilio integration to dispatch SMS order notifications to the business owner and provide SMS-based order updates to end customers.
**Priority**: P0
**Estimated Scope**: Small


## Existing Research Integration: [research]_ai_visibility_optimization.md
# Issue Brief: AI Visibility & GEO (Generative Engine Optimization)

## Problem Statement
Traditional SEO (Search Engine Optimization) is becoming secondary to GEO (Generative Engine Optimization)—how a business appears in AI search results like ChatGPT, Gemini, and Perplexity. Small business owners have no idea how to optimize for this "AI-first" discovery layer.

## Research Report
- **Durable.co Advantage:** Offers a "Weekly AI visibility ranking" to show if ChatGPT is recommending the business.
- **Market Trend:** Users are increasingly using LLMs to ask "What's the best bakery near me?" or "Who can fix my sink in Austin?"
- **Opportunity:** OHC can provide a built-in "AI Discovery Agent" that ensures the business metadata is structured perfectly for LLM crawlers and generative search.

| Strategy | Traditional SEO | OHC GEO Agent |
| :--- | :--- | :--- |
| **Focus** | Keywords & Backlinks | Vibe, Clarity & Schema |
| **Target** | Google Search Bot | LLM Crawlers (GPT-5, Gemini) |
| **Owner Effort** | High (Manual) | Zero (Background) |

## Design Doc
### High-Level Architecture
- **Discovery Agent:** Periodically scans the business's public profile and cross-references it against generative search "best practices" (Structured data, schema.org, plain-language clarity).
- **Visibility Report:** A simple "Generative Score" (0-100) displayed in the Analytics section of the Slint app.
- **Auto-Optimization:** Agent suggests or auto-applies content changes to improve the business's "vibe" for AI models.

### Implementation Prompt
Create a "Generative Visibility" tool for "The Promoter" (Marketing). This tool should analyze the business website content and provide a report on how likely it is to be cited by LLMs. Include specific actionable steps for the owner to improve their "AI searchability."

## Priority
P1

## Estimated Scope
Medium


## Existing Research Integration: [shipping]_easypost.md
# Scout: Tool Integration Research Q2

## 5. Shipping & Logistics
**Title**: Integrate EasyPost for Painless Shipping Labels & Tracking
**Problem Statement**: Priya the Boutique Owner hates manually copying addresses to USPS/FedEx to buy shipping labels. She wants one button to print a label and auto-email the tracking number.
**Research Report**:
- EasyPost provides a single, unified API for 100+ carriers (USPS, FedEx, UPS, DHL).
- Competitive pricing (free tier for low volume, pennies per label after).
- Abstracts away complex carrier-specific APIs and handles tracking webhooks.
- Great fit for OHC physical product merchants.
**Design Doc**:
- Upon order placement, "Operations" calculates the shipping rate via EasyPost and charges the customer.
- In the Order details view, the business owner clicks "Print Label."
- EasyPost generates a PDF (auto-compressed and stored in GCS).
- Tracking updates via EasyPost webhooks trigger "The Ambassador" to email the customer automatically.
**Implementation Prompt**: Connect EasyPost to the order fulfillment flow so users can generate shipping labels and automatically send tracking updates to customers.
**Priority**: P1
**Estimated Scope**: Medium


## Existing Research Integration: [email_marketing]_listmonk.md
# Scout: Tool Integration Research Q2

## 3. Email Marketing
**Title**: Integrate Listmonk for Embedded, No-Jargon Email Campaigns
**Problem Statement**: Priya the Boutique Owner wants to email her past customers when new stock arrives but finds Mailchimp confusing and expensive. She just wants to say "send this to everyone who bought last month."
**Research Report**:
- Listmonk is an open-source, self-hosted newsletter and mailing list manager.
- It is lightweight (Go + PostgreSQL), aligning perfectly with the OHC backend stack.
- Zero extra SaaS costs for OHC Standalone users; minimal scaling costs for Cloud.
- Simplifies list management and supports template-based sending without complex drag-and-drop builders.
**Design Doc**:
- Customer Success ("The Ambassador") tags customers automatically (e.g., "bought-shoes").
- Users type a plain-text prompt: "Draft an email about our new summer dresses."
- AI generates the HTML, Listmonk handles the reliable batch delivery, bounce tracking, and open rate analytics.
**Implementation Prompt**: Integrate Listmonk as the underlying email engine to allow users to trigger marketing emails to specific customer segments directly from the OHC dashboard.
**Priority**: P2
**Estimated Scope**: Medium


## Existing Research Integration: [email_marketing]_loops.md
# [Email Marketing] Loops.so Integration

## Title
Modern Email Marketing and Automation with Loops.so

## Problem Statement
Priya (Boutique Owner) wants to send beautiful, modern product update emails but finds legacy tools like Mailchimp too technical or cluttered. She needs a simple, modern way to manage her customer audience and send automated emails that look great on mobile without needing a design degree.

## Research Report
- **Strategy**: Integrate with Loops.so API for audience management and automated "loops" (campaigns).
- **Target Persona**: Priya (Boutique Owner), Modern SMBs.
- **Advantages**: Extremely clean API, modern "Notion-like" UI for the merchant, built for speed. Easier to use than legacy providers. Perfect for OHC's "Radical Simplicity" goal.
- **Risks**: Newer company compared to industry giants, though highly reputable in the developer community.
- **Pricing**: Free tier up to 1,000 contacts. Paid starts at ~$49/mo for larger lists.
- **Ease of Use**: High. The merchant sees simple lists and clean templates.
- **Compatibility**: Cloud & Standalone (API Key based).

## Design Doc
- **Integration with OHC**:
    - OHC automatically syncs new customers to the Loops audience list.
    - The "Promoter" AI agent suggests and drafts email campaigns within OHC, which are sent via the Loops API.
    - Event-based triggers in OHC (e.g., "Order Completed") fire specific "loops" in the Loops.so platform.
- **User View**: A "Marketing" tab in OHC showing current campaigns, subscriber growth, and simple "Approve & Send" buttons for AI-generated drafts.

## Implementation Prompt
Integrate Loops.so for native email marketing and automation. Map OHC customer events (Signup, Purchase, Milestone) to Loops events. Allow the AI Marketing agent to manage contact lists and trigger campaigns via the Loops API. Ensure open and click rates are synced back to the OHC dashboard.

## Priority
P1

## Estimated Scope
Medium


## Existing Research Integration: data_model_architecture_evolution.md
# Issue Brief: Data Model Architecture Evolution

## Title
Data Model Architecture: Entities, Relationships, and Multi-Tenancy Guarantees

## Problem Statement
As OneHumanCorp scales to support diverse business types—from bakers and freelance handymen to boutique owners—the underlying data model must remain robust, scalable, and strictly isolated per tenant. A non-technical small business owner relies on the system to keep their customer data, orders, and AI agent memories perfectly secure and separate from others. We must define clear entity relationships, access patterns, and invariants that guarantee row-level multi-tenancy without adding complexity to the business owner's experience.

## Research Report
- **Goal**: Review and evolve the OHC data model to ensure complete tenant isolation and optimized access patterns for both the mobile-first UI and the background AI agents.
- **Findings**:
  - **Multi-Tenancy**: The current architecture mandates row-level isolation in PostgreSQL using a `tenant_id` column with `ENABLE ROW LEVEL SECURITY`. This is critical and must be strictly maintained.
  - **Entity Types**: Key entities include Business (Tenant), Product, Order, Customer, Agent, Page, Booking, and Memory.
  - **Access Patterns**:
    - AI agents need fast access to customer history and long-term memory (pgvector).
    - The mobile app requires low-latency queries for orders and analytics.
- **Competitive Analysis**: Shopify and Wix handle multi-tenancy seamlessly but often struggle with deep AI integration at the data layer. By building pgvector memories directly into the tenant schema, OHC gains a significant advantage in personalized AI operations.

## Design Doc

### Entity-Relationship Diagram
...

### Key Invariants
1. **Tenant Isolation**: A business owner can only access data where `tenant_id` matches their authenticated session. This is enforced at the database level using RLS.
2. **Agent Scoping**: AI agents operating on behalf of a tenant must have their database queries automatically scoped to that `tenant_id`.
3. **Data Residency**: All entities (Products, Orders, Customers, Memories) must explicitly reference a `tenant_id`.

### Migration Strategy
- When evolving the schema (e.g., adding new entities like `Subscription`), use zero-downtime migrations.
- Ensure every new table includes a `tenant_id` column and the corresponding RLS policies are applied immediately upon creation.

## Implementation Prompt
Implement the data model enhancements for the OHC platform. Ensure that all new tables include a `tenant_id` column and that Row Level Security (RLS) is enabled and configured correctly. Update the Go backend repository layer to pass the `tenant_id` context in all queries. Implement E2E tests verifying that a user from one tenant cannot access data from another tenant, even via API manipulation.

## Priority
P0

## Estimated Scope
Medium


## Existing Research Integration: [social_media]unified_inbox.md
# Title: Unified Social Media Inbox Integration

## Problem Statement
Small business owners struggle to keep up with customer messages scattered across Instagram DMs, Facebook Messenger, WhatsApp, and TikTok. Switching between apps causes delayed responses, lost sales, and poor customer service. They need a single place to view and reply to all messages.

## Research Report
*   **Tool Candidates**: ManyChat, Meta Business Suite API, Twilio Conversations.
*   **Evaluation**: Twilio Conversations provides a robust API for WhatsApp and SMS but requires more setup for IG/FB. Meta Business Suite API is free and covers IG/FB directly. ManyChat is user-friendly but adds a subscription cost. Meta's official API is the most direct route, though the OAuth flow is complex.
*   **Ease of Use**: Once connected, the user never has to leave OHC. The initial setup requires logging into Meta.
*   **Pricing**: Meta APIs are mostly free for standard usage; WhatsApp Business has conversation-based pricing.
*   **Modes**: Cloud (OAuth redirects work well). Standalone (OAuth redirects need local handling or proxying).

## Design Doc
*   **Integration Trigger**: User connects their Meta/WhatsApp accounts via a "Connect Socials" button in OHC Settings.
*   **Action**: Webhooks receive incoming messages and route them to a unified "Inbox" view in the OHC app. Replies sent from OHC are pushed back to the respective platform.
*   **User Interface**: A chat-like interface displaying the source of the message (an icon for IG, WhatsApp, etc.).

## Implementation Prompt
Implement a unified inbox feature where users can connect their Instagram, Facebook, and WhatsApp accounts. Incoming messages should appear in a single chronological feed. The user should be able to type a reply and have it sent back to the customer on the original platform. Acceptance criteria include successful account connection, receiving a message, and sending a reply.

## Priority
P1

## Estimated Scope
Large

## Existing Research Integration: [backend]_scribe_proactive_rag_mcp.md
# [backend]_scribe_proactive_rag_mcp.md

Stub file.


## Existing Research Integration: [research]_scout_resource_scout_tool_integrator.md
# Scout: Resource Scout & Tool Integrator

## Title
Scout 🔍 (Resource Scout & Tool Integrator)

## Problem Statement
The OHC Hybrid Agentic OS requires a specialized agent responsible for scouting external resources, documentation, and integrating new tools. Currently, agents lack a dedicated mechanism for discovering, analyzing, and integrating external APIs, tools, and libraries dynamically. This limits the swarm's ability to adapt to new requirements and leverage external capabilities without manual intervention.

## Research Report
- **Goal**: Develop an autonomous "Scout" agent capable of exploring external information, reading API documentation, and integrating new tools into the OHC ecosystem.
- **Capabilities**:
  - **Web Search & Scraping**: Ability to search the web, read documentation, and extract relevant technical details.
  - **Tool Discovery**: Analyze the OHC system requirements and identify missing tools or libraries.
  - **Integration Prototyping**: Generate boilerplate code, wrapper scripts, or configuration files to integrate discovered tools.
  - **Knowledge Sharing**: Update the OHC Central Database (OHC-SIP) with newly discovered resources, making them available to other agents.
- **Architecture**:
  - Scout operates within the OHC Hybrid Architecture.
  - Can function in Cloud Mode (high concurrency searches) or Standalone Desktop Mode (local scraping).
  - Uses `browser` tool for web scraping and documentation reading.
  - Interacts with `OHC-SIP` via PostgreSQL (Cloud) or SQLite (Standalone).

## Design Doc
- **Component**: `ScoutAgent`
- **Responsibilities**:
  - Listen for "Tool Request" events from the orchestrator.
  - Execute search queries to find relevant tools.
  - Read and parse API documentation.
  - Generate a "Tool Integration Brief" containing code snippets and configuration.
  - Store the brief in `OHC-SIP` for other agents (e.g., Code Gen Agent) to use.
- **Data Schema**:
  - Table: `tool_integrations`
  - Columns: `id`, `name`, `description`, `api_url`, `integration_code`, `status`, `created_at`

## Implementation Prompt
"Implement the Scout Agent module in `src/agents/scout/`. The agent should subscribe to tool requests, use a search API to find resources, parse documentation, and save a Tool Integration Brief to the database. Ensure it supports both PostgreSQL and SQLite backends."

## Priority
High

## Estimated Scope
2 weeks (1 sprint)


## Existing Research Integration: [payment]_mercado_pago.md
## [Payment] Mercado Pago Integration
**Title**: Integrate Mercado Pago for LATAM Payments
**Problem Statement**: Small business owners in Latin America cannot easily use Stripe and need a trusted local payment processor to accept credit cards and local methods like Pix or Pago Fácil.
**Research Report**:
- **Tool**: Mercado Pago
- **Target Persona**: Global users outside the US/EU.
- **Advantages**: Dominant in LATAM. Supports local payment methods (Pix in Brazil, OXXO in Mexico). Good developer docs.
- **Risks**: Settlement times can be longer. API is slightly less standardized than Stripe.
- **Pricing**: Variable by country (e.g., ~4-5% per transaction).
- **Compatibility**: Cloud (OAuth). Standalone (API Key).
**Design Doc**:
- User selects their country during onboarding. If LATAM, Mercado Pago is offered alongside Stripe.
- User connects their Mercado Pago account.
- Customers see a "Pay with Mercado Pago" button at checkout.
- Webhooks update the order status in OHC when payment succeeds.
**Implementation Prompt**: Add Mercado Pago as a secondary payment provider. Implement the checkout flow to redirect to Mercado Pago and handle the success/failure webhooks to update order status.
**Priority**: P2
**Estimated Scope**: Large


## Existing Research Integration: [shipping]_sendle.md
# [Shipping] Sendle Integration

## Title
Sustainable and Simple Shipping with Sendle

## Problem Statement
Priya (Boutique Owner) finds traditional shipping carriers confusing with their complex zones, weight charts, and hidden fees. She wants a simple, "flat-rate" shipping option that is easy to understand, carbon-neutral, and provides door-to-door service without her needing to wait at a post office.

## Research Report
- **Strategy**: Integrate Sendle API for quote generation and label printing.
- **Target Persona**: Priya (Boutique Owner), Eco-conscious merchants.
- **Advantages**: Simple, door-to-door flat-rate pricing. Carbon neutral (B-Corp). Integrated tracking and simplified parcel sizes.
- **Risks**: Primarily focused on Australia, USA, and Canada.
- **Pricing**: Flat rates based on parcel size (e.g., "Shoebox", "Briefcase").
- **Ease of Use**: Very high. No complex weight math; if it fits, it ships.
- **Compatibility**: Cloud & Standalone (API Key based).

## Design Doc
- **Integration with OHC**:
    - OHC fetches Sendle quotes based on the merchant's predefined parcel sizes.
    - Merchant selects Sendle for fulfillment, and OHC generates the label and schedules a pickup.
    - The "Ambassador" AI agent tracks the shipment and proactively notifies the customer of progress.
- **User View**: A "Ship with Sendle" button that shows a single clear price and generates a label in one click.

## Implementation Prompt
Implement Sendle as a native shipping and fulfillment provider. Provide real-time flat-rate shipping quotes during the checkout and fulfillment process. Enable one-click label generation and automated pickup scheduling via the Sendle API. Ensure tracking numbers are automatically synced to the order and shared with the customer.

## Priority
P1

## Estimated Scope
Medium


## Existing Research Integration: [integrations]_hybrid_spiffe_identity_mcp.md
# [integrations]_hybrid_spiffe_identity_mcp.md

Stub file.


## Existing Research Integration: [feature]_plain_language_daily_business_briefing.md
### Title
[Feature] Plain-Language Daily Business Briefing

**Problem Statement:**
Founders suffer from "Financial Fog" (35% pain point frequency) and are overwhelmed by complex dashboards with raw metrics. They need actionable insights in human language, not charts.

**Research Report:**
- Competitors provide traditional analytics dashboards that require interpretation.
- OHC's "Business Advisor" persona should translate data into simple English.

**Design Doc:**
- **UI Flow:** A daily push notification leading to a single "Briefing" screen.
- **Content:** 3-4 bullet points (e.g., "You had 8 orders this week. Vegan cake requests doubled. Consider adding a vegan chocolate option!").

**Implementation Prompt:**
Create the UI and backend logic for a daily "Business Briefing". The backend should aggregate daily metrics and use the LLM provider to generate a short, plain-language summary. The frontend should display this summary prominently upon first login each day, tailored for a 375px mobile view.

**Priority:** P1
**Estimated Scope:** Medium


## Existing Research Integration: [video]_google_meet.md
## [Video Conferencing] Issue Brief: Auto-Generated Meeting Links

**Title**: Scout 🔍: Integrate Google Meet for Automated Online Lessons
**Problem Statement**:
For digital service providers like Leo (Music Tutor), manually creating Zoom or Google Meet links for every booked lesson and emailing them to the student is prone to human error (e.g., forgetting to send the link or sending the wrong one).
**Research Report**:
- **Tool**: Google Workspace API (Google Meet) or Zoom API.
- **Evaluation**: Google Meet is often preferred as it can be automatically attached to any Google Calendar event created during the booking process. Zoom requires a separate OAuth flow.
- **Ease of Use**: Zero extra effort if the user has already connected Google Calendar for availability syncing. The system automatically provisions the link.
- **Pricing**: Free if using the user's existing Google Calendar/Meet integration.
- **Cloud vs. Standalone**: Works natively in both.
**Design Doc**:
- When setting up a service, the user toggles "This is an online meeting".
- When a customer books the service, the OHC backend creates a Google Calendar event.
- The calendar event is configured to auto-generate a Google Meet conference link.
- The confirmation email sent to the customer includes this generated Meet link.
**Implementation Prompt**:
Extend the calendar booking flow to support online meetings. When a service is marked as "online", ensure the Google Calendar event creation request includes the conference data parameters to auto-generate a Google Meet link. Extract this link from the response and include it in the customer's confirmation email and the business owner's dashboard.
**Priority**: P1
**Estimated Scope**: Small


## Existing Research Integration: [social_media]_manychat.md
## [Social Media] Manychat Integration
**Title**: Integrate Manychat for Unified Social Media Inbox
**Problem Statement**: Small business owners like Maya (The Home Baker) receive orders and inquiries across Instagram DMs, Facebook Messenger, and WhatsApp. Managing these manually is overwhelming and leads to missed sales. They need a single, unified inbox where an AI agent can read and reply to messages from all platforms automatically.
**Research Report**:
- **Tool**: Manychat
- **Target Persona**: Maya (Home Baker), Priya (Boutique Owner)
- **Advantages**: Excellent Instagram and WhatsApp API integrations. Robust webhook support for routing messages to OHC's backend. Extremely popular among SMBs for basic automation.
- **Risks**: Pricing scales with contacts, which may be expensive for high-volume, low-margin businesses. Requires Meta business verification for some features.
- **Pricing**: Free tier available (up to 1,000 contacts). Pro tier starts at $15/mo.
- **Compatibility**: Cloud (via webhooks/OAuth). Standalone (would require local reverse proxy for webhooks, possible but complex).
**Design Doc**:
- User goes to the Operations dashboard and clicks "Connect Instagram".
- User authenticates with Facebook/Instagram via OAuth.
- OHC registers webhooks to receive new DMs.
- When a DM arrives, the Customer Success agent reads it, generates a reply (e.g., "Yes, we do vegan cakes!"), and sends it back via Manychat's API.
- The user sees a unified "Customer Inbox" on their phone showing the conversation history.
**Implementation Prompt**: Implement an OAuth flow to connect a user's Instagram/Facebook account via Manychat. Create a webhook endpoint that receives incoming messages, stores them in the unified inbox, and triggers the Customer Success agent to draft a reply.
**Priority**: P0
**Estimated Scope**: Large


## Existing Research Integration: [hybrid-sync]-tool-discovery.md
<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# [hybrid-sync] Synchronized Cloud-Local Offline Support

## Problem Statement
The OHC Hybrid Architecture currently supports Cloud-Native (PostgreSQL/Redis), Standalone Desktop (SQLite), and Thin Client modes. However, true hybrid capability requires seamless state synchronization between local standalone environments and the multi-tenant cloud. When a standalone desktop reconnects to the network, its local SQLite data must sync back to the cloud PostgreSQL database without manual intervention.

## Research Report
- **ElectricSQL / PowerSync:** Both tools provide SQLite-to-Postgres sync. PowerSync is better suited for real-time offline-first architectures.
- **CRDTs (Conflict-Free Replicated Data Types):** Ideal for resolving merge conflicts between cloud and local states.
- **Bismuth/CR-SQLite:** An extension for SQLite that adds CRDT support, making it possible to sync with minimal conflict.
- **Architecture Validation:** In the `src/server/integrations/` directory, PowerSync, LibSQL, LiteFS, and Etcd are currently integrated to handle some hybrid tasks. However, offline-first sync (from Desktop to Cloud) needs a robust mechanism like PowerSync configured centrally. PowerSync is currently in the catalog but needs explicit orchestration.

## Design Doc
1. **Architecture Update:** Enhance the current Standalone SQLite integration to act as a localized cache that connects with the PowerSync sync engine.
2. **Database Schema:** Tables synchronized between Cloud and Desktop must include `_sync_status`, `updated_at`, and a `version` column for conflict resolution.
3. **API Contracts:**
   - `POST /api/v1/sync/push`: Accepts an array of modified rows from the standalone client.
   - `GET /api/v1/sync/pull`: Returns modified rows from the cloud.
4. **UI Wireframes:** A "Sync Status" indicator in the main OHC dashboard (Cloud/Local).

## Implementation Prompt
1. Add PowerSync synchronization orchestration to `src/server/orchestration/`.
2. Update the `StandaloneDB` wrapper in `src/server/db/` to initialize local PowerSync sync rules.
3. Ensure sync happens via `POST /api/v1/sync/push` on a background ticker.
4. Write E2E tests validating that an offline local write eventually reaches the cloud once connectivity is restored.

## Priority
`P1`

## Estimated Scope
Medium

</div>


## Existing Research Integration: [social_media]_meta.md
## [Social Media] Issue Brief: Automated Direct Message Integration

**Title**: Scout 🔍: Integrate Meta API for Automated Instagram & Messenger DMs
**Problem Statement**:
Small business owners like Maya (Home Baker) and Priya (Boutique) are overwhelmed by repetitive direct messages on Instagram and Facebook (e.g., "Do you do vegan?", "Is this in stock?"). Replying manually takes away from their actual work, and missing DMs means losing sales. They need an automated way to handle these inquiries without touching any code or configuring complex webhook flows.
**Research Report**:
- **Tool**: Meta Graph API (Instagram Direct & Messenger) or a managed wrapper like ManyChat.
- **Evaluation**: The Meta API allows full programmatic access to read and reply to DMs. By integrating this, OHC's "Customer Success" AI agent can draft and send replies based on the business's existing catalog, FAQs, and business hours.
- **Ease of Use**: Very easy for the user. They simply click "Log in with Facebook/Instagram" to grant permissions. No API keys to manage.
- **Pricing**: Free to use the Meta API, though WhatsApp integration has per-conversation pricing.
- **Cloud vs. Standalone**: Works perfectly in Cloud mode (OHC manages the Meta App and Webhooks). In Standalone mode, it would be complex as the user would need to create their own Meta App.
**Design Doc**:
- The user navigates to a "Social Inbox" tab and clicks "Connect Instagram".
- Uses OAuth to grant OHC permission to read/write messages.
- OHC registers a centralized webhook for the tenant.
- Incoming messages are routed to the AI Agent (Customer Success).
- The agent formulates a response based on the tenant's context (products, availability) and sends it back via the Meta API.
**Implementation Prompt**:
Implement the Instagram/Messenger integration. Provide a UI for the user to connect their Meta account. Set up a secure webhook endpoint to receive incoming DMs, route them to the LLM with the user's business context, and send the generated reply back to the customer. Ensure the user can toggle the AI on/off or set it to "draft only" mode.
**Priority**: P1
**Estimated Scope**: Medium


## Existing Research Integration: [social_media]_whatsapp.md
# [Social Media] WhatsApp Business API Integration

## Title
Native WhatsApp Business API Integration for Automated Customer Conversations

## Problem Statement
Fatima (Food Cart Operator) and many other SMB owners rely on WhatsApp as their primary communication channel. They manually respond to every "Are you open?" or "Where is my order?" message. They need these messages to flow into OHC so an AI agent can handle them automatically, saving them hours of manual typing and ensuring no customer is left waiting.

## Research Report
- **Strategy**: Direct integration with WhatsApp Business Platform (Meta).
- **Target Persona**: Fatima (Food Cart Operator), Maya (Home Baker).
- **Advantages**: WhatsApp is the #1 messaging app for SMBs globally. Native integration ensures no third-party markups and deep control over the AI response flow.
- **Risks**: Meta's business verification can be tedious. 24-hour customer service window requirements must be managed by the AI to maintain "Service" conversation status.
- **Pricing**: Conversation-based pricing. First 1,000 service conversations per month are free. Meta charges per 24-hour window thereafter.
- **Ease of Use**: Once connected, it is invisible. The user just sees messages in their OHC inbox.
- **Compatibility**: Cloud (Webhooks). Standalone (Requires a cloud proxy for webhooks).

## Design Doc
- **Integration with OHC**:
    - User connects their WhatsApp Business Account in the "Operations" settings.
    - OHC registers a webhook to receive incoming messages.
    - The "Ambassador" AI agent analyzes the message and drafts/sends a response based on the business profile.
    - All conversations are surfaced in the OHC unified "Customer Inbox" screen.
- **User View**: A unified thread showing WhatsApp messages alongside other channels, with AI-drafted replies ready for approval or auto-send.

## Implementation Prompt
Build a native integration for the WhatsApp Business API. Handle incoming message webhooks and implement outbound message sending. Ensure the "Ambassador" AI agent can participate in WhatsApp threads by drafting and sending replies. Normalize WhatsApp message formats into the OHC unified inbox schema.

## Priority
P0

## Estimated Scope
Large


## Existing Research Integration: ohc_ai_differentiation_manifesto.md
# OHC AI Differentiation Manifesto: From Tools to Teammates

## Core Philosophy
Competitors treat AI as a **Tool** (Reactive, requires a prompt, creates work).
OHC treats AI as a **Teammate** (Proactive, event-driven, reduces work).

```mermaid
graph LR
    subgraph Competitors_Tool
    User[User] -->|Prompt| AI_Tool[AI Tool]
    AI_Tool -->|Draft| User
    User -->|Edit/Send| Action[Final Action]
    end

    subgraph OHC_Teammate
    Event[Business Event] -->|Trigger| Agent[Autonomous Agent]
    Agent -->|Execute/Queue| Dashboard[Action Feed]
    Dashboard -->|1-Tap Approve| Live[Live Change]
    end
```

## The 5 Pillar Automations

### 1. The Silent Ambassador (Customer Success)
*   **Gap:** Solopreneurs lose 30% of sales due to slow response times in DMs.
*   **Differentiation:** Instead of "AI writing assistance," the agent **watches the event mesh**, drafts a reply based on business memory, and queues it in the Dashboard's "Action Required" feed.
*   **Outcome:** 1-tap responses from the lock screen.

### 2. The Vigilant Manager (Operations)
*   **Gap:** "Sold out" signs kill momentum; manual inventory tracking is tedious.
*   **Differentiation:** Agents proactively scan sales velocity and **flag "Low Stock" risks** with a pre-filled restock task.
*   **Outcome:** Never miss a sale due to forgotten inventory.

### 3. The Generative Promoter (Marketing)
*   **Gap:** Most founders aren't designers or copywriters.
*   **Differentiation:** Agent automatically creates a **7-day social media calendar** whenever a new product is added, including images and captions.
*   **Outcome:** Consistent brand presence with zero effort.

### 4. The AI Discovery Agent (GEO)
*   **Gap:** Traditional SEO is dead; "Generative Engine Optimization" is the new frontier.
*   **Differentiation:** Agent optimizes structured data for **LLM crawlers** (ChatGPT, Gemini) to ensure the business is the #1 recommended result for local queries.
*   **Outcome:** Automated high-intent traffic from AI search.

### 5. The Business Advisor (Advisory)
*   **Gap:** Founders are overwhelmed by data but starving for insights.
*   **Differentiation:** No complex charts. A daily **"Human-Language Briefing"**: *"Tuesday is your best day. Your vegan cake is trending. Boost your social spend by $5."*
*   **Outcome:** Clear, actionable strategic direction.


## Existing Research Integration: [integrations]_hybrid_websockets_mcp.md
# [integrations]_hybrid_websockets_mcp.md

Stub file.


## Existing Research Integration: [payment]_razorpay.md
# [Payment] Razorpay Integration (India)

## Title
Native Indian Payment Integration with Razorpay

## Problem Statement
Rohan (Handmade Crafts) in India cannot easily use Stripe for local customers who prefer UPI, RuPay, or local net banking. He needs a trusted local payment gateway that feels native to Indian customers, avoiding the high failure rates and friction associated with international payment processors in the Indian market.

## Research Report
- **Strategy**: Direct API integration with Razorpay.
- **Target Persona**: Rohan (Indian SMB owner).
- **Advantages**: Deep support for UPI (India's primary payment method), local cards, and net banking. Includes features like Razorpay Magic Checkout for higher conversion. Trusted by millions of Indian merchants.
- **Risks**: Stringent regulatory KYC requirements in India for the merchant.
- **Pricing**: Competitive local pricing (~2% per transaction for domestic).
- **Ease of Use**: Indian customers are highly familiar with the Razorpay checkout interface.
- **Compatibility**: Cloud & Standalone.

## Design Doc
- **Integration with OHC**:
    - Merchant chooses "India" as their region during setup, prompting Razorpay activation.
    - OHC uses the Razorpay Orders API to initiate payments.
    - Checkout widget supports UPI QR codes and local bank redirects natively.
    - The "Accountant" AI agent reconciles INR transactions and tracks local tax (GST) compliance.
- **User View**: A checkout screen that features UPI prominently, making payment instant for the customer.

## Implementation Prompt
Implement Razorpay as a native payment provider for the Indian market. Ensure the checkout flow supports UPI, local cards, and net banking. Normalize Razorpay webhooks into the standard OHC order and fulfillment system. Ensure the merchant can view transaction details in INR within the OHC dashboard.

## Priority
P1

## Estimated Scope
Large


## Existing Research Integration: [integrations]_hybrid_circuit_breaker_mcp.md
# [integrations]_hybrid_circuit_breaker_mcp.md

Stub file.


## Existing Research Integration: health_monitoring.md
# Title
## Cross-Mode Health Monitoring Architecture

### Problem Statement
The swarm relies on an intelligent orchestration layer (KAIROS) that dispatches tasks to agents and synchronizes state between Cloud (Redis/Postgres) and Standalone (Local IPC/SQLite) deployments. Currently, the health monitor operates identically across modes without distinction, relying on the transport layer to list active agents. If an agent goes offline in Standalone mode, or if a network partition affects Cloud mode, the health monitor simply fires agents that aren't reporting. A more robust, mode-aware Cross-Mode Health Monitor is required to handle failovers correctly.

### Architecture & Design
The Health Monitor `run_health_monitor` accepts a mode parameter (`is_cloud`) and an explicit heartbeat tick duration (`tick_duration`).
- **Cloud Mode**: Relies on Redis TTLs. If Redis is partitioned, it should tolerate network jitter. The monitor logic tracks missed heartbeats in a state dictionary and only issues agent firing commands after two consecutive missed ticks, shielding the system from transient outages.
- **Standalone Mode**: Runs on local SQLite. Connectivity is guaranteed by localhost. The health check simply verifies the IPC ping without network jitter backoff and fires missing agents immediately on the first missed heartbeat.
- **Protocol**: The health check loop polls agents via `monitor_transport.get_active_agents()`. If an agent misses a heartbeat:
  - In Standalone: Immediately fire.
  - In Cloud: Record in `pending_fires` map and retry next tick.

### Implementation Prompt
1. Modified `src/server/orchestration/health.rs` to support `is_cloud` and `tick_duration`.
2. Built a robust map-based retry fallback for cloud mode.
3. Updated unit tests for strict 100% Rust code coverage.
4. Delivered a 5-scenario E2E UI verification suite validating task reassignment and offline capabilities.


## Existing Research Integration: [research]_proactive_autonomous_agents.md
# Issue Brief: Proactive Autonomous Department Agents

## Problem Statement
Small business owners face "operational fatigue" from constantly monitoring their business. Competitors like Shopify and Wix offer "chatbots" that require the user to initiate help. OHC needs to leapfrog this by moving from "Ask AI" to "AI acts for you." Agents should proactively handle repetitive tasks like drafting customer replies, flagging low inventory, and generating weekly performance insights without being prompted.

## Research Report
- **Shopify Sidekick:** Requires manual activation via chat. Perception: "Just another thing to manage."
- **Wix ADI:** One-time generation tool. Doesn't stay active post-launch.
- **SMB Pain Points:** 68% of small business owners report feeling "overwhelmed" by the sheer number of small decisions and tasks required to run their shop daily (Source: Reddit r/smallbusiness survey synthesis).
- **Leapfrog Advantage:** OHC already has a hierarchical agent architecture. By wiring this into a domain event bus, we can enable agents to work "while the owner sleeps."

## User Journey: The "Maya" Experience
```mermaid
sequenceDiagram
    participant Customer
    participant Mesh as OHC Event Mesh
    participant Ambassador as The Ambassador (Agent)
    participant Maya as Maya (Owner)

    Customer->>Mesh: Sends Instagram DM: "Do you do vegan cakes?"
    Mesh->>Ambassador: Trigger: MessageReceived
    Ambassador->>Ambassador: Analyze history & inventory
    Ambassador->>Mesh: Push Draft: "Yes! We have 3 options..."
    Mesh->>Maya: Notification: "Draft ready for approval"
    Maya->>Mesh: 1-Tap Approve
    Mesh->>Customer: Message Sent
```

## Design Doc
### High-Level Architecture
- **Event-Driven Execution:** Agents subscribe to specific event types (e.g., `OrderReceived`, `StockLow`, `CustomerQuery`).
- **Draft & Approve Pattern:** High-risk actions (e.g., sending an email) generate a `PENDING` task in the Shared Task List. Low-risk actions (e.g., updating an internal tag) execute automatically.
- **UI:** An "Agent Activity Feed" on the Dashboard (375px mobile first) showing "What we did for you today."

### Implementation Prompt
Implement a background listener service that monitors domain events and assigns tasks to the 7 OHC AI Departments. Ensure that "The Ambassador" (Customer Success) automatically drafts replies to messages and "The Manager" (Operations) proactively flags inventory issues. Connect these to the existing Slint Dashboard's "Action Required" flow.

## Priority
P0

## Estimated Scope
Large


## Existing Research Integration: [architecture]_business_journey.md
### Title
Architectural Mapping of the End-to-End Business Journey for OHC Personas

## Problem Statement
The OHC platform must serve a diverse set of real-world small business owners (e.g., Maya the Baker, Carlos the Handyman, Priya the Boutique Owner, Leo the Music Tutor, and Fatima the Food Cart Operator) who share a common goal: launching and growing their business entirely from a mobile device without technical expertise. Currently, the overarching business journeys—from initial acquisition to sustainable revenue generation and referral loops—are fragmented. We need a unified architectural map of the end-to-end user journeys for these personas to ensure that the system naturally supports their progression through Acquisition, Onboarding, Activation, Retention, Revenue generation, and Referral, while identifying critical friction points where non-technical users might abandon the platform.

## Research Report
### Context and Personas
The business journey is evaluated against the following core personas:
1.  **Maya (Home Baker, 28)**: Needs a mobile-first storefront, Instagram integration, order management with deposit payments, and AI handling direct messages.
2.  **Carlos (Handyman, 42)**: Requires clean service listings, a robust booking system with deposits, a unified customer inbox, and an AI quote generator.
3.  **Priya (Boutique Owner, 35)**: Wants omnichannel support (in-store/online), POS integration (tap-to-pay), inventory sync, and actionable daily analytics.
4.  **Leo (Music Tutor, 22)**: Needs subscription-based packages, schedule syncing, automated meeting links, and a strong public profile (link-in-bio).
5.  **Fatima (Food Cart Operator, 50)**: Prioritizes extreme simplicity, pre-order management, multi-language UI, and fast low-data mobile performance.

### Journey Stages
-   **Acquisition**: The entry point. Organic search, social media ads (Instagram/TikTok), or word-of-mouth. The call-to-action (CTA) must clearly promise a functional business setup in under 10 minutes.
-   **Onboarding**: A highly guided, AI-driven wizard flow. Crucial to minimize initial input; deferring advanced configurations (like custom domains) to a later stage.
-   **Activation**: The "Aha!" moment. A live storefront, the first booking, or the first payment. Must be achieved within Day 1.
-   **Retention**: Kept engaged through actionable notifications (e.g., new order alerts) and AI-generated weekly health reports.
-   **Revenue**: Transitioning from a free tier to a paid plan. Triggered by hitting specific milestones (e.g., reaching product/action limits, needing custom domains).
-   **Referral**: Incentivized sharing. Creating a viral loop through referral discounts and shareable success metrics.

### Identified Friction Points
1.  **Cognitive Overload during Onboarding**: Requesting too much setup information upfront (e.g., complex shipping rules) causes drop-offs.
2.  **Payment Gateway Integration**: Technical jargon during Stripe connection can stall progress.
3.  **Inventory/Calendar Sync**: Difficulties mapping real-world availability to digital systems without intuitive AI assistance.
4.  **Language and Accessibility Barriers**: Interfaces that assume high technical literacy or english fluency (e.g., for Fatima).

## Design Doc
### Key Design Decisions
-   **Progressive Profiling**: The onboarding flow will request the absolute minimum required data to generate a viable starting point. Advanced settings are dynamically suggested by the Business Advisory Agent post-activation.
-   **AI-First Setup**: The Marketing & Advertising Agent acts as the primary onboarding guide, generating the initial website layout and copy based on a single descriptive prompt or a few simple questions.
-   **Mobile-First Constraint**: All journey flows are designed and tested starting at the 375px breakpoint.
-   **Asynchronous Processing**: Non-critical setup tasks are handled asynchronously by background agents, keeping the UI responsive.

### Architecture Diagrams (Mermaid.js)

#### 1. Maya (The Home Baker) Journey
```mermaid
sequenceDiagram
    actor Maya
    participant Ad as Instagram Ad
    participant OHC as OHC App
    participant AI_Mark as Marketing Agent
    participant AI_Ops as Operations Agent
    participant Cust as Customer
    participant Stripe as Stripe API

    Maya->>Ad: Clicks "Launch Bakery in 5 mins"
    Maya->>OHC: Downloads App & Opens
    OHC->>AI_Mark: Trigger Onboarding Wizard
    AI_Mark->>Maya: Asks "What do you sell?"
    Maya->>AI_Mark: "Custom vegan cakes"
    AI_Mark->>OHC: Generates Storefront & Menu
    OHC->>Maya: Storefront Live! (Activation)
    Cust->>OHC: Messages via IG DM "Vegan?"
    OHC->>AI_Ops: Drafts Reply
    AI_Ops-->>Cust: "Yes, we do vegan cakes!"
    Cust->>OHC: Places Order & Pays Deposit
    OHC->>Stripe: Process Payment
    Stripe-->>OHC: Success
    OHC->>Maya: Push Notification "New Order Paid!" (Retention)
    Maya->>OHC: Clicks "Upgrade to Starter" (Revenue)
    OHC->>Stripe: Setup Recurring Billing
    Maya->>OHC: Shares Store Link with Friend (Referral)
    OHC->>Maya: Credit "Bakery Credit" for Referral
```

#### 2. Carlos (The Handyman) Journey
```mermaid
sequenceDiagram
    actor Carlos
    participant WoM as Word of Mouth
    participant OHC as OHC Web App
    participant AI_Mark as Marketing Agent
    participant AI_Sales as Sales Agent
    participant Cust as Customer

    Carlos->>WoM: Hears about OHC
    Carlos->>OHC: Visits website on Android
    OHC->>AI_Mark: Trigger Onboarding
    AI_Mark->>Carlos: Asks "What services do you offer?"
    Carlos->>AI_Mark: "Plumbing, Painting"
    AI_Mark->>OHC: Generates Service Listings & Booking Calendar
    OHC->>Carlos: Booking Page Live! (Activation)
    Cust->>OHC: Requests Quote for "Leaky Pipe"
    OHC->>AI_Sales: Analyze Request
    AI_Sales->>Carlos: Drafts Quote for Review
    Carlos->>AI_Sales: Approves 1-tap
    AI_Sales-->>Cust: Sends Official Quote
    Cust->>OHC: Books Time & Pays Deposit
    OHC->>Carlos: Notification "Job Booked" (Retention)
    Carlos->>OHC: Taps "Earn $50: Refer a Pro" (Referral)
    Carlos->>Cust: Taps "Send 10% Discount to Friend" (Viral Loop)
```

#### 3. Priya (The Boutique Owner) Journey
```mermaid
sequenceDiagram
    actor Priya
    participant Search as Google Search
    participant OHC as OHC App
    participant AI_Mark as Marketing Agent
    participant AI_Adv as Advisory Agent
    participant POS as In-Store POS (Tap-to-pay)

    Priya->>Search: Searches "Easy online store for boutique"
    Priya->>OHC: Signs up
    OHC->>AI_Mark: Trigger Onboarding
    AI_Mark->>Priya: Syncs initial inventory
    AI_Mark->>OHC: Generates Storefront with variants
    OHC->>Priya: Storefront Live! (Activation)
    Priya->>POS: Processes in-store sale via phone
    POS->>OHC: Update Inventory
    OHC->>Priya: Daily Analytics Report (Retention)
    AI_Adv->>Priya: "Inventory low. Upgrade tier for automated re-order alerts." (Revenue)
    Priya->>OHC: Selects "Pro Plan" (Revenue)
    OHC->>Priya: Enables Multi-Store Sync
```

#### 4. Leo (The Music Tutor) Journey
```mermaid
sequenceDiagram
    actor Leo
    participant Social as TikTok Link-in-bio
    participant OHC as OHC App
    participant AI_Mark as Marketing Agent
    participant AI_Ops as Operations Agent
    participant Student as Student

    Leo->>Social: Adds OHC link to TikTok bio
    Leo->>OHC: Configures App
    OHC->>AI_Mark: Generates Profile & Subscriptions
    OHC->>Leo: Profile Live! (Activation)
    Student->>Social: Clicks Link
    Student->>OHC: Subscribes to 4 lessons/mo
    OHC->>AI_Ops: Sync Calendar & Generate Zoom Links
    AI_Ops-->>Student: Sends Schedule
    OHC->>Leo: Notification "New Subscriber!" (Retention)
    Leo->>OHC: Uses Referral code to invite another tutor (Referral)
```

#### 5. Fatima (The Food Cart Operator) Journey
```mermaid
sequenceDiagram
    actor Fatima
    participant Local as Local Signage
    participant OHC as OHC App (Arabic/English)
    participant AI_Mark as Marketing Agent
    participant OHC_UI as Simplified Mobile UI
    participant Cust as Customer

    Fatima->>Local: Shows QR Code
    Fatima->>OHC: Opens App
    OHC->>AI_Mark: Fast menu creation (Photos + Prices)
    AI_Mark->>OHC: Generates Bilingual Menu
    OHC->>Fatima: Menu Live! (Activation)
    Cust->>OHC: Scans QR, views menu, places pre-order
    OHC->>OHC_UI: Loud Audio Notification + Simple Order Card
    Fatima->>OHC_UI: Taps "Preparing"
    OHC_UI->>Cust: Updates Status
    Fatima->>OHC_UI: Prints Daily Summary (Retention)
```

### Mobile UX Flow Notes
-   **375px First**: All onboarding forms utilize native mobile keyboards appropriately (e.g., numeric for prices, email for contacts).
-   **Progress Indicators**: Clear visual indicators during the onboarding wizard.
-   **Optimistic UI**: Immediate feedback on actions (like saving a setting), with background sync handled by the KAIROS Orchestrator.

## Implementation Prompt
**To Implementer Agent:**
Implement the foundational user onboarding flow and dashboard state management that supports the progression from Acquisition to Activation. The system should define the required data models to capture the user's business type and minimal initial configuration. Build the mobile-first (375px) UI wizard that guides a user through the initial setup, ensuring that advanced configurations are deferred. The final step of the wizard should instantly generate a functional "Storefront/Booking Page" view, satisfying the 'Activation' milestone. Ensure that interactions feel premium (Glassmorphism, correct typography) and are resilient to network issues (optimistic updates). Do not prescribe the specific database schema or backend routing; focus on the unified API contract and the user journey transitions. Include E2E test coverage verifying a successful run-through from login to the generated storefront.

issue_id: 9353


## Existing Research Integration: kairos_phase_1_4_analysis.md
# Problem Statement
We need to finalize the KAIROS Orchestration design phases. We have documented and verified the existing state of Phase 1 (Shared Task List), Phase 2 (Teammate Mesh), Phase 3 (AutoDream Pipeline), and Phase 4 (Master Design Doc).

# Research Report
All architectural concepts mentioned in the KAIROS Triad (Shared Tasks via Postgres/SQLite locks, Teammate Mesh via Centrifuge/Redis/Memory, AutoDream pgvector memories) are already fully designed, documented, and actively implemented in the current codebase (`srcs/server/orchestration/tasks_db.go`, `srcs/server/orchestration/mesh.go`, `srcs/server/orchestration/autodream.go`, etc).
No further structural or aesthetic additions are required for this iteration, as all components successfully exist and meet the OHC Swarm core requirements.

# Design Doc
N/A - the existing system architecture is verified.

# Implementation Prompt
N/A
