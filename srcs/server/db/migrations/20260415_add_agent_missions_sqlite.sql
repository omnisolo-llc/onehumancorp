CREATE TABLE IF NOT EXISTS agent_missions (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  title TEXT NOT NULL,
  problem_statement TEXT,
  research_report TEXT,
  design_doc TEXT,
  implementation_prompt TEXT,
  priority TEXT,
  estimated_scope TEXT
);

INSERT INTO agent_missions (title, problem_statement, research_report, design_doc, implementation_prompt, priority, estimated_scope)
VALUES (
  '[research] Enhance Agent Harness for OHC using Claude-Class Isolation',
  'OHC needs to achieve parity with Claude-Class Agent Harnesses in isolation, execution safety, and orchestration coordination.',
  'Based on an analysis of Claude Code, their harness supports complex subprocess execution, dynamic permissions, temporary git worktrees, and robust error handling. OHCs srcs/server/agents/provider.go currently lacks deep integration of isolated worktrees and robust background tool coordination compared to Claude.',
  'Implement an isolation abstraction layer supporting worktrees, process sandboxes, and specific telemetry tracking per subagent invocation. Utilize the Teammate Mesh API to sync subagent statuses and errors dynamically.',
  'Implement the IsolationStrategy interface for Agent Harness. Update Provider to support RunInIsolation(worktree string) logic and pipe output streams directly to Redis Pub/Sub.',
  'P1',
  'Medium'
);
