#!/usr/bin/env python3
"""Enforce OmniSolo branding and the OmniSolo OneHumanCorp product name.

Compatibility-sensitive protocol, migration, metric, SPIFFE, and Kubernetes
storage identities are deliberately allowlisted below.  The check is kept
dependency-free so it can run in CI before any application build starts.
"""

from __future__ import annotations

import argparse
import subprocess
import re
import os
import sys
from pathlib import Path


MONO_ROOT = Path(__file__).resolve().parents[1]
CLUSTER_ROOT: Path | None = None

SKIP_PARTS = {
    ".git",
    "node_modules",
    "target",
    "test-results",
    ".next",
    ".turbo",
    "next_out",
}
SKIP_NAMES = {
    "Cargo.lock",
    "package-lock.json",
    "pnpm-lock.yaml",
    "yarn.lock",
    "go.sum",
}

FORBIDDEN = (
    ("uppercase OHC brand", re.compile(r"\bOHC\b")),
    (
        "unqualified or legacy product name",
        re.compile(r"\bOne Human Corp\b|(?<!OmniSolo )\bOneHumanCorp\b|\bONE HUMAN CORP\b"),
    ),
    ("legacy environment prefix", re.compile(r"\bOHC_[A-Z0-9_]+")),
    (
        "legacy cloud origin",
        re.compile(
            r"https?://(?:www\.)?(?:ohc\.app|ohc\.store|ohc\.network|api\.onehumancorp\.com|"
            r"app\.onehumancorp\.com|onehumancorp\.com)"
        ),
    ),
    ("legacy image owner", re.compile(r"(?:ghcr\.io|docker\.io)/onehumancorp\b")),
    ("legacy first-party path", re.compile(r"(?:apps/onehumancorp|helm/ohc|src/server/ohc)")),
    ("legacy Helm helper", re.compile(r'include\s+"ohc\.')),
    ("legacy release binary", re.compile(r"\b(?:ohc-builtin-agent|ohc-server|run-ohc)\b")),
    ("legacy Rust type", re.compile(r"\bOHC[A-Z][A-Za-z0-9_]*\b")),
    ("legacy chart name", re.compile(r"^\s*name:\s*ohc\s*$")),
    (
        "legacy browser-owned identifier",
        re.compile(
            r"(?:__Host-)?ohc_(?:session|oidc_state|token(?![A-Za-z0-9_])|"
            r"tenant_id|active_tenant_id|active_terminal_session_id|pos_[A-Za-z0-9_]+|"
            r"offline_[A-Za-z0-9_]+|catalog_[A-Za-z0-9_${}-]+|builder_[A-Za-z0-9_]+|"
            r"dbc_shared|user|wizard_state)|ohc-localization-storage|"
            r"ohc-registration-(?:challenge|ticket)|"
            r"ohc-session\+jwe|x-ohc-tenant-id|ohc://"
        ),
    ),
    (
        "legacy browser tenant default",
        re.compile(
            r"tenantId\s*(?:=|:)\s*[\"']ohc[\"']|"
            r"searchParams\.[^;]+\|\|\s*[\"']ohc[\"']"
        ),
    ),
    (
        "legacy local application path",
        re.compile(r"\.ohc(?=[/\\\s\"'-]|$)"),
    ),
    (
        "legacy application artifact",
        re.compile(
            r"\bohc-(?:android|ios)-|com\.omnisolo\.ohc|ohc\.mobileprovision|"
            r"(?:/tmp/|\$RUNNER_TEMP/)ohc-(?:base-images|kind-images|docker-compose-images|"
            r"macos-signing|ios-signing|signing|bootstrap|e2e|visual-audit)|"
            r"\bohc-(?:backend|standalone|prometheus-agent)\b|"
            r"\bohc-blobs\b|"
            r"disk-cache:\s*onehumancorp|refs/heads/[^\s\"']*onehumancorp"
        ),
    ),
    (
        "legacy local runtime identifier",
        re.compile(
            r"ohc-atomic-writes|/var/run/ohc_proxy\.sock|/tmp/ohc_(?:blobs|test_)|"
            r"\.ohc_hibernation|\bohc_runtime_dir\b|agent@ohc\.local|"
            r"\bohc_modal_stub\b|\bohc_(?:edit|write)_check\b|"
            r"with_name\(\"ohc\"\)|\.openclaw/ohc"
        ),
    ),
)


def relative(path: Path, root: Path) -> str:
    return path.relative_to(root).as_posix()


def is_skipped(path: Path, root: Path) -> bool:
    if path.resolve() == Path(__file__).resolve():
        return True
    rel_parts = path.relative_to(root).parts
    if any(part in SKIP_PARTS for part in rel_parts):
        return True
    if path.name in SKIP_NAMES:
        return True
    rel = "/".join(rel_parts)
    return rel.startswith("docs/superpowers/") or rel.startswith("scratch/")


