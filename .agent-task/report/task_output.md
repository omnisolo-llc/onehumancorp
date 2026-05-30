issue_title: "[Architecture] Autonomous Smart Waitlist & Capacity Recovery Engine"
issue_description: |
  **Problem Statement:**
  Small business owners frequently lose revenue due to last-minute cancellations or sold-out physical items. They lack the time to manually contact interested customers to fill newly opened slots or restocked items.

  **Research Report:**
  Existing platforms treat capacity as binary and require manual intervention or generic email blasts for waitlists. OHC has an opportunity to build an autonomous "concierge" agent that automatically converts "Buy" buttons to "Join Waitlist" when capacity is full, and executes a waterfall SMS/WhatsApp notification sequence with 1-click checkout links when capacity opens up. This maximizes yield with zero effort from the owner.

  **Design Doc:**
  ### Architecture Diagram
  ```mermaid
  erDiagram
      MERCHANT ||--o{ CAPACITY_RESOURCE : owns
      CAPACITY_RESOURCE {
          string type "Inventory | TimeSlot"
          int available
      }
      CAPACITY_RESOURCE ||--o{ WAITLIST_ENTRY : triggers
      WAITLIST_ENTRY {
          string customer_id
          datetime joined_at
          string status "Pending | Notified | Claimed | Expired"
      }
      WAITLIST_ENTRY }o--|| CUSTOMER : belongs_to

      OPERATIONS_AI_AGENT {
          string role "Capacity Recovery"
      }
      OPERATIONS_AI_AGENT ||--o{ CAPACITY_RESOURCE : monitors
      OPERATIONS_AI_AGENT ||--o{ WAITLIST_ENTRY : executes_waterfall_notification
  ```
  ### UI Wireframes & Mobile UX Flow (375px)
  - **Customer View (Sold Out):** A beautiful glassmorphism card where the standard "Buy" button is replaced with "Join Waitlist". Tapping it asks for a phone number via native iOS/Android keyboard.
  - **Customer View (Notification):** Customer receives an SMS: "A spot opened up! Claim your custom cake order here: [Link]". The link opens a 1-tap Apple Pay/Google Pay checkout.
  - **Merchant View (Dashboard):** A silent background process. Maya sees a small notification: "A 3 PM slot was canceled, but the Waitlist Agent already filled it and collected the $50 deposit."
  ### Key Design Decisions
  1. **Zero-Config Waitlists:** When inventory hits 0 or a calendar slot is filled, the platform automatically transitions the UI to "Waitlist Mode" without the owner flipping a switch.
  2. **Autonomous Waterfall Recovery:** When capacity returns, the Operations Agent acts like a concierge, reaching out to the waitlist chronologically (or based on VIP status) with a time-bound checkout link.
  3. **Conversational SMS/WhatsApp Offers:** Instead of easily-ignored emails, the agent texts the customer: "Hi! A 3 PM slot opened up for Leo's Guitar lesson today. Reply YES to grab it."
  4. **Deposit Capture Integration:** The engine can optionally capture a pre-authorized deposit to join the waitlist, ensuring high intent and instantly capturing revenue upon capacity opening.

  **Implementation Prompt:**
  Implement the Autonomous Smart Waitlist & Capacity Recovery Engine. Introduce a `WaitlistEntry` model tied to the universal capacity ledger. Enhance the Operations AI Agent to subscribe to capacity increase events (cancellations or restocks). When triggered, the agent must execute a waterfall notification system via SMS/email to waitlisted customers, providing secure, time-expiring, 1-click checkout tokens. Ensure the merchant UI (375px) displays waitlist depth and successful autonomous recoveries without requiring manual intervention.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
