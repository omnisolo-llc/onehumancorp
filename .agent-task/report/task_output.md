---
issue_title: "[Architecture] AI-Powered Subscription & Proactive Client Retention Engine"
issue_description: |
  ## Problem Statement
  Small business owners providing recurring services (e.g., Leo, the Music Tutor) or subscription goods struggle with revenue predictability and customer churn. They lack the time and tooling to manually track which clients have lapsed (e.g., a student who hasn't booked a lesson in 2 weeks, or a customer who stopped ordering monthly coffee beans). Setting up complex subscription billing or automated retention marketing campaigns in existing tools requires technical knowledge and navigating complex dashboards, leading to lost recurring revenue.

  ## Research Report
  **Competitor Systems Audit:**
  - **Shopify:** Handles subscriptions primarily via complex third-party apps (Recharge, Skio) which are expensive, not native, and difficult for non-technical users to configure.
  - **Wix / Squarespace:** Offer basic recurring payments, but completely lack intelligent, proactive AI-driven retention capabilities to chase lapsed customers or predict churn.
  - **Acuity / Calendly:** Handle recurring appointments well but do not proactively identify and engage students/clients who have fallen out of their regular booking cadence.

  **Gaps Identified:**
  OHC lacks a unified, AI-driven subscription and retention system. We need an architecture that not only processes recurring billing but also leverages the "Customer Success" and "Sales" AI Agents to autonomously detect lapsed clients, draft personalized re-engagement messages (e.g., "Hi Sarah, it's been 2 weeks since your last guitar lesson. Want to book a slot this Thursday?"), and offer one-tap subscription upgrades from one-off purchases.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Mobile Device
          App[OHC Mobile App 375px] --> SubUI[Subscription & Retention Dashboard];
          SubUI --> LocalDB[(Local Cache)];
      end

      App -- "Manage Subscriptions / Approve Retention Campaigns" --> Gateway[OHC API Gateway];

      Gateway --> SubEngine[Subscription & Billing Engine];
      SubEngine --> MainDB[(Cloud Postgres CRM Ledger)];
      SubEngine --> Stripe[Stripe Billing / Connect];

      Gateway --> Agents[AI Agent Swarm];
      MainDB -- "Churn Prediction / Lapsed Activity" --> Agents;

      subgraph Agent Departments
          Agents --> CSAgent[Customer Success: Drafts Re-engagement SMS/Email];
          Agents --> SalesAgent[Sales: Suggests Subscription Upsells];
          Agents --> FinanceAgent[Finance: Manages Recurring Charges & Dunning];
      end
  ```

  ### Mobile UX Flow (375px First)
  1. **Retention Insight Notification:** Leo receives a push notification: "✨ 3 students haven't booked in 2 weeks. Tap to review re-engagement drafts."
  2. **Review & Approve:** Leo taps the notification, opening a macOS-style Translucent Glass card showing a drafted SMS for a student: "Hey Alex, ready for your next lesson? Here's a link to book: [Link]". Leo taps "Approve & Send".
  3. **Subscription Upsell:** When a customer completes their 3rd one-off purchase, the App surfaces a card recommending Leo to offer a subscription. With one tap, the AI drafts a personalized email offering a 10% discount for subscribing to a monthly package.
  4. **Financial Overview:** A simple tab shows active subscriptions, MRR (Monthly Recurring Revenue) in plain language ("You are guaranteed $400 this month from 4 students"), and any failed payments (handled invisibly by the Finance agent's dunning process).

  ### AI Agent Integration Points
  - **Customer Success Agent:** Monitors the unified ledger for time-since-last-purchase or time-since-last-booking. Automatically drafts context-aware messages based on past interactions.
  - **Finance Agent:** Integrates directly with Stripe Billing to handle recurring charge failures invisibly, only notifying the owner if standard retry logic fails.
  - **Sales Agent:** Identifies frequent buyers and suggests subscription upsells, generating customized one-click checkout links.

  ### Key Design Decisions & Security
  - **Zero-Trust Multi-Tenancy:** Subscription data and client history are strictly isolated per tenant using SPIFFE/SPIRE.
  - **No Marketing Jargon:** We avoid terms like "Dunning", "MRR", "Churn Rate". We use plain language: "Failed payments", "Guaranteed monthly income", "Missing customers".
  - **Proactive, not Reactive:** Instead of waiting for the user to run a report, the system proactively pushes actionable, drafted campaigns to the user's mobile device for simple approval.

  ## Implementation Prompt
  Implement the AI-Powered Subscription & Proactive Client Retention Engine.
  - **User-Facing Outcome:** Business owners can easily offer subscription packages, and receive proactive, AI-drafted messages to re-engage lapsed customers with a single tap on their mobile device.
  - **CUJ:** A client hasn't booked a service in 14 days. The Customer Success AI detects this, drafts an SMS with a direct booking link, and sends a push notification to the business owner. The owner reviews the draft on their 375px mobile screen, taps "Approve", and the client receives the SMS and books a slot.
  - **Acceptance Criteria:**
    - Build the backend cron/worker to evaluate client purchase/booking frequency against expected cadence.
    - Integrate the Customer Success AI to draft re-engagement messages via the Omni-Channel router.
    - Implement the frontend approval cards using OHC design tokens (glassmorphism).
    - Ensure Stripe Billing integration for recurring packages handles deposits and failed payments.
    - 100% unit test coverage for the detection logic and E2E Playwright test for the approval flow.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
