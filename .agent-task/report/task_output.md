issue_title: "Architectural Deep Dive: Offline-Tolerant AI-Native Quote-to-Invoice (Q2I) Engine"
issue_description: |
  ## Problem Statement
  Service-based owners like Carlos (Handyman) and Nora (Agency Principal) struggle with disjointed workflows. They capture demand in DMs or calls, manually create estimates in one tool (or notes), collect deposits through another (Venmo/CashApp), and struggle to track final invoices. This friction costs them missed leads and uncollected revenue.
  They need an assistant-led, unified Quote-to-Invoice (Q2I) flow that works seamlessly from a 375px mobile screen, even without cellular service (e.g., in a customer's basement), drafting proposals and managing deposits automatically.

  ## Research Report
  **Market Gap:** Traditional tools like Joist, Jobber, or QuickBooks are highly administrative. They force the user to "build" a quote by searching through catalogs and entering line items manually. They do not proactively draft quotes based on conversation context or automatically follow up on unpaid deposits.

  **Competitive Analysis:**
  - *Jobber/Housecall Pro:* Good for operations, but zero AI agentic capabilities. High cognitive load for data entry on mobile.
  - *Shopify:* Built for e-commerce, terrible for custom service quoting and milestone billing.
  - *OHC Opportunity:* Leverage the Agent Feed to present pre-drafted quotes derived directly from the Work Triage communications. An owner just reviews, taps "Approve & Send", and the Finance Assistant tracks the deposit.

  ## Design Doc
  **Architecture Diagram (Mermaid)**
  ```mermaid
  erDiagram
      Tenant ||--o{ Customer : has
      Customer ||--o{ Conversation : engages
      Conversation ||--o{ QuoteDraft : triggers
      QuoteDraft ||--|| Quote : becomes
      Quote ||--o{ Invoice : generates
      Invoice ||--o{ PaymentIntent : tracks

      AI_Sales_Agent }|..|{ QuoteDraft : generates
      AI_Finance_Agent }|..|{ Invoice : monitors
  ```

  **Mobile UX Flow (375px First)**
  1. **Work Triage Feed:** Owner sees a card "Carlos, I've drafted a $450 quote for the drywall repair for Customer Sarah based on your WhatsApp chat."
  2. **Review Screen (Translucent Glass UI):** A single pane showing AI-extracted line items. The owner can tap a line to adjust the price with a native number pad.
  3. **One-Tap Action:** "Approve & Send via WhatsApp".
  4. **Offline Capability:** If Carlos is offline, the action is queued locally. A clear "Pending Sync" status token appears. It automatically sends when reconnected.

  **AI Agent Integration Points**
  - *Sales Assistant:* Reads context from Customer & Conversation memory. Extracts service requests and prices using Gemini Pro. Drafts the `QuoteDraft`.
  - *Operations Assistant:* Reserves a provisional time slot on the calendar if the quote includes an estimated date.
  - *Finance Assistant:* Creates the Stripe Payment Intent for the required deposit (e.g., 50% upfront). Listens for webhook success to mark the quote as "Approved by Customer".

  **Key Design Decisions**
  - *Offline-First Sync:* Use an optimistic UI with an Event Sourcing queue (`SyncQueue`) stored in local SQLite/IndexedDB.
  - *Row-Level Security:* All new entities (`quotes`, `invoices`, `sync_events`) must strictly enforce `tenant_id` via Postgres RLS.
  - *Zero Setup:* The catalog of services is built dynamically from past quotes if no formal catalog exists.

  ## Implementation Prompt
  **User-Facing Outcome:** Deliver a complete UI and background agent flow where a service owner can view an AI-generated quote draft, modify it intuitively on a mobile screen, and send it to a customer to collect a deposit.

  **CUJ (Critical User Journey):**
  1. Log into OHC app (mobile viewport).
  2. In the Agent Feed, tap on a newly generated Quote Draft card.
  3. Adjust the quantity of a line item.
  4. Tap "Send Quote".
  5. Verify that the quote appears in the customer's conversation history and a payment link is generated.

  **Acceptance Criteria:**
  - 100% functional on a 375px mobile screen with native-feeling interactions (no horizontal scrolling).
  - Playwright E2E test verifying the full CUJ from Feed to Sent Quote.
  - Must use the shared Translucent Glass design tokens.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []