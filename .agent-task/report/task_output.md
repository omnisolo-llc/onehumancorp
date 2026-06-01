issue_title: "[Architecture] Autonomous AI Event Ticketing & Capacity Management Engine"
issue_description: |
  # Problem Statement
  For small business owners, transitioning from selling static products to hosting live events (e.g., Maya the baker hosting a "Cupcake Decorating Class", Leo the music tutor hosting a "Group Guitar Workshop", or Priya hosting an exclusive "Boutique Fashion Show") introduces immense friction. Currently, they must leave their primary commerce platform and rely on third-party solutions like Eventbrite or Ticketmaster. These external platforms charge exorbitant fees, fragment the customer data, and fail to integrate seamlessly with the owner's existing CRM, marketing tools, or accounting ledgers. The business owner needs an invisible, zero-config engine that allows them to instantly spawn a ticketed event, manage capacity limits, handle QR code generation/check-ins, and seamlessly process refunds or waitlists—all directly from a 375px mobile device.

  # Research Report
  - **Eventbrite:** Industry standard for ticketing, but charges high per-ticket fees and completely isolates the customer relationship from the business's core CRM. Setting up an event requires navigating a complex desktop dashboard.
  - **Shopify:** Ticketing requires clunky third-party apps (e.g., Event Ticketing by Guest Manager). These apps often have brittle sync mechanisms and poor mobile management experiences.
  - **Wix/Squarespace:** Offer basic event modules, but they lack advanced agentic features like autonomous waitlist management or real-time dynamic pricing for last-minute unfilled seats.
  - **OHC Advantage:** OHC can offer an integrated "Ticketing Mesh." The owner just tells their AI Operations Agent: "I'm hosting a baking class this Friday at 7 PM for 10 people, $50 a ticket." The AI instantly generates the event page, provisions the inventory (10 slots), creates unique QR codes for each buyer, and sets up a mobile check-in scanner within the OHC app.

  # Design Doc
  - **Architecture Diagram:**
    ```mermaid
    erDiagram
      TENANT ||--o{ EVENT : "hosts"
      EVENT ||--o{ TICKET_INVENTORY : "has capacity"
      TICKET_INVENTORY ||--o{ TICKET : "issues"
      BUYER ||--o{ TICKET : "purchases"
      TICKET ||--o{ CHECK_IN_EVENT : "validates"
      AI_OPERATIONS_AGENT ||--o{ WAITLIST : "manages"
    ```
  - **Mobile UX Flow (375px First):**
    - **Creation:** A unified "New Event" card where the owner inputs basic details (Title, Date, Capacity, Price). No complex menus.
    - **Purchasing:** Buyers experience a seamless 1-tap checkout via the Universal Buyer Identity. They receive their ticket (with QR code) directly via SMS/WhatsApp or Apple/Google Wallet.
    - **Check-In Mode:** On the day of the event, the owner's OHC app transforms into a high-contrast, offline-capable "Scanner Mode" utilizing the phone's camera to instantly validate QR codes.
  - **AI Coordination:**
    - The **Operations Agent** monitors capacity. If the event sells out, it automatically opens a Waitlist. If someone cancels, the agent autonomously emails the next person on the waitlist with a 1-hour exclusive payment link.
    - The **Marketing Agent** automatically creates promotional social media posts for the event and sends reminder SMS/WhatsApp messages to attendees 24 hours prior.

  # Implementation Prompt
  Implement the Autonomous AI Event Ticketing & Capacity Management Engine. Design the core data models (`EVENT`, `TICKET_INVENTORY`, `TICKET`, `CHECK_IN_EVENT`) ensuring strict `tenant_id` isolation. Develop the backend API endpoints to support ticket issuance, secure QR code generation, and offline-capable check-in validation. Integrate the AI Operations Agent to autonomously manage capacity constraints, cancellations, and waitlist fulfillment. Ensure the checkout and check-in flows are optimized for a 375px mobile viewport and adhere to the OHC Premium Token visual standards.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
