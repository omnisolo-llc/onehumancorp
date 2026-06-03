issue_title: "[Architecture] Autonomous Unified Offline-First Gift Card & Store Credit Engine"
issue_description: |
  # [Architecture] Autonomous Unified Offline-First Gift Card & Store Credit Engine

  ## Problem Statement
  For small business owners like Priya (Boutique Owner) and Maya (The Home Baker), maintaining customer loyalty and managing refunds often necessitates the issuance of gift cards or store credit. However, current platforms segregate online and in-person gift card systems. If Priya issues a digital gift card online, it's often difficult to redeem via Tap-to-Pay in-person without network connectivity, or vice versa. Furthermore, managing the liabilities of outstanding store credit creates an accounting headache. They need an invisible, unified engine that allows a gift card purchased online to be seamlessly redeemed in-store—even if the network drops—and for refunds to be automatically converted to digital store credit with zero manual bookkeeping.

  ## Research Report
  **Competitive Analysis:**
  - **Shopify:** Offers strong gift card functionality, but fully unifying physical POS redemption and online redemption can be clunky when offline capabilities are required. The accounting liability is tracked, but the automated transition of a refund into a gift card credit often requires additional apps.
  - **Square:** The leader in in-person gift cards (both physical and digital), but bridging this to a seamless online e-commerce experience can require their specific web builder, creating ecosystem lock-in.
  - **Wix:** Basic gift card functionality is present, but lacks robust offline-first redemption via mobile tap-to-pay paradigms.

  **Market Gap:**
  There is no built-in, out-of-the-box solution that treats "Store Credit" and "Gift Cards" as a unified, offline-capable cryptographic currency specific to the tenant, instantly available for offline POS redemption and seamlessly tracked by AI financial agents.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Mobile Device (375px)
          App[OHC Mobile App] --> LocalLedger[(Local CRDT Ledger)];
          App --> QRScanner[Camera QR / NFC Scanner];
          LocalLedger --> SyncEngine[Offline Sync Engine];
      end

      SyncEngine -- Network Restored --> Gateway[OHC API Gateway];
      Gateway --> CoreLedger[(Cloud Postgres Ledger)];
      Gateway --> Agents[AI Agent Swarm];

      subgraph Agent Departments
          Agents --> FinanceAgent[Finance: Track Liabilities];
          Agents --> CSAgent[CS: Email Digital Card];
          Agents --> OpsAgent[Ops: Process Refund to Credit];
      end

      CoreLedger --> Stripe[Stripe API - Initial Payment]
  ```

  ### UI/UX Flow (Mobile-First 375px)
  1. **Issuance (Online/In-App):** Customer purchases a $50 gift card. The CS Agent automatically emails a beautifully designed, Apple Wallet/Google Wallet compatible digital card (QR code + NFC payload).
  2. **In-Store Redemption (Offline-Capable):**
     - Priya rings up a $45 dress on her phone.
     - Customer presents the digital gift card.
     - Priya scans the QR code (or taps via NFC) using her OHC app.
     - The app checks the Local CRDT Ledger. Even if offline, if the card's cryptographically signed balance was previously synced, it authorizes the transaction and records a pending CRDT deduction.
  3. **Refund to Credit:** If a customer returns an item, Priya taps "Refund". A modal asks "Refund to Original Payment or Issue Store Credit?". Selecting "Store Credit" instantly texts the customer a new digital gift card link, logged transparently by the Finance Agent.

  ### AI Agent Integration Points
  - **Customer Success Agent:** Automatically designs and distributes the digital gift cards via email/SMS. Follows up with customers who have expiring or unused credit.
  - **Finance & Payments Agent:** Treats outstanding gift cards as a liability on the plain-language financial reports. Automatically reconciles the CRDT sync when offline redemptions hit the cloud.
  - **Legal & Compliance Agent:** Generates region-specific terms and conditions for gift card expiration and non-refundability rules.

  ### Key Design Decisions
  - **Offline-First Cryptographic Verification:** Gift cards must contain signed payloads allowing offline validation of their last-known balance via the CRDT ledger, preventing complete blockage during network outages.
  - **Unified "OHC Tender":** Store credit and gift cards are treated identically in the database schema—as a specific tender type tied to the `tenant_id`.

  ## Implementation Prompt
  Implement the Autonomous Unified Offline-First Gift Card & Store Credit Engine.
  - **User-Facing Outcome:** The user (merchant) can sell digital gift cards and issue store credit for refunds. Customers can redeem these cards both online and in-person via QR/NFC scan on the merchant's OHC mobile app, even in offline mode. The AI automatically tracks the financial liability.
  - **CUJ:**
    1. Merchant processes a return and selects "Issue Store Credit".
    2. Customer receives an SMS with an Apple Wallet compatible digital card.
    3. Customer visits the physical store, purchases an item, and merchant scans the digital card to pay.
    4. Transaction completes successfully, decrementing the card balance, even if the merchant device is temporarily offline (using CRDT sync).
  - **Acceptance Criteria:**
    - Develop the data model for `GiftCard` and `LedgerTransaction` supporting offline CRDT synchronization.
    - Implement the issuance workflow in the mobile UI (375px) utilizing the translucent glass design system.
    - Integrate the Finance Agent to track the liability and the CS Agent to handle distribution.
    - Guarantee row-level multi-tenant isolation.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []