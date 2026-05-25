issue_title: "Implement Invisible Omnichannel Fulfillment and Pickup Engine"
issue_description: |
  # [architecture] Invisible Omnichannel Fulfillment and Pickup Engine

  ## Problem Statement
  Small business owners selling physical goods or prepared food (like Maya the baker, Priya the boutique owner, and Fatima the food cart operator) face massive friction when trying to get their products to their customers. They must juggle multiple complex fulfillment modes: shipping physical boxes (which requires calculating package weights, printing labels, and navigating carrier rates), managing local deliveries (routing, driver communication), and handling in-person order pickups (coordinating times, notifying customers when ready, confirming the right person got the right order). Current platforms force merchants to use separate, disjointed tools or complex settings for each of these modes, causing missed deliveries, cold food, frustrated customers, and significant wasted time for the business owner. They need an invisible, AI-driven fulfillment engine that seamlessly handles the logistics behind the scenes, allowing them to focus simply on making the product.

  ## Research Report
  *   **Shopify:** Offers strong shipping capabilities (Shopify Shipping) but it is highly complex for beginners. It requires setting up shipping profiles, weighing products, and configuring package sizes. Local delivery and pickup are add-on features that require manual configuration of radii and preparation times. It lacks autonomous coordination; the merchant still does the heavy lifting of logistics management.
  *   **Wix:** Provides basic shipping and pickup options, but like Shopify, relies on manual merchant configuration for rates, regions, and times. The UX is desktop-centric and not optimized for the fast-paced environment of a food cart or a busy boutique.
  *   **Squarespace / GoDaddy:** Offer rudimentary shipping integrations but lack the sophistication needed for hybrid fulfillment (e.g., a business that ships nationally but also offers local same-day delivery via Courier networks).
  *   **OneHumanCorp (OHC) Differentiation - "Invisible Autonomy":** OHC eliminates the logistics configuration burden. The Fulfillment Engine automatically determines the optimal fulfillment method (Shipping, Local Delivery via 3rd party like Uber Direct/DoorDash Drive, or Local Pickup) based on customer location and product type. AI agents handle the printing of labels, the dispatch of local couriers, and the automated SMS coordination with customers for pickups.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      ORDER_EVENT ||--o{ FULFILLMENT_ROUTER : "Triggers"

      FULFILLMENT_ROUTER {
          string order_id
          string fulfillment_mode "Shipping | LocalDelivery | Pickup"
          string tenant_id "Multi-tenant isolation"
      }

      FULFILLMENT_ROUTER ||--o{ OPS_AGENT : "Delegates to"

      OPS_AGENT ||--o{ SHIPPING_CARRIER_API : "Generates Labels (USPS/UPS)"
      OPS_AGENT ||--o{ LOCAL_COURIER_API : "Dispatches (Uber/DoorDash)"
      OPS_AGENT ||--o{ CS_AGENT : "Coordinates Customer SMS"

      OPS_AGENT }|--|| FULFILLMENT_STATE : "Updates"

      FULFILLMENT_STATE {
          string status "Preparing | ReadyForPickup | Shipped | Delivered"
          string tracking_url
          string courier_info
      }

      FULFILLMENT_STATE ||--o{ MOBILE_UI : "Syncs to Dashboard"
  ```

  ### UI Wireframes & 375px Baseline
  **Core Layout: macOS-style Translucent Glass + Ubiquiti UniFi Modular Dashboard Cards**
  *   **Global Viewport:** 375px width (Mobile First). No horizontal scrolling.
  *   **App Bar:** Blurred glass top nav with the business logo.
  *   **Fulfillment Queue (The Hub):**
      *   A clean, vertically scrolling list of active orders categorized by their next required action, not just a chronological list.
      *   **"To Pack" Section:** Cards for items needing physical packing.
          *   Card shows item image, quantity, and a single bold button: `[Print Label]` (for shipping) or `[Mark Ready]` (for pickup).
          *   When `[Print Label]` is tapped, the AI has already generated the cheapest valid USPS/UPS label in the background; it immediately beams to the connected thermal printer.
      *   **"Awaiting Pickup" Section:** Cards for local customers or couriers en route.
          *   Displays customer/courier name and a live ETA pill (e.g., `ETA: 5 mins`).
          *   Card has a swipe-to-complete action: `[Swipe to Hand Off]`.
  *   **Order Detail View:**
      *   Frosted glass background showing full order details.
      *   "AI Logistics Summary" at the top: e.g., "✨ Shipping via USPS Ground. Label printed. Customer notified."

  ### Mobile UX Flow
  1. **Notification:** Priya receives a push notification: "🛍️ New Order: 2 Summer Dresses. Shipping to NY. Label is ready."
  2. **Action:** She taps the notification, opening the OHC Fulfillment Hub.
  3. **Execution (Shipping):** She sees the order card. She taps `[Print Label]`. The pre-calculated, AI-optimized shipping label prints instantly from her local wireless thermal printer. She packs the box and slaps the label on.
  4. **Execution (Pickup/Delivery):** For a local food order (Fatima), the card says "Local Delivery via DoorDash". She taps `[Mark Ready]`. The AI automatically dispatches the driver and sends an SMS to the customer: "Your food is on the way!"
  5. **Completion:** When the driver arrives or the customer picks up, a quick `[Swipe to Hand Off]` marks the order entirely complete.

  ### AI Agent Integration Points
  *   **Operations (Ops) Department:** The core brain here. It analyzes the order, the product weights (learned over time if not explicitly entered), and the customer address to instantly select the best shipping rate or dispatch a local courier API (like Uber Direct) without merchant intervention.
  *   **Customer Service (CS) Department:** Handles the "last mile" communication. It sends dynamic SMS updates to the customer (e.g., "Your cake is ready for pickup! We close at 5 PM today.") and handles replies like "I'm running 10 mins late!" by alerting the merchant.
  *   **Finance Department:** Ensures any dynamic shipping or courier costs are accurately reconciled in the merchant's ledger and that appropriate taxes were collected on shipping.

  ### Key Design Decisions (Why, not How)
  *   **Action-Oriented UI:** The merchant doesn't need to see a complex grid of all orders. They only need to know *what to do right now*. The UI groups tasks by action (Pack, Hand Off) rather than status.
  *   **Pre-computed Logistics:** The merchant should never have to manually compare USPS vs UPS rates on their phone. The Ops Agent must compute this invisibly at the moment of checkout and present the single best option.
  *   **Zero-Trust Isolation:** Order data contains highly sensitive PII (addresses, phone numbers). The `FULFILLMENT_ROUTER` must rigorously enforce multi-tenant boundaries using SPIFFE identities.

  ## Implementation Prompt
  **To the Implementer Swarm:**
  Your goal is to build the backend architecture and mobile-first UI for the "Invisible Omnichannel Fulfillment Engine," enabling a merchant to manage shipping labels, local courier dispatch, and in-person pickups from a single streamlined view.

  **Customer User Journey (CUJ):**
  1. A customer places an order requiring physical fulfillment (either shipping, local delivery, or pickup).
  2. The OHC Ops Agent automatically determines the fulfillment mode and pre-computes the necessary logistics (e.g., generating a shipping label or scheduling a local courier).
  3. The merchant opens the mobile app and sees the order in an action-oriented queue ("To Pack" or "Awaiting Pickup").
  4. With a single tap or swipe, the merchant executes the logistics (printing the label or handing off the item), and the AI automatically updates the customer.

  **Acceptance Criteria:**
  *   **Mobile Parity:** The Fulfillment Hub UI must be implemented perfectly for a 375px viewport, utilizing the Translucent Glass design system and action-oriented cards.
  *   **Intelligent Routing:** The backend system must process an incoming mocked order event and automatically assign it the correct fulfillment state and required actions without manual merchant configuration.
  *   **Agent Coordination:** Demonstrate the Ops Agent triggering a mocked label generation or courier dispatch, and the CS Agent generating the corresponding customer notification.
  *   **Isolation Guarantee:** Implement strict multi-tenant boundary checks so a merchant can only access fulfillment data for their own `organization_id`.
  *   **Simplicity:** The merchant UI must hide all complex shipping settings (box sizes, rate comparison tables). These should be handled autonomously by the backend.

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
