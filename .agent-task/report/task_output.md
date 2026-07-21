issue_title: "Autonomous AI-Driven Trade Credit & Vendor Terms Orchestration Engine"
issue_description: |
  # OHC Research & Architecture Report: Autonomous AI-Driven Trade Credit & Vendor Terms Orchestration Engine (OHC Vendor Credit Hub)

  ## 1. Executive Summary
  This report introduces the architectural design for the **OHC Vendor Credit Hub**—an autonomous, AI-driven trade credit, micro-factoring, and vendor terms orchestration engine. Small business owners and independent operators constantly struggle with cash flow. They must purchase raw materials or inventory upfront from suppliers, but only collect revenue weeks later when products are sold or services are rendered. Traditional credit checks and manual trade credit negotiations are too complex and administrative for non-technical operators.
  The OHC Vendor Credit Hub closes this "Liquidity Gap" by utilizing real-time OHC transactional and booking ledger history to underwrite credit limits automatically, auto-negotiate Net-30/60 terms with wholesale suppliers via zero-knowledge proofs of financial health, and offer on-demand micro-factoring/invoice discounting for corporate contracts.

  ---

  ## 2. Problem Statement
  Small business owners like **Maya (baker)**, **Carlos (handyman)**, **Priya (boutique owner)**, and **Nora (agency principal)** face persistent cash-flow crunches. They operate in high-cost environments but lack traditional corporate credit ratings (such as D&B) or detailed audited financials to secure trade credit from wholesale suppliers.

  As a result:
  - **Maya** has to pay organic flour mills and packaging suppliers upfront, reducing her cash buffer during peak custom cake order seasons.
  - **Priya** is forced to skip ordering high-margin seasonal collections from apparel manufacturers because she cannot afford to shell out $5,000 upfront.
  - **Carlos** must pay out-of-pocket for expensive plumbing or carpentry parts before a client's final invoice clears.
  - **Nora** has contractors who demand weekly payments, but her B2B corporate clients only pay her invoice on strict Net-60 terms, creating a massive cash squeeze.

  Currently, no small-business platform solves this. Traditional credit application forms are heavily manual, confusing, and desktop-first, leading to high abandonment rates and forcing owners to use predatory personal credit cards or high-interest merchant cash advances (MCAs).

  ---

  ## 3. Deep Research & Competitive Market Analysis

  ### 3.1 Competitive Comparison
  | Platform | Offering | Limitations for Small Operators / Personas |
  |---|---|---|
  | **Shopify Capital** | Merchant Cash Advances (MCAs) based on online Shopify sales. | - Highly expensive fee factors.<br>- Only captures online Shopify checkout data.<br>- Cannot pay third-party wholesale suppliers directly.<br>- No B2B Net-terms auto-negotiation or micro-factoring. |
  | **Stripe Capital** | Direct business cash advances repaid via daily sales deductions. | - Reactive, invitation-only funding.<br>- No facility to request custom trade-credit/payment terms from suppliers.<br>- No support for invoice-based services or contractor payments. |
  | **QuickBooks / Xero Factoring** | Complex, manually configured invoice financing. | - Requires formal accountant/CPA verification.<br>- Intrusive desktop interface designed for professional bookkeepers, not owners.<br>- High friction, requiring manual document uploads and offline phone calls. |
  | **Melio & Brex** | Credit-card billing and expense tracking. | - Requires existing premium business credit card limits.<br>- Does not autonomously underwrite limits based on transaction flows.<br>- No AI-led auto-negotiation of Net-30/60 terms. |

  ### 3.2 OHC's Strategic Differentiator
  The OHC Vendor Credit Hub is the world's first **assistant-first, ledger-driven trade credit engine**. By integrating directly into the OHC multi-tenant transactional ledger and booking calendar, OHC can dynamically estimate a business's real-time repayment capacity. It allows the owner to tap a single button on a 375px mobile screen, empowering AI agents to handle the complex underwriting, vendor negotiations, and automated invoice discounting in the background.

  ---

  ## 4. Architectural Design (Design Doc)

  ### 4.1 System Architecture Diagram
  The OHC Vendor Credit Hub is designed as an autonomous coprocessor. It sits adjacent to the core transaction ledger and coordinates with external suppliers and the OHC Capital Account.

  ```mermaid
  graph TD
      subgraph Mobile_UI [375px Mobile Client]
          Dashboard[Credit Hub Card]
          TermNegotiator[1-Tap Term Negotiator]
          InvoiceFactor[Invoice Discounting Sheet]
      end

      subgraph OHC_API_Gateway [OHC API Gateway & Auth]
          Auth[SPIFFE/SPIRE Token Validator]
          RLS[Row-Level Tenant Isolation]
      end

      subgraph Core_Services [OHC Ledger Services]
          Ledger[(PostgreSQL OHC Ledger)]
          UnderwritingEngine[Dynamic Credit Underwriting Engine]
          CapitalReserve[(OHC Internal Capital Ledger)]
      end

      subgraph AI_Department [AI Assistant Teammates]
          Accountant[The Accountant Agent]
          Negotiator[The Negotiator Sub-Agent]
          Messenger[The Messenger CS Agent]
      end

      subgraph External_Network [External World]
          WholesaleVendor[Supplier / Vendor Email & API]
          ACH[Stripe / ACH Bank Network]
      end

      Dashboard -->|Request Credit Score| Auth
      Auth -->|Check Tenant Context| RLS
      RLS -->|Read Transactions & Bookings| Ledger
      Ledger -->|Query Metrics| UnderwritingEngine
      UnderwritingEngine -->|Formulate Credit Limit| Dashboard

      TermNegotiator -->|Trigger Term Negotiation| Negotiator
      Negotiator -->|Analyze Backlog Proofs| UnderwritingEngine
      Negotiator -->|Draft & Send Zero-Knowledge PDF Application| WholesaleVendor
      WholesaleVendor -->|Inquiry / Clarification| Messenger
      Messenger -->|Notify/Clarify| Dashboard

      InvoiceFactor -->|Submit Invoice for Factoring| Accountant
      Accountant -->|Verify Invoice Authenticity| Ledger
      Accountant -->|Disburse Cash Advance| CapitalReserve
      CapitalReserve -->|ACH Outward Payout| ACH
  ```

  ### 4.2 Data Model & Multi-Tenant Invariants
  All tables implement Row-Level Security (RLS) bound strictly to the `tenant_id` to ensure complete cryptographic isolation in our SaaS multi-tenant database.

  ```mermaid
  erDiagram
      ORGANIZATION ||--o{ CREDIT_FACILITY : owns
      ORGANIZATION ||--o{ VENDOR_RELATION : maintains
      VENDOR_RELATION ||--o{ SUPPLIER_INVOICE : registers
      ORGANIZATION ||--o{ FACTORING_DISCOUNT : triggers
      SUPPLIER_INVOICE ||--o{ LEDGER_SWEEP_CONFIG : configures

      CREDIT_FACILITY {
          uuid id PK
          string tenant_id FK "Row-Level Isolation Key"
          double approved_limit_usd
          double utilized_amount_usd
          double dynamic_score "0.0 - 100.0"
          string underwriter_version
          timestamp updated_at
      }

      VENDOR_RELATION {
          uuid id PK
          string tenant_id FK
          string vendor_name
          string vendor_email
          string current_terms "COD | NET_15 | NET_30 | NET_60"
          string term_status "APPROVED | NEGOTIATING | DENIED"
          timestamp terms_granted_at
      }

      SUPPLIER_INVOICE {
          uuid id PK
          string tenant_id FK
          uuid vendor_relation_id FK
          string invoice_number
          double total_amount
          string currency
          timestamp due_date
          string status "UNPAID | SWEEPING | PAID | OVERDUE"
      }

      FACTORING_DISCOUNT {
          uuid id PK
          string tenant_id FK
          string client_invoice_id FK "Referenced B2B Client Invoice"
          double invoice_amount
          double advance_rate "e.g., 0.85"
          double flat_fee_pct "e.g., 0.02"
          double advanced_amount_usd
          string factoring_status "APPLIED | DISBURSED | REPAID"
          timestamp disbursed_at
      }

      LEDGER_SWEEP_CONFIG {
          uuid id PK
          uuid supplier_invoice_id FK
          double daily_sweep_pct "e.g., 0.10"
          double maximum_sweep_usd
          double accumulated_sweep_usd
          timestamp last_sweep_run
      }
  ```

  **Key Multi-Tenant Isolation Invariants:**
  1. **Strict SQL Filtering**: All data mutations, score queries, and credit facilities MUST execute under filters targeting `tenant_id = current_setting('request.jwt.claim.tenant_id')`.
  2. **Zero-Knowledge Proofs (ZKP)**: When negotiating terms with `WholesaleVendor`, the Negotiator Agent NEVER shares the raw transaction logs or private customer list of the tenant. Instead, it generates a cryptographically verifiable summary containing aggregates (e.g., "Monthly Volume > $12,000 for 6 consecutive months" and "Current backlog value > $4,500").
  3. **Idempotent Sweeps**: All auto-budgeting sweep actions must include strict transaction idempotency keys to prevent double-charging or over-sweeping merchant bank accounts.

  ---

  ## 5. Mobile-First UX Flow & Touch Targets (375px First)

  To pass the **"Grandmother Test"**, the system hides all financial jargon (underwriting models, factoring fee APR, sweep logic ratios) behind premium visual cards and explicit "Advanced" toggles.

  ### 5.1 Step-by-Step UI Flow
  1. **The Credit Pulse Card (Main Dashboard)**:
     - Placed at the top of the owner's command center on a 375px screen.
     - A macOS-style translucent glass card (`backdrop-filter: blur(20px)`, dark gradient borders) displays:
       - *"Your Business Credit Capacity is Good ($7,500)*"
       - A subtle interactive progress bar showing current credit utilization ($1,200 / $7,500).
       - Minimum touch target for the card is `64px` in height, providing comfortable thumb-tapping.
  2. **The "Vendor Credit Hub" Sheet**:
     - Tapping the Credit Pulse Card slides up a beautiful bottom sheet.
     - Divided into two premium, UniFi-style modular tabs:
       - **Tab A: Vendor Terms** (Shows listed wholesale vendors, term status like "Net-30 Active", and a primary `[ Request Net Terms ]` button).
       - **Tab B: Invoice Advance** (Lists active, outgoing client invoices with an estimated instant payout amount and a glassmorphic `[ Advance Funds ]` toggle).
  3. **The 1-Tap Net Terms Negotiation Flow**:
     - The user taps `[ Request Net Terms ]`.
     - A sheet lists known vendors parsed from recent billing expenses.
     - User selects a vendor card (e.g., "West Coast Organic Flour Mill") and selects terms requested (Net-30).
     - The Negotiator Agent presents a clean visual summary: *"I will draft a terms request and attach a verified proof of your $8,200 monthly sales. No sensitive customer info will be shared."*
     - User taps a high-contrast, emerald-green button `[ Let AI Negotiate ]` (size `48x48px`).
  4. **The Invoice Discounting Action**:
     - Nora (Agency Principal) views an unpaid $10,000 invoice due in 45 days.
     - OHC displays a glowing, translucent card: *"Get $8,500 immediately. Repay when the client pays (2% flat fee)."*
     - Nora swipes a premium custom slider `[ Swipe to Factoring Payout >>> ]`.
     - The screen chimes, and the status instantly updates to `Factored - Disbursing via ACH`.
  5. **Auto-Budgeting Sweep Pulse**:
     - Maya sees a transparent dial on her supplier invoice card: `[ Sweep 10% of Daily Sales to Invoice Balance ]`. It ensures that $800 invoice is smoothly funded by the due date without a major lump-sum cash hit.

  ---

  ## 6. AI Agent Integration & Coordination Protocol

  The system is orchestrated by three highly specialized, coordinated AI agents:

  ### 6.1 Teammate Mesh Coordination
  - **The Accountant (Finance Agent)**:
    - Runs in the background, monitoring the OHC transactional ledger.
    - Computes real-time sales velocity, revenue trends, and dynamic credit scoring.
    - Coordinates the daily auto-budgeting sweep, initiating localized micro-transfers of revenue into the credit reserve envelopes.
  - **The Negotiator (Sales/Underwriting Sub-Agent)**:
    - Triggered when the owner requests Net Terms with a supplier.
    - Generates the Zero-Knowledge PDF application packet.
    - Establishes communication with the wholesale supplier via email or API, presenting the business's verified financial proofs.
  - **The Messenger (Customer Success Agent)**:
    - Intercepts inbound supplier emails or queries (e.g., "Please clarify your last month's inventory dip").
    - Leverages RAG to find the reason (e.g., "The boutique was closed for 3 days due to local power outage").
    - Drafts a professional response, displaying it as a pending notification on the owner's OHC dashboard for 1-tap approval before sending.

  ---

  ## 7. Implementation Prompt for the Engineering Swarm (MANDATORY)

  ```text
  You are an expert full-stack engineer. Build the "Autonomous AI-Driven Trade Credit & Vendor Terms Orchestration Engine" (OHC Vendor Credit Hub) inside the OHC platform.

  ### 1. User Journey (Critical User Journey - CUJ)
  - Priya (Boutique Owner) accesses her 375px mobile dashboard. She sees a glassmorphic "Credit Capacity" card indicating she is pre-approved for $5,000 based on her live OHC sales.
  - She taps the card to open the Credit Hub. In the "Vendor Terms" tab, she selects her supplier ("Milano Fabrics") and taps "Request Net-30 Terms".
  - The Negotiator AI autonomously compiles her zero-knowledge financial health report and sends an email request to Milano Fabrics with an onboarding verification link.
  - Concurrently, Maya (Baker) receives a $1,200 supplier invoice due in 30 days. She activates "Daily Sales Sweep" at 10% to budget for the payment.
  - Each day, the Accountant AI sweeps 10% of Maya's transactions to the credit reserve ledger. On day 30, the balance is fully accrued, and OHC triggers an automated ACH payout to clear the invoice.
  - Nora (Agency Principal) submits a $5,000 client invoice, selects "Instant Payout", and immediately receives $4,250 (85% advance) into her OHC payout ledger, with OHC managing client collections in the background.

  ### 2. Technical Acceptance Criteria
  - Database & RLS: Define the tables for `CreditFacility`, `VendorRelation`, `SupplierInvoice`, `FactoringDiscount`, and `LedgerSweepConfig`. Apply Postgres Row-Level Security bound to tenant_id.
  - Underwriting Engine: Implement a calculation service that computes credit capacity and limits by analyzing postgres `ledger_entries` (debits/credits) and upcoming bookings history.
  - Agent Workflow: Develop the Negotiator AI runner that compiles zero-knowledge PDF business packets and sends them via an automated mock email client.
  - Auto-Sweep Scheduler: Build a background job worker utilizing SKIP LOCKED queue patterns to execute the daily sweep transfers safely and with idempotent transaction keys.
  - Premium Mobile UI: Build 375px responsive screens for the Credit Capacity card, Vendor Terms sheet, and Factoring Discount slider using macOS-style translucent glass styles (backdrop-blur, emerald indicators) and 44x44px minimal touch targets.
  - E2E Playwright Tests: Implement a complete E2E test suite covering:
    - Querying the underwriting credit score for a seeded merchant.
    - Triggering an AI-led terms negotiation with a mock supplier.
    - Simulating daily sales ledger sweeps and verifying balance accrual.
    - Factoring an invoice and asserting the updated balances in the internal capital ledger.
  ```

  ---

  ## 8. Priority & Scope
  - **Priority**: `P1` (High - Critical for cash-flow solving and establishing OHC as a premium, indispensable operator platform).
  - **Estimated Scope**: `Large` (Involves transaction calculation, background sweep scheduling, document compilation, and multi-tenant UI flows).
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
