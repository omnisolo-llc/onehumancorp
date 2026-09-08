"""Prepare service-owned ephemeral state for native live conformance."""

import hashlib
import json
from pathlib import Path
import sqlite3
import sys
import uuid

TENANT = "tenant-live-harness-matrix"
PROJECT = "project-live-harness-matrix"
WORKSPACE = "workspace-live-harness-matrix"


def initialize(root, harnesses):
    root = Path(root)
    scopes, sessions = [], {}
    for harness in harnesses:
        sessions[harness] = {}
        for role in ("writer", "reader"):
            session = str(uuid.uuid4())
            sessions[harness][role] = session
            scopes.append(
                dict(
                    tenant_id=TENANT,
                    project_id=PROJECT,
                    workspace_id=WORKSPACE,
                    session_id=session,
                    task_id=None,
                    attempt_id=None,
                )
            )
    with sqlite3.connect(root / "memory.db") as db:
        db.execute(
            "CREATE VIRTUAL TABLE agent_memory USING fts5(content,tags,created_at UNINDEXED)"
        )
    for name in ("blobs", "tools"):
        (root / name).mkdir(mode=0o700)
    project = hashlib.sha256(
        json.dumps([TENANT, PROJECT], separators=(",", ":")).encode()
    ).hexdigest()
    selected = root / "tools" / project
    selected.mkdir(mode=0o700)
    (selected / "conformance.txt").write_text("OMNISOLO_LOCAL_TOOL_OK")
    config = dict(
        version=1,
        scopes=scopes,
        sqlite_url="sqlite:///services/memory.db",
        blob_root="/services/blobs",
        tool_root="/services/tools",
        allowed_tools=["Read"],
        browser=True,
    )
    (root / "config.json").write_text(json.dumps(config))
    (root / "sessions.json").write_text(json.dumps(sessions))
    return config


if __name__ == "__main__":
    initialize(sys.argv[1], sys.argv[2:])
