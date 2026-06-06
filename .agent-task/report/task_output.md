issue_title: "Research: Automated Taxation & Compliance Workflow"
issue_description: |
  # Research Report: Automated Taxation & Compliance Workflow for OHC Users

  ## Executive Summary
  This report identifies a critical gap in the OneHumanCorp (OHC) platform ecosystem: the lack of a proactive, agent-driven taxation and compliance management system. Current platforms (Shopify, Wix) provide tools for tax calculation but leave the burden of compliance, filing, and proactive planning to the user. OHC can capture significant market share by introducing "The Protector," a dedicated Legal & Compliance Agent that handles these complex, stressful tasks invisibly.

  ## 1. Market Mapping & Competitor Discovery (Track 1)

  **The Status Quo:**
  - **Shopify:** Offers Shopify Tax, an automated tax calculation tool. However, it still requires the user to understand nexus laws, register in appropriate jurisdictions, and manually file returns.
  - **Wix/Squarespace:** Rely on third-party integrations like Avalara or TaxJar, adding to the "App Tax" burden and complexity.
  - **The Pain Point:** Small business owners, especially non-technical ones like Maya (the baker) or Carlos (the handyman), find tax compliance terrifying. They often under-collect, miss filing deadlines, or face unexpected tax bills.

  ## 2. OHC Gap & Pain Point Identification (Track 3)

  **The Gap:** OHC currently lacks an integrated, proactive system for managing sales tax, income tax estimates, and regulatory compliance.

  **Persona Focus:** Maya (the baker)
  - Maya sells cakes locally but occasionally ships nationally.
  - She is unaware of economic nexus laws and when she needs to start collecting sales tax in other states.
  - She dreads tax season because she has to manually compile all her OHC sales data for her accountant.

  ## 3. Deep Dive Architecture Design (Track 2)

  ### The Protector: Legal & Compliance Agent

  **Core Responsibilities:**
  1. **Automated Nexus Monitoring:** Continuously tracks sales volume and transactions per state/jurisdiction to identify when economic nexus is triggered.
  2. **Proactive Registration Alerts:** When nexus is approaching or triggered, The Protector pushes a notification to the user, advising them to register for a tax permit.
  3. **Seamless Tax Calculation:** Automatically calculates and applies the correct sales tax rate at checkout, based on the customer's location and the product type (e.g., groceries vs. prepared food for Maya).
  4. **Automated Filing Preparation:** Generates pre-filled tax reports tailored to specific jurisdictional requirements, ready for submission or review by an accountant.

  ### Data Model & Sync Protocol

  - **Central Ledger (PostgreSQL):** Every transaction must include a detailed tax breakdown (state, county, city, special district rates).
  - **Tax Rules Engine:** A dynamic rules engine that is regularly updated with the latest tax rates and regulations for all supported jurisdictions.
  - **Agent Coordination:** The Protector must coordinate with the Finance Agent (to track revenue) and the Operations Agent (to track shipping destinations).

  ```mermaid
  erDiagram
      Tenant ||--o{ Transaction : has
      Transaction ||--o{ TaxBreakdown : contains
      TaxRulesEngine ||--|{ Jurisdictions : defines
      TaxRulesEngine {
          string rule_id
          string state
          float rate
      }
      TaxBreakdown {
          string state
          float total_tax
      }
  ```

  ### Mobile UX Flow (375px)

  **Push Notification UI Flow:**
  - **Screen 1 (Home Feed):** A UniFi-style modular dashboard card appears on the 375px screen: "Tax Threshold Approaching in TX. You have reached 80% of the $500k nexus limit." Buttons: [View Guide] [Dismiss].
  - **Screen 2 (Guide & Action):** Translucent glassmorphism modal opens with simple steps. Input field for Tax Permit ID once registered.
  - **Screen 3 (Success):** "TX Tax Collection Activated" confirmation.

  ## 4. Proposed Implementation Steps & Issue Prompt

  **Feature Name:** The Protector - Automated Taxation & Compliance
  **Target Persona:** Maya the Baker

  **Outcome:** An automated system that tracks Maya's sales, alerts her when she needs to collect sales tax in a new state, and generates ready-to-file tax reports.

  **Critical User Journey (CUJ):**
  1. Maya's business grows, and she starts shipping cakes to a neighboring state.
  2. The Protector Agent monitors her sales volume in that state.
  3. Once Maya hits 80% of the economic nexus threshold for that state, The Protector sends a push notification: "You are approaching the sales tax threshold for State X. I've prepared a guide on how to register. Tap to review."
  4. Maya reviews the guide and registers. She updates her OHC settings with her new permit number.
  5. The Protector automatically begins calculating and collecting sales tax for all future orders to that state.
  6. At the end of the quarter, The Protector generates a comprehensive sales tax report, ready for Maya to file.

  **Next Actions for Engineering:**
  - **Step 1:** Integrate a robust tax calculation engine (e.g., Stripe Tax API) into the checkout flow.
  - **Step 2:** Develop the "Nexus Monitoring" module within The Protector Agent, tracking sales volume against state thresholds.
  - **Step 3:** Implement the push notification and reporting interface for the user, ensuring it's accessible and easy to understand on a 375px mobile screen.

  **Top 5 things that do not make sense in the repository:**
  1. The API and Domain folders lack clear boundary separation in some areas.
  2. There is no unified error handling or consistent logging strategy visible.
  3. Some E2E tests are mocked instead of testing the full lifecycle.
  4. Redundant dependency declarations in Bazel files across various modules.
  5. Insufficient documentation on how AI agents coordinate across departments.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
