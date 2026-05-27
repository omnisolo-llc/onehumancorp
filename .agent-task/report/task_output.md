issue_title: "Implement Autonomous Mobile-First Actionable Insights & Telemetry Mesh"
issue_description: |
  # Autonomous Mobile-First Actionable Insights & Telemetry Mesh

  ## Problem Statement
  Small business owners—whether it’s Maya the baker, Carlos the handyman, or Fatima the food cart owner—are not data analysts. They don't have the time or expertise to stare at multi-tab dashboards, line charts, or complex funnels to understand their business health. Traditional platforms like Shopify or Wix provide overwhelming "Analytics" pages that require the user to actively hunt for problems and interpret data.

  For example, if Maya's vegan cakes are selling faster than her regular cakes, a traditional dashboard would just show a pie chart of sales. Maya has to:
  1. Notice the trend.
  2. Realize she might run out of vegan ingredients.
  3. Manually trigger a supplier order or adjust pricing.

  **The Gap:** Owners need plain-language, one-tap actionable recommendations ("You have low stock on Vegan Cakes. Tap to reorder supplies.") pushed proactively to their mobile devices, rather than passive data visualizations. The platform must invisibly synthesize cross-domain telemetry (inventory, sales velocity, site traffic, local events) and autonomously present "What to do next."

  ## Research Report
  **Competitor Analysis:**
  - **Shopify:** Provides robust, desktop-first analytics dashboards. Even with "Shopify Magic" (AI), it is largely reactive (e.g., generating reports). It requires active user engagement to pull insights.
  - **Wix:** Similar to Shopify; offers deep analytics but relies on the user to interpret the data and take action.
  - **Square:** Offers sales reporting, but proactive insights are limited and not unified across all business operations (e.g., marketing + inventory + staffing).

  **The "Leapfrog" Opportunity for OHC:**
  Move from *Passive Analytics* to *Active Intelligence*.
  By integrating a unified event mesh (sales, inventory, CRM, web traffic) with our AI Agents (Operations, Finance, Marketing), we can continuously monitor the state of the business in the background. When anomalies or opportunities are detected, the system generates a concise, plain-language notification with a 1-tap resolution button on the mobile client.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Event Sources
          Sales[Sales Ledger]
          Inventory[Inventory Mesh]
          Traffic[Storefront Traffic]
          CRM[Customer Interactions]
      end

      subgraph Autonomous Mesh
          Bus[KAIROS Unified Event Bus / NATS]
          Mem[Episodic Memory / Vector DB]
          InsightAgent[Analytics & Operations Agent]
      end

      subgraph Mobile Interface
          Push[Push Notification Service]
          App[Mobile Translucent Glass UI]
      end

      Sales --> Bus
      Inventory --> Bus
      Traffic --> Bus
      CRM --> Bus

      Bus --> InsightAgent
      Bus --> Mem

      InsightAgent -- "Synthesizes Context" --> Mem
      InsightAgent -- "Detects Opportunity/Risk" --> Push
      Push -- "Plain-language Action" --> App
      App -- "1-Tap Approval" --> InsightAgent
      InsightAgent -- "Executes Workflow" --> Bus
  ```

  ### Mobile UX Flow (375px First)
  **Visual Excellence Mandate:** macOS-style Translucent Glass materials combined with clean modular dashboard cards.

  1. **The Notification (Proactive Push):**
     - *Lock Screen:* "📈 Vegan Cakes are selling 3x faster today. Tap to adjust."
  2. **The Insight Card (App Open):**
     - A single, beautiful, translucent card dominates the screen.
     - *Plain Language Text:* "You've sold 15 Vegan Cakes today, usually you sell 5. You only have enough flour for 3 more."
     - *Action Buttons (Large, tappable targets):*
       - [ Primary (Green) ]: "Reorder Flour ($25)"
       - [ Secondary (Grey) ]: "Mark 'Sold Out' when empty"
       - [ Tertiary (Text) ]: "Dismiss"
  3. **The 1-Tap Resolution:**
     - User taps "Reorder Flour".
     - *Feedback:* Gentle haptic pulse. Card smoothly transitions to a success state: "Done. Flour ordered from Supplier."
     - No complex supplier portals or inventory forms are shown unless the user toggles "Advanced Settings".

  ### AI Agent Integration Points
  - **Operations Agent:** Monitors inventory levels vs. sales velocity. Triggers supply reorders or "sold out" website updates.
  - **Marketing Agent:** Monitors web traffic vs. conversion. (e.g., "100 people looked at the consulting package, but 0 booked. Want me to email them a 10% discount?")
  - **Finance Agent:** Monitors cash flow and upcoming bills. (e.g., "You have a $500 software bill tomorrow, but only $400 in the account. Want me to send reminders to 3 clients with overdue invoices?")

  ### Multi-Tenancy & Security
  - Strict Zero-Trust boundaries. The Insight Agent only has access to the isolated event stream for the specific `organization_id` via SPIFFE/SPIRE identity checks.
  - All 1-tap execution workflows are cryptographically signed by the user's OIDC token before the Agent acts.

  ## Implementation Prompt
  **To the Implementer:**
  Design and implement the underlying data pipelines and API endpoints for the "Actionable Insights Mesh".

  **User Journey (CUJ):**
  As a business owner, I want my app to tell me exactly what I need to do right now to optimize my business, without me having to look at charts, so I can approve actions with a single tap.

  **Acceptance Criteria:**
  1. Create the backend event consumer that listens to at least two domains (e.g., Sales and Inventory) on the unified bus.
  2. Implement the AI evaluation loop that processes these events and generates a plain-language insight payload (Insight Text + Action Enum).
  3. Ensure the payload is strictly isolated by `organization_id`.
  4. Create the API endpoint that the mobile client calls to "approve" and execute the suggested action.
  5. Do NOT build the mobile UI; just ensure the API payloads support the Translucent Glass card UX described (provide text, primary action, secondary action).
  6. Latency for action execution must be under 500ms to feel instantaneous.

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []