issue_title: "Autonomous Multimodal Visual Quoting & Intake Engine"
issue_description: |
  ## Problem Statement
  Small business owners like Carlos (Handyman) and Maya (Baker) spend hours manually reviewing text messages, Instagram DMs, and WhatsApp photos from customers asking "how much to fix this?" or "can you make a cake like this?". They are often forced to switch between their inbox, photo gallery, and quoting tools, losing context and taking too long to reply. Existing platforms require manual creation of line items for custom work, which leads to abandoned leads and slow turnaround times.

  ## Research Report
  *   **Codebase & Docs Audit:** Our current architecture handles structured checkout flows well but lacks an asynchronous, multimodal intake pipeline. The `Omnichannel AI Inbox` handles text, but fails to extract structured data (dimensions, materials, effort) from image payloads for automated quoting.
  *   **Competitor Systems Audit:** Platforms like Wix and Shopify rely on static "Contact Us" forms or basic file uploads. They do not natively pass images through multimodal LLMs to generate draft estimates. Independent tools like Joist (for contractors) require heavy manual data entry.
  *   **Identify Gaps:** OHC is missing a **Multimodal Intake Pipeline** that seamlessly ties into our quoting and invoicing engine. We need the ability to accept an image via any channel (SMS, IG DM, Web), run visual analysis to estimate scope, and instantly draft a quote for the business owner's 1-tap approval.

  ## Design Doc
  ### High-Level Architecture
  ```mermaid
  graph TD;
      Customer[Customer on IG/SMS/Web] -->|Sends Photo + Request| Gateway[Zero-Trust Edge Gateway];
      Gateway --> MCP_Hub[MCP Gateway Switchboard];
      MCP_Hub --> Inbox[(Unified Inbox Event Mesh)];
      Inbox --> VisionSidecar[Multimodal Vision Sidecar Worker];
      VisionSidecar -->|Extracts Scope, Materials, Effort| SalesAgent[Sales & Operations Agent];
      SalesAgent --> LocalDB[(OHC KAIROS Quote Ledger)];
      LocalDB --> UI[OHC App: Draft Quote View];
  ```

  ### Data Model & Invariants
  *   **IntakePayload:** Contains the origin channel, raw image blob (stored in ephemeral S3 bucket), and user text context.
  *   **VisualEstimate:** A structured JSON object derived by the Multimodal Vision Sidecar representing extracted dimensions, recognized objects (e.g., "damaged drywall", "3-tier cake"), and estimated complexity score.
  *   **Tenant Isolation:** Images are temporarily stored using tenant-isolated prefix keys and encrypted with tenant-specific SVIDs via SPIFFE.

  ### AI Department Coordination
  *   **Customer Success Agent:** Acknowledges receipt of the image ("Thanks! Let me take a look at this drywall...").
  *   **Operations Agent:** Evaluates the `VisualEstimate` against the `Global Inventory / Ledger` to check if required materials are in stock.
  *   **Sales Agent:** Drafts the actual quote with line items and pricing, pushing a notification to the owner for approval.

  ### Mobile UX Flow (375px First)
  1.  **Lock Screen Notification:** "New Quote Drafted: Drywall Repair from +1 555-1234."
  2.  **Quote Review View:** A split-screen card (Glassmorphism design). Top half shows the customer's submitted photo with AI-generated bounding boxes/highlights. Bottom half shows the auto-generated line items (Materials, Labor).
  3.  **Action:** Carlos taps "Approve & Send" (massive 60x60px primary action button) or taps a line item to adjust the price using the native numeric keyboard.

  ### Performance & Offline Targets
  *   The initial acknowledgement to the customer must fire within < 2 seconds.
  *   The multimodal inference and quote drafting must occur asynchronously, targeting a < 15 second total turnaround to generate the draft.
  *   If Carlos is offline, he can still view cached drafted quotes and hit "Approve", which queues via the `SyncDaemon`.

  ## Implementation Prompt
  **Objective:** Implement the Multimodal Vision Sidecar and integrate it with the Sales Agent quoting pipeline.

  **User Journey (CUJ) & Acceptance Criteria:**
  1.  When an image attachment is received via the unified inbox, it must be securely routed to the Multimodal Vision Sidecar.
  2.  The sidecar must extract structured attributes (e.g., item type, visible damage, estimated size) without requiring the user to define a schema upfront.
  3.  The Sales Agent must use these attributes to generate a draft quote, appearing in the mobile app's "Action Required" queue.
  4.  The business owner must be able to approve or modify the quote in 1-tap, which then sends a secure, localized invoice link back to the customer on their original platform (e.g., Instagram DM).

  **Constraints:**
  Do not prescribe specific vector DBs or multimodal LLMs. Design the gRPC/protobuf contracts between the Inbox Event Mesh and the Vision Sidecar to be model-agnostic. Ensure strict Zero-Trust tenant isolation for all uploaded media.

  ## Priority
  `P0`

  ## Estimated Scope
  Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
