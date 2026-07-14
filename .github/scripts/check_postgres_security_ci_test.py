#!/usr/bin/env python3
"""Behavioral regression tests for the PostgreSQL security CI contract."""

from pathlib import Path
import os
import subprocess
import tempfile
from unittest import mock

from check_postgres_security_ci import ContractError, check_workflow, validate_yaml


ROOT = Path(__file__).resolve().parents[2]
WORKFLOW = ROOT / ".github" / "workflows" / "ci.yml"


def expect_text_rejected(candidate_text: str, label: str) -> None:
    with tempfile.TemporaryDirectory() as directory:
        candidate = Path(directory) / "ci.yml"
        candidate.write_text(candidate_text, encoding="utf-8")
        try:
            check_workflow(candidate)
        except ContractError:
            return
    raise AssertionError(f"contract accepted weakened {label}")


def expect_rejected(workflow: str, old: str, new: str, label: str) -> None:
    if old not in workflow:
        raise AssertionError(f"test fixture cannot mutate {label}: marker missing")
    expect_text_rejected(workflow.replace(old, new), label)


def assert_bash_env_can_preempt_a_step() -> None:
    with tempfile.TemporaryDirectory() as directory:
        bash_env = Path(directory) / "bash_env.sh"
        marker = Path(directory) / "body-ran"
        bash_env.write_text("exit 0\n", encoding="utf-8")
        environment = os.environ.copy()
        environment.update(BASH_ENV=str(bash_env), BODY_MARKER=str(marker))
        result = subprocess.run(
            ["bash", "-c", 'printf reached > "$BODY_MARKER"; exit 97'],
            check=False,
            env=environment,
        )
        if result.returncode != 0 or marker.exists():
            raise AssertionError("BASH_ENV regression did not preempt the Bash step body")


def assert_real_yaml_parser_rejects_unquoted_colon_space() -> None:
    with tempfile.TemporaryDirectory() as directory:
        invalid = Path(directory) / "invalid.yml"
        invalid.write_text(
            "jobs:\n  security:\n    steps:\n      - run: cargo test multitenancy_isolation:: -- --nocapture\n",
            encoding="utf-8",
        )
        try:
            validate_yaml(invalid)
        except ContractError:
            return
    raise AssertionError("real YAML parser accepted an unquoted colon-space scalar")


def assert_parser_absence_fails_closed() -> None:
    real_import = __import__

    def import_without_yaml(name, *args, **kwargs):
        if name == "yaml":
            raise ImportError("simulated missing PyYAML")
        return real_import(name, *args, **kwargs)

    with (
        mock.patch("builtins.__import__", side_effect=import_without_yaml),
        mock.patch("check_postgres_security_ci.shutil.which", return_value=None),
    ):
        try:
            validate_yaml(WORKFLOW)
        except ContractError:
            return
    raise AssertionError("YAML validation did not fail closed without either parser")


def assert_real_yaml_parser_rejects_nested_duplicates() -> None:
    with tempfile.TemporaryDirectory() as directory:
        duplicate = Path(directory) / "duplicate.yml"
        duplicate.write_text(
            "root:\n  nested:\n    security: enabled\n    security: disabled\n",
            encoding="utf-8",
        )
        try:
            validate_yaml(duplicate)
        except ContractError:
            return
    raise AssertionError("real YAML parser accepted a recursively duplicated key")


