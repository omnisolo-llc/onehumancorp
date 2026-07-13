#!/usr/bin/env python3
"""Static contract for the required PostgreSQL/RLS security test lane."""

from pathlib import Path
import re
import sys


class ContractError(AssertionError):
    pass


def require(haystack: str, needle: str, context: str) -> None:
    if needle not in haystack:
        raise ContractError(f"{context}: missing {needle!r}")


def job_block(workflow: str, name: str) -> str:
    match = re.search(
        rf"(?ms)^  {re.escape(name)}:\n.*?(?=^  [A-Za-z0-9_-]+:\n|\Z)",
        workflow,
    )
    if match is None:
        raise ContractError(f"missing job {name!r}")
    return match.group(0)


def check_workflow(path: Path) -> None:
    workflow = path.read_text(encoding="utf-8")
    security = job_block(workflow, "postgres-security")
    required = job_block(workflow, "ci-required")
    changes = job_block(workflow, "check-changes")

    for marker, context in (
        ("needs:\n      - check-changes", "change dependency"),
        ("needs.check-changes.outputs.markdown-only == 'false'", "markdown-only skip policy"),
        ("services:", "PostgreSQL service"),
        ("image: pgvector/pgvector:pg16", "pgvector image"),
        ("pg_isready", "service health check"),
        ('OHC_REQUIRE_POSTGRES_TESTS: "1"', "required test environment"),
        ("OHC_POSTGRES_ADMIN_URL: postgresql://postgres:postgres@127.0.0.1:5432/ohc_security", "admin URL"),
        ("OHC_DATABASE_URL: postgresql://ohc_security_test:ohc_security_test@127.0.0.1:5432/ohc_security", "application-role URL"),
        ("current_user = 'ohc_security_test'", "application-role identity assertion"),
        ("AND NOT rolsuper", "non-superuser assertion"),
        ("AND NOT rolbypassrls", "NOBYPASSRLS assertion"),
        ("current_setting('row_security') = 'on'", "row_security assertion"),
        ("cargo test -p server_auth multitenancy_isolation:: -- --nocapture", "exact multitenancy suite"),
    ):
        require(security, marker, context)

    require(required, "      - postgres-security", "ci-required dependency")
    require(
        required,
        "POSTGRES_SECURITY_RESULT: ${{ needs.postgres-security.result }}",
        "ci-required result propagation",
    )
    require(
        required,
        'require_success "postgres-security" "$POSTGRES_SECURITY_RESULT"',
        "non-markdown required result",
    )
    require(
        changes,
        "python3 .github/scripts/check_postgres_security_ci_test.py",
        "always-run behavioral contract",
    )
    require(
        changes,
        "python3 .github/scripts/check_postgres_security_ci.py",
        "always-run static contract",
    )


def main() -> int:
    root = Path(__file__).resolve().parents[2]
    path = Path(sys.argv[1]) if len(sys.argv) > 1 else root / ".github" / "workflows" / "ci.yml"
    try:
        check_workflow(path)
    except (ContractError, OSError) as error:
        print(f"postgres security CI contract: {error}", file=sys.stderr)
        return 1
    print("postgres security CI contract: ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