def compatibility_line(path: Path, root: Path, line: str) -> bool:
    if path.name in {"omnisolo-compatibility.md", "omnisolo_branding_contract_test.rs"}:
        return True
    if path.name == "production_feature_smoke.spec.ts" and ".test(body" in line:
        return True
    # Retain established deployment/test inputs introduced by the live harness
    # integration. These configure credentials or process ownership, not branding.
    retained_inputs = {"OHC_DOCKER_UID", "OHC_DOCKER_GID", "OHC_POSTGRES_PASSWORD_FILE",
                       "OHC_TEST_REDIS_URL", "OHC_TEST_PG_URL", "OHC_DEFAULT_TENANT_ID",
                       "OHC_DATABASE_URL", "OHC_ADMIN_EMAIL", "OHC_ADMIN_PASSWORD", "OHC_ADMIN_ORGANIZATION_ID"}
    if any(name in line for name in retained_inputs):
        return True
    if path.name == ".dockerignore" and line.strip() == ".ohc/":
        return True
    if path.name == "2026-09-08-harness-plan-review.md" and ("OHC_MESH_NODE_ID" in line or "~/.ohc/mesh/node-id" in line):
        return True
    if path.name == "Header.test.tsx" and "not.toMatch" in line:
        return True
    rel = relative(path, root)

    # Historical plans are not product runtime or deployment contracts.
    if rel.startswith("docs/superpowers/"):
        return True

    # Protobuf package paths and generated protocol fixtures are wire identities.
    if root == MONO_ROOT and rel.startswith("src/proto/"):
        return True

    # Migration filenames, SQL identifiers, and existing queue/ledger tables are
    # storage contracts.  The implementation module names around them are new.
    if root == MONO_ROOT and (
        rel.startswith("src/server/migrations/")
        or rel.startswith("src/server/db/migrations/")
    ):
        return True
    if root == MONO_ROOT and (
        "ohc_job_queue" in line
        or "ohc_universal_ledger" in line
        or "spiffe://ohc.app/" in line
    ):
        return True
    if root == MONO_ROOT and "ohc.global" in line and (
        rel in {"src/agents/builtin/auth.rs", "src/server/auth/grpc.rs"}
        or rel == "docs/technical/features/identity-security/federation.md"
    ):
        return True

    # Standalone startup migrates the previous state directory and encrypted
    # database/key filenames in place.  These literals are migration inputs,
    # never new output names.
    if root == MONO_ROOT and rel == "src/server/config/mod.rs" and (
        '".ohc"' in line
        or '"ohc-standalone.db' in line
        or '".ohc_sqlite_key"' in line
        or '".ohc_jwt_secret"' in line
    ):
        return True

    # The public queue endpoint is an existing API contract; its implementation
    # module and all new UI/product copy are OmniSolo-named.
    if root == MONO_ROOT and "/api/v1/ohc_job_queue" in line:
        return True

    # This source-policy test lists legacy browser credential keys solely to
    # reject them from production browser code.
    if root == MONO_ROOT and rel == "src/ui/next/src/lib/auth/browserAuthAuthority.source.test.ts":
        return True

    # The cluster repository contains a pre-existing live verification change
    # with legacy variable fallbacks.  Do not rewrite or fail that file.
    if root == CLUSTER_ROOT and rel == "tests/verify-onehumancorp-live.sh":
        return True

    # These names identify existing namespaces, Vault paths, CNPG/database
    # ownership, and OCI resources.  Changing them would orphan state.
    if root == CLUSTER_ROOT and "onehumancorp" in line:
        stateful_markers = (
            "onehumancorp-onehumancorp",
            "vault/onehumancorp",
            "vault_secrets.onehumancorp",
            "/var/lib/onehumancorp",
            "targetNamespace: onehumancorp",
            "namespace: onehumancorp",
            "name: onehumancorp",
            "onehumancorp-mysql",
            "onehumancorp-heatwave",
            "onehumancorp_heatwave",
        )
        if any(marker in line for marker in stateful_markers):
            return True
        if rel.startswith("ansible/roles/oci_heatwave/"):
            return True

    return False


def scan_paths(root: Path):
    # Respect repository ignore rules: local secrets and generated outputs are
    # neither product source nor reliable branding evidence.
    result = subprocess.run(
        ["git", "-C", str(root), "ls-files", "--cached", "--others", "--exclude-standard", "-z"],
        capture_output=True, check=True,
    )
    for name in dict.fromkeys(result.stdout.split(b"\0")):
        if name:
            yield root / os.fsdecode(name)


def scan_root(root: Path) -> list[str]:
    if not root.exists():
        return [f"missing scan root: {root}"]
    failures: list[str] = []
    for path in scan_paths(root):
        if not path.is_file() or is_skipped(path, root):
            continue
        try:
            text = path.read_text(encoding="utf-8")
        except (OSError, UnicodeDecodeError):
            continue

        for line_number, line in enumerate(text.splitlines(), start=1):
            for label, pattern in FORBIDDEN:
                match = pattern.search(line)
                if match and not compatibility_line(path, root, line):
                    failures.append(
                        f"{root.name}/{relative(path, root)}:{line_number}: "
                        f"{label}"
                    )
                    break
    return failures


def required_contract_failures() -> list[str]:
    required = (
        MONO_ROOT / "src/ui/next/src/lib/branding.ts",
        MONO_ROOT / "src/ui/next/src/e2e/omnisolo-branding.spec.ts",
        MONO_ROOT / "deploy/helm/omnisolo/Chart.yaml",
    )
    if CLUSTER_ROOT is not None:
        required += (
            CLUSTER_ROOT / "apps/omnisolo/Chart.yaml",
            CLUSTER_ROOT / "apps/omnisolo/fluxcd.yaml",
        )
    return [f"missing required OmniSolo contract file: {path}" for path in required if not path.exists()]


def main() -> int:
    global CLUSTER_ROOT
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--cluster-root", type=Path, help="also audit an explicitly selected deployment repository")
    CLUSTER_ROOT = parser.parse_args().cluster_root
    failures = required_contract_failures()
    failures.extend(scan_root(MONO_ROOT))
    if CLUSTER_ROOT is not None:
        failures.extend(scan_root(CLUSTER_ROOT))
    if failures:
        print("OmniSolo branding contract: FAIL", file=sys.stderr)
        print("\n".join(failures), file=sys.stderr)
        return 1

    print("OmniSolo branding contract: PASS")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
