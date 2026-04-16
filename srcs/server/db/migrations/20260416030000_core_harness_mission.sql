-- +goose Up
INSERT INTO agent_missions (title, problem_statement, research_report, design_doc, implementation_prompt, priority, estimated_scope)
VALUES (
  '[core] Implement Hybrid Agent Harness with AST Validation and OpenTelemetry',
  'OHC''s Agent Harness needs strict, OS-level sub-process isolation (Bubblewrap), granular AST-level command validation to block subshell obfuscation, and OpenTelemetry instrumentation to match and exceed the safety and observability of Claude Code and OpenClaw.',
  'Claude Code uses a SandboxManager with strict FsReadRestrictionConfig/FsWriteRestrictionConfig mappings and deep bwrap namespace sandboxing. It blocks unsafe compound commands and redirection via AST analysis. OpenClaw uses a multi-harness plugin registry (pi-embedded-runner). OHC Strategy: Combine OpenClaw''s flexible registry with Claude''s strict AST validation, adding SPIFFE identity for zero-trust authorization and native OpenTelemetry for observability.',
  '1. Registry: Implement an AgentHarness interface in Go with a registry for different execution strategies. 2. Validation: Integrate tree-sitter-bash to validate all terminal commands before execution, explicitly blocking dangerous built-ins and compound commands that bypass sandboxing. 3. Isolation: Use bwrap for OS-level namespace sandboxing on Linux targets, coupled with a Socat Unix Socket proxy to strictly control network egress. 4. Telemetry: Instrument the harness with OpenTelemetry, emitting ohc_sandbox_violation_total to Prometheus when AST validation or bwrap policies fail.',
  '1. In srcs/server/harness/, create registry.go defining the AgentHarness interface (Execute(ctx, command string) (Result, error)). 2. Create ast_validator.go using tree-sitter to parse and validate bash commands. Block commands with subshells or unsafe redirections. 3. Create bwrap_runner.go that wraps commands in bwrap --unshare-net ... and routes network traffic through a socat proxy. 4. Add OpenTelemetry metrics (meter.Int64Counter("ohc_sandbox_violation_total")) in ast_validator.go and bwrap_runner.go. 5. Write unit tests verifying that ast_validator.go correctly blocks echo "su"$(echo "do") and that bwrap_runner.go correctly formats the bwrap arguments.',
  'P0',
  'Large'
);

-- +goose Down
DELETE FROM agent_missions WHERE title = '[core] Implement Hybrid Agent Harness with AST Validation and OpenTelemetry';
