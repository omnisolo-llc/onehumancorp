issue_title: "Offline-First Ticketing & Event Management Engine"
issue_description: |
  ## Problem Statement
  For small business owners like Leo (the music tutor organizing a recital) or Fatima (the food cart operator hosting a pop-up tasting event), organizing in-person events is chaotic. They lack a unified system to sell tickets online, scan them at the door reliably (especially when Wi-Fi/cellular is spotty at venues), and manage attendee capacity. Existing solutions are either too complex (Eventbrite requires managing another platform) or fail when offline. They need a simple, integrated ticketing solution that works flawlessly offline on any mobile device.

  ## Research Report
  Current event management tools often fail the "grandmother test" and struggle with offline scenarios:
  - **Eventbrite**: Industry standard, but charges high fees, requires users to leave the OHC ecosystem, and is overly complex for a simple 50-person local event.
  - **Shopify POS**: Excellent for physical products but lacks native event ticketing and robust offline QR code scanning features.
  - **Wix Events**: Good integration but offline check-in can be unreliable without a constant network connection.
  OHC needs a native ticketing engine that generates secure QR codes, caches attendee lists locally on the merchant's device, and syncs asynchronously when a connection is restored.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Customer
      participant Storefront as OHC Web Storefront
      participant Backend as OHC Backend / Database
      participant MerchantApp as OHC Mobile App (Offline/Online)
      participant Agent as OHC Operations Agent

      Customer->>Storefront: Purchases Event Ticket
      Storefront->>Backend: Process Payment & Generate Ticket QR
      Backend->>Storefront: Return Ticket & Email Receipt
      Backend-->>MerchantApp: Background Sync: Update Attendee List (when online)

      Note over MerchantApp, Customer: Day of Event (Network Offline)
      Customer->>MerchantApp: Presents QR Code at Door
      MerchantApp->>MerchantApp: Scan QR & Verify locally against cached list
      MerchantApp->>MerchantApp: Mark Attendee as "Checked In" (Local State)

      Note over MerchantApp, Backend: Network Restored
      MerchantApp-->>Backend: Asynchronous Sync: Push Check-in Data
      Backend->>Agent: Trigger: Event Follow-up
      Agent->>Customer: Send "Thank you for attending" Email
  ```

  ### Key Design Decisions
  - **Offline-First Check-in**: The mobile app will sync the attendee list prior to the event. QR scans validate against the local cache, ensuring zero latency at the door even in basements or rural areas.
  - **Cryptographic QR Codes**: Tickets use signed JWTs encoded into QR codes to prevent forgery without needing an online lookup.
  - **Asynchronous Sync**: Check-in statuses are stored locally and synced back to the server using a robust queue when connectivity resumes.
  - **Mobile UX**: The scanning interface will be a full-screen camera view with giant, high-contrast visual and audio feedback (Green/Red) optimized for fast processing.
  - **Agent Integration**: Post-event, the Customer Success agent automatically requests reviews or sends follow-up offers based on synced attendance data.

  ## Implementation Prompt
  **Objective:** Implement the backend and mobile data sync architecture for the Offline-First Ticketing & Event Management Engine.
  **Acceptance Criteria:**
  1. Create data models for Events, Tickets, and Attendee Check-ins with multi-tenant isolation.
  2. Develop the API endpoints for ticket generation (cryptographically signed) and offline check-in synchronization (handling conflict resolution).
  3. Design the mobile sync mechanism (using local storage/SQLite) that pre-fetches the attendee list and pushes check-in statuses when online.
  4. Integrate with the Operations Agent to trigger a post-event workflow (e.g., sending a thank-you note).
  5. Provide a simple end-to-end test simulating a ticket purchase, an offline scan, and a subsequent network sync.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
