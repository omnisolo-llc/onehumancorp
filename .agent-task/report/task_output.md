# OHC AI Agent Department Architecture

## 1. Title
AI Agent Department Architecture

## 2. Problem Statement
Small business owners (Maya, Carlos, Priya) often struggle to manage the various operational aspects of their businesses (sales, marketing, customer service, finance) without hiring staff or becoming overwhelmed by administrative tasks. They need a system that invisibly handles this complexity, operating like a complete organizational team, while maintaining a unified context and requiring only high-level approvals from the owner. The gap in current platforms is the lack of interconnected, autonomous "departments" that collaborate on workflows (like fulfillment or marketing campaigns) based on a shared understanding of the business's data and history.

## 3. Research Report
- **Competitive Landscape**: Traditional website builders (Wix, Squarespace, GoDaddy) and e-commerce platforms (Shopify) offer static tools and disjointed automation rules (e.g., "if this then that"). They do not offer proactive, autonomous agents that manage complete workflows. For example, Shopify requires third-party apps for marketing automation and customer service, leading to fragmented data and inconsistent user experiences. Wix and Squarespace offer basic AI for site generation but lack operational AI agents.
- **The OHC Advantage**: OHC organizes its AI into functional "Departments" (Operations, Marketing, Sales, Customer Success, Finance, Legal, Advisory). This structure mirrors real business operations, making it intuitive for non-technical users.
- **Contextual Memory**: A key differentiator is the shared, long-term memory (AutoDream pipeline via pgvector), allowing agents to recall past interactions and business history, unlike isolated chatbots or simple rule-based triggers.

## 4. Design Doc

### 4.1 Architecture Diagram
```mermaid
sequenceDiagram
    participant O as KAIROS Orchestrator
    participant Hub as Teammate Mesh (Hub)
    participant Op as Operations Agent (The Manager)
    participant CS as Customer Success Agent (The Ambassador)
    participant Fin as Finance Agent (The Accountant)
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

### 4.2 Mobile UX Flow (375px First)
- **Dashboard Interface**: The mobile app dashboard provides a unified "Activity Feed" summarizing agent actions across all departments.
- **1-Tap Approvals**: For "Draft-for-Review" actions (e.g., sending a marketing email, issuing a refund), the owner receives a push notification. The app displays a clear, jargon-free summary (e.g., "The Ambassador has drafted a reply to Maya's customer.") with simple "Approve" or "Edit" buttons. Touch targets are large (≥ 44x44px).
- **Advisory Reports**: Weekly health reports from "The Advisor" are presented using accessible charts and simple insights (e.g., "Vegan cakes are trending. We should promote them.") rather than raw data tables.
- **Design Tokens**: The UI utilizes premium OHC design tokens: Glassmorphism overlays for modals, Outfit/Inter typography for legibility, and subtle shimmer effects during loading states to meet the "Grandmother Test."

### 4.3 AI Integration Points
- **Department Interactions**: Departments communicate via the KAIROS Orchestrator's Shared Task List and Teammate Mesh, using distributed locks to prevent collisions.
- **Unified Context (AutoDream)**: All agents read from and write to the shared `autodream_memories` table (via pgvector) to maintain a holistic view of the business and customer interactions.
- **Approval Tiers**: Actions are categorized as "Auto-Execute" (low risk, internal updates) or "Draft-for-Review" (high risk, customer-facing).

### 4.4 Key Design Decisions
- **Functional Naming**: Organizing AI into departments with friendly names (e.g., "The Promoter" instead of "Marketing Automation Module") reduces cognitive load and intimidation for non-technical users.
- **Shared Memory vs. Isolated Context**: Implementing a central memory store ensures all agents operate from a single source of truth, preventing contradictory actions (e.g., Customer Success offering a discount while Finance flags an unpaid invoice).
- **Graceful Degradation (Usage Tiers)**: Agent activity is tied to SaaS subscription tiers. When limits are reached, actions are paused, and clear upgrade prompts are displayed to the user.

## 5. Implementation Prompt
**To Implementer Agent:**
Implement the foundational event routing and state management for the AI Agent Departments within the KAIROS Orchestrator. The system should define the structure for agent missions, incorporating an `ActionRisk` level (e.g., Auto-Execute vs. Draft-for-Review). Build the "Draft-for-Review" workflow engine, allowing agents to submit high-risk tasks into a pending state. Create the corresponding callback endpoints to handle approval or rejection signals from the mobile dashboard. Ensure that inter-departmental handoffs (e.g., Operations marking an order ready, triggering Customer Success to draft a message) are durably managed using the Teammate Mesh and distributed locks. Do not prescribe specific LLM implementations or database schema details; focus on the unified API contract and the user-facing workflow behavior. Ensure the feature is fully testable and includes coverage for the approval transition states.

## 6. Priority
P0

## 7. Estimated Scope
Large
