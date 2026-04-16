-- Mission created by Oracle Agent
INSERT INTO agent_missions (mission_id, domain, objective, status, priority, reporter)
VALUES (
  'mission_claude_class_harness_interceptor',
  'lib/harness',
  'Implement Claude-Class OS Sandbox Interceptor based on research from leaked Claude Code (Issue #5442). Includes bwrap/sandbox-exec wrappers, bash command validation, env scrubbing, and violation XML feedback loop.',
  'PENDING',
  'P0',
  'oracle'
);
