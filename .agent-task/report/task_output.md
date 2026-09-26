issue_title: "🚀 Nova: Measurable retention and acquisition loop"
issue_description: |
  **Title**: 🚀 Nova: Measurable retention and acquisition loop

  **Problem Statement**: The current platform lacks a verified, measurable retention and acquisition loop (OHC-09) to reliably attribute qualified leads, paid work, spend, and margin for the supported digital-service segment. This prevents evaluating the acquisition funnel and owner conversion outcomes.

  **Research Report**:
  Based on `docs/research/business_capability_and_usage_economics_audit.md` and `docs/research/native_migration_and_remediation.md`, the platform's current state preserves existing commerce, fulfillment, and service modules but lacks the requisite baseline measurement for a growth loop. The audit indicates that OHC-04 through OHC-08 must be validated before OHC-09 can be reliably implemented. Crucially, "No representative customer-serving cost, owner interviews, willingness-to-pay result or paid-retention result has been measured in this work" (F14). We cannot attribute margin or measure retention without the underlying economics and consent-aware campaign loops being functional and trusted. Therefore, this growth feature is currently blocked by the completion of OHC-04 to OHC-08 and actual owner economic/metric data.

  **Design Doc**:
  ```mermaid
  flowchart TD
      A[Inbound Inquiry / Lead Capture] --> B[Qualification & Proposal]
      B --> C[Accepted Scope & Deposit]
      C --> D[Service Delivery & Customer Review]
      D --> E[Final Invoice & Reconciliation]
      E --> F{Growth Loop Check}
      F -->|Satisfaction Met| G[Automated Review/Referral Request]
      F -->|Needs Recovery| H[Support & Follow-up]
      G --> I[Attributable New Lead / Repeat Booking]
      I --> A

      style F fill:#f9f,stroke:#333,stroke-width:2px
      style G fill:#bbf,stroke:#333,stroke-width:2px
      style I fill:#bbf,stroke:#333,stroke-width:2px
  ```

  **Mobile UX Flow**: N/A for a blocked/no-work backend metric implementation.
  **AI Agent Integration Points**: Growth Agent/Nova orchestrating campaign attribution, reading from `hub.rs` telemetry, and generating review/rebooking outreach.

  **Implementation Prompt**: Implement the retention and acquisition loop (OHC-09) tracing attribution and margin, integrating with existing commerce and billing pipelines once prerequisites are met.

  **Priority**: P1 (Dependent on P0/P1 prerequisites OHC-04 to OHC-08)

  **Estimated Scope**: Blocked / No-Work. Required prerequisites: Validated completion of OHC-04 (inquiry to quote), OHC-05 (deposit/booking), OHC-06 (delivery/invoice), and OHC-08 (collection/recovery). Actual cost baseline and verified owner consent models.

  **Superpowers Workflow Provenance**:
  - Loaded skills: `using-superpowers`, `brainstorming`, `writing-plans`, `executing-plans`, `subagent-driven-development`
  - Repository URL: https://github.com/obra/superpowers.git
  - Revision hash: 8ca22dba9a94f28898bbce59f2537ff4d87c747d
  - Checks performed: Evaluated `docs/research/business_capability_and_usage_economics_audit.md` (specifically F14, F15 gaps) and `docs/research/native_migration_and_remediation.md` (specifically OHC-04 through OHC-08 prerequisites).
  - Outcomes: Blocked / No-Work finding. Required external data (measured economics, willingness-to-pay, validated prior journey steps) are missing.
issue_priority: P1
issue_category: growth
issue_type: report
issue_label: ohc:journey:J1
assignees: []
