issue_title: "Implement Autonomous Omnichannel Waitlist & Reservation Mesh"
issue_description: |
  # [architecture] Autonomous Omnichannel Waitlist & Reservation Mesh

  ## Title
  Autonomous Omnichannel Waitlist & Reservation Mesh

  ## Problem Statement
  Small business owners like Maya (the baker taking custom orders) and Fatima (the food cart operator) often experience massive, sudden demand spikes (e.g., a viral TikTok video or a holiday rush). When inventory sells out, they currently lose those potential customers entirely. Attempting to manage a manual waitlist via scattered Instagram DMs, text messages, and a website form leads to overwhelming operational fatigue and lost revenue. They need a system that intelligently and invisibly captures customer intent across every channel, manages a smart waitlist, and automatically notifies customers and takes deposits the second capacity opens up—without the business owner doing anything but making their product.

  ## Research Report
  *   **Shopify:** Basic "back in stock" email notifications. No built-in omnichannel waitlist management. Merchants must install expensive 3rd-party apps that only work on the web storefront, ignoring social media intent.
  *   **Wix / Squarespace:** Simple form-based waitlists. Zero integration with social DMs or automated deposit taking. Requires manual matching of waitlist to inventory.
  *   **OneHumanCorp (OHC) Differentiation - "Invisible Autonomy":** Instead of making the merchant manage a spreadsheet of interested customers, the Ambassador Agent and Operations Agent collaborate seamlessly. Whether a customer comments on an Instagram post, sends an SMS, or visits the storefront, the waitlist mesh captures the intent, dynamically queues the customer, and autonomously executes checkout flows when inventory or capacity is available.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      CUSTOMER_INTENT ||--o{ OMNICHANNEL_ROUTER : "Ingests (IG, SMS, Web)"
      OMNICHANNEL_ROUTER }|--|| WAITLIST_MESH : "Routes to"

      WAITLIST_MESH {
          string spiffe_identity "Zero Trust Routing"
          string tenant_id "Multi-tenant Isolation"
          string product_or_slot_id
      }

      WAITLIST_MESH ||--o{ OPS_AGENT : "Checks Capacity"
      WAITLIST_MESH ||--o{ AMBASSADOR_AGENT : "Triggers Communication"

      OPS_AGENT }|--|| INVENTORY_LEDGER : "Monitors"
      AMBASSADOR_AGENT }|--|| CHECKOUT_ENGINE : "Generates Deposit Links"
  ```

  ### UI Wireframes & 375px Baseline
  **Core Layout: macOS-style Translucent Glass + Ubiquiti UniFi Modular Dashboard Cards**
  *   **Global Viewport:** 375px width (Mobile First). No horizontal scrolling.
  *   **App Bar:** Blurred glass top nav with the business logo.
  *   **Waitlist Hub View:**
      *   A clean, visually appealing dashboard card indicating total uncaptured revenue locked in waitlists (e.g., "✨ 142 people waiting for Vegan Cakes - $4,260 potential").
      *   Frosted glass list (`rgba(255, 255, 255, 0.05)` with `backdrop-filter: blur(10px)`) of high-demand items or services.
  *   **Demand Detail View:**
      *   Tapping an item shows a simple queue. The user can hit a single, prominent button: `[ ⚡ Auto-Fulfill Top 10 ]` when they produce more.
      *   Advanced logic (like VIP prioritization) is hidden under "Advanced Settings".

  ### Mobile UX Flow
  1. **Demand Spike:** Maya's custom cake sells out. Customers start DMing her on Instagram asking for it.
  2. **Invisible Capture:** The Ambassador Agent replies to DMs: "We're sold out for this week, but I've added you to the priority waitlist. I'll message you first when spots open!"
  3. **Restock:** Maya updates her inventory or availability. The Ops Agent detects the change.
  4. **Autonomous Notification & Deposit:** The Ambassador Agent automatically reaches out to the top waitlisted customers with an integrated 1-click deposit link.
  5. **Dashboard Feed:** Maya receives a push notification and sees a frosted glass card in her feed: "✨ 5 waitlisted cakes automatically converted. $750 collected."

  ### AI Agent Integration Points
  *   **Customer Service (CS) / Ambassador Agent:** Interprets intent from unstructured DMs ("put me on the list", "when is it back") and automatically adds customers to the waitlist mesh. Sends automated follow-ups with payment links.
  *   **Operations Department:** Monitors the Inventory Ledger. The second new capacity is detected, it signals the Ambassador Agent to begin the outreach cascade.
  *   **Finance Department:** Handles the secure generation of deposit links or pre-authorizations for the waitlisted customers.

  ### Key Design Decisions (Why, not How)
  *   **Omnichannel Ingestion:** The waitlist must be agnostic to the channel. A customer asking on Instagram is just as valuable as one clicking a button on the website.
  *   **Deposit Driven:** Waitlists are useless if they don't convert. The system must bias towards securing a micro-deposit or pre-authorization to guarantee revenue.
  *   **Zero Trust multi-tenant:** The Waitlist Mesh must strictly enforce tenant boundaries to prevent cross-contamination of customer PII and intent data.

  ## Implementation Prompt
  **To the Implementer Swarm:**
  Your goal is to build the "Autonomous Omnichannel Waitlist & Reservation Mesh". This system must invisibly capture customer demand when inventory is zero or the calendar is full, directly from omnichannel sources (DMs, SMS, Web), and automatically convert that demand into revenue when capacity opens up.

  **Customer User Journey (CUJ):**
  1. A highly demanded item (e.g., custom cake) goes out of stock.
  2. A customer sends an Instagram DM asking if they can buy one.
  3. The system parses the intent, adds them to the waitlist for that specific product, and replies automatically.
  4. The merchant updates their inventory, adding 5 new cakes.
  5. The system automatically messages the first 5 people on the waitlist with a payment link to secure their cake.

  **Acceptance Criteria:**
  *   **Mobile Parity:** The UI for managing demand must be flawlessly implemented for a 375px viewport using Translucent Glass materials.
  *   **Intent Parsing:** Ensure the system can mock-ingest a plain-text DM and successfully route it to the Waitlist Mesh.
  *   **Autonomous Cascade:** When inventory is artificially increased in the test, the system must trigger the outgoing notification/payment-link event automatically without manual merchant intervention.
  *   **Isolation Guarantee:** Strict multi-tenant boundaries must be applied to the Waitlist Mesh; one tenant cannot see another's queued demand.
  *   **Simplicity:** No complex spreadsheet views for the merchant. Hide all complex queue logic behind the "Advanced Settings" toggle.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []