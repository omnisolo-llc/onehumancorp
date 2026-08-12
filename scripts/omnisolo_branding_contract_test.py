#!/usr/bin/env python3
"""Reject accidental reintroduction of first-party OHC branding.

Compatibility-sensitive protocol, migration, metric, SPIFFE, and Kubernetes
storage identities are deliberately allowlisted below.  The check is kept
dependency-free so it can run in CI before any application build starts.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path


MONO_ROOT = Path(__file__).resolve().parents[1]
CLUSTER_ROOT = Path("/home/kevin/myk3s")

SKIP_PARTS = {
    ".git",
    "node_modules",
    "target",
    "test-results",
    ".next",
    ".turbo",
}
SKIP_NAMES = {
    "Cargo.lock",
    "package-lock.json",
    "pnpm-lock.yaml",
    "yarn.lock",
}

FORBIDDEN = (
    ("uppercase OHC brand", re.compile(r"\bOHC\b")),
    ("legacy company name", re.compile(r"\bOne Human Corp\b|\bOneHumanCorp\b")),
    ("legacy environment prefix", re.compile(r"\bOHC_[A-Z0-9_]+")),
    (
        "legacy cloud origin",
        re.compile(
            r"https?://(?:www\.)?(?:ohc\.app|ohc\.store|api\.onehumancorp\.com|"
            r"app\.onehumancorp\.com|onehumancorp\.com)"
        ),
    ),
    ("legacy image owner", re.compile(r"(?:ghcr\.io|docker\.io)/onehumancorp\b")),
    ("legacy first-party path", re.compile(r"(?:apps/onehumancorp|helm/ohc|src/server/ohc)")),
    ("legacy Helm helper", re.compile(r'include\s+"ohc\.')),
    ("legacy release binary", re.compile(r"\b(?:ohc-builtin-agent|ohc-server|run-ohc)\b")),
    ("legacy chart name", re.compile(r"^\s*name:\s*ohc\s*$")),
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

    # The public queue endpoint is an existing API contract; its implementation
    # module and all new UI/product copy are OmniSolo-named.
    if root == MONO_ROOT and "/api/v1/ohc_job_queue" in line:
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


def scan_root(root: Path) -> list[str]:
    if not root.exists():
        return [f"missing scan root: {root}"]

    failures: list[str] = []
    for path in root.rglob("*"):
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
                        f"{label}: {line.strip()}"
                    )
                    break
    return failures


def required_contract_failures() -> list[str]:
    required = (
        MONO_ROOT / "src/ui/next/src/lib/branding.ts",
        MONO_ROOT / "src/ui/next/src/e2e/omnisolo-branding.spec.ts",
        MONO_ROOT / "deploy/helm/omnisolo/Chart.yaml",
        CLUSTER_ROOT / "apps/omnisolo/Chart.yaml",
        CLUSTER_ROOT / "apps/omnisolo/fluxcd.yaml",
    )
    return [f"missing required OmniSolo contract file: {path}" for path in required if not path.exists()]


def main() -> int:
    failures = required_contract_failures()
    failures.extend(scan_root(MONO_ROOT))
    failures.extend(scan_root(CLUSTER_ROOT))
    if failures:
        print("OmniSolo branding contract: FAIL", file=sys.stderr)
        print("\n".join(failures), file=sys.stderr)
        return 1

    print("OmniSolo branding contract: PASS")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
