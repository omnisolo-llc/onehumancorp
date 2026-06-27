issue_title: "Universal Capacity & Appointment Ledger (UCAL) Architecture"
issue_description: |
  ## Universal Capacity & Appointment Ledger (UCAL)

  ### Problem Statement
  OneHumanCorp (OHC) owners like Maya (baker), Carlos (handyman), and Leo (music tutor) struggle with fragmented and siloed scheduling systems. Maya needs to balance oven capacity and prep time; Carlos needs realistic travel buffers between jobs; Leo needs to manage online vs. in-person availability on a shared time resource. Current scheduling tools are binary (available/unavailable) and don't account for multi-unit capacity, resource constraints, or high-velocity concurrency. This leads to overbooking, stress, and lost revenue for the operator.

  ### Research Report
  Our research into SMB operations reveals that "available time" is rarely a simple switch. It is a ledger of capacity.
  - **Competitive Analysis:**
    - **Shopify:** Good for products, but relies on 3rd-party apps (e.g., Appointly) for bookings, creating data silos.
    - **Calendly:** Excellent for simple slots, but lacks "capacity units" (e.g., multiple people in one slot) and doesn't integrate with product inventory.
    - **OHC Differentiation:** UCAL provides a single, atomic source of truth for ALL workable capacity. It unifies Products, Services, Resources, and AI Agents into one ledger, enabling "Self-Healing Schedules" where AI agents autonomously adjust buffers and locks based on real-time business signals.

  ### Design Doc

  #### Architecture Diagram (UCAL Flow)
  ```mermaid
  graph TD
      subgraph WorkIntake "Work Intake (Signals)"
          DM[Instagram DM]
          Web[Web Form]
      end

      subgraph AI "AI Agent Departments"
          Ops[Operations Agent]
          Logistics[Logistics Agent]
          Advisor[Advisor Agent]
      end

      subgraph UCAL "Universal Capacity & Appointment Ledger"
          Resource[(UCAL Resources)]
          Ledger[(UCAL Ledger - Atomic)]
          Buffers[(Dynamic Buffers)]
      end

      subgraph BusinessResult "Business Result"
          Book[Confirmed Booking]
          Prep[Prep Task]
          Travel[Travel Route]
      end

      DM --> Ops
      Ops -- Checks Capacity --> Ledger
      Ops -- Drafts Reply --> DM
      Logistics -- Injects Travel Buffer --> Buffers
      Buffers --> Ledger
      Ledger -- Invariants Check --> Resource
      Ledger --> BusinessResult
      Advisor -- Scans Yield --> Ledger
  ```

  #### Data Model & Invariants
  - **Entities:**
    - `ucal_resources`: Maps staff, equipment, or space to a base capacity.
    - `ucal_ledger`: Atomic log of capacity units consumed over time windows. Includes status (LOCKED, TENTATIVE, BUFFER).
    - `ucal_dynamic_buffers`: Context-specific prep or travel time linked to ledger entries.
  - **Invariants:**
    - **Capacity Integrity:** Sum of consumed units must never exceed total available units for any time slice.
    - **Multi-Tenant Isolation:** Strictly enforced via PostgreSQL Row-Level Security (RLS) on all UCAL tables.
    - **Atomic Locking:** All capacity writes must use FOR UPDATE locks to prevent race conditions during high-volume signal processing.

  #### Mobile UX Flow (375px First)
  - **Capacity Heatmap:** A premium, glassmorphism-styled dashboard card showing a 7-day color-coded capacity summary.
  - **Buffer Slider:** Inline controls for Carlos to manually adjust AI-suggested travel time via a 44x44px touch target.
  - **Overload Alerts:** High-visibility triage cards that trigger when concurrency limits are breached (e.g., "Lunch Rush 120% Full").

  #### AI Agent Integration Points
  - **The Operations Agent:** Queries UCAL for availability during work triage and places tentative locks.
  - **The Logistics Agent:** Calculates and inserts travel buffers for field service personas.
  - **The Advisor Agent:** Identifies low-occupancy gaps and suggests promotional campaigns to the owner.

  ### Implementation Prompt (For Implementer Swarm)
  "Build the Universal Capacity & Appointment Ledger (UCAL) system. Implement the core schema (ucal_resources, ucal_ledger, ucal_dynamic_buffers) in PostgreSQL with RLS enabled. Create a service layer that provides atomic 'Check & Lock' and 'Release' functions for capacity units. Integrate this ledger with the existing 'Work Triage' system so that the Operations Agent can query availability and place tentative locks when drafting replies to customer inquiries. Ensure the UI includes a 'Capacity Heatmap' card for the dashboard (375px optimized) using OHC glassmorphism tokens. Acceptance criteria: Zero overbooking under high concurrency, 100% tenant isolation, and sub-100ms capacity check latency."

  ### Priority: P0
  ### Estimated Scope: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
