issue_title: "Edge-Accelerated Zero-Config Flash Sale & Drop Engine"
issue_description: |
  # [Architecture] Edge-Accelerated Flash Sale Drop Engine

  ## Title
  **Edge-Accelerated Zero-Config Flash Sale & Drop Engine**

  ## Problem Statement
  Small business owners like Priya (boutique owner) and Maya (baker) rely on high-hype product "drops" (e.g., a limited run of 50 exclusive dresses or special holiday cakes) to drive sudden revenue spikes and social media engagement. Currently, managing a drop is a technical and operational nightmare. If Priya hypes a drop to her 10,000 Instagram followers and they all hit her storefront at the exact same minute, standard e-commerce platforms either crash or force her to manually set up complex "waiting room" plugins. Worse, inventory overselling happens frequently because the local inventory sync isn't fast enough, leaving Priya to deal with angry customers who paid but won't get the item.

  She needs a system where she can just set a date/time on a product in her app and say "This drops on Friday at 5 PM." The platform should handle the sudden traffic spike, prevent bots, enforce strict inventory limits without overselling, and automatically notify her waitlisted customers, all completely invisibly without her touching a single server setting.

  ## Research Report
  ### Competitive Landscape
  *   **Shopify:** Handles high traffic well for enterprise customers (Shopify Plus), but smaller merchants on standard plans often struggle with bot attacks during drops or need third-party apps for waiting rooms. Overselling still occasionally occurs during extreme flash sales without dedicated setup.
  *   **Wix & Squarespace:** Not designed for instant massive concurrency. A sudden influx of thousands of simultaneous checkouts on a single item often leads to slow load times, timeouts, and inventory desynchronization.
  *   **Ticketmaster / Livenation:** Uses robust queuing, but it's an awful, high-friction user experience (long loading bars, confusing captchas) that kills the boutique/premium feel a small creator wants.

  ### Opportunity
  OneHumanCorp (OHC) can offer "Enterprise-Grade Drops for Solopreneurs". By leveraging edge-caching (Cloudflare/Fastly) combined with a high-performance Rust-based Redis queue for inventory decrementing, OHC can ensure a 0% oversell rate and sub-second page loads during maximum traffic spikes. Maya just sets a timer; the OHC Marketing AI sends the hype emails, the Ops AI spins up the elastic queue capacity behind the scenes, and the customers get a smooth, native-feeling checkout experience.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Global Edge Network
          CDN[Edge Cache - Cloudflare/Fastly]
          WAF[Bot Protection & WAF]
      end

      subgraph OHC Cloud Core
          Gateway[API Gateway / Load Balancer]
          DropQueue[High-Speed Redis Queue]
          Postgres[(Primary Ledger DB)]
          CheckoutService[Rust Checkout Engine]
      end

      subgraph Mobile Device
          App[OHC Mobile App - Priya's Phone]
      end

      Customer[Customer on Mobile/Web] --> WAF
      WAF --> CDN
      CDN -- Cache Miss / Checkout --> Gateway
      Gateway --> CheckoutService
      CheckoutService --> DropQueue
      DropQueue -- Async Commit --> Postgres
      App -- Real-time Sales Dash --> Gateway
  ```

  ### UI Wireframes / Screen Flow (375px)
  1.  **Product Creation Screen:** Standard product details (Photos, Price, Variants).
  2.  **"Make it a Drop" Toggle:** User flips a switch.
  3.  **Drop Settings Card:**
      *   **Date & Time Picker:** iOS native scroll wheels for when it goes live.
      *   **Pre-Drop Teaser Toggle:** "Show product as 'Coming Soon' on storefront now?"
      *   **Waitlist Toggle:** "Collect emails/SMS for a reminder."
  4.  **Live Drop Dashboard:** A real-time, pulse-animated dashboard showing active viewers, inventory remaining, and revenue collected, updating instantly.

  ### Mobile UX Flow
  1.  Priya opens OHC App, taps "New Product".
  2.  She uploads photos of the new dress, sets price to $150, inventory to 50.
  3.  She taps "Enable Flash Drop" and sets the date for Friday, 5 PM.
  4.  She taps "Save".
  5.  *Invisible Magic:* The platform pre-warms the edge cache, schedules the Marketing AI to email the waitlist at 4:45 PM on Friday, and allocates Redis queue space.
  6.  On Friday at 5 PM, Priya watches the "Live Drop" dashboard on her phone as inventory ticks down to 0 in 3 minutes. No crashes. No oversells.

  ### AI Agent Integration Points
  *   **Marketing Agent:** Automatically detects the new Drop and drafts an email/SMS campaign for Priya to approve. Executes the campaign 15 minutes before the drop.
  *   **Operations Agent:** Monitors the live drop. If traffic exceeds normal thresholds, it invisibly autoscales the backend queue capacity. If the item sells out, it automatically updates the storefront to "Sold Out" and can trigger a follow-up email to the waitlist offering a pre-order for the next batch.

  ### Key Design Decisions
  *   **Decoupled Inventory Queue:** Inventory reservation during a drop happens in a high-speed Redis queue (or similar in-memory store), not by locking rows in the main Postgres database. This is critical to prevent database deadlocks and ensure 0% overselling under extreme concurrency.
  *   **Edge-First Static Assets:** The product page for the drop must be 100% statically generated and cached at the edge until the moment the timer hits zero.
  *   **No "Waiting Room" UI:** We optimize the backend to handle the load gracefully rather than punishing the user with a "You are in line" screen, preserving the premium feel.

  ## Implementation Prompt
  **Objective:** Implement the backend queueing mechanism and the mobile-first UI for the "Flash Drop" feature.

  **User-Facing Outcome:** A merchant can toggle "Enable Flash Drop" on any product, set a future date/time, and the system will automatically handle high-concurrency checkout traffic without overselling inventory or crashing the storefront.

  **CUJ (Critical User Journey):**
  1. User sets a product to drop at a specific future time.
  2. Storefront displays "Coming Soon" with a countdown timer.
  3. At the exact drop time, the "Add to Cart" button unlocks for all concurrent shoppers.
  4. System processes purchases instantly, strictly enforcing the inventory limit even if 10x more users try to buy at the exact same millisecond.

  **Acceptance Criteria:**
  *   Mobile UI includes the "Enable Flash Drop" toggle and date picker.
  *   Storefront strictly enforces the "locked" state until the drop time.
  *   Backend inventory decrement logic guarantees exactly-once processing and zero oversells under load testing (e.g., 500 concurrent requests for 50 items must result in exactly 50 sales).
  *   Integration with Marketing Agent to trigger pre-drop notifications.

  ## Priority
  P1

  ## Estimated Scope
  Large

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
