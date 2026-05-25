issue_title: "[architecture] Autonomous Unified Communications & Lead Triage Mesh"
issue_description: |
  # Issue Brief: Autonomous Unified Communications & Lead Triage Mesh

  ## Title
  [architecture] Autonomous Unified Communications & Lead Triage Mesh

  ## Problem Statement
  For non-technical small business owners like **Maya (baker, 28)** and **Carlos (handyman, 42)**, managing customer communications is overwhelming and scattered. Maya receives custom cake requests via Instagram DMs, WhatsApp, and email, often losing track of orders or missing deposits because she is too busy baking. Carlos misses potential leads because he cannot answer the phone or reply to SMS messages while working in a client's basement with spotty cell service. They don't need a complex CRM dashboard; they need an intelligent, invisible assistant that unifies all incoming messages into a single offline-capable queue, uses AI to automatically draft quotes, handle FAQs ("Do you do vegan cakes?"), and triages urgent leads without requiring manual intervention.

  ## Research Report
  Current SMB platforms (Shopify, Wix, Squarespace) treat communications as secondary to the storefront or require expensive, clunky third-party app integrations (e.g., Zendesk, Intercom, or fragmented social integrations).
  - **Shopify/Wix:** Rely on basic chat widgets or external apps. They do not natively aggregate Instagram DMs, WhatsApp, and SMS into a single offline-first inbox that can automatically generate quotes or payment links.
  - **Intercom/Zendesk:** Built for enterprise support teams, not for solo operators on mobile devices. They are complex to configure and require constant online connectivity.
  - **Opportunity for OHC:** A massive gap exists for an edge-native, zero-config unified inbox. By leveraging the OHC Hybrid AI OS, the KAIROS Orchestrator can intercept messages across all channels, use the Customer Success and Sales Agent departments to auto-draft context-aware replies (e.g., pulling live inventory or calendar availability), and present the owner with simple 1-tap approval notifications on their mobile device.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Incoming Channels: IG, SMS, WhatsApp] --> B[OHC API Gateway]
      B --> C[Unified Inbox Event Queue NATS]

      C --> D[AI Triage & Context Engine]
      D --> E{Agent Department Routing}

      E -->|FAQ/Support| F[Customer Success Agent]
      E -->|New Lead/Quote| G[Sales Agent]

      F --> H[Draft Reply Generation]
      G --> I[Draft Quote/Booking Generation]

      H --> J[Local-First Edge DB / SQLite]
      I --> J

      J --> K[Mobile App 375px - 1-Tap Approval]
      K -->|Approved| L[Outbound Webhook/API to Channel]
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **The "Grandmother Test" Mobile Inbox:** A highly simplified, macOS-style Translucent Glass UI. No complex folder structures.
  - **Screen 1 (Unified Feed):** A single vertical feed of message cards. Each card displays the sender's avatar, platform icon (IG, WhatsApp), and a brief preview.
  - **Screen 2 (Thread View):** Clean chat interface. At the bottom, instead of just a keyboard, an AI-drafted reply is pre-populated in a highlighted card (e.g., "Maya, here is a draft reply confirming you have vegan options").
  - **Interaction:** Maya taps a prominent "Approve & Send" button (44x44px touch target) or "Edit" to tweak. If Carlos receives a lead, the AI draft includes a fully interactive "Tap to send Quote" card.

  ### AI Agent Integration Points
  - **Customer Success Agent:** Monitors the inbound queue for FAQs. Accesses the Knowledge Base and Inventory Mesh invisibly to check if an item is in stock.
  - **Sales Agent:** Detects buying intent. Interfaces with the Universal Capacity & Inventory Ledger to check Carlos's calendar and auto-generates a booking deposit link.
  - **Memory Layer:** Retains context across channels (e.g., knows that an Instagram DM user is the same person who previously SMS'd about an order).

  ### Zero Trust & Security
  - Complete multi-tenant isolation at the edge database level.
  - All external API tokens (Meta, Twilio) are encrypted and isolated per tenant using SPIFFE/SPIRE identity boundaries.

  ### Performance & Offline Targets
  - **Offline Capability:** The mobile app must aggressively cache the inbox using a local SQLite/IndexedDB instance. Carlos can review drafted quotes and hit "Approve" while offline in a basement; the actions queue locally and sync instantly when connectivity returns.

  ## Implementation Prompt
  **To the Implementer Agent:**
  Your task is to implement the Autonomous Unified Communications & Lead Triage Mesh.
  - Build the edge-first data model to support a unified inbox queue handling SMS, IG DMs, and WhatsApp.
  - Integrate the NATS event bus to route incoming messages to the KAIROS Orchestrator.
  - Implement the 375px mobile-first UI components for the Unified Feed and 1-Tap Approval Chat view using the existing design tokens (Translucent Glass, large touch targets).
  - Ensure the AI drafting pipeline works seamlessly in the background and gracefully degrades to local caching when the device is offline.
  - **Acceptance Criteria:** A user (Maya) receives a simulated Instagram DM, the system automatically drafts a context-aware reply using the Customer Success Agent, and Maya can approve it with one tap on a mobile viewport layout. Offline actions must queue and replay successfully. DO NOT prescribe specific DB schemas; design the data access patterns to fit the offline-first requirements.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