def main() -> None:
    assert_bash_env_can_preempt_a_step()
    assert_real_yaml_parser_rejects_unquoted_colon_space()
    assert_parser_absence_fails_closed()
    assert_real_yaml_parser_rejects_nested_duplicates()
    check_workflow(WORKFLOW)
    workflow = WORKFLOW.read_text(encoding="utf-8")
    mutations = (
        (
            '  FORCE_JAVASCRIPT_ACTIONS_TO_NODE24: "true"\n  NODE_DISABLE_COMPILE_CACHE: "1"',
            '  FORCE_JAVASCRIPT_ACTIONS_TO_NODE24: "true"\n  NODE_DISABLE_COMPILE_CACHE: "1"\n  BASH_ENV: /tmp/skip-security.sh',
            "workflow BASH_ENV override",
        ),
        ("pgvector/pgvector:pg16", "postgres:16", "pgvector service"),
        ("OHC_REQUIRE_POSTGRES_TESTS: \"1\"", "OHC_REQUIRE_POSTGRES_TESTS: \"0\"", "required mode"),
        (
            "      OHC_DATABASE_URL: postgresql://ohc_security_test:ohc_security_test@127.0.0.1:5432/ohc_security",
            '      OHC_DATABASE_URL: postgresql://ohc_security_test:ohc_security_test@127.0.0.1:5432/ohc_security\n      PATH: "/tmp/fake-bin"',
            "postgres-security job PATH override",
        ),
        ("AND NOT rolbypassrls", "OR rolbypassrls", "NOBYPASSRLS assertion"),
        ("current_setting('row_security') = 'on'", "true", "row_security assertion"),
        ("cargo test -p server_auth multitenancy_isolation:: -- --nocapture", "cargo test -p server_auth", "exact suite command"),
        ("POSTGRES_SECURITY_RESULT: ${{ needs.postgres-security.result }}", "POSTGRES_SECURITY_RESULT: success", "required result propagation"),
        ('require_success "postgres-security" "$POSTGRES_SECURITY_RESULT"', 'allow_success_or_skipped "postgres-security" "$POSTGRES_SECURITY_RESULT"', "non-markdown enforcement"),
        (
            '        run: "cargo test -p server_auth multitenancy_isolation:: -- --nocapture"',
            "        run: |\n          # cargo test -p server_auth multitenancy_isolation:: -- --nocapture\n          true",
            "commented suite command",
        ),
        (
            '        run: "cargo test -p server_auth multitenancy_isolation:: -- --nocapture"',
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
            "      - name: Run PostgreSQL tenant-isolation suite\n        run:",
            "      - name: Run PostgreSQL tenant-isolation suite\n        shell: bash {0} || true\n        run:",
            "suite swallowing shell override",
        ),
        (
            "      - name: Run PostgreSQL tenant-isolation suite\n        run:",
            '      - name: Run PostgreSQL tenant-isolation suite\n        env:\n          OHC_REQUIRE_POSTGRES_TESTS: "0"\n          OHC_DATABASE_URL: ""\n        run:',
            "suite optional-skip environment",
        ),
        (
            "      - name: Run PostgreSQL tenant-isolation suite\n        run:",
            '      - name: Run PostgreSQL tenant-isolation suite\n        ? env\n        :\n          OHC_REQUIRE_POSTGRES_TESTS: "0"\n          OHC_DATABASE_URL: ""\n        run:',
            "explicit-key suite optional-skip environment",
        ),
        (
            "      - name: Run PostgreSQL tenant-isolation suite\n        run:",
            '      - name: Run PostgreSQL tenant-isolation suite\n        env:\n          PATH: "/tmp/fake-bin"\n        run:',
            "suite PATH override",
        ),
        (
            "      - name: Run PostgreSQL tenant-isolation suite\n        run:",
            "      - name: Run PostgreSQL tenant-isolation suite\n        working-directory: /tmp\n        run:",
            "suite working-directory override",
        ),
        (
            "      - name: Run PostgreSQL tenant-isolation suite\n        run:",
            "      - name: Run PostgreSQL tenant-isolation suite\n        \"shell\": bash {0} || true\n        run:",
            "quoted suite shell override",
        ),
        (
            "      - name: Run PostgreSQL tenant-isolation suite\n        run:",
            "      - name: Run PostgreSQL tenant-isolation suite\n        shell : bash {0} || true\n        run:",
            "spaced suite shell override",
        ),
        (
            "      - name: Run PostgreSQL tenant-isolation suite\n        run:",
            "      - name: Run PostgreSQL tenant-isolation suite\n        ? shell\n        : bash {0} || true\n        run:",
            "explicit-key suite shell override",
        ),
        (
            "      - name: Provision and verify non-superuser application role\n        run:",
            "      - name: Provision and verify non-superuser application role\n        shell: bash {0} || true\n        run:",
            "role-proof swallowing shell override",
        ),
        (
            "      - name: Provision and verify non-superuser application role\n        run:",
            '      - name: Provision and verify non-superuser application role\n        env:\n          PATH: "/tmp/fake-bin"\n        run:',
            "role-proof PATH override",
        ),
        (
            "      - name: Check required CI results\n        env:",
            "      - name: Check required CI results\n        if: ${{ false }}\n        env:",
            "disabled required-result step",
        ),
        (
            "      - name: Check required CI results\n        env:",
            "      - name: Check required CI results\n        shell: bash {0} || true\n        env:",
            "required-result swallowing shell override",
        ),
        (
            "      - name: Check required CI results\n        env:",
            "      - name: Check required CI results\n        working-directory: /tmp\n        env:",
            "required-result working-directory override",
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
            "      - name: Check tracked artifacts\n        run:",
            "      - name: Check tracked artifacts\n        shell: bash {0} || true\n        run:",
            "contract swallowing shell override",
        ),
        (
            "      - name: Check tracked artifacts\n        run:",
            '      - name: Check tracked artifacts\n        env:\n          PATH: "/tmp/fake-bin"\n        run:',
            "contract PATH override",
        ),
        (
            "  postgres-security:\n    name: PostgreSQL tenant isolation",
            "  postgres-security:\n    name: PostgreSQL tenant isolation\n    continue-on-error: true",
            "ignored postgres-security job failure",
        ),
        (
            "  postgres-security:\n    name: PostgreSQL tenant isolation",
            "  postgres-security:\n    name: PostgreSQL tenant isolation\n    defaults:\n      run:\n        shell: bash {0} || true",
            "postgres-security swallowing shell default",
        ),
        (
            "  postgres-security:\n    name: PostgreSQL tenant isolation",
            "  postgres-security:\n    name: PostgreSQL tenant isolation\n    \"defaults\":\n      run:\n        shell: bash {0} || true",
            "quoted postgres-security shell default",
        ),
        (
            "  ci-required:\n    name: CI Required",
            "  ci-required:\n    name: CI Required\n    continue-on-error: true",
            "ignored ci-required job failure",
        ),
        (
            "  ci-required:\n    name: CI Required",
            "  ci-required:\n    name: CI Required\n    \"continue-on-error\": true",
            "quoted ignored ci-required job failure",
        ),
        (
            "  ci-required:\n    name: CI Required",
            "  ci-required:\n    name: CI Required\n    ? continue-on-error\n    : true",
            "explicit-key ignored ci-required job failure",
        ),
        (
            "  ci-required:\n    name: CI Required",
            "  ci-required:\n    name: CI Required\n    defaults:\n      run:\n        shell: bash {0} || true",
            "ci-required swallowing shell default",
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
        (
            "  check-changes:\n    name: Check what files changed",
            "  check-changes:\n    name: Check what files changed\n    defaults:\n      run:\n        shell: bash {0} || true",
            "check-changes swallowing shell default",
        ),
        (
            "defaults:\n  run:\n    shell: bash",
            "defaults:\n  run:\n    shell: bash {0} || true",
            "workflow swallowing shell default",
        ),
        (
            "defaults:\n  run:\n    shell: bash\n\n",
            "",
            "workflow missing shell default",
        ),
    )
    for old, new, label in mutations:
        expect_rejected(workflow, old, new, label)
    duplicate_documents = (
        (
            '\nenv:\n  FORCE_JAVASCRIPT_ACTIONS_TO_NODE24: "true"\n  NODE_DISABLE_COMPILE_CACHE: "1"\n  BASH_ENV: /tmp/skip-security.sh\n',
            "duplicate top-level env",
        ),
        (
            "\ndefaults:\n  run:\n    shell: bash {0} || true\n",
            "duplicate top-level defaults",
        ),
        (
            "\njobs:\n  shadow-security:\n    runs-on: ubuntu-latest\n    steps:\n      - run: true\n",
            "duplicate top-level jobs",
        ),
    )
    for suffix, label in duplicate_documents:
        expect_text_rejected(workflow + suffix, label)
    print("postgres security CI contract behavior: ok")


if __name__ == "__main__":
    main()
