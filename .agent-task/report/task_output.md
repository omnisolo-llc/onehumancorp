issue_title: "Autonomous Work Triage & Unified Owner Feed Architecture"
issue_description: |
  ## Problem Statement
  Small business owners and operators (like Maya the Baker or Carlos the Handyman) are overwhelmed by incoming signals from disparate sources: Instagram DMs, SMS messages, missed calls, website form fills, new orders, and inventory alerts. They do not have the time or operational expertise to constantly monitor multiple dashboards, classify the intent behind every message, draft replies, and decide what action to take next. Traditional "unified inbox" solutions only consolidate messages into a single view, which is essentially just a longer list of things to read.

  What owners truly need is a proactive, mobile-first work assistant that triages these incoming signals, groups them into meaningful contexts, automatically drafts the appropriate responses or operational actions, and pushes an actionable "Agent Feed" to their phone. The product gap is moving from *information aggregation* to *actionable AI automation*.

  ## Research Report
  ### Competitive Analysis
  - **Traditional Helpdesks (Zendesk, Freshdesk):** Built for large support teams; way too complex and expensive for a single owner/operator. Requires manual triage rules.
  - **Shopify Inbox / Wix Inbox:** Consolidates store-related messages but lacks deep, autonomous AI drafting that spans across booking, inventory, and complex customer relationship contexts. Mostly reactive.
  - **Meta Business Suite:** Groups FB/IG messages but does not connect to the owner's booking or inventory systems. No proactive action drafting.
  - **OHC Differentiation:** OHC will implement a proactive **Agent Feed**. Instead of a static inbox, the OHC "Work Triage" engine intercepts signals, uses the LLM to classify intent, orchestrates the appropriate AI Agent (e.g., The Ambassador for customer service, The Operations Manager for scheduling) to draft a response or propose an action, and delivers an Action Card to the owner's 375px mobile screen for a 1-tap approval.

  ### User Personas & Pain Points
  - **Maya (Baker):** Gets DMs asking "Do you have vegan cake?" She needs the system to check inventory and draft the reply while she is baking.
  - **Carlos (Handyman):** Gets web leads while on a job. Needs the system to automatically draft a quote based on the inquiry and add it to his feed to review when he finishes his current job.
  - **Fatima (Food Cart):** Receives pre-orders. Needs the system to summarize them into a simple, prioritized prep list without her needing to open a laptop.

  ## Design Doc
  ### Architectural Overview
  We are proposing the **Autonomous Work Triage & Unified Feed Architecture**. This system acts as the central nervous system of OHC, processing all inbound events and transforming them into actionable feed items for the owner.

  ```mermaid
  graph TD
      subgraph Inbound Signals
          Webhooks[Webhooks: Stripe, IG, SMS]
          InternalEvents[Internal Events: Inventory, Orders]
      end

      subgraph Work Triage Engine
          EventBus[Event Bus / Queue]
          IntentClassifier[LLM Intent Classifier]
          RAG[Context Resolution via RAG]
      end

      subgraph AI Departments
          Ambassador[The Ambassador: CS]
          OpsManager[Operations Manager]
          Sales[Sales Agent]
      end

      subgraph Owner Feed UX
          ActionCard[Action Card Generator]
          MobileApp[Mobile App: 375px Viewport]
      end

      Webhooks --> EventBus
      InternalEvents --> EventBus
      EventBus --> IntentClassifier
      IntentClassifier --> RAG
      RAG --> Ambassador
      RAG --> OpsManager
      RAG --> Sales
      Ambassador --> ActionCard
      OpsManager --> ActionCard
      Sales --> ActionCard
      ActionCard --> MobileApp
  ```

  ### Multi-Tenant & Security Constraints
  - **Row-Level Security:** Every event, message thread, and drafted action must be strictly scoped to the `tenant_id` in PostgreSQL.
  - **Distributed Locking:** When an event is being processed by the Work Triage Engine, a Redis lock (`ohc:lock:{tenant_id}:triage:{event_id}`) must be acquired to prevent duplicate AI agent invocations for the same signal.
  - **Idempotency:** Action approvals (e.g., sending a drafted message) must use idempotency keys to ensure they are only executed once, especially important on flaky mobile networks.

  ### Mobile UX Flow (375px First)
  1. **The Feed Screen:** The primary view is a vertical list of prioritized Action Cards. No horizontal scrolling.
  2. **Action Card:** Displays the context (e.g., "New IG Message from Sarah"), the AI-drafted summary/response, and clear CTAs: "Approve", "Edit", "Dismiss".
  3. **Edit Flow:** Tapping "Edit" opens a native keyboard experience to tweak the AI's draft before sending.
  4. **Offline Support:** Feed items are cached locally. Approvals made offline are queued and synced when the network returns.

  ## Implementation Prompt
  **User-Facing Outcome:** The owner opens the OHC app and sees a prioritized feed of Action Cards. Instead of reading raw messages, they see AI-drafted replies and suggested actions based on recent customer inquiries or system events. They can tap "Approve" to execute the action instantly.

  **CUJ (Critical User Journey):**
  1. An external webhook (e.g., an Instagram DM simulation) hits the inbound signal endpoint.
  2. The Work Triage Engine processes the signal, classifying its intent.
  3. The appropriate AI agent drafts a response and creates an Action Card.
  4. The owner views the Action Card in their mobile feed.
  5. The owner taps "Approve", which finalizes the action and removes the card from the pending feed.

  **Acceptance Criteria:**
  - The feature must be accessible and fully functional on a 375px wide screen.
  - The AI agent must successfully draft a context-aware response based on the inbound signal.
  - Approving an action must be idempotent.
  - All database operations must strictly enforce `tenant_id` isolation.
  - Playwright E2E tests must cover the complete flow: receiving the signal, viewing the card in the UI, and approving it.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
