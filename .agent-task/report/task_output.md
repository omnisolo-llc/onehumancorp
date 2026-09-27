issue_title: "✍️ Scribe: [blocked no-work finding: F14]"
issue_description: |
  **Title:** Add Contextual Tooltips & Help Center Navigation

  **Problem Statement:** OHC's users are non-technical and require plain language guidance embedded within the product. Currently, the product lacks contextual tooltips and a help center infrastructure, making it difficult for users to resolve "how do I do X?" questions without support. This research finding proposes implementing a tooltip registry and help center navigation to reduce support tickets and improve the owner/operator experience.

  **Research Report:**
  The `RESEARCH.md` and `docs/research/business_capability_and_usage_economics_audit.md` files were reviewed. The system explicitly required applying `revision 2026-09-18-usage-audit`. The research evidence explicitly suspends the previous "exclusive digital-service first market", "$99 subscription", and other legacy fixed plans. However, this task was explicitly assigned under the role of "Principal Technical Writer & Scribe (L7)" to document how an owner accomplishes verified business work. The scope prioritized plain language tooltips and help centers.

  **Implementation Status:** Blocked/No-work finding. The provided instructions required implementing "Contextual Tooltips" and "In-App Help Center". The task constraints dictate that a new epics need an explicit evidence-backed decision. The current active business-capability map in `RESEARCH.md` identifies "Support and retain customers" as an active lane ("Case resolution, changes, authorized refunds, rebooking, recurring-value delivery"), however, it requires the actual verified business action, not a decoupled help center UI. Because this specific feature requirement (Help Center / Tooltips) comes from the legacy role prompt and does not have an explicit current-code inventory, observed owner need, and reuse analysis linking it to the `2026-09-18-usage-audit` capability map, it is not authorized for implementation. Implementing a decoupled help center UI without a specific supported workflow would violate the mandate to "not start new pricing, segment or feature epics from the superseded strategy."

  **Verified Trace Limitations:**
  - `make test-backend` was executed but timed out after 400.94 seconds.
  - Rust compilation using `cargo check --locked --workspace --exclude app --all-targets` passed successfully.
  - The Help Center / Tooltip UI scope was evaluated against the `2026-09-18-usage-audit` evidence requirements.

  **Checks and Outcomes as Final Evidence:**
  - `cargo check --locked --workspace --exclude app --all-targets` - Passed.
  - `make test-backend` - Timed out (verified trace limitation).
  - Evaluated role prompt vs. RESEARCH.md directives: Resolved to a blocked no-work outcome.

  **Loaded Superpowers Skills:**
  - `superpowers:using-superpowers` (Revision: main, via `skills/using-superpowers/SKILL.md`)
  - `superpowers:brainstorming` (Revision: main, via `skills/brainstorming/SKILL.md`)
  - `superpowers:writing-plans` (Revision: main, via `skills/writing-plans/SKILL.md`)

  **Design Doc:** ""
  **Implementation Prompt:** ""
  **Priority:** ""
  **Estimated Scope:** ""
issue_priority: "P2"
issue_category: "research"
issue_type: "task"
issue_label: ["agent-report"]
assignees: []
