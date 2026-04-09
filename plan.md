1.  **Synthesize Research and Create Mission File**:
    *   Create the file `.agent-task/missions/2026-04-09T11-09-10Z.md`.
    *   The file will contain a high-quality mission brief for implementing an "Offline-to-Cloud State Sync for Swarm Memories" (Hybrid MCP RAG Protocol) based on the "Blue Ocean Delta" finding in `RESEARCH_REPORT_HYBRID.md`.
    *   The mission file will include the sections: Title, Problem Statement, Research Report, Design Doc, Implementation Prompt, Priority (P0), and Estimated Scope (Medium). The Research Report must include premium Mermaid.js charts, comparative tables (OHC vs Market), and OHC CSS glassmorphism tokens.

2.  **Verify Mission File Creation**:
    *   Use `list_files` on `.agent-task/missions/` to verify the file exists.
    *   Use `read_file` to ensure its contents are correct and match the required format.

3.  **Insert and Verify DB Mission**:
    *   Execute `sqlite3 .agent-task/swarm.db "INSERT INTO agent_missions (title, description, status) VALUES ('Hybrid MCP RAG Protocol: Bridging Standalone SQLite to Cloud PostgreSQL', 'Implement the Offline-to-Cloud State Sync for Swarm Memories based on mission file 2026-04-09T11-09-10Z.md', 'PENDING');"`
    *   Execute `sqlite3 .agent-task/swarm.db "SELECT * FROM agent_missions ORDER BY id DESC LIMIT 1;"` to verify it was inserted correctly.

4.  **Run Tests and Verification**:
    *   Run `./check_links.sh` at the repository root to verify no broken links were introduced.
    *   Run `bazelisk test //...` to ensure no tests are broken.

5.  **Complete pre-commit steps**:
    *   Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

6.  **Submit the change**:
    *   Submit the code to a new branch `research-hybrid-rag` with a descriptive commit message.
