issue_title: "Architecture: Agentic Food Pre-Order & Pickup Workflow for Low-Latency Devices"
issue_description: |
  # Research Report: Agentic Food Pre-Order & Pickup Workflow for Low-Latency Devices

  ## Problem Statement
  Food cart operators and local food vendors (like Fatima) rely on high-volume, time-sensitive pre-orders. Traditional POS or e-commerce platforms (like Shopify or Toast) are either too expensive, too complex to set up, or require dedicated hardware. SMB vendors need a lightweight, mobile-first solution that supports offline-capable menus, multi-language UI, rapid pre-order with online payments, and real-time pickup notifications without overwhelming their low-end devices.

  ## Research Report
  - **Market Context**: Platforms like Toast and Square dominate the restaurant industry, but are overkill for a food cart operator. They have high setup costs, monthly fees, and complex inventory systems.
  - **The OHC Opportunity**: Providing a mobile-first, zero-hardware pre-order system built for low-end devices. This leverages OHC's Operations Agent to manage order states autonomously, reducing the cognitive load on the operator.
  - **Competitor Gaps**:
    - *Toast*: Expensive hardware, high monthly fees, complex setup.
    - *Square*: Better for simple transactions, but the online ordering flow is not optimized for rapid, real-time agentic notifications on slow data connections.
    - *UberEats/Doordash*: Exorbitant fees (up to 30%), detached customer relationship.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Web App] -->|Places Order & Pays via Stripe| B(Order Gateway)
      B --> C[PostgreSQL Central Ledger]
      B --> D[Event Mesh]
      D --> E[Operations Agent]
      E -->|Translates & Formats| F[Push Notification Service]
      F -->|Low-bandwidth payload| G[Vendor Mobile App Android]
      G -->|Local Caching| H[Printable Daily List]
  ```

  ### Mobile UX Flow (375px First)
  1. **Customer Pre-Order Flow**: A fast, edge-cached menu with clear "Sold Out" toggles. Customers select items, choose a pickup time, and pay via Google/Apple Pay.
  2. **Vendor App (Low-End Android)**: The vendor receives a high-contrast, large-font push notification ("New Order: 2x Falafel Platter, Pickup 12:30 PM").
  3. **Multi-Language Support**: The UI auto-translates customer notes from English to the vendor's preferred language (e.g., Arabic) using the Operations Agent.

  ### AI Integration Points
  - **Operations Agent**: Monitors order flow. If an item is ordered rapidly and stock is low, it suggests toggling "Sold Out" via a 1-tap notification. Automatically translates incoming English customer notes to Arabic for Fatima.

  ## Implementation Prompt
  **Feature Name**: Agentic Food Pre-Order & Real-Time Pickup Workflow
  **Target Persona**: Fatima the Food Cart Operator
  **Outcome**: A lightweight, offline-capable order management system. Customers can pre-order and pay online, and Fatima receives instant, translated notifications on her low-end Android device with simple order progression taps (Accept, Ready, Completed).

  **Next Actions**:
  1. Implement the `Order` data model with specific states for food pickup (`pending`, `preparing`, `ready_for_pickup`, `completed`).
  2. Build the low-bandwidth, high-contrast mobile vendor view for order management.
  3. Integrate the Operations Agent to handle automatic language translation of order notes and low-stock anomaly detection.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
