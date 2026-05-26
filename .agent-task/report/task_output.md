issue_title: "[Integration] Integrate PrintNode for Universal Cloud Printing"
issue_description: |
  ## Problem Statement
  Fatima (Food Cart Operator) and Priya (Boutique Owner) rely on physical outputs like receipts and kitchen tickets to run their daily operations. While OHC supports local mesh printing, small business owners often struggle with unsupported legacy hardware, dropped Bluetooth connections, or complex local network configurations. They need a bulletproof way to reliably send print jobs from their mobile device to any receipt printer, anywhere, without needing an IT degree.

  ## Research Report
  - **Tool Evaluated**: PrintNode (printnode.com)
  - **Ecosystem Audit**: Competitors like Square lock users into expensive, proprietary hardware. Shopify requires paid third-party apps for robust kitchen printing.
  - **User-First Value Mapping**: PrintNode acts as a universal bridge. The user installs a lightweight client on an old PC or Mac connected to their existing USB or Network printer. OHC can then send print jobs (receipts, tickets) to that printer over the internet from anywhere. This means Fatima can use her cheap USB thermal printer without needing to buy a $300 Bluetooth/Wi-Fi model, and Priya can print receipts from her phone even if she's not on the store's Wi-Fi.
  - **Capabilities & Limits**: PrintNode supports RAW printing (ESC/POS) which is critical for thermal receipt printers, as well as PDF printing. It has a robust REST API and webhooks.
  - **SaaS Viability**: PrintNode offers a generous free tier for small volumes and affordable paid tiers. It is highly reliable and takes the burden of managing complex printer drivers off the OHC platform.

  ## Design Doc
  - **Trigger**: An order is placed online, or an in-person tap-to-pay transaction completes.
  - **Actions**: OHC's backend intercepts the "Order Completed" event. It checks if the user has a linked PrintNode account and a default printer selected. If so, OHC generates the receipt (ESC/POS or PDF) and sends a request to the PrintNode API to print it immediately.
  - **User View**: In OHC "Operations" settings, the user clicks "Connect PrintNode", enters their API key, and selects their default receipt printer from a simple dropdown list of their connected printers.

  ## Implementation Prompt
  **User-Facing Outcome**: The user can connect their PrintNode account to OHC and automatically print receipts and kitchen tickets to any printer they own, regardless of whether their mobile device is on the same network as the printer.

  **Acceptance Criteria**:
  - User can connect a PrintNode account via API key.
  - User can select a default printer from their PrintNode account for Receipts and Kitchen Tickets.
  - When a transaction is completed, OHC automatically sends a print job to the selected PrintNode printer.
  - The system must support both standard PDF printing and RAW (ESC/POS) printing for thermal receipt printers.
  - The AI "Operations Agent" should monitor the PrintNode API for offline printer status and notify the user (e.g., "Fatima, your kitchen printer is offline!").

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
