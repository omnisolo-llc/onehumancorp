#!/usr/bin/env python3
"""Indentation-aware contract for the required PostgreSQL/RLS CI lane.

This intentionally validates only the job and step shapes used by ci.yml. It is
not a general YAML parser.
"""

from dataclasses import dataclass
from pathlib import Path
import re
import sys


class ContractError(AssertionError):
    pass


EXPECTED_REQUIRED_RESULT_LINES = (
    "set -euo pipefail",
    'echo "check-changes: ${CHECK_CHANGES_RESULT}"',
    'echo "bazel-build: ${BAZEL_BUILD_RESULT}"',
    'echo "bazel-test: ${BAZEL_TEST_RESULT}"',
    'echo "bazel-test-e2e: ${BAZEL_TEST_E2E_RESULT}"',
    'echo "kind-e2e: ${KIND_E2E_RESULT}"',
    'echo "docker-e2e: ${DOCKER_E2E_RESULT}"',
    'echo "postgres-security: ${POSTGRES_SECURITY_RESULT}"',
    "require_success() {",
    'local name="$1"',
    'local result="$2"',
    'if [[ "$result" != "success" ]]; then',
    'echo "::error::${name} finished with result \'${result}\', expected \'success\'."',
    "false",
    "fi",
    "}",
    "allow_success_or_skipped() {",
    'local name="$1"',
    'local result="$2"',
    'if [[ "$result" != "success" && "$result" != "skipped" ]]; then',
    'echo "::error::${name} finished with result \'${result}\', expected \'success\' or \'skipped\'."',
    "false",
    "fi",
    "}",
    'require_success "check-changes" "$CHECK_CHANGES_RESULT"',
    'if [[ "$EVENT_NAME" == "schedule" || "$EVENT_NAME" == "workflow_dispatch" ]]; then',
    'require_success "bazel-build" "$BAZEL_BUILD_RESULT"',
    "else",
    'allow_success_or_skipped "bazel-build" "$BAZEL_BUILD_RESULT"',
    "fi",
    'if [[ "$MARKDOWN_ONLY" == "true" ]]; then',
    'allow_success_or_skipped "bazel-test" "$BAZEL_TEST_RESULT"',
    'allow_success_or_skipped "bazel-test-e2e" "$BAZEL_TEST_E2E_RESULT"',
    'allow_success_or_skipped "kind-e2e" "$KIND_E2E_RESULT"',
    'allow_success_or_skipped "docker-e2e" "$DOCKER_E2E_RESULT"',
    'allow_success_or_skipped "postgres-security" "$POSTGRES_SECURITY_RESULT"',
    "else",
    'require_success "bazel-test" "$BAZEL_TEST_RESULT"',
    'require_success "bazel-test-e2e" "$BAZEL_TEST_E2E_RESULT"',
    'require_success "kind-e2e" "$KIND_E2E_RESULT"',
    'require_success "docker-e2e" "$DOCKER_E2E_RESULT"',
    'require_success "postgres-security" "$POSTGRES_SECURITY_RESULT"',
    "fi",
)

EXPECTED_HYGIENE_LINES = (
    ".github/scripts/check_repo_hygiene_test.sh",
    ".github/scripts/check_repo_hygiene.sh",
    "python3 .github/scripts/check_postgres_security_ci_test.py",
    "python3 .github/scripts/check_postgres_security_ci.py",
)

ADMIN_PSQL_HEREDOC = 'psql "$OHC_POSTGRES_ADMIN_URL" --set ON_ERROR_STOP=1 <<\'SQL\''
APP_PSQL_HEREDOC = 'psql "$OHC_DATABASE_URL" --set ON_ERROR_STOP=1 <<\'SQL\''
EXPECTED_WORKFLOW_DEFAULTS = ("defaults:", "  run:", "    shell: bash")


@dataclass(frozen=True)
class Step:
    name: str
    lines: tuple[str, ...]

    def run(self) -> tuple[str, str]:
        for index, line in enumerate(self.lines):
            match = re.fullmatch(r"        run:\s*(.*)", line)
            if match is None:
                continue
            value = match.group(1)
            if value not in ("|", "|-", ">", ">-"):
                return "scalar", value
            body: list[str] = []
            for body_line in self.lines[index + 1 :]:
                if body_line.strip() and indentation(body_line) <= 8:
                    break
                body.append(body_line[10:] if body_line.startswith("          ") else body_line)
            return "block", "\n".join(body)
        raise ContractError(f"step {self.name!r} has no active run field")


def indentation(line: str) -> int:
    return len(line) - len(line.lstrip(" "))


def active_config_lines(lines: tuple[str, ...]) -> tuple[str, ...]:
    return tuple(line for line in lines if line.strip() and not line.lstrip().startswith("#"))


def mapping_block(lines: tuple[str, ...], name: str, indent: int) -> tuple[str, ...]:
    header = " " * indent + name + ":"
    try:
        start = lines.index(header)
    except ValueError as error:
        raise ContractError(f"missing active mapping {name!r} at indentation {indent}") from error
    end = len(lines)
    for index in range(start + 1, len(lines)):
        line = lines[index]
        if line.strip() and indentation(line) <= indent:
            end = index
            break
    return lines[start:end]


