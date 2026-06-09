issue_title: "Implement Autonomous Zero-Drop Auto-Quoting & Booking Engine"
issue_description: |
  ## Title: Implement Autonomous Zero-Drop Auto-Quoting & Booking Engine

  ## Problem Statement
  Business owners in the field services, repair, and consulting sectors (like Carlos, the Field Service Owner) suffer from the "Now What?" problem. They capture a lead or inquiry via a contact form or DM but lose the potential customer because they are actively working and cannot respond with a quote or booking availability in real time. The latency between an inquiry and a quote often leads to dropped opportunities, representing significant lost revenue. A simple static website or basic CRM contact list does not solve the root issue: the lack of autonomous operational follow-through.

  ## Research Report
  Our competitive analysis compared traditional platform builders (Shopify, Wix) and rising AI-native upstarts (Durable) against OHC's vision:

  - **Traditional Builders (Shopify/Wix):** Excel at complex retail/e-commerce but lack out-of-the-box, autonomous service quoting. Owners must string together multiple third-party plugins (e.g., Zapier + Calendly + separate quoting tools) which creates fragility and high technical overhead.
  - **AI-Native Competitors (Durable):** Offer incredible setup speed (30-second website generation) and basic CRM capabilities. However, user sentiment analysis reveals that their AI capabilities are "shallow." While the website looks good, the CRM is merely a contact list and does not autonomously follow up or schedule the work. It forces the owner back into a manual loop.
  - **The OHC Opportunity:** By utilizing our "Teammate Mesh" architecture and agentic departments, we can build a "Zero-Drop" workflow. Instead of just notifying the owner of a new lead, the Sales Agent dynamically parses the request, checks the Operations calendar, applies base pricing rules, and instantly sends an actionable quote and booking link to the customer. This closes the gap between demand generation and revenue capture without requiring the owner to put down their tools.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Customer
      participant OHC Inbox (Work Triage)
      participant Sales Agent (Quoting)
      participant Ops Agent (Calendar)
      participant Owner Mobile App

      Customer->>OHC Inbox: Submits detailed service request
      OHC Inbox->>Sales Agent: Triggers Lead Analysis
      Sales Agent->>Ops Agent: Request available slots & estimated duration
      Ops Agent-->>Sales Agent: Returns available calendar slots
      Sales Agent->>Sales Agent: Applies owner's base pricing rules
      Sales Agent-->>Customer: Emails/SMS dynamic quote + booking link
      Customer->>Ops Agent: Accepts quote, pays deposit & books slot
      Ops Agent->>Owner Mobile App: Push Notification: "New Job Booked & Paid"
  ```

  ### Mobile UX Flow (375px First)
  1. **Configuration (Owner):** The owner navigates to the "Operations & Quoting" tab on their mobile app. They toggle "Autonomous Quoting" ON and set their base pricing rules (e.g., "$75/hr base, $50 travel fee") using a simple, native-keyboard form.
  2. **Intake (Customer):** The customer visits the mobile-optimized OHC storefront and uses a conversational form to describe their issue (e.g., "My kitchen sink is leaking under the cabinet").
  3. **Agent Action (Invisible):** The backend agents coordinate via Redis Redlock to avoid race conditions, apply the pricing rules to the described issue, and generate a secure, temporary payment link.
  4. **Booking (Customer):** The customer receives an SMS/Email with the quote and taps the payment link. They view available slots, select one, and pay the deposit using a native mobile flow (e.g., Stripe Checkout).
  5. **Notification (Owner):** The owner receives a clean push notification containing the job details, verified payment status, and calendar update. The 375px daily work feed surfaces this as an upcoming commitment.

  ### AI Agent Integration Points
  - **Work Triage / Intake:** Parses the incoming plain-text request to identify the service type, urgency, and required resources.
  - **Sales & Revenue Assistant:** Uses the extracted parameters to calculate an estimated price based on the tenant's predefined rules. Drafts a natural-language email or SMS containing the quote.
  - **Operations Assistant:** Interacts with the tenant's unified booking engine (or integrated Cal.com calendar) to reserve a tentative slot until the deposit is paid, then locks it upon successful payment.

  ### Key Design Decisions and Why
  - **Asynchronous Agent Coordination:** We use the established KAIROS queue (`FOR UPDATE SKIP LOCKED` on `shared_tasks`) to coordinate the Sales and Ops agents. This ensures high reliability and retry capability if a third-party calendar API temporarily fails.
  - **Zero-Trust Multi-Tenancy:** All quoting rules and calendar checks are strictly isolated using PostgreSQL row-level security (`tenant_id`), preventing cross-tenant data leakage.
  - **Fallback to Human:** If the AI determines the request is too complex or lacks necessary information, it autonomously sends a follow-up question to the customer rather than generating an inaccurate quote, ensuring professional interactions.

  ## Implementation Prompt
  **User-Facing Outcome:** Implement the "Autonomous Auto-Quote & Book" feature. When a customer submits a service inquiry, the system must automatically parse the request, calculate a quote based on the owner's configured pricing rules, and send the customer a message with the quote and a booking link. The owner simply receives a notification when the job is booked and paid.

  **Critical User Journey (CUJ):**
  1. Owner configures base pricing and toggles autonomous quoting ON in settings.
  2. Customer submits an inquiry on the storefront.
  3. The Sales agent drafts a quote and checks the Ops agent's calendar.
  4. The system sends an email/SMS quote with a booking and deposit link.
  5. Customer pays; the job appears in the owner's daily feed.

  **Acceptance Criteria:**
  - The quoting logic must run entirely without manual owner intervention.
  - Pricing rules must be applied accurately to conversational input.
  - The calendar must temporarily block tentative slots and lock them upon payment.
  - The feature must be fully manageable from a mobile (375px) viewport, adhering to the Translucent Glass design tokens.
  - Ensure edge cases (e.g., unclear requests) gracefully fall back to requesting more info.

  **Priority:** P0
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
