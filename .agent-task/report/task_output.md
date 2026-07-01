issue_title: "Implement Universal Offline Thermal Print Mesh & Multilingual KDS Engine"
issue_description: |
  # Research Report: OHC Multilingual KDS & Offline Thermal Print Mesh

  ## Problem Statement
  Food cart operators like Fatima struggle to process pre-orders smoothly due to language barriers (she operates in Arabic, while customers order in English) and unreliable internet connections. Currently, operators must manually translate orders, which slows down service, causes mistakes, and leads to lost revenue. Moreover, existing digital KDS systems require expensive hardware and reliable internet. Fatima needs a resilient, mobile-first KDS (Kitchen Display System) that turns any low-end smartphone into a real-time, multilingual pre-order receiver with offline thermal printing capability.

  ## Research Report
  - **Findings & Competitive Analysis:**
    - **Shopify / Wix:** Good for e-commerce, but their POS systems lack robust offline capability and native multilingual order translation built into the merchant display. Merchants rely on unreliable third-party plugins.
    - **Square:** Offers a decent KDS, but requires proprietary hardware and lacks the autonomous translation layer.
    - **OHC Opportunity:** By leveraging our offline-first architecture (SQLite + Powersync) and local Translation Mesh, we can provide a zero-hardware KDS that instantly translates English customer orders into Arabic for the merchant and reliably routes receipts to standard ESC/POS thermal printers via Bluetooth, even without internet.
  - **Persona Focus:** Fatima (Food Cart Operator, limited English, low-end Android device, intermittent connectivity).

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Web Pre-order (English)] -->|API| B(Backend Server)
      B --> C[Postgres DB]
      C -->|Powersync| D[Mobile App Local DB (SQLite)]
      D --> E{Local Translation Mesh}
      E --> F[KDS UI (Arabic)]
      F --> G[Thermal Print Formatting Engine]
      G -->|Bluetooth ESC/POS| H[Local Thermal Printer]

      subgraph Offline Operations
      D --> E
      E --> F
      F --> G
      G --> H
      end
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Home Dashboard:** An "Active Orders" feed. New orders flash with a clear visual indicator.
  - **Order Card:** Displays the order details in Arabic (translated from the customer's English input). Big, tapable buttons for "Mark Complete" and "Print Receipt".
  - **Print Settings Modal:** Simple pairing screen for local Bluetooth ESC/POS printers. "Test Print" button to verify the connection.
  - **UX Considerations:** High contrast, large touch targets (min 44x44px), and clear offline status indicators.

  ### AI Agent Integration Points
  - **Local Translation Mesh:** When a new order syncs to the local device, the translation mesh (backed by an LLM worker running in the cloud or a local small model) instantly translates item names and customer notes into the merchant's preferred language (e.g., Arabic).

  ### Key Design Decisions
  - **Offline-First:** All critical KDS operations (viewing orders, printing) must work entirely offline, relying on the local SQLite database synced via Powersync.
  - **Standardized Printing:** Implement a robust ESC/POS formatting engine capable of handling complex character sets (like Arabic UTF-8) by rendering them into image buffers or using appropriate codepages for thermal printing.
  - **Zero-Hardware:** The entire solution must run on a standard, low-end smartphone.

  ## Implementation Prompt
  **User-Facing Outcome:** As a food cart operator, I receive an order from a customer in English, but my phone displays the order in Arabic. Even if my internet drops, I can still see the order, mark it as complete, and instantly print an Arabic receipt from my portable Bluetooth printer.
  **CUJ & Acceptance Criteria:**
  1. Set up a local SQLite database that syncs orders using Powersync.
  2. Implement the KDS UI to display incoming orders, localized into the merchant's language.
  3. Build the Thermal Print Formatting Engine to convert order details into ESC/POS commands (handling Arabic text correctly).
  4. Ensure the UI includes a simple way to connect and print to a Bluetooth thermal printer.
  5. The entire view and print flow must function when the device is fully offline (disconnected from network).
  6. Provide automated tests verifying the print formatting engine correctly generates ESC/POS bytes for a given localized order payload.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
