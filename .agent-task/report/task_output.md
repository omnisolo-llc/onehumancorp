issue_title: "Privacy Audit: Contrast data handling in Cloud vs Standalone"
issue_description: |
  **Title**: Privacy Audit: Cloud vs Standalone Data Handling

  **Problem Statement**:
  The hybrid architecture (Cloud vs. Standalone) must ensure privacy-by-design. We need to audit the codebase to ensure no non-consented telemetry, data exfiltration, or PII leakage in multi-tenant environments.

  **Research Report**:
  A review of the current code reveals that the foundational multi-tenant and privacy components are mostly intact but require validation regarding PII leakage. We ran `src/server/pii_leakage_check.sh src/server` and `.github/scripts/check_postgres_security_ci_test.py` via `run_in_bash_session` to perform the audit which yielded:
  - `PASS: No obvious PII leakage found in tracing logs.`
  - `postgres security CI contract behavior: ok`
  The standalone wrapper has not shown any non-consented telemetry mechanisms.

  **Loaded Skills:**
  - `https://github.com/obra/superpowers/`

  **Git Revision Commit Hash:**
  `5bf4e78011075bcfc0dc295f0724994cd123ee71`

  **Design Doc**:
  N/A (Audit finding)

  **Implementation Prompt**:
  N/A (Audit finding)

  **Priority**: P1
  **Estimated Scope**: Small
issue_priority: "P1"
issue_category: "Security"
issue_type: "Audit"
issue_label: "privacy"
assignees: []
