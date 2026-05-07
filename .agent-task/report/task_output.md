# 🤖 AI Agent Department Architecture

## 1. Problem Statement
Small business owners often spend disproportionate amounts of time on administrative, operational, and customer-facing tasks that distract from their core craft. Maya the baker shouldn't spend her evening manually updating inventory; Carlos the handyman shouldn't miss leads because he's on a job and can't reply to a quote request. They need an "invisible team" that handles these complexities automatically, allowing them to focus on what they do best. The current OHC platform requires users to manually execute many of these workflows, adding friction and increasing the time-to-value for new merchants.

## 2. Research Report
### Market Context
- **Shopify:** Offers "Shopify Magic" (AI-generated product descriptions, basic chat), but requires manual triggering and configuration.
- **Wix:** Provides AI site generation, but operational tasks (booking, email marketing) remain largely manual.
- **Squarespace:** Similar to Wix; AI is positioned as a distinct "tool" rather than an integrated "employee."
- **Opportunity:** OHC can differentiate by offering a "done-for-you" operational model out-of-the-box. Instead of providing "AI tools," OHC provides "AI Departments" (e.g., "The Manager," "The Promoter") that act autonomously within guardrails.

### User Personas & Pain Points
- **Maya (Baker, 28):** Overwhelmed by Instagram DMs asking about custom cakes. Needs "The Ambassador" to handle initial inquiries and "The Operations Manager" to auto-schedule fulfillment.
- **Carlos (Handyman, 42):** Misses leads when working. Needs "The Salesperson" to generate quotes and follow up automatically.
- **Priya (Boutique, 35):** Struggles with multi-channel inventory. Needs "The Manager" to keep Shopify/in-store stock synced and "The Promoter" to run email campaigns based on slow-moving inventory.

### Proposed Solution: OHC AI Departments
We will introduce distinct "Departments" that mirror a real-world business structure. These departments operate in the background, triggered by events, schedules, or direct requests.

## 3. Design Doc

### 3.1 Architectural Philosophy
- **Event-Driven:** Agents react to platform events (e.g., `OrderPlaced`, `MessageReceived`).
- **Review vs. Auto-Execute:** Owners can configure trust levels. "Draft mode" requires Maya to approve an Instagram DM reply; "Auto mode" sends it directly.
- **Shared Memory:** All departments share a context of the business (inventory, policies, tone of voice, past customer interactions).
- **Mobile-First Visibility:** The owner sees a unified "Activity Feed" on their phone showing what the AI has done or needs approval for.

### 3.2 AI Departments
1.  **Operations ("The Manager"):**
    -   *Role:* Inventory tracking, order fulfillment, low-stock alerts.
    -   *Triggers:* `OrderPlaced`, `ProductUpdated`.
2.  **Marketing & Advertising ("The Promoter"):**
    -   *Role:* Social media posts, email newsletters, SEO updates.
    -   *Triggers:* Scheduled (e.g., weekly), `NewProductAdded`.
3.  **Sales & Acquisition ("The Salesperson"):**
    -   *Role:* Quote generation, lead follow-up.
    -   *Triggers:* `QuoteRequested`, `CartAbandoned`.
4.  **Customer Success ("The Ambassador"):**
    -   *Role:* Message replies, review requests.
    -   *Triggers:* `MessageReceived`, `OrderDelivered`.
5.  **Finance & Payments ("The Accountant"):**
    -   *Role:* Payment summaries, late invoice reminders.
    -   *Triggers:* `PaymentReceived`, `InvoiceOverdue`.

### 3.3 System Architecture Diagram

```mermaid
graph TD
    %% Core Entities
    BusinessOwner[Business Owner<br/>(Mobile App)]
    Customer[Customer<br/>(Storefront / Social)]
    EventBus[Central Event Bus]
    Memory[Shared Business Memory<br/>Context & Policies]

    %% AI Departments
    subgraph AI Departments
        Ops[Operations<br/>'The Manager']
        Mktg[Marketing<br/>'The Promoter']
        Sales[Sales<br/>'The Salesperson']
        CS[Customer Success<br/>'The Ambassador']
    end

    %% Interactions
    Customer -- "Places Order/Sends DM" --> EventBus
    EventBus -- "Routes Event" --> Ops
    EventBus -- "Routes Event" --> CS

    Ops -- "Reads/Writes Context" --> Memory
    CS -- "Reads Context" --> Memory

    Ops -- "Updates Inventory" --> EventBus
    EventBus -- "Triggers Next Action" --> Mktg

    Ops -- "Requires Approval/Logs Action" --> BusinessOwner
    CS -- "Drafts Reply/Logs Action" --> BusinessOwner
    BusinessOwner -- "Approves/Rejects" --> EventBus
```

### 3.4 Mobile UX Flow
1.  **Activity Feed:** The home screen of the OHC app shows a chronological feed:
    -   *Card 1:* "The Manager updated stock for 'Vegan Cake' to 0 (Sold Out)."
    -   *Card 2:* "The Ambassador drafted a reply to Sarah's Instagram DM. [Approve] [Edit] [Reject]"
2.  **Department Settings:** A simple settings screen allows users to toggle departments on/off and set the autonomy level (Draft vs. Auto).
3.  **Business Context:** A dedicated section to set the "Rules of the Business" (e.g., "Always be polite," "We never offer discounts on custom cakes").

### 3.5 AI Usage and Constraints
-   **Context Limits:** Throttled per tenant based on tier.
-   **Guardrails:** The AI cannot modify core business settings (like banking details) or execute irreversible financial transactions without explicit approval.

## 4. Implementation Prompt
**Target:** Implementer Agent
**CUJ (Customer User Journey):**
As a business owner, I want to see an "Activity Feed" on my mobile dashboard that shows actions taken by my "AI Departments." When a customer sends a message asking about a product, "The Ambassador" department should automatically draft a reply based on my business context and present it to me in the feed for approval.

**Acceptance Criteria:**
1.  Implement an event-driven mechanism where a `MessageReceived` event triggers the "Customer Success" AI agent.
2.  The agent must generate a context-aware draft response.
3.  The drafted response must appear in a unified "Activity Feed" accessible via the mobile UI.
4.  The business owner must be able to approve, edit, or reject the draft from the feed.
5.  Upon approval, the response is sent.
6.  The UI must reflect the "Draft" and "Sent" states clearly.
7.  Do NOT prescribe database schemas or specific API frameworks; design the core logic and interfaces to support this flow.

## 5. Metadata
-   **Priority:** P1
-   **Estimated Scope:** Medium
