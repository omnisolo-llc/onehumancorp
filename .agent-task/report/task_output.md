issue_title: "[architecture] Universal Autonomous Staff Management Mesh"
issue_description: |
  # Research Report: Universal Autonomous Staff Management Mesh

  ## Problem Statement
  As small business owners grow from solo operations to hiring their first employees, they immediately hit a wall of administrative complexity. Existing solutions require complex manual setup, app downloads for staff, and manual tip calculations. OHC needs a zero-friction, offline-capable staff mesh where adding an employee is as simple as texting them a link, and the AI handles the shift reminders, permissions, and tip splitting invisibly.

  ## Competitive Analysis
  - **Square:** Excellent POS integration, timecards. Very complex setup, requires multiple apps, manual tip pool rules.
  - **Shopify:** Granular permissions per role. Web-first design, poor mobile POS switching experience.
  - **OHC (Target):** Zero-app-install SMS onboarding, Invisible AI tip splitting, Offline-first clock-ins. Abstract RBAC into simple English ("Can run register").

  ## Persona Pain Points
  - **Maya:** "I want my assistant to check off custom cake orders on the iPad, but I don't want her to see my total monthly sales or bank account."
  - **Fatima:** "My lunch rush staff change every week. Setting them up in a system takes too long. I need them to just type a 4-digit PIN on the terminal and start ringing up falafel."

  ## Architecture
  *   **Offline-First PIN Authentication:** Staff PINs and roles are synced to edge terminals for offline capability.
  *   **Data Entities:** `StaffMember`, `SecurePin`, `RoleTemplate`, `TimecardEvent`.
  *   **AI Payroll Agent:** Calculates shift duration & splits tips automatically.

  ## Next Steps (Implementer Task)
  1. Create core `StaffMember` and `TimecardEvent` data entities with strict tenant isolation.
  2. Implement a secure, offline-capable local storage mechanism for caching hashed staff PINs.
  3. Develop the 375px mobile UI for the Manager Team view and the Terminal PIN unlock screen.
  4. Ensure POS UI dynamically adapts based on the active staff session's role.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
