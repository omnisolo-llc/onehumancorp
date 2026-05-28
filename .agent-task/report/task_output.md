issue_title: "[Architecture] Zero-Hardware AI QR Table Ordering Mesh"
issue_description: |
  # [Architecture] Zero-Hardware AI QR Table Ordering Mesh

  ## Title
  Zero-Hardware AI QR Table Ordering Mesh

  ## Problem Statement
  Food and beverage operators (like Fatima with her halal food cart, or small pop-up cafes) struggle to manage table service and long queues without investing in expensive POS hardware or clunky third-party apps. They need a simple, intuitive way to generate QR codes that allow customers to view a live menu, order, and pay directly from their phones. This system must automatically sync with the merchant's mobile KDS (Kitchen Display System) and inventory in real-time, functioning smoothly even in congested network environments.

  ## Research Report
  *   **Square POS:** Requires dedicated hardware or complex setup for table management. QR ordering is treated as a separate "online" flow rather than a unified in-person experience, leading to fragmented operations.
  *   **Toast:** Highly robust but prohibitively expensive for solopreneurs. Requires professional installation, dedicated local network infrastructure, and expensive proprietary tablets.
  *   **Wix Restaurants:** Offers QR menus, but the customer experience is often a clunky web wrapper. It relies heavily on constant high-speed internet and lacks native integration with a high-performance mobile KDS.
  *   **The OHC Differentiator:** OHC leverages a zero-hardware, zero-config approach. The merchant can generate a dynamic, printable QR code directly from their OHC app. When a customer scans it, they access an edge-cached, dynamic AI-driven menu with instant tap-to-pay checkout (Apple/Google Pay). The order is routed via a hybrid event mesh directly to Fatima's low-end Android device (acting as the KDS).

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Customer as Customer Mobile (Browser)
      participant EdgeGateway as OHC Edge Gateway
      participant AiMenu as AI Menu & Pricing Agent
      participant EventMesh as Hybrid Event Mesh
      participant Merchant as Merchant KDS (OHC App)

      Customer->>EdgeGateway: Scans QR & Requests Menu
      EdgeGateway->>AiMenu: Fetches dynamic, localized menu
      AiMenu-->>EdgeGateway: Returns Edge-Cached Menu UI
      EdgeGateway-->>Customer: Renders Menu (Translucent Glass UI)
      Customer->>EdgeGateway: Submits Order & Payment (Apple/Google Pay)
      EdgeGateway->>EventMesh: Publishes `Order.Placed` event
      EventMesh-->>Merchant: Real-Time Push Notification & KDS Update
  ```

  ### Key Design Decisions & UI Specs
  *   **Zero-Hardware Generation:** Merchants can generate table-specific QR codes instantly within the OHC app and print them using the Universal Offline Thermal Print Mesh or save as PDFs.
  *   **Edge-Cached Menus:** The customer-facing menu must load in under 1 second, achieved by aggressive edge caching of the menu structure and assets.
  *   **Mobile-First UX (375px):** The customer menu adopts the macOS-style Translucent Glass materials. It must pass the "grandmother test"—large tap targets, high contrast text, and a sticky "View Cart & Pay" button at the bottom of the viewport.
  *   **Zero Trust Isolation:** Each QR code encodes a signed, cryptographically secure token tied to the specific tenant and table. This prevents cross-tenant data leakage and malicious order injection.

  ## Implementation Prompt
  **To the Implementer Swarm:**
  Your goal is to build the foundational architecture and UI for the "Zero-Hardware AI QR Table Ordering Mesh."
  *   **Customer User Journey (CUJ):**
      1. Fatima navigates to the "Tables & QR" section in the OHC app.
      2. She taps "Generate QR for Table 5" and prints it.
      3. A customer scans the QR code, instantly loading the live menu on their phone.
      4. The customer selects items and completes payment using Apple/Google Pay.
      5. The order instantly appears on Fatima's mobile KDS view.
  *   **Acceptance Criteria:**
      *   **QR Generation:** Implement the logic to generate secure, tenant-scoped QR codes.
      *   **Customer UI:** Build the 375px customer-facing menu view using the specified design tokens (Translucent Glass).
      *   **Routing & Sync:** Ensure the submitted order payload is correctly formatted and published to the Event Mesh for real-time delivery to the merchant app.
      *   **Security:** Enforce strict multi-tenant boundaries (SPIFFE/SPIRE context) so a customer scanning Table 5's QR code cannot view or modify data for another business.
  *   **Priority:** P1
  *   **Estimated Scope:** Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
