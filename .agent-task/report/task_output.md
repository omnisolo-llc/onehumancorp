issue_title: "[Architecture] Multilingual Realtime Order Fulfillment Mesh"
issue_description: |
  # Title: [Architecture] Multilingual Realtime Order Fulfillment Mesh

  ## Problem Statement
  For food cart operators, market vendors, and pop-up retailers like **Fatima** (50, food cart operator, limited English), existing e-commerce systems are practically useless in the field. They require expensive, bulky POS hardware, rely on stable high-speed internet, and assume the merchant operates exclusively in English. During a lunch rush, Fatima needs to instantly toggle a dish as "sold out," receive immediate audio or visual notifications for new pre-orders, and manage a queue seamlessly, all from a low-end Android device. The platform needs to bridge the gap between English-speaking customers placing orders online and non-English-speaking merchants fulfilling them in real-time.

  ## Research Report
  - **Competitor Systems Audit**:
    - **Square / Toast**: Designed for restaurants with dedicated, expensive KDS (Kitchen Display System) hardware. They are complex to set up, require a steady internet connection, and often lack deep, on-the-fly bidirectional translation for non-English speakers.
    - **Shopify**: Great for shipping physical products, but its real-time local pickup and local delivery notifications are not designed for the rapid-fire pace of a food cart lunch rush.
    - **WhatsApp Business**: Heavily used in developing markets, but it's unstructured. It's just chat. There's no menu sync, inventory countdown, or structured order status workflow.
  - **OHC Advantage**: We can provide a "KDS in your pocket." By leveraging our offline-first architecture, device-native notifications, and AI translation agents, OHC can instantly convert incoming English orders into Arabic (or any other language) on the merchant's screen, and translate the merchant's "ready for pickup" tap back into an English SMS for the customer.

  ## Design Doc

  ### 1. Architecture Diagram
  ```mermaid
  erDiagram
      CUSTOMER ||--o{ ORDER : places
      ORDER {
          string id
          string status
          json items
          datetime expected_pickup
      }
      ORDER ||--o{ NOTIFICATION : triggers
      NOTIFICATION {
          string type
          string delivery_method
          string payload
      }
      AI_TRANSLATOR_AGENT ||--o{ ORDER : translates
      AI_TRANSLATOR_AGENT {
          string source_lang
          string target_lang
      }
      MERCHANT_DEVICE ||--o{ ORDER : manages
      MERCHANT_DEVICE {
          string device_id
          boolean offline_mode
          string preferred_language
      }
  ```

  ### 2. UI Wireframes & Screen Flow (375px Mobile-First)
  - **Active Orders Tab**:
    - Large, high-contrast cards.
    - Top card is the next order due.
    - Each card shows: Order Number (huge font), Time remaining until pickup, and a list of items translated into the merchant's language (e.g., Arabic).
    - One massive, thumb-friendly button per card: "MARK READY" (in the merchant's language).
  - **Menu/Inventory Tab**:
    - Grid of menu items with photos.
    - Next to each photo is a large toggle switch for "Available" / "Sold Out". Tapping it instantly updates the customer-facing storefront via the realtime sync mesh.

  ### 3. Mobile UX Flow
  1. **New Order Arrives**: Customer places an order on the web storefront (in English).
  2. **Alert**: Fatima's Android phone plays a distinct chime, even if the screen is locked. A high-priority push notification appears (translated to Arabic).
  3. **Fulfillment**: Fatima taps the notification, opening the OHC app to the Active Orders tab. She prepares the food.
  4. **Completion**: She taps the massive "MARK READY" button.
  5. **Customer Notification**: The AI Operations Agent instantly texts the customer (in English): "Your order from Fatima's Cart is ready for pickup!"

  ### 4. AI Agent Integration Points
  - **Translation Agent**: Automatically translates menu items created by the merchant into the storefront's display language, and translates incoming customer order notes into the merchant's preferred language.
  - **Operations Agent**: Monitors the queue. If an order sits in "Accepted" for too long past the expected pickup time, it gently prompts the merchant or sends a "running slightly behind" update to the customer, based on predefined heuristics.

  ### 5. Key Design Decisions
  - **Optimized for Low-End Devices**: The frontend must use hyper-efficient rendering, avoiding heavy JavaScript frameworks where possible for the merchant view, leaning on native device capabilities and SQLite for local offline storage.
  - **Audio-First Alerts**: In a noisy environment (street cart), visual notifications aren't enough. Critical alerts must use distinct, loud audio cues.
  - **Bi-Directional Real-Time Sync**: Uses WebSockets (or similar tech like NATS over WebSockets) for instant UI updates when a customer orders or when the merchant marks an item sold out.
  - **Granular Multi-Tenant Isolation**: Orders for Fatima's cart are strictly isolated cryptographically from orders for Maya's bakery, ensuring no cross-talk even in the realtime event mesh.

  ## Implementation Prompt
  Design and implement the real-time order fulfillment mesh backend and the merchant-facing mobile web UI.
  1. Create the backend services to handle incoming orders, store them securely in the multi-tenant database, and broadcast them via WebSockets to the merchant's device.
  2. Integrate the AI Translation Agent to automatically translate order items and customer notes based on the merchant's configured language preferences.
  3. Build the 375px mobile-first UI with the "Active Orders" view (large cards, distinct "Mark Ready" button) and the "Menu" view (quick "Sold Out" toggles).
  4. Ensure the UI handles network drops gracefully (offline mode) and syncs state once reconnected.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
