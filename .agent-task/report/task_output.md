outcome: blocked
issue_title: "Research: Unified Work Triage Agent for Mobile-First OHC"
issue_description: |
  The issue requests a Unified Work Triage Agent with a mobile-first Flutter UI and Gemini Pro backend. This is explicitly blocked by the OneHumanCorp operating contract (revision 2026-09-18-usage-audit), which mandates reusing the current Rust/Tauri/Next.js architecture ("do not create another frontend/rewrite") and requires the completion of native build migration and defect remediation before starting new research-only feature epics ("Evidence comes before another concrete product plan"). Workspace tests (`make lint` and `make test`) failed due to pre-existing errors (`next: not found`), recorded here as unrelated verification blockers.
issue_priority: "P0"
issue_category: ""
issue_type: ""
issue_label: ""
assignees: ""
