issue_title: "[Architecture] Hardware Connectivity Platform for POS"
issue_description: |
  # Architecture: Hardware Connectivity Platform for POS

  ## Problem Statement
  While Tap-to-Pay on iPhone/Android is critical for low-barrier entry (like Maya selling cakes at a market), higher-volume or stationary businesses like Priya's Boutique or Fatima's Food Cart require robust integration with dedicated POS hardware (e.g., Stripe Reader S700 or BBPOS WisePad 3). Currently, OHC handles online checkouts and has foundational code for Terminal tokens, but lacks a unified architecture to manage, pair, monitor, and route payments to dedicated physical hardware. Without this, OHC cannot serve omnichannel retail personas effectively.

  ## Research Report
  - **OHC Gaps:** We have foundational concepts for Stripe Terminal connection tokens (`src/server/integrations/stripe/terminal.rs`), but searching the codebase for `pos_hardware_devices` or `reader_id` yields zero results. There is no architecture for managing physical reader states, locations, or the pairing process.
  - **Competitor Systems:**
    - *Square:* Excels at hardware management. Pairing a reader is seamless, and the software clearly displays reader battery, connection status, and software update progress.
    - *Shopify POS:* Provides a hardware management section in their app, allowing users to select which reader to route a payment to, especially useful in multi-register setups.
  - **Opportunity:** Building a "Hardware Connectivity Platform" within OHC will allow users to easily pair and manage physical card readers. This moves OHC from a purely digital/mobile-only solution to a complete omnichannel retail system.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
    App[OHC Mobile App] -->|Discovers Readers| SDK[Stripe Terminal SDK]
    SDK -->|Bluetooth/Local Network| Hardware[Physical Card Reader]
    App -->|Requests Token| API[OHC Backend]
    API -->|Connects| Stripe[Stripe API]
    Stripe -->|Token| API
    API -->|Token| App

    App -->|Registers Reader| DeviceRegistry[(Device Registry DB)]
    DeviceRegistry -->|Monitors Health| OpsAgent[AI Operations Agent]
  ```

  ### Mobile UX Flow
  1. **Hardware Settings:** Priya navigates to Settings > Hardware in the OHC app.
  2. **Discovery:** She taps "Connect a Reader". The app uses the Stripe Terminal SDK to scan for nearby Bluetooth or local network readers.
  3. **Pairing:** She selects her "BBPOS WisePad 3". The app negotiates the connection and saves the reader ID to the OHC backend's device registry.
  4. **Checkout Routing:** During a sale, if a physical reader is connected and active, the payment intent is automatically routed to that hardware instead of prompting Tap-to-Pay on the phone itself.

  ### AI Agent Integration Points
  - **AI Operations Agent:** Monitors the health of registered readers. If a reader hasn't connected in days or its battery is consistently low, the agent proactively messages Priya: "Your main register card reader has low battery. Want me to order a replacement charger?"

  ### Key Design Decisions
  - **Device Registry:** Implement a `pos_hardware_devices` table to track paired readers, their last known status, battery level, and assigned location/tenant.
  - **Intelligent Routing:** The POS client logic must dynamically choose between Tap-to-Pay or a physical reader based on availability and user preference.

  ## Implementation Prompt
  As an implementer, build the backend foundation for hardware management.
  1. Create a `pos_hardware_devices` database migration to store reader metadata (id, tenant_id, stripe_reader_id, status, last_seen, battery_level).
  2. Implement API endpoints (`GET /api/v1/pos/hardware`, `POST /api/v1/pos/hardware/register`) for the mobile app to list and register discovered readers.
  3. Ensure these endpoints enforce strict multi-tenant isolation.
  4. Update the Payment Intent creation flow to optionally accept a `reader_id` to route the capture to specific hardware.

  **Priority:** P2
  **Estimated Scope:** Medium
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
