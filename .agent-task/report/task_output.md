issue_title: "Universal Gift Card and Store Credit Ledger"
issue_description: |
  # [Architecture] Universal Gift Card and Store Credit Ledger

  ## Problem Statement

  Small business owners like Priya (boutique owner) and Maya (baker) frequently need to issue store credit for returned items or sell branded digital gift cards across their various sales channels (Instagram DMs, online storefronts, and in-person POS). Currently, managing gift cards and store credit involves external tools, disjointed spreadsheets, or complicated accounting workarounds. This fragmentation fails the "grandmother test," causing friction, lost revenue, and poor customer experiences. OHC needs a unified, zero-config ledger that natively tracks, issues, and redeems gift cards and store credit seamlessly on mobile, functioning identically whether online or offline.

  ## Research Report

  ### Context and Market Analysis
  Gift cards are a massive revenue driver for SMBs, acting as upfront cash flow, while store credit is essential for retaining value on returns instead of processing cash refunds.
  - **Shopify:** Offers gift cards, but historically gated them behind higher pricing tiers, and omni-channel syncing (online vs. physical POS) can sometimes require specific hardware or apps.
  - **Wix/Squarespace:** Point of sale capabilities exist, but gift card redemption often feels fragmented between physical locations and the online store.
  - **Square:** Good physical gift card integration, but locked tightly into their hardware ecosystem.

  By integrating a Universal Gift Card and Store Credit Ledger natively into OHC, we give business owners an enterprise-grade capability without the setup complexity. A merchant can sell a gift card on an Instagram DM via an AI agent, and the customer can instantly tap to redeem it in person.

  ### Key Learnings
  1. **Omnichannel Parity is Critical:** A gift card purchased online must be instantly redeemable at the mobile POS or via an Instagram DM checkout.
  2. **Offline Resilience:** Merchants operating at pop-up shops or food carts (like Fatima) may have spotty cellular connections. Offline redemption with secure queueing is necessary.
  3. **Multi-tenant Security:** Strict isolation is required. Gift card codes are effectively digital currency and must be protected against cross-tenant fraud or unauthorized issuance using Zero Trust principles.

  ## Design Doc

  ### Key Design Decisions
  - **Unified Ledger Backend:** Treat gift cards and store credit as identical financial entities in a dedicated ledger, varying only by their issuance context (purchased vs. returned item).
  - **Mobile-First Issuance & Redemption:** Native OS capabilities (camera for scanning QR/barcodes, Apple Wallet/Google Wallet integration) to eliminate physical card necessity.
  - **Offline Mode & Event Queueing:** Allow offline redemption by caching cryptographic proofs or utilizing short-lived offline tokens. If fully offline, allow bounded risk acceptance synced upon reconnection.
  - **Zero Trust Security:** Enforce strict multi-tenant isolation (SPIFFE/SPIRE) so one merchant's ledger cannot be queried or manipulated by another.

  ### Architecture Diagram (Mermaid.js)

  ```mermaid
  erDiagram
      MERCHANT ||--o{ CREDIT_LEDGER : owns
      CUSTOMER ||--o{ CREDIT_BALANCE : holds
      CREDIT_LEDGER ||--o{ CREDIT_BALANCE : tracks
      CREDIT_BALANCE ||--o{ TRANSACTION : records

      MERCHANT {
          string id
          string tenant_id
      }
      CUSTOMER {
          string id
          string phone_number
      }
      CREDIT_LEDGER {
          string ledger_id
          float total_liability
      }
      CREDIT_BALANCE {
          string code_hash
          string type
          float current_amount
      }
      TRANSACTION {
          string tx_id
          float amount_changed
          string context
          boolean is_offline_queued
      }
  ```

  ```mermaid
  sequenceDiagram
      actor Customer
      participant App as OHC Mobile App / DM Agent
      participant CoreAPI as OHC Core API
      participant Ledger as Gift Card Ledger
      participant FinAgent as Finance Agent
      participant OpsAgent as Operations Agent

      Customer->>App: Presents Return / Buys Gift Card
      App->>CoreAPI: Request Store Credit / Issue Card (Offline Fallback)
      CoreAPI->>Ledger: Generate Code & Credit Balance
      Ledger-->>CoreAPI: Code Hash & Confirmation
      CoreAPI->>FinAgent: Update Liabilities & Revenue
      CoreAPI->>OpsAgent: Generate Apple/Google Wallet Pass
      OpsAgent-->>App: Deliver Digital Pass / QR Code
      App-->>Customer: Display & Send via SMS/Email
  ```

  ### Mobile-First UX Flow (375px)
  Every screen follows the macOS-style Translucent Glass materials combined with clean Ubiquiti UniFi modular dashboard card layouts.
  1. **Issuance Dashboard:** A clean, bold screen with modular preset amount cards ($25, $50, $100) and a custom amount keypad. One tap to "Issue to Customer" via SMS or AirDrop.
  2. **Redemption Scanner:** A full-screen camera view with a translucent glass bottom-sheet overlay. Automatically recognizes OHC gift card QR codes or Apple/Google Wallet NFC passes.
  3. **Cart Integration:** In the checkout modal, if a customer profile is linked to a store credit balance, a massive, high-contrast "Apply $X Store Credit" toggle appears. No hidden menus.
  4. **Offline Indicator:** A subtle, sticky banner displaying "Offline Mode: Queuing Redemptions" if the network drops, completely hiding API/sync complexities from the user.

  ### AI Agent Integration Points
  - **Operations Agent:** Intercepts return requests in the inbox and automatically proposes issuing store credit + 10% bonus instead of a cash refund, protecting merchant cash flow.
  - **Finance Agent:** Instantly reconciles the transaction, moving funds from recognized revenue to deferred liability, keeping the merchant's plain-language daily briefing accurate.
  - **Marketing Agent:** Monitors unused gift card balances and proactively texts customers before major holidays (e.g., "You still have $15 at Priya's Boutique! Come use it this weekend.").

  ## Implementation Prompt
  **Prompt for Implementer Agent:**
  Implement the Universal Gift Card and Store Credit Ledger for the OHC platform. The solution must support both physical POS redemption and online/DM checkout.
  Ensure the UX strictly follows the mobile-first (375px) Translucent Glass / modular card design system, passing the "grandmother test."
  Design the backend coordination to handle secure, multi-tenant ledger entries, generate Apple/Google Wallet passes, and coordinate with AI Finance and Operations agents for automated liability tracking and return management.
  Implement an offline-capable transaction queue for spotty network environments, ensuring zero-trust multi-tenant isolation via SPIFFE/SPIRE. Describe WHAT the system does without prescribing specific lower-level library or API function signatures.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
Scope: Medium
