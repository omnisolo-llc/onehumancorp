# AI Agent Department Architecture

## Title
AI Agent Department Architecture: Invisible Employees for Small Businesses

## Problem Statement
Small business owners like Maya (the baker), Carlos (the handyman), and Fatima (the food cart operator) spend countless hours managing the "back office" of their businesses. They are overwhelmed by answering Instagram DMs at 2 AM, sending invoices, following up on quotes, and managing inventory. They don't have the budget to hire a dedicated support team or marketing manager, and they don't want to learn complex software tools to automate these tasks. They need "invisible employees"—AI agents that act like real departments in a company, handling operations, marketing, sales, and customer success naturally and invisibly, so they can focus on their actual craft.

## Research Report
### Findings & Persona Needs
- **Maya (Baker):** Needs "The Manager" to handle custom cake deposits and "The Ambassador" to reply to late-night DMs.
- **Carlos (Handyman):** Relies on "The Salesperson" to generate quotes from service requests and follow up with leads.
- **Priya (Boutique):** Requires "The Promoter" to sync inventory to social media and send daily analytics.
- **Leo (Tutor):** Needs "The Operations" team to manage booking calendar syncs and auto-generate meeting links.
- **Fatima (Food Cart):** Depends on "The Manager" for multi-lingual pre-order tracking and notifications on her low-end Android device.

### Competitive Analysis
| Feature | OneHumanCorp (OHC) | Shopify | Wix | Squarespace |
|---------|--------------------|---------|-----|-------------|
| **AI Abstraction** | "Departments" (e.g., The Accountant) | Plugins & Apps | Basic AI Text Gen | Basic Setup AI |
| **Setup Time** | < 10 mins (Zero Code) | Hours/Days | Hours | Hours |
| **Automation** | Invisible background agents | Complex Zapier/Rules | Limited | Manual |
| **Mobile-First** | Native parity, runs fully on phone | Desktop-centric admin | Desktop-centric | Limited mobile admin |

## Design Doc
### High-Level Architecture
The AI Agent Departments function as discrete, specialized entities that coordinate to run the business. They use episodic memory to understand the business context and coordinate via an event-driven internal mesh.

#### Architecture Diagram

```mermaid
graph TD;
    User[Customer via Instagram/Web] -->|Interaction| Storefront[OHC Storefront / Inbox];
    Storefront --> Router[Central Router / Orchestrator];
    Router -->|Order Placed| Ops[The Manager: Operations];
    Router -->|Question Asked| Support[The Ambassador: Customer Success];
    Router -->|Quote Requested| Sales[The Salesperson: Sales & Acquisition];
    Ops -->|Payment Required| Fin[The Accountant: Finance];
    Ops -->|Fulfillment Update| Support;

    subgraph AI Agent Departments
        Ops
        Support
        Sales
        Fin
        Marketing[The Promoter: Marketing]
        Legal[The Protector: Legal]
        Advisory[The Advisor: Business Strategy]
    end

    AI Agent Departments --> Memory[(Long-term Episodic Memory Vector DB)];
    AI Agent Departments --> Approval[Approval & Throttling Gate];
    Approval -->|Auto-Execute or Draft| Output[Action: Email, SMS, Web Update];
```

#### Mobile UX Flow (375px First)
1. **Home Screen (The Dashboard):** A clean feed showing what the "employees" did today. e.g., "The Ambassador replied to 4 Instagram DMs."
2. **Department Settings:** User taps a department (e.g., "The Promoter").
3. **Toggle Capabilities:** Simple toggles: "Post weekly updates to Instagram", "Reply to comments".
4. **Approval Mode:** A toggle for "Auto-run" vs "Review drafts before sending".

#### Key Design Decisions
- **Event-Driven Triggers:** Departments are triggered by real-world events (new order, message received, weekly schedule) rather than manual user prompts.
- **Inter-Department Coordination:** The Manager (Ops) successfully completes an order, which emits an event that The Ambassador (CS) catches to send a thank-you note.
- **Memory & Context:** All departments share a central memory vector store, ensuring "The Salesperson" knows if a customer previously complained to "The Ambassador."
- **Budgeting & Throttling:** Actions are gated by tier limits. If "The Promoter" hits the monthly limit, tasks are paused and the user is gently prompted to upgrade.

## Implementation Prompt
**To the Implementer Swarm:**
Your task is to build the AI Agent Department orchestration layer.
- **CUJ (Critical User Journey):** A user (Maya) sets up her store and enables "The Ambassador" to handle customer inquiries. A customer sends a message asking about vegan options. The system routes this to The Ambassador, which checks the store's memory, drafts a polite reply confirming vegan options, and sends it automatically.
- **Acceptance Criteria:**
  - Departments must be abstracted behind friendly names (The Manager, The Ambassador).
  - Agents must subscribe to business events and trigger without manual user intervention.
  - Implement a shared memory interface so departments can read/write context about specific customers.
  - Implement an approval gate that supports both "Draft for Review" and "Auto-Execute" modes based on user settings.
  - UI must reflect actions taken by the agents in a simple feed view.

## Priority
P0

## Estimated Scope
Large
