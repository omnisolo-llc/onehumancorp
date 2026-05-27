issue_title: "Implement Autonomous Yield Management & Dynamic Pricing Engine"
issue_description: |
  # Autonomous Yield Management & Dynamic Pricing Engine

  ## Problem Statement
  Small business owners leave significant revenue on the table because they lack the time, data, and tools to optimize pricing dynamically. Fatima throws away unsold food at the end of the day because she can't manually blast out a "happy hour" discount. Carlos is overbooked on weekends but underbooked on Tuesdays, yet charges the same flat rate. Maya manually calculates "rush fees" for last-minute cake orders, often undercharging and burning out. Leo has unbooked afternoon music lesson slots.

  They need enterprise-grade yield management (like airlines, hotels, or Uber) completely invisibilized and managed by AI, requiring zero manual configuration.

  ## Research Report
  **Market Landscape & Competitive Analysis**
  - **Shopify / Wix / Squarespace:** Offer basic, static discount codes or manual sale prices. Any dynamic pricing requires expensive, complex third-party apps (e.g., Bold Custom Pricing, Prisync) that require rule-setting, coding, and constant monitoring.
  - **Mindbody / Acuity:** Have rudimentary "off-peak" pricing, but it must be manually configured per time block and isn't integrated with autonomous marketing outreach.
  - **Uber Eats / DoorDash:** Use algorithmic pricing to balance supply and demand, but charge exorbitant fees to the merchant and don't share the data.

  **The OHC Opportunity**
  By seamlessly connecting our Universal Capacity & Inventory Ledger with our AI Marketing and Sales agents, we can introduce automated "Inventory Clearing" for perishables, "Surge Pricing" for high-demand services, and automated "Rush-Fee" calculations for custom orders. This can boost SMB revenue by 15-25% purely through intelligent optimization, functioning entirely in the background.

  ## Design Doc

  ### Architecture Diagram

  ```mermaid
  graph TD
      A[Universal Capacity & Inventory Ledger] -->|Velocity & Stock Events| B(Autonomous Yield Manager)
      C[Calendar & Booking Engine] -->|Utilization Rates| B
      B -->|Calculates Optimal Price| D{AI Department Coordination}

      D -->|Finance Agent| E[Update Checkout Ledger & Storefront Price]
      D -->|Marketing Agent| F[Generate & Send SMS/Email/IG Promo]
      D -->|Sales Agent| G[Negotiate via Chat Inbox with New Price]

      E --> H[Mobile Storefront UI - 375px]
      F --> I[Customer Phone]
      G --> I
  ```

  ### AI Agent Integration Points
  1. **Operations AI:** Continuously monitors inventory depletion rates and calendar booking velocity. Identifies anomalies (e.g., 80% of falafel remaining at 3 PM, or 5 weekend handyman slots requested within an hour).
  2. **Finance AI:** Calculates the floor price to ensure the business never loses money on a transaction, even when heavily discounting. Evaluates historical margin data.
  3. **Marketing AI:** When prices drop for clearance, automatically drafts and sends targeted SMS/WhatsApp messages to local, high-intent past customers ("Hey! Fatima's cart has $5 Falafel bowls for the next hour only!").
  4. **Sales/CS AI:** When responding to Instagram DMs, the agent can quote dynamic prices ("Carlos is fully booked Saturday, but I can get you in for a $50 weekend surge fee, or standard rate on Tuesday. Which works?").

  ### Mobile UX Flow (375px First)
  *Zero-config, Grandmother Test Approved.*

  **1. Setup Flow (Item/Service Creation):**
  - Instead of complex pricing matrices, a single elegant toggle card using macOS-style Translucent Glass materials.
  - **Toggle:** "Enable Smart Pricing"
  - **Subtitle:** "Let AI automatically discount unsold items at the end of the day, or charge more during rush hours to maximize your profit."
  - **Advanced (Hidden):** Minimum price floor (defaulted to cost + 10%).

  **2. Owner Notification Feed:**
  - Clean, actionable push notifications:
    - *"Fatima, you had 20 meals left. AI dropped the price by 30% and texted 40 nearby regulars. 15 just sold!"*
    - *"Carlos, 4 people asked for Saturday repairs today. AI automatically added a 20% weekend surge fee to the latest booking."*

  **3. Customer Facing Storefront:**
  - Smooth animations showing a "Flash Sale" or "Dynamic Rate" with urgency indicators (e.g., "Only 3 spots left at this price").

  ### Security & Zero Trust
  - All pricing changes must be cryptographically signed by the Finance Agent and logged in the immutable Ledger to prevent race conditions during checkout.
  - SPIFFE/SPIRE identities strictly enforce that only the designated AI Yield Manager for a specific tenant can alter their pricing models.

  ## Implementation Prompt
  **To the Implementer Agent:**
  Implement the Yield Management worker and data structures.
  - **Goal:** Enable dynamic pricing without user intervention. Create a background service that listens to the `Inventory.Depletion` and `Calendar.Booking` event streams via NATS.
  - **Requirements:**
    - Define `YieldStrategy` schemas (e.g., Perishable Clearance, Service Surge, Custom Order Rush).
    - Provide a safe, bounded mechanism for the AI to adjust the `current_price` of a SKU or Service in real-time.
    - Implement the "Smart Pricing" toggle in the React Native mobile app using our UniFi-inspired design system.
    - Ensure latency targets are met: pricing changes must propagate to the edge (Storefront UI) in < 200ms.
  - **Priority:** P1
  - **Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
