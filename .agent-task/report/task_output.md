issue_title: "[architecture] Universal Autonomous Staff Management Mesh"
issue_description: |
  # Universal Autonomous Staff & Shift Management Mesh Research Report

  ## 1. Problem Statement
  As our small business owners scale from single-person operations to hiring their first employees, they immediately hit a wall of administrative complexity. The objective of this research is to architect a zero-friction, offline-capable staff mesh where adding an employee is as simple as texting them a link, and the AI handles shift reminders, permissions, and tip splitting invisibly.

  ## 2. Research Report
  *   **Shopify/Square Complexity:** Shopify and Square require complex manual setup, app downloads for staff, and manual tip calculations.
  *   **Target Opportunity:** OHC must abstract RBAC (Role-Based Access Control) into simple English (e.g., "Cashier") and focus on zero-app-install SMS onboarding, invisible AI tip splitting, and offline-first clock-ins.
  *   **Persona Pain Points:** Maya needs staff to see orders but not revenue. Carlos needs to dispatch his apprentice. Fatima needs her staff to ring up orders using a fast, offline 4-digit PIN on a shared POS terminal.

  ## 3. Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ STAFF_MEMBER : employs
      STAFF_MEMBER ||--o{ SECURE_PIN : authenticates_locally
      TENANT ||--o{ ROLE_TEMPLATE : defines
      STAFF_MEMBER }|--|| ROLE_TEMPLATE : assigned
      STAFF_MEMBER ||--o{ TIMECARD_EVENT : logs
      TERMINAL ||--o{ TIMECARD_EVENT : queues_offline
      TIMECARD_EVENT }|--|| AI_PAYROLL_AGENT : processed_by
      AI_PAYROLL_AGENT ||--o{ TIP_LEDGER : allocates
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)

  **Screen 1: Manager Team View**
  - **Top:** Clean translucent glass header: "Your Team".
  - **Middle:** Large cards for each staff member showing their current status (e.g., 🟢 "Sarah - Clocked In (2h 15m)").
  - **FAB (Floating Action Button):** Large "+" button. Tapping it opens a half-sheet: "Who are you hiring?" with a simple phone number input and a role selector (Cashier, Manager, Driver). No complex permission checkboxes.

  **Screen 2: Staff PIN Entry (Terminal View)**
  - **Full Screen:** A massive, high-contrast numpad.
  - **Top:** "Enter your PIN to unlock".
  - **UX:** Fast, snappy, works instantly even if the device is in airplane mode. Upon correct PIN, the UI physically "unlocks" with a smooth motion transition to the Point of Sale screen, but the "Reports" and "Settings" tabs are entirely hidden based on the locally cached role.

  **Screen 3: Staff Personal Hub (Via Web Link, No App Needed)**
  - Accessed via a magic link sent via SMS.
  - Shows their upcoming schedule, total hours worked this week, and estimated tips earned.
  - Big "Request Time Off" button.

  ### AI Agent Integration Points
  - **AI HR Agent:** Handles the conversational onboarding ("Add Sarah as a cashier"). Monitors shift anomalies.
  - **AI Payroll/Finance Agent:** Automatically ingests the `TIMECARD_EVENT` ledger, combines it with the `TIP_LEDGER` from transactions during that shift, and calculates precise tip splits (e.g., proportionally by hours worked) without the owner doing any math.

  ### Key Design Decisions
  1. **No App Required for Staff:** Staff manage their shifts and view earnings via an SMS magic link to a Progressive Web App (PWA). This eliminates onboarding friction for high-turnover roles.
  2. **Offline-First PIN Authentication:** Staff PINs and basic role configurations are synced to edge terminals. A device can be entirely offline and still allow staff to clock in, ring up orders, and clock out.
  3. **Implicit Over Explicit Permissions:** Instead of showing business owners a matrix of 50 checkboxes, we use plain-language roles (Cashier). The AI maps these to granular technical permissions behind the scenes.

  ## 4. Implementation Prompt
  Implement the foundational Staff Mesh and Offline-First Authentication module for the OHC POS terminal.

  1. The business owner navigates to the Team screen and adds a new staff member by providing only a name, phone number, and a predefined role ("Cashier").
  2. The system generates a secure PIN setup link and sends it to the staff member.
  3. Once the PIN is set, the POS terminal securely caches the hashed PIN and role mapping locally.
  4. The staff member enters their PIN on the POS terminal. The terminal unlocks, restricting UI elements (e.g., hiding financial reports) based on the "Cashier" role, and allows them to clock in/out locally, even if disconnected from the internet.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
