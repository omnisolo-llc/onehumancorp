issue_title: "[Inventory] Autonomous Physical Store Inventory Sync"
issue_description: |
  # Issue Brief: Autonomous Physical Store Inventory Sync

  ## Problem Statement
  Small business owners with both a physical presence and an online storefront (like Priya, the Boutique Owner) struggle with "inventory drift." Selling an item in-store means they have to manually log into their website to decrement the stock. This leads to double-selling (selling out-of-stock items online), frustrated customers, and wasted time. Current platforms (Shopify POS, Square) require purchasing expensive proprietary hardware or navigating complex, developer-oriented multi-location inventory settings. A non-technical owner needs a way for their phone to act as the single source of truth for both physical and digital sales without manual data entry.

  ## Research Report
  - **Competitor Landscape**:
    - **Shopify**: Excellent online-to-offline sync *if* the user pays for Shopify POS Pro and uses their specific hardware. Setup is complex and requires understanding of "locations" and "inventory states."
    - **Square**: Strong physical POS, but the online store integration is often clunky, and setting up variants (size/color) is confusing for beginners.
    - **Wix**: Basic inventory tracking, but not optimized for fast-paced, in-person retail checkouts via mobile.
  - **User Needs**: The user needs their smartphone to function as a unified terminal. When an item is sold via "tap-to-pay" on the phone, the online inventory must instantly reflect the change.
  - **AI Differentiation**: Instead of just tracking numbers, the **Operations Agent** monitors velocity. If Priya sells out of "Red Summer Dresses," the agent automatically updates the storefront, but also drafts an email to her supplier to reorder, or suggests running a promotion on the remaining "Blue Summer Dresses."

  ## Design Doc
  ### High-Level Architecture
  - **Trigger**: A transaction occurs (either an online checkout via Stripe, or an in-person tap-to-pay via Stripe Terminal SDK on the mobile app).
  - **Agent Action (Operations Dept)**:
    - The agent intercepts the successful payment webhook/event.
    - Decrements the specific SKU/Variant in the global `INVENTORY_LEDGER`.
    - Pushes a real-time update to the Edge Cache (CDN) to ensure the online storefront reflects the new stock level instantly.
  - **Agent Action (Advisory/Sales Dept)**:
    - Evaluates stock levels against defined thresholds.
    - If sold out, triggers the "Sold Out" UI state.
    - Optionally drafts a reorder request or marketing adjustment.

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Physical as Mobile Tap-to-Pay
      participant Stripe as Stripe Terminal
      participant OHC_Core as OHC Core API
      participant Ledger as INVENTORY_LEDGER
      participant OpsAgent as Operations Agent
      participant CDN as Storefront CDN

      Physical->>Stripe: Process Payment (SKU: 123)
      Stripe-->>OHC_Core: Webhook (Payment Succeeded)
      OHC_Core->>Ledger: Decrement Stock (SKU: 123, Qty: -1)
      Ledger-->>OpsAgent: Event: Stock Changed
      OpsAgent->>CDN: Invalidate Cache / Update UI
      OpsAgent->>OpsAgent: Check thresholds (Sold Out?)
      alt Sold Out
          OpsAgent->>Physical: Notification: "Red Dress sold out. Drafted reorder email."
      end
  ```

  ### Mobile UX Flow (375px First)
  1. **The Checkout**: Priya uses the OHC app. She taps the "Camera" icon to scan the barcode of the Red Dress, or taps the visual product catalog.
  2. **The Payment**: She taps "Charge $45" and the customer taps their credit card to the back of Priya's iPhone (NFC).
  3. **The Sync**: A small, translucent, non-intrusive toast notification appears: *"Paid $45. Online stock updated to 3."*
  4. **Zero Configuration**: There is no "Locations" tab or "Sync Status" page. The app assumes the phone is the store.

  ## Implementation Prompt
  Implement the "Autonomous Physical Store Inventory Sync" feature. Develop an event-driven flow where successful transactions (both online and in-person via terminal/tap-to-pay) trigger an immediate, robust decrement in the central `INVENTORY_LEDGER`. Integrate the Operations Agent to monitor these stock changes in the background, updating the public storefront cache, and triggering low-stock notifications or reorder drafts for the business owner. Ensure the multi-tenant data model strictly isolates inventory per business.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
