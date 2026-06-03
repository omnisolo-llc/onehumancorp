issue_title: "[Operations] Mobile-First Inventory Scanner"
issue_description: |
  **Problem Statement**
  Managing inventory on the go is difficult for small business owners (like Fatima). They need a mobile-first scanner to quickly scan products and update inventory using their device camera, replacing manual data entry.

  **Research Report**
  - **Pain Points:** Operations fatigue from relying on spreadsheets or notebooks. Existing tools (Shopify POS, Wix Owner) are either clunky for quick updates or limit inventory management on mobile.
  - **Competitive Advantage:** Integrating an on-device scanner with OHC's "Vigilant Manager" AI agent allows instant database updates and auto-evaluates restock thresholds in the background.

  **Design Doc**
  - **Architecture:** User camera -> barcode/image recognition -> local/remote inventory update -> triggers Vigilant Manager.
  - **Mobile UX Flow:**
    1. Tap large "Scan Inventory" FAB.
    2. Camera opens with targeting overlay.
    3. Product detected instantly, displaying a bottom sheet with count adjuster.
    4. Confirm stock update.

  **Implementation Prompt**
  Implement a fast, mobile-first inventory scanner. The feature should allow a user to tap a floating action button on the inventory/dashboard page to open a camera view, scan a product barcode, and display a bottom sheet to adjust inventory count. Ensure the action integrates with the backend and notifies the Vigilant Manager agent.

issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
