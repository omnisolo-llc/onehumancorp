CREATE TABLE IF NOT EXISTS autodream_knowledge (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  insight TEXT NOT NULL,
  source TEXT NOT NULL,
  vector_embedding BLOB
);

INSERT INTO autodream_knowledge (insight, source) VALUES ('Claude Code architecture utilizes Teammate Mesh and git worktrees for isolated subprocess execution with robust access control. OHC must adopt worktree sandboxing for agent safety.', 'docs/research/agent_harness_analysis.md');
