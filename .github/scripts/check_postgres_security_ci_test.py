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
    )
    for old, new, label in mutations:
        expect_rejected(workflow, old, new, label)
    print("postgres security CI contract behavior: ok")


if __name__ == "__main__":
    main()
