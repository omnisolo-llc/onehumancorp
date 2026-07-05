issue_title: "[Platform] Unified AI-Powered Quote-to-Cash Workflow for Service Operators"
issue_description: |
  # Platform Architecture Deep Dive: Unified AI-Powered Quote-to-Cash Workflow

  ## Problem Statement
  Service-based owners like Carlos (Handyman) and Nora (Agency Principal) struggle with disjointed workflows. They receive leads from various channels, manually draft estimates, send them via email or text, manually track approvals, schedule visits, perform the work, and then chase invoices. This fragmented process leads to lost leads, delayed payments, and significant manual overhead.

  ## Research Report
  - **Market Context:** Traditional SMB tools like Jobber or Housecall Pro provide the features but are often complex to set up and require the operator to actively manage the software. Platforms like Shopify are heavily product-focused and don't natively support dynamic service estimates and bookings.
  - **The OHC Opportunity:** By integrating quoting, scheduling, and invoicing into a single AI-driven workflow, OHC can function as a true "assistant." Instead of the owner logging into a dashboard to create a quote, the AI drafts the quote based on the lead context and simply asks for owner approval.
  - **Competitor Gaps:**
    - *Jobber/Housecall Pro:* Complex, software-heavy, requires manual data entry.
    - *Shopify:* Poor support for dynamic service pricing and scheduling.
    - *Stripe Invoicing:* Good for payments but lacks the front-end quoting and scheduling workflow.

  ## Design Doc
  ### Architecture & Data Model (PostgreSQL)

  ```mermaid
  erDiagram
      Tenant ||--o{ ServiceLead : has
      Tenant ||--o{ Estimate : has
      Tenant ||--o{ Job : has
      Tenant ||--o{ Invoice : has

      ServiceLead ||--o{ Estimate : generates
      Estimate ||--o| Job : transitions_to
      Job ||--o| Invoice : generates

      Tenant {
          uuid id PK
          string name
      }
      ServiceLead {
          uuid id PK
          uuid tenant_id FK
          string channel
          string requirements
      }
      Estimate {
          uuid id PK
          uuid tenant_id FK
          uuid service_lead_id FK
          decimal total_price
          string status
      }
      Job {
          uuid id PK
          uuid tenant_id FK
          uuid estimate_id FK
          datetime scheduled_time
          string status
      }
      Invoice {
          uuid id PK
          uuid tenant_id FK
          uuid job_id FK
          decimal amount
          string stripe_payment_intent
          string status
      }
  ```

  - `ServiceLead`: Represents an incoming inquiry (from DM, form, etc.).
  - `Estimate`: A generated quote linked to a `ServiceLead` with line items, total price, and terms.
  - `Job`: Created upon estimate approval, linking the estimate to a scheduled time and resource.
  - `Invoice`: Generated from a completed `Job` for payment collection.
  - **Multi-Tenant:** All entities must enforce `tenant_id` isolation.

  ### AI Agent Coordination Sequence

  ```mermaid
  sequenceDiagram
      participant Customer
      participant OHC Webhook
      participant SalesAssistant
      participant OwnerFeed
      participant OperationsAssistant
      participant FinanceAssistant

      Customer->>OHC Webhook: Sends inquiry (e.g. "Need drywall fixed")
      OHC Webhook->>SalesAssistant: Triggers new ServiceLead
      SalesAssistant->>SalesAssistant: Extracts requirements & estimates cost
      SalesAssistant->>OwnerFeed: Pushes "Draft Estimate" Action Card
      OwnerFeed->>OwnerFeed: Carlos taps "Approve"
      OwnerFeed->>Customer: Sends Estimate SMS link
      Customer->>Customer: Approves & selects time
      Customer->>OperationsAssistant: Confirms schedule
      OperationsAssistant->>OperationsAssistant: Resolves calendar & creates Job
      OperationsAssistant->>OwnerFeed: Notifies Carlos of scheduled Job
      OwnerFeed->>OwnerFeed: Carlos marks Job "Complete"
      OwnerFeed->>FinanceAssistant: Triggers Invoice creation
      FinanceAssistant->>Customer: Sends Stripe Invoice link
  ```

  ### UI Wireframes & Screen Flow
  1. **Triage Feed:** Carlos sees a new lead from a web form. The Sales Assistant has already drafted an estimate for "Drywall Repair" for $350.
  2. **Approval Card:** Carlos reviews the drafted estimate on a clear, touch-friendly card (Action Card). He taps "Approve & Send."
  3. **Client Flow:** The client receives an SMS with a link to view the estimate, approve it, and select a time slot.
  4. **Completion:** After the visit, Carlos taps "Mark Complete" on the job card, prompting the Finance Assistant to send the final invoice via Stripe.

  ### Key Design Decisions
  - **Proactive AI Actions:** Instead of creating a passive inbox, we use AI to proactively draft `Estimates` based on `ServiceLead` data, drastically reducing time-to-quote.
  - **Single Feed Interface:** Avoid complex multi-page dashboards. Present actionable cards directly in the Owner Feed for approvals, keeping the operator in control but reducing clicks.
  - **Strict Multi-Tenancy:** Ensure `tenant_id` is present on every table and verified at the database level using Row-Level Security (RLS) to prevent cross-tenant data leaks.
  - **Stripe Integration:** Decouple the core invoicing logic from Stripe's implementation details, but use Stripe Payment Intents for reliable payment capture upon job completion.

  ### AI Integration
  - **Sales Assistant:** Parses incoming leads, extracts requirements, and automatically drafts an `Estimate` based on the owner's historical pricing or predefined service catalog. It then presents an "Action Card" to the owner.
  - **Operations Assistant:** Once an estimate is approved by the client, it automatically schedules the `Job`, resolving calendar conflicts.
  - **Finance Assistant:** Upon job completion (marked by the owner or location data), automatically drafts an `Invoice` and sends payment reminders.

  ## Implementation Prompt
  **Feature Name:** OHC AI-Powered Quote-to-Cash Workflow
  **Target Persona:** Carlos (Handyman)
  **Outcome:** Carlos receives a lead, reviews an AI-drafted estimate on his phone, and upon client approval, the system handles scheduling and invoicing automatically.

  **Next Actions for Engineering:**
  1. Implement the core Data Models (`ServiceLead`, `Estimate`, `Job`, `Invoice`) with strict `tenant_id` isolation in PostgreSQL.
  2. Develop the AI Sales Assistant capability to parse leads and generate draft `Estimates`.
  3. Build the mobile-first (375px) "Action Card" UI for the owner to review, edit, and approve estimates.
  4. Create the client-facing approval and scheduling flow.
  5. Integrate with Stripe to handle final `Invoice` generation and payment collection.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