def steps(job: tuple[str, ...]) -> tuple[Step, ...]:
    try:
        start = job.index("    steps:") + 1
    except ValueError as error:
        raise ContractError("job has no active steps mapping") from error
    found: list[Step] = []
    index = start
    while index < len(job):
        line = job[index]
        if not line.startswith("      - "):
            index += 1
            continue
        end = index + 1
        while end < len(job) and not job[end].startswith("      - "):
            end += 1
        block = job[index:end]
        name_line = block[0]
        if name_line.startswith("      - name: "):
            found.append(Step(name_line.removeprefix("      - name: "), tuple(block)))
        index = end
    return tuple(found)


def named_step(job: tuple[str, ...], name: str) -> Step:
    matches = [step for step in steps(job) if step.name == name]
    if len(matches) != 1:
        raise ContractError(f"expected exactly one active step named {name!r}, found {len(matches)}")
    return matches[0]


def require_non_ignorable_job(job: tuple[str, ...], context: str) -> None:
    if any(line.startswith("    continue-on-error:") for line in active_config_lines(job)):
        raise ContractError(f"{context}: job-level continue-on-error is forbidden")
    if "    defaults:" in active_config_lines(job):
        raise ContractError(f"{context}: job-level run defaults are forbidden")


def require_unconditional_step(step: Step, context: str) -> None:
    for line in active_config_lines(step.lines):
        if line.startswith("        if:"):
            raise ContractError(f"{context}: step-level if condition is forbidden")
        if line.startswith("        continue-on-error:"):
            raise ContractError(f"{context}: continue-on-error is forbidden")
        if line.startswith("        shell:"):
            raise ContractError(f"{context}: shell override is forbidden")


def require_active(lines: tuple[str, ...], exact: str, context: str) -> None:
    if exact not in active_config_lines(lines):
        raise ContractError(f"{context}: missing active line {exact!r}")


def active_script(script: str, context: str) -> tuple[str, ...]:
    lines = tuple(
        line.strip()
        for line in script.splitlines()
        if line.strip() and not line.lstrip().startswith("#")
    )
    if any(re.search(r"\bif\s+(?:\[\[?\s*)?false\b", line) for line in lines):
        raise ContractError(f"{context}: contains an explicit unreachable `if false` block")
    return lines


def require_script_line(lines: tuple[str, ...], exact: str, context: str) -> None:
    if exact not in lines:
        raise ContractError(f"{context}: missing active executable line {exact!r}")


def require_no_control_transfer_before(
    lines: tuple[str, ...], protected: tuple[str, ...], context: str
) -> None:
    indexes: list[int] = []
    for exact in protected:
        require_script_line(lines, exact, context)
        indexes.append(lines.index(exact))
    for line in lines[: max(indexes)]:
        active = line.split(" #", 1)[0]
        if re.search(r"\b(?:exec|exit|return)\b", active):
            raise ContractError(
                f"{context}: active exec/exit/return token {line!r} precedes required enforcement"
            )


def exact_psql_heredocs(lines: tuple[str, ...]) -> tuple[tuple[str, ...], tuple[str, ...]]:
    if not lines or lines[0] != ADMIN_PSQL_HEREDOC:
        raise ContractError(f"application-role proof must start with exact owner {ADMIN_PSQL_HEREDOC!r}")
    try:
        admin_end = lines.index("SQL", 1)
    except ValueError as error:
        raise ContractError("admin psql heredoc has no active SQL terminator") from error
    if admin_end + 1 >= len(lines) or lines[admin_end + 1] != APP_PSQL_HEREDOC:
        raise ContractError(f"application-role proof requires exact owner {APP_PSQL_HEREDOC!r}")
    try:
        app_end = lines.index("SQL", admin_end + 2)
    except ValueError as error:
        raise ContractError("application-role psql heredoc has no active SQL terminator") from error
    if app_end != len(lines) - 1:
        raise ContractError("application-role proof contains active commands outside its two psql heredocs")
    return lines[1:admin_end], lines[admin_end + 2 : app_end]


