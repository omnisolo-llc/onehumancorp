issue_title: Zero-Touch Expense & Receipt Reconciliation Engine
issue_description: >
  **Title**

  Zero-Touch Expense & Receipt Reconciliation Engine


  **Problem Statement**

  Small business owners—like Maya the baker buying flour in bulk, or Fatima the
  food cart operator purchasing daily produce—spend hours every month manually
  organizing crumpled receipts, matching bank statements to expenses, and
  categorizing them for taxes. This manual labor is error-prone, stressful, and
  pulls them away from their actual work. They need an automated, invisible
  system that seamlessly captures receipts (via a quick phone camera snap or
  email forwarding), matches them to bank feeds, auto-categorizes them for tax
  compliance, and seamlessly integrates into their unified financial ledger
  without requiring any accounting knowledge.


  **Research Report**


  ### Competitive Landscape

  *   **Expensify / Dext**: Strong standalone receipt capture and extraction
  tools, but they are typically designed to feed into larger accounting software
  (like QuickBooks or Xero), adding integration friction and extra subscription
  costs.

  *   **QuickBooks / Xero**: The industry standards for accounting, but they
  fail the "grandmother test". They require an understanding of charts of
  accounts, reconciliation rules, and double-entry bookkeeping. Their mobile
  apps are often secondary to the desktop experience.

  *   **Ramp / Brex**: Excellent automated expense tracking and matching, but
  built around corporate card issuance. They do not cater to the solopreneur who
  might occasionally mix personal and business accounts or rely heavily on
  cash/debit transactions.

  *   **Tailor / Keeper**: Good tax-focused tools for freelancers, but lack deep
  integration into the daily operational platform where the business owner
  actually runs their store or bookings.


  ### The OHC Gap

  OneHumanCorp currently lacks a native, AI-driven expense management and
  receipt reconciliation system. While we are building out invoicing and
  ledgers, business owners also need to track the money *going out*. We need an
  architecture where the Finance AI Agent can invisibly ingest receipts via an
  email inbox or an instant mobile camera snap, extract the data, match it
  against connected bank feeds, and categorize it for taxes—only notifying the
  user for 1-tap approvals on ambiguous transactions.


  **Design Doc**


  ### Architecture Diagram


  ```mermaid

  graph TD;
      MobileCamera[Mobile App: Camera Snap] --> IngestionAPI;
      EmailForward[Email Forwarding \n receipts@ohc.com] --> IngestionAPI;
      BankFeed[Plaid / Stripe Issuing \n Bank Feed] --> MatcherEngine;

      IngestionAPI --> DocumentStore[(Encrypted S3 \n Receipt Store)];
      IngestionAPI --> OCR[Vision AI / OCR Service];

      OCR --> ExtractionAgent[Finance AI Agent: \n Data Extraction];

      ExtractionAgent --> MatcherEngine[AI Reconciliation Engine];
      MatcherEngine --> Ledger[(Universal Ledger \n PostgreSQL)];

      MatcherEngine -- Ambiguous Match --> ReviewQueue[Human Review Queue];
      ReviewQueue --> MobileUI[Mobile App: 1-Tap Approval];
      MobileUI --> Ledger;
  ```


  ### Mobile UX Flow (375px Viewport)

  1.  **Capture**: A floating Action Button (FAB) on the main dashboard
  instantly opens the camera. The user snaps a receipt and immediately closes
  the app. No forms to fill.

  2.  **Processing**: In the background, a subtle, translucent notification
  toast appears: *"Analyzing receipt from Home Depot..."*

  3.  **Auto-Match**: If the system confidently matches the $45.20 Home Depot
  receipt to a synced bank transaction, it silently adds it to the Ledger
  categorized as "Supplies & Materials".

  4.  **Exception Handling**: If a match is ambiguous, a clean, macOS-style
  glass card appears on the dashboard: *"Did you spend $45.20 at Home Depot
  yesterday?"* with [Yes] / [No] buttons.

  5.  **Analytics**: A simple dial on the Finance tab shows "Total Expenses this
  Month" vs "Total Revenue", updating in real-time.


  ### AI Agent Integration Points

  *   **Finance Operations Agent**: Continuously polls the connected bank feeds
  and the incoming receipt queues (camera, email). Uses vision models to extract
  Vendor, Date, Total Amount, and Tax from images.

  *   **Tax Categorization Agent**: Uses contextual memory (e.g., knows Maya is
  a baker) to automatically categorize "Bob's Red Mill" as "Cost of Goods Sold"
  rather than a general grocery expense.

  *   **Proactive Nudge Agent**: If a recurring bank transaction (e.g., $15.99
  to Adobe) lacks a receipt for 3 months, the agent sends a friendly push
  notification: *"Hey, missing a receipt for Adobe—want to forward it to me?"*


  ### Key Design Decisions

  *   **Invisible by Default**: Users should never have to manually type in
  date, amount, or vendor. The AI must extract and match with >95% confidence
  before committing to the ledger.

  *   **Zero-Trust Isolation**: Receipts and bank transaction data are highly
  sensitive. Tenant data must be cryptographically isolated using SPIFFE
  workload identities for the extraction agents.

  *   **Mobile-First Snap**: The camera interface must open instantly (under
  500ms) to ensure the user actually captures the receipt before throwing it
  away. Desktop fallback is via drag-and-drop or email forwarding.


  **Implementation Prompt**


  **Role**: You are a backend and mobile implementer agent for OneHumanCorp.


  **Task**: Implement the Zero-Touch Expense & Receipt Reconciliation Engine.


  **User Journey (CUJ)**:

  Maya the baker buys a 50lb bag of flour. She opens the OHC app, taps the
  camera icon, snaps the receipt, and puts her phone away. The system processes
  the image, extracts the total ($45.00), matches it with a transaction from her
  connected bank feed, categorizes it as "Cost of Goods Sold", and attaches the
  digital receipt to the ledger entry invisibly.


  **Acceptance Criteria**:

  1.  Create an ingestion pipeline (API endpoint) that accepts image uploads
  (receipts) and securely stores them with tenant isolation.

  2.  Integrate a Vision AI service to extract Vendor, Date, Amount, and Tax
  from the uploaded images.

  3.  Build a matching heuristic/engine that pairs the extracted receipt data
  with raw transaction rows in the Ledger database.

  4.  Implement a fallback mechanism: if confidence is low, create a "Pending
  Review" record that surfaces in the mobile API for 1-tap approval.

  5.  All data models must enforce multi-tenant boundaries. Do not prescribe
  specific libraries, but ensure the solution scales horizontally in the
  cloud-native deployment mode.


  **Priority**

  P1


  **Estimated Scope**

  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label:
  - agent-report
assignees: []
