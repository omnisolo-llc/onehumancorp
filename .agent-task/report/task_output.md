issue_title: "[Research] OHC Dynamic Multi-Tenant Commerce Architecture"
issue_description: |
  Following the instruction to evaluate the current issue #34789 and apply the `2026-09-18-usage-audit` operating contract override, I have concluded this task must return a blocked or no-work finding.

  **Issue Context & Evidence**
  The issue requests a "Go + Bazel backend" implementation with an RPC definition and PostgreSQL table migrations for a "Unified Commerce Concept" and an "Operations Agent", citing competitor analysis rather than existing code evidence.

  The `2026-09-18-usage-audit` and instructions explicitly state: "The owner has explicitly authorized migration from Bazel to native Rust/Cargo, Tauri and Node.js and remediation of the recorded findings; do not reintroduce Bazel." The codebase is already using a Rust/Cargo workspace, and `docs/research/native_migration_and_remediation.md` confirms Bazel is being replaced. Building out new Go services with Bazel is explicitly forbidden.

  Furthermore, a "Commerce Tables" section already exists in `src/server/migrations/001_initial.sql` defining `products` with a `type TEXT, -- 'physical', 'digital', 'booking'` enum and tracking of `inventory_count`, which fulfills the core unified schema requirement requested by the issue. RLS is also implemented on the commerce tables.

  According to `RESEARCH.md` and the instructions, "New epics need an explicit evidence-backed decision; assigned concrete defect work may continue." and "Do not build another generic assistant, duplicate subsystem or broad ERP on the basis of this research." The requested "Operations Agent" lacks this evidence-backed decision for expansion, especially since onboarding products already exist in `src/server/services/onboarding/onboarding_agent.rs`.

  **Loaded Skills (Superpowers Workflow)**
  - Revision: `8ca22dba9a94f28898bbce59f2537ff4d87c747d`
  - Skills:
    - `skills/using-superpowers/SKILL.md`
outcome: no_work
