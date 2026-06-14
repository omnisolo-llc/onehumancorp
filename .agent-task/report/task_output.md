issue_title: "[Architectural Gap] Autonomous Tap-to-Pay Offline POS Engine"
issue_description: |
  # Research Report: Autonomous Tap-to-Pay Offline POS Engine

  ## Problem Statement
  Small business owners like Priya (boutique operator) and Fatima (food cart operator) face immense friction in the physical world. Existing mobile POS solutions (like Square or Shopify POS) are either disconnected from the online inventory or require high-end hardware and constant connectivity. For an owner trying to take payment at a farmers market, food cart, or busy shop floor, a network drop means lost revenue. Furthermore, these platforms lack AI agentic integration to automatically follow up with in-person customers or predict inventory needs based on physical sales. The gap is a mobile-first, offline-tolerant Tap-to-Pay engine that syncs seamlessly with the unified KAIROS backend.

  ## Research Report
  - **Market Context**: Square dominates the physical POS space but fails to provide deep, unified AI agent workflows. Shopify POS is robust but expensive and complex, often requiring external hardware.
  - **Pain Points**: Network reliability (especially in food carts or markets), hardware costs (need to buy dedicated card readers instead of using the existing phone), and inventory desync between physical and online sales.
  - **The Gap**: OHC currently lacks a dedicated, mobile-first Tap-to-Pay engine that leverages the phone's built-in NFC capabilities while maintaining optimistic state offline and reconciling with the KAIROS backend via AI agents.

  ## Design Doc

  ### Architecture
  ```mermaid
  graph TD
      A[Mobile App (Flutter)] -->|NFC Tap| B(Stripe Terminal SDK / Local Secure Element)
      B -->|Payment Intent| C{Network Check}
      C -- Online --> D[OHC Core API (Go)]
      C -- Offline --> E[Local SQLite Ledger (Encrypted)]
      E -->|Background Sync| D
      D --> F[KAIROS Orchestrator]
      F --> G[Inventory Agent (Deduct Stock)]
      F --> H[Customer Agent (Send Digital Receipt / Follow-up)]
  ```

  ### UI/UX Flow (Mobile First - 375px)
  1. **Home Screen**: A prominent, translucent "New Sale" floating action button (FAB) using UniFi-style modular cards.
  2. **Cart/Amount**: Quick numpad for custom amounts or visual grid for catalog items (optimized for 44x44px touch targets).
  3. **Payment Screen**: A full-screen, vibrant, pulsing "Tap to Pay" interface indicating the phone is ready to receive NFC payment.
  4. **Success/Offline State**: Immediate visual confirmation. If offline, a small, subtle badge indicates "Syncing when online" without blocking the next transaction.
  5. **Post-Sale**: Option to input phone/email for receipt, triggering the Customer Agent for future marketing.

  ### AI Agent Integration
  - **Inventory Agent**: Automatically adjusts stock levels once the offline ledger syncs.
  - **Customer Agent**: If customer contact info is captured, it drafts a "Thank you" email and potential review request.
  - **Decision Agent**: Includes offline sales in the daily briefing ("You took 15 offline payments today totaling $450").

  ## Implementation Prompt
  Implement the foundational mobile Tap-to-Pay offline POS engine architecture. Start by designing the local SQLite ledger schema in the Flutter mobile app to securely store pending offline transactions. Create the optimistic UI flow for taking a payment, including the n-key pad and the "Tap to Pay" waiting screen, ensuring it looks premium (macOS translucent glass style) on a 375px viewport. Then, define the sync API endpoints in the Go backend that the mobile app will call to reconcile the offline ledger once connectivity is restored. Finally, connect the sync endpoint to the KAIROS message bus so that the Inventory and Customer agents are notified of physical sales. **Acceptance Criteria**: A user can simulate an offline payment in the mobile app, the UI reflects a successful local transaction, and upon simulating network restoration, the transaction syncs to the backend and triggers a KAIROS event.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
