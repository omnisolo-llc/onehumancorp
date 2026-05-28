issue_title: "Implement Autonomous QR Spatial Commerce Engine"
issue_description: |
  **Title**: Autonomous QR Spatial Commerce & Contextual Ordering Engine

  **Problem Statement**:
  For offline-first small business owners (like Fatima at her food cart, or Priya in her boutique), bridging the gap between physical inventory/space and digital commerce is filled with friction. They currently rely on manual price tags, static PDF menus accessed via QR codes, or separate tablet-based Point of Sale (POS) systems that don't seamlessly sync with online storefronts. The gap: There is no native, fully integrated, autonomous capability in OneHumanCorp to link physical locations (tables, store sections, popup stalls) directly to a dynamic, localized digital checkout experience that maintains context (e.g., "Table 4", "Summer Collection Rack") and syncs instantly with the universal ledger and capacity mesh.

  **Research Report**:
  - **Competitor Analysis:**
    - **Shopify:** Requires third-party apps for table-side ordering or custom QR code generation linked to specific cart attributes. It is not natively optimized for contextual spatial ordering out of the box.
    - **Square:** Offers QR code ordering specifically for restaurants, but lacks the flexibility to apply this natively to other business types (like Priya's boutique or Leo's booking portal) without complex configurations.
    - **Wix/Squarespace:** Primarily static sites; any spatial or contextual QR ordering requires heavy customization or embedded widgets.
  - **The Opportunity:** A unified "Spatial Commerce Engine" that allows any business type to instantly generate "Context-Aware Smart QRs". When scanned by a customer, the OHC platform instantly spins up a localized, high-performance edge-cached WebApp (375px optimized). The WebApp knows exactly *where* the customer is (e.g., Food Cart Window, Boutique Fitting Room) and automatically routes the resulting order, payment, and AI agent interactions (like upselling or answering product questions) with that physical context intact.

  **Design Doc**:
  - **Architecture Diagram**:
    ```mermaid
    erDiagram
        PhysicalContext ||--o{ SmartQRCode : generates
        SmartQRCode ||--|| EdgeCachedStorefront : triggers
        EdgeCachedStorefront ||--o{ ShoppingSession : initiates
        ShoppingSession ||--|| UniversalLedger : records
        ShoppingSession ||--o{ AgentInteraction : contains
    ```
  - **System Flow**:
    ```mermaid
    sequenceDiagram
        Customer->>MobileBrowser: Scans Smart QR Code
        MobileBrowser->>EdgeNode: Request contextual storefront
        EdgeNode->>OHC_Core: Fetch current catalog & context (e.g. Table 4)
        OHC_Core-->>EdgeNode: Return 375px optimized storefront + Context ID
        EdgeNode-->>MobileBrowser: Render Storefront
        Customer->>MobileBrowser: Apple Pay / Google Pay 1-tap checkout
        MobileBrowser->>OHC_Core: Submit order with Context ID
        OHC_Core->>BusinessOwnerDevice: Push notification: "New Order at Table 4"
    ```
  - **UI Wireframes & Mobile UX Flow (375px First)**:
    - **Customer View**: Clean, macOS-style Translucent Glass cards. UniFi-style modular grid of products. Sticky bottom sheet with a 1-tap Apple Pay/Google Pay button. A floating chat bubble allows the customer to ask questions.
    - **Business Owner View**: "Add Physical Location" button. Instantly displays a high-res, printable QR code. Live Activity Feed showing active scans and carts.
  - **Mobile Parity & Performance Targets**: Storefront must render in < 800ms globally via Edge caching. Offline support queueing via SQLite.
  - **AI Agent Integration Points**:
    - **Customer Support Agent**: Sits alongside the storefront.
    - **Operations Agent**: Monitors live scans for discounts.
    - **Marketing Agent**: Tracks physical locations conversion rates.

  **Implementation Prompt**:
  **Objective:** Build the Autonomous QR Spatial Commerce & Contextual Ordering Engine.
  **User Journey (CUJ):**
  1. As a business owner, I want to create a Spatial Link and get a printable QR code.
  2. As a customer, I want to scan that QR code and see an edge-cached 375px storefront with physical context.
  3. As a customer, I want to easily checkout with 1-tap.
  4. As a business owner, I want to receive instant push notifications with ledger sync.

  **Acceptance Criteria:**
  - Support generating unique contexts per tenant.
  - Route QR scans to localized storefront sessions containing context ID.
  - Give AI Agents access to context ID for responses.
  - Strict multi-tenant isolation.
  - Pass the "grandmother test" (no tech jargon).

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
