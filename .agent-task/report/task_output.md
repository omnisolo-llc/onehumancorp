issue_title: "Implement Invisible Contract & Liability Engine"
issue_description: |
  # Title: Invisible Contract & Liability Engine: Zero-Friction AI Legal Agreements

  ## Problem Statement
  For small business owners like Carlos (a handyman) or Maya (a baker doing high-ticket wedding cakes), securing a client agreement is a source of immense friction and anxiety. Currently, if Carlos wants to quote a $5,000 deck repair, he has to leave his quoting app, open a clunky third-party tool like DocuSign or HelloSign, manually draft a PDF on a desktop, email a link to the client, and then manually reconcile the signature status before he can request a deposit. This multi-step, desktop-bound process breaks the mobile checkout flow, leading to lost leads and delayed payments. They need a system where legal protections—whether a full service contract, a photo release, or a liability waiver—are invisibly generated and seamlessly integrated into the 375px deposit checkout flow, requiring zero legal knowledge or manual PDF generation.

  ## Research Report
  **Market & Competitor Analysis:**
  - **Shopify:** Offers basic "Terms of Service" checkboxes at checkout. Cannot handle dynamic, multi-party electronic signatures or job-specific service contracts native to the platform.
  - **Wix / Squarespace:** Similar to Shopify, they offer static form builders or require clunky integrations with third-party apps (e.g., DocuSign integrations) that incur extra monthly costs and break the native UX.
  - **Jobber / HoneyBook:** Built for service businesses and offer integrated contracts, but they are often desktop-first, highly rigid, and lack autonomous AI drafting. They still force the user to "build a template" manually.

  **OHC Advantage:**
  By introducing the "Invisible Contract & Liability Engine," OHC eliminates the concept of "drafting a contract." The Legal AI Agent autonomously drafts a localized, plain-language agreement based on the job details (e.g., line items, total cost, dates). The e-signature block is embedded directly into the 1-tap mobile checkout flow, unified with the deposit payment. This provides Zero Trust security, multi-tenant isolation, and complete mobile parity.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ QUOTE : issues
      QUOTE ||--|{ LINE_ITEM : contains
      QUOTE ||--o| CONTRACT_LEDGER : requires
      CONTRACT_LEDGER ||--o| SIGNATURE_EVENT : captures
      CONTRACT_LEDGER {
          string status
          string contract_hash
          string jurisdiction
      }
      SIGNATURE_EVENT {
          string timestamp
          string ip_address
          string cryptographically_signed_blob
      }
  ```

  ### AI Agent Integration Points
  - **The Protector (Legal AI Agent):** Autonomously reads the `Quote` or `Booking` intent. Using context from the Tenant's business profile and local jurisdiction, it generates a mathematically hashed, plain-language contract or waiver.
  - **The Vigilant Manager (Ops AI):** Monitors the `CONTRACT_LEDGER`. If a signature is pending for 24 hours, it autonomously triggers a gentle SMS/Email reminder to the client.

  ### Key Design Decisions and Why
  1. **Dynamic Generation vs. Static Templates:** We use AI to generate the contract dynamically based on real-time transaction data rather than forcing Maya or Carlos to maintain legal templates. This passes the "grandmother test."
  2. **Unified Checkout & Signature:** The signature event is a prerequisite step in the payment flow, completely eliminating the "signed but not paid" or "paid but not signed" limbo states.
  3. **Immutability & Cryptographic Hashing:** To ensure legal enforceability, the generated contract text and the signature event are cryptographically hashed and stored in an immutable ledger, ensuring absolute Zero-Trust data integrity.

  ### Mobile UX & Screen Flow (375px First)
  1. **The Merchant View (Carlos):** While drafting a quote on his Android phone, Carlos toggles a switch: "Require Signature & Deposit." No PDFs are shown.
  2. **The Client View (Checkout):** The client receives an SMS link. Tapping it opens a sleek, glassmorphic 375px view.
  3. **The Agreement Sheet:** Above the "Pay $500 Deposit" button is a translucent, scrolling card outlining the key terms in plain language (e.g., "Materials are non-refundable").
  4. **The Signing Interaction:** The client taps "Sign," which expands a smooth bottom sheet where they trace their signature with a finger. Upon lifting their finger, the signature instantly maps onto the document, and the "Pay Deposit" button illuminates.
  5. **Offline Resilience:** The signature canvas utilizes local caching. If the client signs while on a subway, the app optimistically accepts the signature and queues the cryptographic payload for synchronization upon network reconnection.

  ## Implementation Prompt
  **Prompt for Implementer:**
  Implement the "Invisible Contract & Liability Engine" for OneHumanCorp. Your goal is to build a frictionless, mobile-first (375px) e-signature and contract agreement flow integrated directly into the deposit checkout sequence.

  You must create a seamless bottom-sheet UX for finger-drawn signatures that works flawlessly on low-end mobile devices and gracefully handles offline scenarios (optimistic UI updates). Implement the backend logic to accept, cryptographically hash, and immutably store the signature event linked to a specific Quote or Booking. Ensure the Legal AI Agent is integrated to dynamically generate the agreement text based on the job's context.

  Do not prescribe specific database schemas, column names, or function signatures. Focus on establishing the API contract between the frontend signature component and the backend ledger, ensuring strict Zero-Trust multi-tenant isolation. All UI components must adhere to OHC's premium, Translucent Glass materials design system. Include comprehensive E2E tests verifying the complete flow from Quote creation to successful signature and payment.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
