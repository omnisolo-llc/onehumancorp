issue_title: "[architecture]_predictive_inventory_intelligence"
issue_description: |
  # Predictive Inventory and Supply Chain Intelligence Agent

  ## Problem Statement
  Business owners selling physical products, like Maya (The Home Baker) and Priya (The Boutique Owner), often struggle with inventory management. They manually count stock, guess when to reorder supplies, and sometimes oversell items they don't have ingredients or stock to fulfill. This leads to frustrated customers, lost revenue, and late nights doing manual arithmetic.

  They need an invisible system that predicts when they will run out of stock based on sales velocity and seasonal trends, and proactively drafts reorder carts or alerts them to adjust their menus/catalogs.

  ## Research Report
  Current platforms like Shopify and Wix offer basic stock counting (e.g., "You have 5 left"). However, they do not offer *predictive intelligence* without expensive third-party apps (like Inventory Planner) that are far too complex for non-technical users.

  **Competitor Analysis:**
  - **Shopify:** Native inventory tracking is retrospective. Predictive analytics requires paid apps with steep learning curves.
  - **Wix:** Basic stock management. No predictive reordering or automated supply chain alerts.
  - **Square:** Offers low stock alerts but lacks AI-driven sales velocity predictions (e.g., "You're selling 2x more vanilla cakes this week; reorder flour by Tuesday").

  **OHC Advantage:** By leveraging the KAIROS Orchestrator and the Operations Agent Department, OHC can analyze real-time sales data, historical trends, and current inventory levels to provide actionable, plain-language alerts (e.g., "Priya, based on this week's sales, your red summer dresses will sell out in 3 days. Should I draft a reorder with your supplier?").

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Purchase Event] --> B(KAIROS Event Mesh / NATS)
      B --> C{Operations Agent}
      C --> D[(PostgreSQL - Inventory & Sales Data)]
      D --> E[Time-Series Analysis / pgvector memory]
      E --> C
      C --> F{Business Advisory Agent}
      F --> G[Push Notification / Unified Inbox Alert]
      F --> H[Draft Supplier Reorder]
  ```

  ### AI Agent Integration
  - **Operations Agent ("The Manager"):** Tracks real-time decrements in stock. Calculates sales velocity (units sold per day).
  - **Business Advisory Agent ("The Advisor"):** Uses the velocity data to generate proactive alerts. Formats the data into the plain-language "CEO Digest" (e.g., "You need to restock flour.").

  ### Mobile UX Flow (375px first)
  1. **The Alert:** The user receives a push notification: "⚠️ Stock Prediction: Vegan Cakes selling fast."
  2. **The Card (UniFi Layout):** Tapping the alert opens a premium Glassmorphism card in the OHC app.
     - **Headline:** "Vegan Cakes are trending."
     - **Insight:** "At the current rate, you will run out of ingredients by Thursday."
     - **Action Buttons (44x44px touch targets):**
       - [Draft Reorder]
       - [Mark as Sold Out]
       - [Dismiss]
  3. **1-Tap Action:** If "Draft Reorder" is tapped, the agent pre-fills a cart or email to their registered supplier.

  ### Security & Multi-Tenancy
  - All inventory predictions must query data strictly filtered by `tenant_id` using PostgreSQL RLS.
  - The Operations Agent lock mechanism (`ohc:lock:{tenant_id}:inventory:{product_id}`) ensures no race conditions during high-volume flash sales.

  ## Implementation Prompt
  Implement the Predictive Inventory Engine within the Operations Agent domain.
  1. Create a background worker that calculates a 7-day rolling sales velocity for physical products.
  2. If the current stock divided by the sales velocity is less than the user's defined lead time (default 3 days), trigger an alert.
  3. Design the mobile-first UI card to display this alert in the unified dashboard, allowing the user to take immediate action (e.g., adjust stock or draft a reorder email). Ensure all touch targets are at least 44x44px and utilize the design system's glassmorphism tokens.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
