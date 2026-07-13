#!/usr/bin/env python3
"""Behavioral regression tests for the PostgreSQL security CI contract."""

from pathlib import Path
import tempfile

from check_postgres_security_ci import ContractError, check_workflow


ROOT = Path(__file__).resolve().parents[2]
WORKFLOW = ROOT / ".github" / "workflows" / "ci.yml"


def expect_rejected(workflow: str, old: str, new: str, label: str) -> None:
    if old not in workflow:
        raise AssertionError(f"test fixture cannot mutate {label}: marker missing")
    with tempfile.TemporaryDirectory() as directory:
        candidate = Path(directory) / "ci.yml"
        candidate.write_text(workflow.replace(old, new), encoding="utf-8")
        try:
            check_workflow(candidate)
        except ContractError:
            return
    raise AssertionError(f"contract accepted weakened {label}")


def main() -> None:
    check_workflow(WORKFLOW)
    workflow = WORKFLOW.read_text(encoding="utf-8")
    mutations = (
        ("pgvector/pgvector:pg16", "postgres:16", "pgvector service"),
        ("OHC_REQUIRE_POSTGRES_TESTS: \"1\"", "OHC_REQUIRE_POSTGRES_TESTS: \"0\"", "required mode"),
        ("AND NOT rolbypassrls", "OR rolbypassrls", "NOBYPASSRLS assertion"),
        ("current_setting('row_security') = 'on'", "true", "row_security assertion"),
        ("cargo test -p server_auth multitenancy_isolation:: -- --nocapture", "cargo test -p server_auth", "exact suite command"),
        ("POSTGRES_SECURITY_RESULT: ${{ needs.postgres-security.result }}", "POSTGRES_SECURITY_RESULT: success", "required result propagation"),
        ('require_success "postgres-security" "$POSTGRES_SECURITY_RESULT"', 'allow_success_or_skipped "postgres-security" "$POSTGRES_SECURITY_RESULT"', "non-markdown enforcement"),
        (
            "        run: cargo test -p server_auth multitenancy_isolation:: -- --nocapture",
            "        run: |\n          # cargo test -p server_auth multitenancy_isolation:: -- --nocapture\n          true",
            "commented suite command",
        ),
        (
            "        run: cargo test -p server_auth multitenancy_isolation:: -- --nocapture",
            "        run: |\n          if false; then\n            cargo test -p server_auth multitenancy_isolation:: -- --nocapture\n          fi",
            "unreachable suite command",
        ),
        (
            "                AND NOT rolsuper\n                AND NOT rolinherit\n                AND NOT rolbypassrls\n                AND pg_has_role(current_user, 'ohc_bypassrls', 'MEMBER')",
            "                # AND NOT rolsuper\n                # AND NOT rolinherit\n                # AND NOT rolbypassrls\n                # AND pg_has_role(current_user, 'ohc_bypassrls', 'MEMBER')",
            "commented role assertions",
        ),
        (
            '          psql "$OHC_POSTGRES_ADMIN_URL" --set ON_ERROR_STOP=1 <<\'SQL\'',
            '          if false; then\n            psql "$OHC_POSTGRES_ADMIN_URL" --set ON_ERROR_STOP=1 <<\'SQL\'',
            "unreachable role proof",
        ),
        (
            '            require_success "postgres-security" "$POSTGRES_SECURITY_RESULT"',
            '            # require_success "postgres-security" "$POSTGRES_SECURITY_RESULT"\n            true',
            "commented required-result enforcement",
        ),
        (
            '            require_success "postgres-security" "$POSTGRES_SECURITY_RESULT"',
            '            if false; then\n              require_success "postgres-security" "$POSTGRES_SECURITY_RESULT"\n            fi',
            "unreachable required-result enforcement",
        ),
        (
            "          set -euo pipefail\n\n          echo \"check-changes: ${CHECK_CHANGES_RESULT}\"",
            "          set -euo pipefail\n          exit 0\n\n          echo \"check-changes: ${CHECK_CHANGES_RESULT}\"",
            "required-result direct early success",
        ),
        (
            "          set -euo pipefail\n\n          echo \"check-changes: ${CHECK_CHANGES_RESULT}\"",
            "          set -euo pipefail\n          if true; then exit 0; fi\n\n          echo \"check-changes: ${CHECK_CHANGES_RESULT}\"",
            "required-result guarded early success",
        ),
        (
            "          set -euo pipefail\n\n          echo \"check-changes: ${CHECK_CHANGES_RESULT}\"",
            "          set -euo pipefail\n          true; exit 0\n\n          echo \"check-changes: ${CHECK_CHANGES_RESULT}\"",
            "required-result compound early success",
        ),
        (
            "          set -euo pipefail\n\n          echo \"check-changes: ${CHECK_CHANGES_RESULT}\"",
            "          set -euo pipefail\n          if :; then exit 0; fi\n\n          echo \"check-changes: ${CHECK_CHANGES_RESULT}\"",
            "required-result colon-guarded early success",
        ),
        (
            "          set -euo pipefail\n\n          echo \"check-changes: ${CHECK_CHANGES_RESULT}\"",
            "          set -euo pipefail\n          if [[ 1 -eq 1 ]]; then\n            exit 0\n          fi\n\n          echo \"check-changes: ${CHECK_CHANGES_RESULT}\"",
            "required-result multiline early success",
        ),
        (
            "          set -euo pipefail\n\n          echo \"check-changes: ${CHECK_CHANGES_RESULT}\"",
            "          set -euo pipefail\n          exit 00\n\n          echo \"check-changes: ${CHECK_CHANGES_RESULT}\"",
            "required-result alternate-zero early success",
        ),
        (
            "          set -euo pipefail\n\n          echo \"check-changes: ${CHECK_CHANGES_RESULT}\"",
            "          set -euo pipefail\n          exec /bin/true\n\n          echo \"check-changes: ${CHECK_CHANGES_RESULT}\"",
            "required-result exec replacement",
        ),
        (
            '            if [[ "$result" != "success" ]]; then',
            '            if [[ "$result" == "success" ]]; then',
            "weakened require-success condition",
        ),
        (
            "              false",
            "              true",
            "weakened required-result failure command",
        ),
        (
            "          .github/scripts/check_repo_hygiene_test.sh",
            "          exit 0\n          .github/scripts/check_repo_hygiene_test.sh",
            "check-changes direct early success",
        ),
        (
            "          .github/scripts/check_repo_hygiene_test.sh",
            "          if true; then return 0; fi\n          .github/scripts/check_repo_hygiene_test.sh",
            "check-changes guarded early return",
        ),
        (
            "          .github/scripts/check_repo_hygiene_test.sh",
            "          true; exit 0\n          .github/scripts/check_repo_hygiene_test.sh",
            "check-changes compound early success",
        ),
        (
            "          .github/scripts/check_repo_hygiene_test.sh",
            "          if :; then exit 0; fi\n          .github/scripts/check_repo_hygiene_test.sh",
            "check-changes colon-guarded early success",
        ),
        (
            "          .github/scripts/check_repo_hygiene_test.sh",
            "          if [[ 1 -eq 1 ]]; then\n            return 0\n          fi\n          .github/scripts/check_repo_hygiene_test.sh",
            "check-changes multiline early return",
        ),
        (
            "          .github/scripts/check_repo_hygiene_test.sh",
            "          exit 00\n          .github/scripts/check_repo_hygiene_test.sh",
            "check-changes alternate-zero early success",
        ),
        (
            "          .github/scripts/check_repo_hygiene_test.sh",
            "          exec /bin/true\n          .github/scripts/check_repo_hygiene_test.sh",
            "check-changes exec replacement",
        ),
        (
            '          psql "$OHC_POSTGRES_ADMIN_URL" --set ON_ERROR_STOP=1 <<\'SQL\'',
            '          exit 00\n          psql "$OHC_POSTGRES_ADMIN_URL" --set ON_ERROR_STOP=1 <<\'SQL\'',
            "application-role proof early success",
        ),
        (
            '          psql "$OHC_POSTGRES_ADMIN_URL" --set ON_ERROR_STOP=1 <<\'SQL\'',
            "          true <<'SQL'",
            "inert admin SQL heredoc owner",
        ),
        (
            '          psql "$OHC_DATABASE_URL" --set ON_ERROR_STOP=1 <<\'SQL\'',
            "          true <<'SQL'",
            "inert application-role SQL heredoc owner",
        ),
        (
            '          psql "$OHC_DATABASE_URL" --set ON_ERROR_STOP=1 <<\'SQL\'',
            '          "$(printf psql)" "$OHC_DATABASE_URL" --set ON_ERROR_STOP=1 <<\'SQL\'',
            "substituted application-role SQL owner",
        ),
        (
            '          psql "$OHC_DATABASE_URL" --set ON_ERROR_STOP=1 <<\'SQL\'',
            '          command psql "$OHC_DATABASE_URL" --set ON_ERROR_STOP=1 <<\'SQL\'',
            "wrapped application-role SQL owner",
        ),
        (
            '          psql "$OHC_POSTGRES_ADMIN_URL" --set ON_ERROR_STOP=1 <<\'SQL\'',
            '          exec /bin/true\n          psql "$OHC_POSTGRES_ADMIN_URL" --set ON_ERROR_STOP=1 <<\'SQL\'',
            "application-role proof exec replacement",
        ),
        (
            "      - name: Run PostgreSQL tenant-isolation suite\n        run:",
            "      - name: Run PostgreSQL tenant-isolation suite\n        if: ${{ false }}\n        run:",
            "disabled suite step",
        ),
        (
            "      - name: Run PostgreSQL tenant-isolation suite\n        run:",
            "      - name: Run PostgreSQL tenant-isolation suite\n        continue-on-error: true\n        run:",
            "ignored suite failure",
        ),
        (
            "      - name: Check required CI results\n        env:",
            "      - name: Check required CI results\n        if: ${{ false }}\n        env:",
            "disabled required-result step",
        ),
        (
            "      - name: Provision and verify non-superuser application role\n        run:",
            "      - name: Provision and verify non-superuser application role\n        continue-on-error: true\n        run:",
            "ignored role-proof failure",
        ),
        (
            "      - name: Check tracked artifacts\n        run:",
            "      - name: Check tracked artifacts\n        if: ${{ false }}\n        run:",
            "disabled always-run contract step",
        ),
        (
            "  postgres-security:\n    name: PostgreSQL tenant isolation",
            "  postgres-security:\n    name: PostgreSQL tenant isolation\n    continue-on-error: true",
            "ignored postgres-security job failure",
        ),
        (
            "  ci-required:\n    name: CI Required",
            "  ci-required:\n    name: CI Required\n    continue-on-error: true",
            "ignored ci-required job failure",
        ),
        (
            "    if: ${{ always() }}",
            "    if: ${{ success() }}",
            "conditional ci-required job",
        ),
        (
            "    if: ${{ always() }}\n",
            "",
            "ci-required job missing always condition",
        ),
        (
            "  check-changes:\n    name: Check what files changed",
            "  check-changes:\n    name: Check what files changed\n    continue-on-error: true",
            "ignored check-changes job failure",
        ),
    )
    for old, new, label in mutations:
        expect_rejected(workflow, old, new, label)
    print("postgres security CI contract behavior: ok")


if __name__ == "__main__":
    main()
