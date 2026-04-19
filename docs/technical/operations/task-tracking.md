# Task Tracking Policy

GitHub issues are the only supported task tracker for this repository.

## Rules

- Open work belongs in GitHub issues.
- Documentation, code, and release work should reference issue numbers when applicable.
- Local `.agent-task` mission files are retired and should not be recreated.
- `OHC_MISSIONS_DIR` is import-only and exists solely to support controlled migration or ingestion workflows.

## Recommended Labels

- `task`
- `epic`
- `docs`
- `backend`
- `frontend`
- `infra`
- `research`

## Terminal Workflow

Use `deploy/scripts/ohc-swarm-status.sh` to query repository issues when `gh` is installed and authenticated.

## Documentation Gate

Before opening implementation work, link the relevant design doc, CUJ, and test plan when they exist.
