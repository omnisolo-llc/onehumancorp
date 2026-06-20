issue_title: "[Research] AI-Powered Staff Task & Shift Coordination Architecture"
issue_description: |
  # Research Report: AI-Powered Staff Task & Shift Coordination Architecture

  ## 1. Problem Statement
  Location managers and operators (e.g., Jun the Location Manager, Fatima the Food Cart Operator) struggle with coordinating daily tasks, managing shift handoffs, and escalating issues to owners. Existing task management tools are disconnected from the core business operations (sales, inventory, customer feedback) and require too much manual data entry. Operators need a system that automatically generates task lists based on daily demand, tracks completion, and provides simple shift summaries.

  ## 2. Research Report
  - **Market Context**: Most POS and scheduling platforms (Square, Toast, Homebase) offer basic checklist or scheduling features, but they are passive. They do not proactively assign tasks based on real-time data (e.g., "high customer volume today, assign an extra cleaning check"). Tools like Slack or WhatsApp are used for communication but lack structured task tracking.
  - **The OHC Opportunity**: By integrating task coordination directly into the OHC platform, the AI Assistant can dynamically generate, assign, and track tasks based on real-time business signals (sales, inventory levels, customer requests).
  - **Competitor Gaps**:
    - *Square/Toast*: Checklists are static and disconnected from AI-driven insights.
    - *Homebase/7shifts*: Good for scheduling, but disconnected from real-time operational execution and customer feedback.
    - *WhatsApp/Slack*: Unstructured, no audit trail for task completion.

  ## 3. Design Doc
  ### Data Model (PostgreSQL)
  - `Task`: A specific action to be completed, with status, priority, and optional deadline.
  - `Shift`: A defined work period for a specific location.
  - `StaffMember`: An employee assigned to shifts and tasks.
  - `TaskAssignment`: Links a `Task` to a `StaffMember` or `Shift`.

  ### AI Integration
  - **Operations Agent ("The Manager")**: Automatically generates tasks based on daily operations (e.g., low inventory triggers a restock task, high sales volume triggers an extra cleanup task). Summarizes shift performance and escalates unresolved issues to the owner.
  - **Work Triage**: Prioritizes incoming tasks and alerts for staff on duty, ensuring critical issues are addressed first.

  ### Mobile UX Flow (375px)
  1. **Staff View**: A simple, touch-friendly task list. Staff can check off tasks (large touch targets), add notes, or flag issues.
  2. **Manager View (Jun)**: A dashboard showing overall shift progress, unresolved tasks, and AI-generated shift summaries.

  ### Architecture Diagram
  ```mermaid
  erDiagram
      Tenant ||--o{ Shift : has
      Tenant ||--o{ StaffMember : employs
      Tenant ||--o{ Task : contains
      Shift ||--o{ TaskAssignment : includes
      StaffMember ||--o{ TaskAssignment : assigned_to
      Task ||--o{ TaskAssignment : part_of

      Shift {
          uuid id PK
          uuid tenant_id FK
          datetime start_time
          datetime end_time
      }
      StaffMember {
          uuid id PK
          uuid tenant_id FK
          string name
          string role
      }
      Task {
          uuid id PK
          uuid tenant_id FK
          string description
          string status
          string priority
      }
      TaskAssignment {
          uuid id PK
          uuid task_id FK
          uuid staff_id FK
          uuid shift_id FK
      }
  ```

  ## 4. Implementation Prompt
  **Feature Name**: AI-Powered Staff Task Coordination
  **Target Persona**: Jun the Location Manager
  **Outcome**: Jun can manage daily tasks and shift handoffs through a unified interface. The AI automatically suggests tasks based on operational data and provides a shift summary.

  **Next Actions**:
  1. Implement the core Data Models (`Task`, `Shift`, `StaffMember`, `TaskAssignment`) with strict row-level security for multi-tenancy.
  2. Develop the Staff Mobile UX (task checklist, issue flagging) optimized for 375px viewports.
  3. Integrate the Operations Agent to generate tasks based on mock or real operational triggers (e.g., inventory thresholds).
  4. Build the Manager Dashboard for Jun to view shift progress and AI summaries.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
