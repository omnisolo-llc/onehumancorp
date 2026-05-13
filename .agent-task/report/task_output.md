# [architecture] AI Agent Department Architecture

## Title
AI Agent Department Architecture: Invisible, specialized AI teams running the business background operations.

## Problem Statement
Small business owners like Maya (the baker) and Carlos (the handyman) are overwhelmed. They don't just need a website—they need a team. When Maya gets an Instagram DM asking "do you do vegan cakes?" at 2 AM, she misses out on sales while she sleeps. When Carlos finishes a job, he forgets to ask for a review or follow up for future maintenance. Business owners need reliable, specialized "employees" who can manage marketing, handle customer success, run the operations, and keep the books without requiring complex setup, prompting, or code. They need an invisible organization that just works.

## Research Report
**Findings & Competitive Analysis:**
* **Shopify & Wix**: Provide basic automation rules (e.g., "if abandoned cart, send email") and chatbots, but lack an autonomous, multi-agent organization that coordinates complex operations without explicit flow building.
* **Squarespace**: Offers aesthetic storefronts but requires the user to manually trigger campaigns and manage scheduling.
* **GoDaddy**: Has some AI for initial website generation, but no active, continuously running AI departments to manage the business post-launch.
* **Pain Points**: Non-technical users find traditional workflow builders (like Zapier or Shopify Flow) too complex. They think in terms of roles ("I need someone to handle customer questions") rather than logic trees ("If webhook X fires, check condition Y, then execute Z").

## Design Doc

### Architecture Diagram
```mermaid
graph TD;
    subgraph Trigger Sources
        UserAction[User Action / Mobile App]
        WebEvent[Website Event / Order placed]
        ExternalEvent[External Integration / IG DM]
        Schedule[Time-based Schedule / Weekly]
    end

    subgraph KAIROS Orchestration Hub
        Router[Event Router]
        Memory[Long-term Embedded Vector Truth]
        Budget[Tenant Resource Budget & Throttle]
    end

    subgraph AI Departments
        Manager[Operations: "The Manager"]
        Promoter[Marketing: "The Promoter"]
        Salesperson[Sales: "The Salesperson"]
        Ambassador[Customer Success: "The Ambassador"]
        Accountant[Finance: "The Accountant"]
        Protector[Legal: "The Protector"]
        Advisor[Advisory: "The Advisor"]
    end

    UserAction --> Router
    WebEvent --> Router
    ExternalEvent --> Router
    Schedule --> Router

    Router <--> Memory
    Router -- Approves/Throttles --> Budget

    Router --> Manager
    Router --> Promoter
    Router --> Salesperson
    Router --> Ambassador
    Router --> Accountant
    Router --> Protector
    Router --> Advisor

    Manager --> Ambassador: Order Processed -> Notify Customer
    Salesperson --> Manager: Quote Accepted -> Create Job
    Accountant --> Advisor: Monthly Data -> Generate Report
```

### UI Wireframes & Mobile UX Flow (375px First)
* **Screen 1: The Team Dashboard (Mobile Home)**
  * Premium Glassmorphism UI (backdrop-filter: blur(20px)).
  * Shows avatars for each department (e.g., "The Manager", "The Ambassador").
  * Badges indicate activity: "The Ambassador drafted 3 replies to reviews."
* **Screen 2: Department Detail View**
  * Tapping "The Ambassador" shows a timeline of actions taken and pending approvals.
  * Toggle: "Auto-execute" vs. "Draft for review".
* **Screen 3: Approval Swipe UI**
  * When a department needs approval (e.g., "The Promoter" drafted a new Instagram post), the business owner gets a push notification.
  * Opening the notification reveals a Tinder-style swipe interface: Swipe right to approve and publish, swipe left to reject or edit.

### AI Agent Integration Points
* **Triggers**: Departments listen to the centralized event router. Events can be scheduled (e.g., Advisor runs on Sunday evenings), event-driven (e.g., order placed triggers Manager), or on-demand (user requests a new marketing campaign).
* **Coordination**: Agents communicate via the KAIROS state machine. When Operations finishes fulfilling an order, it emits an event that Customer Success consumes to send a personalized thank you.
* **Memory & Context**: Agents retrieve context from the centralized vector memory (AutoDream Pipeline). A customer's past orders, interactions, and preferences are summarized and injected into the prompt.
* **Approvals**: High-risk actions (spending money, posting publicly, sending legal notices) default to "Draft-for-review" requiring user swipe approval. Low-risk actions (tagging an order, summarizing data) auto-execute.
* **Budgeting**: The system tracks token and compute usage per tenant, gracefully throttling background tasks or prompting for upgrades when limits are approached.

### Key Design Decisions and Why
* **Role-Based Personas**: Framing AI as "The Manager" or "The Accountant" bridges the mental gap for non-technical users, making complex AI interactions feel like managing a real team.
* **Centralized Memory over Siloed Context**: All agents share a single source of truth about the customer so "The Salesperson" doesn't contradict "The Ambassador".
* **Swipe-to-Approve**: Reduces friction. The business owner acts as the final decision maker with minimal effort, enforcing control without complexity.
* **Event-Driven Coordination**: Loose coupling between departments allows us to add or upgrade individual roles without breaking the entire operational flow.

## Implementation Prompt
**For the Implementer Agent:**
Implement the "AI Team Dashboard" and "Approval Inbox" features for the mobile app (375px first, desktop additive).
* **Outcome**: The user can view their AI departments on the home screen, see a feed of what each "employee" has done today, and approve/reject drafted actions in a dedicated inbox.
* **Customer User Journey (CUJ)**:
  1. The user opens the app and sees "The Ambassador" has 2 items awaiting approval.
  2. The user taps "The Ambassador" and sees two drafted responses to recent customer reviews.
  3. The user approves one (which publishes it immediately) and edits the other before approving.
  4. The user returns to the dashboard and sees the activity feed updated.
* **Acceptance Criteria**:
  * Implement the dashboard showing department statuses.
  * Implement the Approval Inbox with the ability to approve, edit, or reject drafted actions.
  * Use the OneHumanCorp premium design system (Glassmorphism, Outfit/Inter typography, mobile-first spacing).
  * Ensure the UI is fully functional on a 375px viewport.
  * Connect the UI to the backend to fetch real drafted actions and submit approvals.

## Priority
P0

## Estimated Scope
Large
