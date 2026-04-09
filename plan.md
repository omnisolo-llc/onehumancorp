1.  **Create Proactive Documentation Mission**
    -   Since all pending missions are assigned to other agents (e.g., Researcher), and my role is strictly `Scribe`, I will create a new proactive mission file `.agent-task/missions/$(date +%s)_scribe_proactive_hybrid_rag_walkthrough.md`.
    -   This mission will document the newly designed Hybrid MCP RAG Protocol (bridging Standalone SQLite to Cloud PostgreSQL) using Premium aesthetic tokens (Glassmorphism, Outfit/Inter typography).
2.  **Verify Mission Creation**
    -   `cat .agent-task/missions/*_scribe_proactive_hybrid_rag_walkthrough.md`
3.  **Implement Documentation**
    -   Create `docs/walkthroughs/hybrid_mcp_rag_sync.md`.
    -   Write an interactive walkthrough featuring a Mermaid.js diagram illustrating the Last-Write-Wins synchronization, Standalone SQLite degradation, and Cloud Postgres escalation.
    -   Ensure the document is wrapped in the mandatory Glassmorphism markdown wrapper (`backdrop-filter: blur(20px)`, `background: rgba(255, 255, 255, 0.03)`).
4.  **Update Help Portal**
    -   Link the new walkthrough in `docs/walkthroughs/help_portal.md` (or the equivalent documentation index).
5.  **Verify Documentation**
    -   `cat docs/walkthroughs/hybrid_mcp_rag_sync.md`
    -   Verify link injection using `grep "hybrid_mcp_rag_sync" docs/walkthroughs/help_portal.md`
6.  **Pre commit steps**
    -   Complete pre commit steps to ensure proper testing, verification, review, and reflection are done.
7.  **Submit Changes**
    -   Update the mission file status to `DONE`.
    -   Submit PR with the title: `✍️ Scribe: [new documentation feature] Hybrid MCP RAG Sync Visual Walkthrough`.
