# [Architecture] AI Agent Department Architecture Design

## Title
Implement KAIROS AI Agent Department Architecture for Autonomous SMB Operations

## Problem Statement
Small business owners—bakers, handymen, boutique owners, tutors, and food cart operators—are overwhelmed by the operational complexity of running a business. They don't have the time or technical expertise to manage websites, marketing, customer support, legal compliance, and financial reporting. Existing platforms (Shopify, Wix) treat AI as a bolted-on chatbot or a disjointed tool, forcing the owner to remain the manual operator of a complex machine. Users need an invisible, proactive team of AI "employees" organized into understandable "Departments" that autonomously manage the business, allowing the owner to focus entirely on their craft.

## Research Report
**Market Landscape & Competitive Analysis:**
- **Shopify:** Focuses on Sidekick, a reactive, chat-based assistant. It requires the user to prompt it ("How do I set up a discount code?") and remains highly manual. It does not proactively manage departments.
- **Wix:** Introduces AI for site generation and minor content creation, but lacks autonomous business operation capabilities. It functions as a design tool, not a business management OS.
- **Squarespace:** Very limited AI; primarily focuses on templates and aesthetics.
- **GoDaddy (Airo):** Basic domain-to-website generation, but stops at the setup phase. No ongoing operational autonomy.
- **Durable:** Excellent at "Speed to Site" (generation under 30 seconds), but struggles to provide deep, ongoing operational management across all business functions (finance, legal, customer success).

**OHC Competitive Advantage:**
OHC's differentiation lies in treating AI as infrastructure. By structuring AI into functional "Departments" (e.g., "The Manager," "The Promoter," "The Accountant"), OHC mirrors a real-world business structure. These agents work invisibly in the background, communicating with each other and proactively executing tasks or drafting them for review, rather than waiting for user prompts.

## Design Doc

### Architecture Diagram

```mermaid
graph TD
    subgraph Trigger Mechanisms
        Event[Platform Events: New Order, Message]
        Schedule[Scheduled Cron: Weekly Reports]
        Demand[User Action: "Create a flyer"]
    end

    subgraph KAIROS Event Mesh
        Router[Orchestration Hub / Router]
    end

    subgraph Memory & Context
        LongTerm[(Episodic / Long-Term Memory)]
        ShortTerm[(Short-Term / Active Context)]
    end

    subgraph AI Agent Departments
        Ops[Operations: The Manager]
        Marketing[Marketing & Adv: The Promoter]
        Sales[Sales & Acquisition: The Salesperson]
        CS[Customer Success: The Ambassador]
        Finance[Finance & Payments: The Accountant]
        Legal[Legal & Compliance: The Protector]
        Advisory[Business Advisory: The Advisor]
    end

    subgraph Execution & Approval
        AutoExecute[Auto-Execute Action]
        DraftReview[Draft for Review UI]
    end

    Event --> Router
    Schedule --> Router
    Demand --> Router

    Router --> Ops
    Router --> Marketing
    Router --> Sales
    Router --> CS
    Router --> Finance
    Router --> Legal
    Router --> Advisory

    Ops <--> Memory
    Marketing <--> Memory
    Sales <--> Memory
    CS <--> Memory
    Finance <--> Memory
    Legal <--> Memory
    Advisory <--> Memory

    Ops -.-> CS : "Order Fulfilled"
    Sales -.-> Ops : "Quote Accepted"
    Finance -.-> Advisory : "Monthly Revenue Down"

    Ops --> AutoExecute
    Marketing --> DraftReview
    CS --> DraftReview
    Finance --> AutoExecute
    Legal --> DraftReview
    Advisory --> AutoExecute
```

### UI Wireframes & Screen Flow (375px First)

**Screen 1: Home Dashboard (The Pulse)**
- **Header:** "Good Morning, Maya. Here’s what your team did while you slept."
- **Feed:** A scrollable list of recent agent actions.
  - *The Ambassador (CS):* "Drafted replies to 3 Instagram DMs asking about vegan options. [Review & Send]"
  - *The Manager (Ops):* "Processed custom cake deposit from Sarah. Added to calendar."
  - *The Promoter (Marketing):* "Scheduled a post for your new chocolate cake. [View Draft]"

