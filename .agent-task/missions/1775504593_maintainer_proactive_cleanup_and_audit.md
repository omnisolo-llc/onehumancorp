status: DONE
---
Title: Resource Cleanup and Fast-and-Loose Code Audit
Problem Statement: The standalone wrapper was running unconditional cleanup, deleting temporary files even when running in cloud mode. Additionally, the DBInspector MCP had insecure multi-tenant code (`search_path` parsing) lacking proper regex validation.
Research Report:
I reviewed `standalone_ohc.sh` and identified that `find "${STATE_DIR}" -name "*.tmp" -type f -delete 2>/dev/null || true` was run indiscriminately.
I reviewed `mcp.go` inside `dbinspector` and identified that the `sanitizedOrgID` logic only escaped quotes, lacking a strict schema validation check, making it vulnerable or insecure in multi-tenant cloud scenarios.
Design Doc:
1. Update `standalone_ohc.sh` to wrap tmp file deletion with `if [[ "${OHC_STANDALONE:-}" == "true" ]]; then`.
2. Add a comprehensive Bash test in `standalone_ohc_test.sh` that mocks the Go server, and verify cleanup works properly.
3. Update `mcp.go` in `dbinspector` to strictly validate `OrganizationID` using the regex `^[a-zA-Z0-9_\-]+$` before applying it to the PostgreSQL `search_path`.
Implementation Prompt:
Update the standalone cleanup script to be mode-aware, harden the dbinspector MCP multi-tenant query logic against injection, and create unit tests.
Priority: High
Estimated Scope: Small
