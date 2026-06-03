issue_title: "[Architecture] Invisible Mobile Wallet Pass Engine (Apple/Google Wallet)"
issue_description: |
  ## Problem Statement
  Small business owners like Leo (music tutor) and Carlos (handyman) rely on customer appointments and bookings, but frequently deal with no-shows. Maya (baker) offers loyalty punch cards, but physical cards get lost, and users forget they have them. Customers want a seamless, digital way to store their bookings, receipts, and loyalty cards directly on their phones. OHC needs an invisible engine that automatically issues Apple Wallet and Google Wallet passes for every booking, digital ticket, and loyalty program, keeping the business front-and-center on the customer's device.

  ## Research Report
  - **Codebase & Competitor Audit**: Shopify has some third-party apps for Apple Wallet loyalty, but they are expensive and complex. Wix Bookings sends emails, but no native Apple Wallet integration. OHC's current booking and loyalty architecture lacks a unified capability to generate cryptographically signed `.pkpass` files or Google Wallet objects.
  - **The Gap**: We need an architecture that seamlessly hooks into the OHC event mesh (booking confirmed, loyalty point earned) and automatically issues a digital pass. This pass must be dynamically updated (e.g., if a booking time changes, the pass updates instantly via APNs).
  - **Data & Market Validation**: Studies show that Apple Wallet / Google Wallet passes have a 90% retention rate on devices, far higher than dedicated apps. Passes also support location-based lock screen notifications (e.g., "Your appointment with Carlos is in 1 hour" appearing when the customer is near the location).

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Customer
      participant OHC_Core_API
      participant Event_Mesh
      participant Wallet_Agent
      participant APNs (Apple)
      participant DB (PostgreSQL)

      Customer->>OHC_Core_API: Books Appointment (Leo's Music)
      OHC_Core_API->>Event_Mesh: Emit `booking.confirmed`
      Event_Mesh->>Wallet_Agent: Trigger Pass Generation
      Wallet_Agent->>DB: Fetch Tenant Config (Apple certs/colors)
      Wallet_Agent->>Wallet_Agent: Generate .pkpass (Signed)
      Wallet_Agent->>OHC_Core_API: Store Pass URL
      OHC_Core_API-->>Customer: Return Booking Page with "Add to Apple Wallet"

      Note over Customer, APNs: When Booking Updates
      OHC_Core_API->>Event_Mesh: Emit `booking.updated`
      Event_Mesh->>Wallet_Agent: Process Update
      Wallet_Agent->>APNs: Send Push Notification (Update Pass)
      APNs->>Customer: Pass Updates silently & shows notification
  ```

  ### Business Journey Mapping
  - **Customer Booking**: Customer books a guitar lesson with Leo. On the confirmation screen (and email), there is a single "Add to Apple Wallet" button.
  - **Retention & Reminder**: The pass sits in their Apple Wallet. 2 hours before the lesson, a lock-screen notification automatically reminds them.
  - **Loyalty Integration**: Maya's customer buys a 5th cake. The loyalty pass in their wallet automatically updates to show "5/10 punches" and sends a notification: "You're halfway to a free cake!"

  ### Data Model & Invariants
  - **Pass Entity**: `WalletPass { id, tenant_id, pass_type, template_id, status }`
  - **Multi-Tenant Isolation**: Apple Developer certificates for `.pkpass` signing can be unified under OHC's umbrella cert, with pass identifiers (e.g., `pass.store.ohc.maya-cakes`) dynamically generated. The DB must strictly isolate pass records by `tenant_id`.

  ### AI Department Coordination
  - **Marketing Agent**: Suggests the owner turn on Apple Wallet loyalty cards. Automatically designs the pass using the tenant's brand colors (Glassmorphism design tokens) and logo.
  - **Operations Agent**: Triggers pass updates if a booking is delayed or cancelled, automatically notifying all affected customers without the owner lifting a finger.

  ### Mobile-First & Security Integrity
  - **Performance**: Generating a `.pkpass` involves cryptographic signing. This must happen asynchronously via the background job queue to avoid blocking the main booking request thread.
  - **Security**: Endpoint serving the passes (`/api/v1/passes/:id`) must prevent enumeration via UUIDv4 and ensure Apple's strict TLS requirements are met.

  ## Implementation Prompt
  **To Implementer Agent:**
  Design and implement the Mobile Wallet Pass Engine within the Rust backend.
  1. Create a background worker that listens for `booking.confirmed` and `loyalty.earned` events to generate Apple Wallet (`.pkpass`) and Google Wallet objects.
  2. Implement the cryptographic signing logic for `.pkpass` files using OHC's master certificates.
  3. Create API endpoints for device registration and pass updates (to support Apple APNs push notifications for dynamic passes).
  4. Ensure all wallet passes adhere strictly to multi-tenant boundaries and store their state in PostgreSQL.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