**Screen 2: Department Settings (The Office)**
- **List View:** Operations, Marketing, Sales, Customer Success, Finance, Legal, Advisory.
- **Toggles:** For each department, a global "Autonomy Level" slider:
  - *Level 1: Draft Only (I review everything)*
  - *Level 2: Standard (Auto-execute routine tasks, draft complex ones)*
  - *Level 3: Full Autopilot (Just give me a summary)*

**Screen 3: Review & Approve Flow**
- **Trigger:** Tapping a "[Review & Send]" action on the Home Dashboard.
- **Content:** Displays the AI-generated artifact (e.g., a drafted email reply or a drafted social media post).
- **Actions:**
  - Primary button (44px height): "Approve & Send"
  - Secondary button: "Edit manually"
  - Tertiary button: "Regenerate"

### Mobile UX Flow
1. User receives a push notification: "The Ambassador has 3 draft replies for your review."
2. User taps notification, opening the App directly to the Review & Approve flow.
3. User swipes right on each draft to instantly approve, or taps to edit using the native mobile keyboard.
4. Once all drafts are processed, a success animation plays, and the user is returned to the Home Dashboard.

### AI Agent Integration Points
- **Operations ("The Manager"):** Listens for order placements, booking confirmations, and inventory changes. Updates calendars, tracks stock levels, and coordinates pickups.
- **Marketing & Advertising ("The Promoter"):** Triggers on demand (user requests a flyer) or on schedule (weekly social media posts).
- **Sales & Acquisition ("The Salesperson"):** Triggers on incoming inquiries or abandoned cart events. Generates quotes and follow-ups.
- **Customer Success ("The Ambassador"):** Triggers on incoming messages across integrated channels (Instagram, Email, WhatsApp) and order status changes.
- **Finance & Payments ("The Accountant"):** Listens for payment events, deposit completions, and subscription renewals.
- **Legal & Compliance ("The Protector"):** Triggers on new product creation (to generate terms/policies) or new custom orders (to draft contracts).
- **Business Advisory ("The Advisor"):** Triggers on a weekly schedule. Aggregates data from all departments to produce actionable business health reports.

### Key Design Decisions & Why
1. **Event-Driven Coordination:** Departments must coordinate asynchronously via an event mesh (e.g., Operations emitting an "Order Fulfilled" event that Customer Success listens to). *Why:* Prevents tight coupling and allows agents to act independently based on business state changes.
2. **Draft vs. Auto-Execute Toggles:** Users must have the ability to explicitly set autonomy levels per department. *Why:* Builds trust. A user might trust "The Manager" to auto-update inventory but want to review every message "The Ambassador" sends to customers.
3. **Unified Memory Layer:** All departments share access to a central memory and context store. *Why:* Prevents the "amnesic agent" problem. "The Salesperson" needs to know that "The Ambassador" just handled a complaint from a customer before trying to upsell them.
4. **Tenant Budgeting & Throttling:** AI usage limits (actions per month) are tracked centrally and decremented as agents act. When the limit is approached, "The Advisor" suggests a tier upgrade. *Why:* Ensures predictable COGS and clear upselling paths based on value delivered.

## Implementation Prompt
**Task for Implementer:** Build the foundational state and routing logic for the OHC AI Agent Departments.
1. Implement the high-level routing mechanism that dispatches events, schedules, or user demands to the appropriate AI Department (Operations, Marketing, Sales, Customer Success, Finance, Legal, Advisory).
2. Create the "Autonomy Level" setting for each department, supporting both "Draft-for-Review" and "Auto-Execute" modes.
3. Build the backend and mobile-first UI for the "Review & Approve" flow, allowing a user to see drafted actions, edit them, or approve them with a single tap.
4. Ensure all departments log their actions to a centralized feed visible on the Home Dashboard.
5. All UI must be built mobile-first (375px), with touch targets at least 44x44px.
6. Acceptance Criteria: A test user can trigger an event that causes an agent to generate a draft. The user can view the draft in the UI, edit it, and approve it. An event triggered for an auto-execute department must successfully bypass the review step and log directly to the dashboard feed. Implement at least five E2E/UI tests covering these flows from login to completion.

## Priority
P0 (Critical)

## Estimated Scope
Large
