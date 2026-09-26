issue_title: "🚀 Nova: OHC-09: Measurable retention/acquisition experiment"
issue_description: |
  # Research Report: OHC-09 - Measurable retention/acquisition experiment

  **Superpowers Provenance:**
  - **Skill loaded:** `skills/using-superpowers/SKILL.md` (and `skills/brainstorming/SKILL.md` via workflow)
  - **Revision:** `8ca22dba9a94f28898bbce59f2537ff4d87c747d`
  - **Checks:** Explicit syntax validation and exact staging commands executed.
  - **Outcomes:** Blocked/No-work finding documented due to missing owner metrics and missing baseline measurements.

  ## Problem Statement / Research Context
  Investigate OHC-09: "Measurable retention/acquisition experiment (growth)". The target is to build a consent-aware campaign/rebooking loop that attributes qualified leads, paid work, spend, and margin while respecting limits automatically.

  ## Blocked Prerequisites / Findings
  The required implementation is **blocked** and yields a "no-work" finding because:
  1. We currently lack the prerequisite measured owner economic/metric data.
  2. The OHC-06 dependency (Delivery -> invoice -> collected/reconciled balance) is required before OHC-09 can be confidently implemented, and the underlying financial/reconciliation paths have unresolved gaps (F08, F09, etc.).
  3. The instruction explicitly states to "Measure cost before setting rates" and to "prove a reusable operating loop" before expansion, which necessitates first obtaining the data about actual model/tool usage, costs, and stable usage identity (which is currently an open gap per F05 and F14).

  No viral widget or forced growth implementation is justified at this time.

  ## Required Funnel Diagram

  ```mermaid
  flowchart LR
      A[Target: Measure Retention/Acquisition] --> B{Pre-req: Verified OHC-06}
      B -->|Blocked| C[Missing: reconciled invoice data]
      A --> D{Pre-req: Actual Owner Metrics}
      D -->|Blocked| E[Missing: baseline economics]
      C & E --> F((Blocked / No-Work))
  ```

issue_priority: "P2"
issue_category: "growth"
issue_type: "research"
issue_label: "ohc:lane:growth"
assignees: []
