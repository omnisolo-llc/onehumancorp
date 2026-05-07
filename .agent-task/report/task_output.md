# [architecture]_ai_agent_department

## Problem Statement
Small business owners (like Maya, a baker, or Carlos, a handyman) are overwhelmed by the cognitive overhead of managing distinct business domains (marketing, fulfillment, customer success, finance) using disjointed tools. Traditional SaaS requires manual data synthesis and task handoffs between systems. They need a unified platform where these functions act autonomously, mirroring the structure of a real business, but with a simple, jargon-free interface that requires minimal oversight and guarantees mobile parity (the "30-second rule").

## Research Report
Current market solutions (Shopify, Wix, Squarespace) offer integrations, but they are passive tools. For example, Shopify requires the user to manually draft an email campaign after analyzing sales data.

**Findings & Data:**
- **Cognitive Load:** SMB owners spend up to 40% of their time on non-core activities (administrative tasks).
- **Handoff Friction:** The biggest points of failure are between departments (e.g., Ops fulfilling an order but Customer Success failing to notify the customer promptly).
- **Adoption:** Tools that require "dashboard analytics" synthesis are often abandoned. Users prefer "next-action suggestions."

**Competitive Comparison:**
- **Shopify:** Excellent e-commerce ops, but marketing requires separate app subscriptions (Klaviyo) and manual rule configuration.
- **Wix:** Basic automation, but lacks predictive "Business Advisory" capabilities.
- **GoDaddy:** Simplified UI but extremely limited cross-domain AI autonomy.

**OHC Differentiation:**
OHC introduces an active, invisible "AI Department" model. The system operates autonomously via 7 functional agents: Operations ("The Manager"), Marketing ("The Promoter"), Sales ("The Salesperson"), Customer Success ("The Ambassador"), Finance ("The Accountant"), Legal ("The Protector"), and Business Advisory ("The Advisor").

## Design Doc

### Key Design Decisions
- **Unified Event Mesh:** Departments coordinate via the Orchestration Hub (KAIROS), using a Shared Task List and Teammate Mesh for durable, collision-free handoffs.
- **1-Tap Approval for High-Risk Actions:** Agent autonomy is tiered. Low-risk internal actions (e.g., tagging inventory) execute automatically. High-risk external actions (e.g., sending an email, issuing a refund, posting to social media) are placed in a draft state and pushed to the mobile dashboard for a single-tap approval.
- **Unified Memory Context:** Agents share context seamlessly. If Operations processes an order, Customer Success immediately knows the customer's history without explicit data syncing.
- **Tier-Based Budgeting:** Agent activity is throttled based on the tenant's subscription tier to control compute costs while ensuring the core experience is available to all.

### Architecture Diagram (Mermaid.js)

```mermaid
sequenceDiagram
    participant O as KAIROS Orchestrator
    participant Hub as Teammate Mesh (Hub)
    participant Op as Operations Agent ("The Manager")
    participant CS as Customer Success Agent ("The Ambassador")
    participant Fin as Finance Agent ("The Accountant")
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

### Mobile UX Flow (375px First)
1. **Push Notification:** User receives a notification on their mobile device (e.g., "The Ambassador has drafted a response to Maya's custom cake inquiry.").
2. **Dashboard Action Feed:** Opening the app displays a prominent, jargon-free Action Feed card.
3. **Draft Review:** The user taps the card to review the proposed action.
4. **1-Tap Approval:** A large, easily accessible button (≥44x44px touch target) allows the user to approve, edit, or reject the action.

## Implementation Prompt
**Title:** Implement "Draft-for-Review" AI Action Engine
**User Journey (CUJ):** As a small business owner, when my Customer Success agent ("The Ambassador") drafts a response to a high-value customer inquiry, I want to see a simple notification on my phone and approve it with one tap, so that I maintain control over my brand's voice without spending time writing emails from scratch.
**Acceptance Criteria:**
- The system must support an `ActionRisk` classification in the agent payload.
- High-risk actions must pause execution and enter a `pending_approval` state in the database.
- The UI must render a mobile-optimized (375px) approval card in the dashboard Action Feed.
- Approving the action via the UI must resume execution in the Orchestrator.
- All operations must enforce tenant isolation.

## Priority
`P1` (High)

## Estimated Scope
Medium