def check_workflow(path: Path) -> None:
    workflow = tuple(path.read_text(encoding="utf-8").splitlines())
    workflow_defaults = mapping_block(workflow, "defaults", 0)
    if active_config_lines(workflow_defaults) != EXPECTED_WORKFLOW_DEFAULTS:
        raise ContractError("workflow defaults must be exactly defaults.run.shell: bash")
    jobs = mapping_block(workflow, "jobs", 0)
    security = mapping_block(jobs, "postgres-security", 2)
    required = mapping_block(jobs, "ci-required", 2)
    changes = mapping_block(jobs, "check-changes", 2)
    require_non_ignorable_job(security, "postgres-security")
    require_non_ignorable_job(required, "ci-required")
    require_non_ignorable_job(changes, "check-changes")

    for exact, context in (
        ("      - check-changes", "change dependency"),
        ("    if: ${{ needs.check-changes.outputs.markdown-only == 'false' }}", "markdown-only skip policy"),
        ("    services:", "PostgreSQL service"),
        ("        image: pgvector/pgvector:pg16", "pgvector image"),
        ('      OHC_REQUIRE_POSTGRES_TESTS: "1"', "required test environment"),
        ("      OHC_POSTGRES_ADMIN_URL: postgresql://postgres:postgres@127.0.0.1:5432/ohc_security", "admin URL"),
        ("      OHC_DATABASE_URL: postgresql://ohc_security_test:ohc_security_test@127.0.0.1:5432/ohc_security", "application-role URL"),
    ):
        require_active(security, exact, context)
    if not any("pg_isready" in line for line in active_config_lines(security)):
        raise ContractError("service health check: missing active pg_isready configuration")

    role_step = named_step(security, "Provision and verify non-superuser application role")
    require_unconditional_step(role_step, "application-role proof")
    role_style, role_run = role_step.run()
    if role_style != "block":
        raise ContractError("application-role proof must be an active block run step")
    role_lines = active_script(role_run, "application-role proof")
    _admin_sql, app_sql = exact_psql_heredocs(role_lines)
    role_assertions = (
        "AND current_user = 'ohc_security_test'",
        "AND NOT rolsuper",
        "AND NOT rolinherit",
        "AND NOT rolbypassrls",
        "AND pg_has_role(current_user, 'ohc_bypassrls', 'MEMBER')",
        "IF NOT (current_setting('row_security') = 'on') THEN",
    )
    for exact, context in (
        ("AND current_user = 'ohc_security_test'", "application-role identity assertion"),
        ("AND NOT rolsuper", "non-superuser assertion"),
        ("AND NOT rolinherit", "NOINHERIT assertion"),
        ("AND NOT rolbypassrls", "NOBYPASSRLS assertion"),
        ("AND pg_has_role(current_user, 'ohc_bypassrls', 'MEMBER')", "explicit SET ROLE membership assertion"),
        ("IF NOT (current_setting('row_security') = 'on') THEN", "row_security assertion"),
    ):
        require_script_line(app_sql, exact, context)
    require_no_control_transfer_before(role_lines, role_assertions, "application-role proof")

    suite_step = named_step(security, "Run PostgreSQL tenant-isolation suite")
    require_unconditional_step(suite_step, "exact multitenancy suite")
    suite_style, suite_run = suite_step.run()
    exact_suite = "cargo test -p server_auth multitenancy_isolation:: -- --nocapture"
    if suite_style != "scalar" or suite_run != exact_suite:
        raise ContractError(f"exact multitenancy suite must be active scalar `run: {exact_suite}`")

    require_active(required, "      - postgres-security", "ci-required dependency")
    require_active(required, "    if: ${{ always() }}", "ci-required always-run policy")
    require_active(
        required,
        "          POSTGRES_SECURITY_RESULT: ${{ needs.postgres-security.result }}",
        "ci-required result propagation",
    )
    required_step = named_step(required, "Check required CI results")
    require_unconditional_step(required_step, "required-result enforcement")
    required_style, required_run = required_step.run()
    if required_style != "block":
        raise ContractError("required-result enforcement must be an active block run step")
    required_lines = active_script(required_run, "required-result enforcement")
    if required_lines != EXPECTED_REQUIRED_RESULT_LINES:
        raise ContractError("required-result step does not match the exact fail-closed script shape")
    enforcement = 'require_success "postgres-security" "$POSTGRES_SECURITY_RESULT"'
    require_no_control_transfer_before(
        required_lines, (enforcement,), "non-markdown required result"
    )
    try:
        markdown_if = required_lines.index('if [[ "$MARKDOWN_ONLY" == "true" ]]; then')
        else_index = required_lines.index("else", markdown_if + 1)
        enforcement_index = required_lines.index(enforcement)
        fi_index = required_lines.index("fi", else_index + 1)
    except ValueError as error:
        raise ContractError("postgres-security enforcement is not in the active non-markdown branch") from error
    if not (markdown_if < else_index < enforcement_index < fi_index):
        raise ContractError("postgres-security enforcement is not in the active non-markdown branch")

    hygiene_step = named_step(changes, "Check tracked artifacts")
    require_unconditional_step(hygiene_step, "check-changes hygiene")
    hygiene_style, hygiene_run = hygiene_step.run()
    if hygiene_style != "block":
        raise ContractError("check-changes hygiene must be an active block run step")
    hygiene_lines = active_script(hygiene_run, "check-changes hygiene")
    if hygiene_lines != EXPECTED_HYGIENE_LINES:
        raise ContractError("check-changes hygiene does not match the exact contract script shape")
    require_no_control_transfer_before(
        hygiene_lines,
        (
            "python3 .github/scripts/check_postgres_security_ci_test.py",
            "python3 .github/scripts/check_postgres_security_ci.py",
        ),
        "always-run postgres security contracts",
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
