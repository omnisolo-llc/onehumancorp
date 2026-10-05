#!/usr/bin/env python3
"""PyYAML structure and OHC policy for the required PostgreSQL/RLS CI lane."""

from dataclasses import dataclass
from pathlib import Path
import re
import sys


class ContractError(AssertionError):
    pass


EXPECTED_REQUIRED_RESULT_LINES = (
    "set -euo pipefail",
    'echo "dependency-audit: ${DEPENDENCY_AUDIT_RESULT}"',
    'echo "check-changes: ${CHECK_CHANGES_RESULT}"',
    'echo "native-build: ${NATIVE_BUILD_RESULT}"',
    'echo "native-test: ${NATIVE_TEST_RESULT}"',
    'echo "native-e2e: ${NATIVE_E2E_RESULT}"',
    'echo "native-click-coverage: ${NATIVE_CLICK_COVERAGE_RESULT}"',
    'echo "native-web: ${NATIVE_WEB_RESULT}"',
    'echo "native-node: ${NATIVE_NODE_RESULT}"',
    'echo "native-images: ${NATIVE_IMAGES_RESULT}"',
    'echo "native-desktop: ${NATIVE_DESKTOP_RESULT}"',
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
    'if [[ "$MARKDOWN_ONLY" != "true" || "$EVENT_NAME" == "schedule" || "$EVENT_NAME" == "workflow_dispatch" ]]; then',
    'require_success "native-build" "$NATIVE_BUILD_RESULT"',
    "else",
    'allow_success_or_skipped "native-build" "$NATIVE_BUILD_RESULT"',
    "fi",
    'if [[ "$MARKDOWN_ONLY" == "true" ]]; then',
    'allow_success_or_skipped "dependency-audit" "$DEPENDENCY_AUDIT_RESULT"',
    'allow_success_or_skipped "native-test" "$NATIVE_TEST_RESULT"',
    'allow_success_or_skipped "native-e2e" "$NATIVE_E2E_RESULT"',
    'allow_success_or_skipped "native-click-coverage" "$NATIVE_CLICK_COVERAGE_RESULT"',
    'allow_success_or_skipped "native-web" "$NATIVE_WEB_RESULT"',
    'allow_success_or_skipped "native-node" "$NATIVE_NODE_RESULT"',
    'allow_success_or_skipped "native-images" "$NATIVE_IMAGES_RESULT"',
    'allow_success_or_skipped "native-desktop" "$NATIVE_DESKTOP_RESULT"',
    'allow_success_or_skipped "kind-e2e" "$KIND_E2E_RESULT"',
    'allow_success_or_skipped "docker-e2e" "$DOCKER_E2E_RESULT"',
    'allow_success_or_skipped "postgres-security" "$POSTGRES_SECURITY_RESULT"',
    "else",
    'require_success "dependency-audit" "$DEPENDENCY_AUDIT_RESULT"',
    'require_success "native-test" "$NATIVE_TEST_RESULT"',
    'require_success "native-e2e" "$NATIVE_E2E_RESULT"',
    'require_success "native-click-coverage" "$NATIVE_CLICK_COVERAGE_RESULT"',
    'require_success "native-web" "$NATIVE_WEB_RESULT"',
    'require_success "native-node" "$NATIVE_NODE_RESULT"',
    'require_success "native-images" "$NATIVE_IMAGES_RESULT"',
    'require_success "native-desktop" "$NATIVE_DESKTOP_RESULT"',
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
EXPECTED_PYYAML_BOOTSTRAP_LINES = (
    "sudo apt-get update && sudo apt-get install -y python3-yaml",
)
EXPECTED_POSTGRES_TOOLCHAIN_LINES = (
    "sudo apt-get update",
    "sudo apt-get install -y --no-install-recommends postgresql-client protobuf-compiler redis-server",
)

ADMIN_PSQL_HEREDOC = 'psql "$OMNISOLO_POSTGRES_ADMIN_URL" --set ON_ERROR_STOP=1 <<\'SQL\''
APP_PSQL_HEREDOC = 'psql "$OMNISOLO_DATABASE_URL" --set ON_ERROR_STOP=1 <<\'SQL\''
EXPECTED_WORKFLOW_DEFAULTS = {"run": {"shell": "bash"}}
EXPECTED_WORKFLOW_ENV = {"FORCE_JAVASCRIPT_ACTIONS_TO_NODE24": "true"}
EXPECTED_POSTGRES_JOB_KEYS = (
    "name",
    "needs",
    "if",
    "runs-on",
    "timeout-minutes",
    "services",
    "env",
    "steps",
)
EXPECTED_REQUIRED_JOB_KEYS = ("name", "needs", "if", "runs-on", "timeout-minutes", "permissions", "steps")
EXPECTED_CHANGES_JOB_KEYS = ("name", "runs-on", "timeout-minutes", "outputs", "steps")
EXPECTED_POSTGRES_ENV = {
    "OMNISOLO_REQUIRE_POSTGRES_TESTS": "1",
    "OMNISOLO_POSTGRES_ADMIN_URL": "postgresql://postgres:postgres@127.0.0.1:5432/ohc_security",
    "OMNISOLO_DATABASE_URL": "postgresql://ohc_security_test:ohc_security_test@127.0.0.1:5432/ohc_security",
}
EXPECTED_REQUIRED_ENV = {
    "EVENT_NAME": "${{ github.event_name }}",
    "MARKDOWN_ONLY": "${{ needs.check-changes.outputs.markdown-only }}",
    **{
        job.upper().replace("-", "_") + "_RESULT": "${{ needs." + job + ".result }}"
        for job in (
            "dependency-audit", "check-changes", "native-build", "native-test", "native-e2e",
            "native-click-coverage", "native-web", "native-node", "native-images",
            "native-desktop", "kind-e2e", "docker-e2e", "postgres-security",
        )
    },
}


def mapping(node, context: str) -> dict:
    if node is None or node.id != "mapping":
        raise ContractError(f"{context}: expected a YAML mapping")
    return {key.value: value for key, value in node.value}


def field(node, name: str):
    values = mapping(node, name)
    if name not in values:
        raise ContractError(f"missing active field {name!r} at line {node.start_mark.line + 1}")
    return values[name]


def plain(node):
    """Compare parsed structure without YAML 1.1 coercion of Actions keys/scalars."""
    if node.id == "mapping":
        return {key.value: plain(value) for key, value in node.value}
    if node.id == "sequence":
        return [plain(value) for value in node.value]
    return node.value


def require_value(node, expected, context: str) -> None:
    if plain(node) != expected:
        raise ContractError(f"{context}: expected {expected!r} at line {node.start_mark.line + 1}")


def require_exact_keys(node, expected: tuple[str, ...], context: str) -> None:
    actual = tuple(mapping(node, context))
    if actual != expected:
        raise ContractError(f"{context}: expected keys {expected!r}, found {actual!r}")


@dataclass(frozen=True)
class Step:
    name: str
    node: object

    def run(self) -> tuple[str, str]:
        node = field(self.node, "run")
        if node.id != "scalar":
            raise ContractError(f"step {self.name!r} run must be a scalar")
        # Folded scalars change shell line boundaries; only literal blocks carry
        # the exact protected script, regardless of YAML chomping indicators.
        return ("block" if node.style == "|" else "scalar"), node.value


def steps(job) -> tuple[Step, ...]:
    node = field(job, "steps")
    if node.id != "sequence":
        raise ContractError("job steps must be a sequence")
    found = []
    for item in node.value:
        values = mapping(item, "step")
        if "name" in values:
            name = values["name"]
            if name.id != "scalar":
                raise ContractError("step name must be a scalar")
            found.append(Step(name.value, item))
    return tuple(found)


def named_step(job, name: str) -> Step:
    matches = [step for step in steps(job) if step.name == name]
    if len(matches) != 1:
        raise ContractError(f"expected exactly one active step named {name!r}, found {len(matches)}")
    return matches[0]


def require_non_ignorable_job(job, context: str) -> None:
    values = mapping(job, context)
    for key in ("continue-on-error", "defaults"):
        if key in values:
            raise ContractError(f"{context}: job-level {key} is forbidden")


def require_unconditional_step(step: Step, context: str) -> None:
    values = mapping(step.node, context)
    for key in ("if", "continue-on-error", "shell"):
        if key in values:
            raise ContractError(f"{context}: step-level {key} is forbidden")


def require_exact_step_keys(step: Step, expected: tuple[str, ...], context: str) -> None:
    require_exact_keys(step.node, ("name", *expected), context)


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


def validate_yaml(path: Path):
    try:
        import yaml
    except ImportError as error:
        raise ContractError("PyYAML is required; install the native python3-yaml prerequisite") from error

    source = path.read_text(encoding="utf-8")
    try:
        # Explicit key and alias syntax are outside the supported workflow shape.
        # Inspect parser tokens, not source indentation or guessed key spelling.
        for token in yaml.scan(source):
            if isinstance(token, yaml.tokens.AliasToken):
                raise ContractError(f"YAML alias is forbidden at line {token.start_mark.line + 1}")
            if isinstance(token, yaml.tokens.KeyToken) and source[token.start_mark.index:token.end_mark.index] == "?":
                raise ContractError(f"explicit YAML key is forbidden at line {token.start_mark.line + 1}")
        root = yaml.compose(source, Loader=yaml.SafeLoader)
    except yaml.YAMLError as error:
        raise ContractError(f"workflow is not valid YAML: {error}") from error

    allowed_tags = {"tag:yaml.org,2002:" + kind for kind in (
        "map", "seq", "str", "null", "bool", "int", "float", "timestamp",
    )}

    def validate(node):
        if node.tag not in allowed_tags:
            label = "merge key" if node.tag == "tag:yaml.org,2002:merge" else "tag"
            raise ContractError(f"unsupported YAML {label} {node.tag!r} at line {node.start_mark.line + 1}")
        if node.id == "mapping":
            seen = set()
            for key, value in node.value:
                if key.id != "scalar":
                    raise ContractError(f"non-scalar YAML mapping key at line {key.start_mark.line + 1}")
                validate(key)
                if key.value in seen:
                    raise ContractError(f"duplicate YAML mapping key {key.value!r} at line {key.start_mark.line + 1}")
                seen.add(key.value)
                validate(value)
        elif node.id == "sequence":
            for value in node.value:
                validate(value)

    if root is None:
        raise ContractError("workflow is empty")
    validate(root)
    mapping(root, "workflow")
    return root


def check_workflow(path: Path) -> None:
    workflow = validate_yaml(path)
    require_value(field(workflow, "env"), EXPECTED_WORKFLOW_ENV, "workflow env allowlist")
    require_value(field(workflow, "defaults"), EXPECTED_WORKFLOW_DEFAULTS, "workflow shell defaults")
    jobs = field(workflow, "jobs")
    security = field(jobs, "postgres-security")
    required = field(jobs, "ci-required")
    changes = field(jobs, "check-changes")
    require_exact_keys(security, EXPECTED_POSTGRES_JOB_KEYS, "postgres-security job")
    require_exact_keys(required, EXPECTED_REQUIRED_JOB_KEYS, "ci-required job")
    require_value(field(required, "permissions"), {"contents": "read", "actions": "read"}, "ci-required read-only permissions")
    require_exact_keys(changes, EXPECTED_CHANGES_JOB_KEYS, "check-changes job")
    require_non_ignorable_job(security, "postgres-security")
    require_non_ignorable_job(required, "ci-required")
    require_non_ignorable_job(changes, "check-changes")
    require_value(field(changes, "runs-on"), "ubuntu-latest", "reliable check-changes runner")
    needs = plain(field(security, "needs"))
    if not isinstance(needs, list) or "check-changes" not in needs:
        raise ContractError("postgres-security must depend on check-changes")
    require_value(field(security, "if"), "${{ needs.check-changes.outputs.markdown-only == 'false' }}", "markdown-only skip policy")
    postgres = field(field(security, "services"), "postgres")
    require_value(field(postgres, "image"), "pgvector/pgvector:pg16", "pgvector image")
    require_value(field(security, "env"), EXPECTED_POSTGRES_ENV, "postgres-security env allowlist")
    health = field(postgres, "options")
    if health.id != "scalar" or "pg_isready" not in health.value:
        raise ContractError("service health check: missing active pg_isready configuration")

    toolchain_step = named_step(security, "Install PostgreSQL client")
    require_exact_step_keys(toolchain_step, ("run",), "PostgreSQL test toolchain")
    require_unconditional_step(toolchain_step, "PostgreSQL test toolchain")
    toolchain_style, toolchain_run = toolchain_step.run()
    if toolchain_style != "block":
        raise ContractError("PostgreSQL test toolchain must be an active block run step")
    if active_script(toolchain_run, "PostgreSQL test toolchain") != EXPECTED_POSTGRES_TOOLCHAIN_LINES:
        raise ContractError("PostgreSQL test toolchain does not match the exact install commands")

    role_step = named_step(security, "Provision and verify non-superuser application role")
    require_exact_step_keys(role_step, ("run",), "application-role proof")
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
    require_exact_step_keys(suite_step, ("run",), "exact multitenancy suite")
    require_unconditional_step(suite_step, "exact multitenancy suite")
    suite_style, suite_run = suite_step.run()
    exact_suite = "cargo test --locked -p server_auth multitenancy_isolation:: -- --nocapture"
    quoted_suite = f'"{exact_suite}"'
    if suite_style != "scalar" or suite_run != exact_suite or field(suite_step.node, "run").style != '"':
        raise ContractError(f"exact multitenancy suite must be active quoted scalar `run: {quoted_suite}`")

    required_needs = plain(field(required, "needs"))
    for dependency in ("dependency-audit", "native-node", "native-images", "native-build", "postgres-security"):
        if not isinstance(required_needs, list) or dependency not in required_needs:
            raise ContractError(f"ci-required is missing dependency {dependency!r}")
    require_value(field(required, "if"), "${{ always() }}", "ci-required always-run policy")
    required_step = named_step(required, "Check required CI results")
    require_exact_step_keys(required_step, ("env", "run"), "required-result enforcement")
    require_value(field(required_step.node, "env"), EXPECTED_REQUIRED_ENV, "required-result env allowlist")
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

    pyyaml_step = named_step(changes, "Install PyYAML")
    require_exact_step_keys(pyyaml_step, ("run",), "PyYAML bootstrap")
    require_unconditional_step(pyyaml_step, "PyYAML bootstrap")
    pyyaml_style, pyyaml_run = pyyaml_step.run()
    if pyyaml_style != "block":
        raise ContractError("PyYAML bootstrap must be an active block run step")
    if active_script(pyyaml_run, "PyYAML bootstrap") != EXPECTED_PYYAML_BOOTSTRAP_LINES:
        raise ContractError("PyYAML bootstrap does not match the exact install command")

    hygiene_step = named_step(changes, "Check tracked artifacts")
    if pyyaml_step.node.start_mark.index >= hygiene_step.node.start_mark.index:
        raise ContractError("PyYAML bootstrap must run before tracked-artifact checks")
    require_exact_step_keys(hygiene_step, ("run",), "check-changes hygiene")
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
