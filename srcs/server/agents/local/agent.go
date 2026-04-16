package local

import (
	"context"
	"fmt"
	"log/slog"
	"os"
	"strconv"
	"strings"
	"time"
)

const (
	// maxAgentTurns is the upper bound on LLM turns per task execution.
	// Mirrors CC-Source's AGENT_MAX_TURNS logic; helps prevent infinite loops.
	maxAgentTurns = 50

	// systemPrompt is the default system prompt injected into every local agent.
	systemPrompt = `You are a capable software engineering assistant with access to a set of tools.
Your task is to help users by analysing their request, breaking it down into steps, and
executing those steps using the available tools. Think step by step.

Guidelines:
- Always read files before editing them.
- Run bash commands to explore the codebase when needed.
- Use grep/glob to find relevant files.
- Prefer targeted edits over full rewrites.
- When you are done, provide a clear summary of what you did.
- If you encounter errors, try to understand them and fix them.
- Do not ask clarifying questions; make reasonable assumptions and proceed.`
)

// AgentConfig holds the configuration for a local agent run.
type AgentConfig struct {
	// LLM is the language model client to use.  If nil, defaultLLMClient() is used.
	LLM LLMClient

	// Tools is the list of tools available to the agent.  If nil, DefaultTools() is used.
	Tools []Tool

	// SystemPrompt overrides the default system prompt.
	SystemPrompt string

	// SystemPromptSuffix is appended to the effective system prompt.
	// Used to inject communication-mode instructions (e.g. caveman mode) into sub-agents.
	SystemPromptSuffix string

	// MaxTurns is the maximum number of LLM turns.  Defaults to maxAgentTurns.
	MaxTurns int

	// MaxTokensPerTurn is the max_tokens per API call.  Defaults to 8192.
	MaxTokensPerTurn int
}

// Agent is a single agentic session that drives the LLM-tool loop for one task.
type Agent struct {
	cfg   AgentConfig
	state *TaskState
	out   *taskOutput
}

// NewAgent creates an Agent for the given task state.
func NewAgent(state *TaskState, cfg AgentConfig) *Agent {
	if cfg.LLM == nil {
		// Attempt to wrap with cache if environment DB is available (e.g. from state context)
		// Since NewAgent doesn't take DB directly, it will just use default base LLM.
		// However, callers that want caching should provide it in cfg.LLM.
		cfg.LLM = defaultLLMClient()
	}
	if cfg.Tools == nil {
		cfg.Tools = DefaultTools()
	}
	if cfg.SystemPrompt == "" {
		cfg.SystemPrompt = systemPrompt
	}
	// Append the suffix (e.g. caveman communication mode instructions).
	if cfg.SystemPromptSuffix != "" {
		cfg.SystemPrompt += cfg.SystemPromptSuffix
	}
	if cfg.MaxTurns <= 0 {
		cfg.MaxTurns = maxAgentTurns
		if s := os.Getenv("OHC_LOCAL_AGENT_MAX_TURNS"); s != "" {
			if n, err := strconv.Atoi(s); err == nil && n > 0 {
				cfg.MaxTurns = n
			}
		}
	}
	if cfg.MaxTokensPerTurn <= 0 {
		cfg.MaxTokensPerTurn = 4096
	}
	return &Agent{cfg: cfg, state: state}
}

// Run drives the agentic loop until the task completes, fails, or is killed.
// It is designed to be called in its own goroutine.
func (a *Agent) Run(ctx context.Context) {
	ts := a.state
	ts.setStatus(TaskStatusRunning)

	// Open the output file.
	out, err := newTaskOutput(ts.OutputFile)
	if err != nil {
		slog.Error("local agent: failed to open output file", "task", ts.ID, "err", err)
		ts.err.Store(err.Error())
		ts.setStatus(TaskStatusFailed)
		return
	}
	a.out = out
	defer out.Close()

	_ = out.AppendString(fmt.Sprintf("=== Agent task started: %s ===\n", ts.Description))
	_ = out.AppendString(fmt.Sprintf("Prompt: %s\n\n", ts.Prompt))

	toolDefs := make([]ToolDefinition, len(a.cfg.Tools))
	for i, t := range a.cfg.Tools {
		toolDefs[i] = t.Definition()
	}

	// Build the initial conversation.
	messages := []ConversationMessage{
		{
			Role:    "user",
			Content: []ContentPart{{Type: "text", Text: ts.Prompt}},
		},
	}

	result, runErr := a.loop(ctx, messages, toolDefs)

	if ctx.Err() != nil || ts.Status() == TaskStatusKilled {
		ts.setStatus(TaskStatusKilled)
		_ = out.AppendString("\n=== Task killed ===\n")
		return
	}

	if runErr != nil {
		ts.err.Store(runErr.Error())
		ts.setStatus(TaskStatusFailed)
		_ = out.AppendString(fmt.Sprintf("\n=== Task FAILED: %v ===\n", runErr))
		return
	}

	ts.result.Store(result)
	ts.setStatus(TaskStatusCompleted)
	_ = out.AppendString(fmt.Sprintf("\n=== Task completed ===\nResult: %s\n", result))
}

// loop runs the main LLM → tool-execution → LLM cycle.
func (a *Agent) loop(ctx context.Context, messages []ConversationMessage, toolDefs []ToolDefinition) (string, error) {
	ts := a.state
	maxTurns := a.cfg.MaxTurns
	lastText := ""

	for turn := 0; turn < maxTurns; turn++ {
		if ctx.Err() != nil || ts.Status() == TaskStatusKilled {
			return "", ctx.Err()
		}

		slog.Debug("local agent: LLM turn", "task", ts.ID, "turn", turn+1)

		req := CompletionRequest{
			SystemPrompt: a.cfg.SystemPrompt,
			Messages:     messages,
			Tools:        toolDefs,
			MaxTokens:    a.cfg.MaxTokensPerTurn,
		}

		resp, err := a.cfg.LLM.Complete(ctx, req)
		if err != nil {
			return "", fmt.Errorf("LLM turn %d: %w", turn+1, err)
		}

		// Track token usage.
		ts.progress.recordTokens(resp.InputTokens, resp.OutputTokens)

		// Log the assistant's text output.
		if resp.Text != "" {
			lastText = resp.Text
			_ = a.out.AppendString("\n[assistant] " + resp.Text + "\n")
		}

		// If the model is done (no tool calls), we're finished.
		if resp.StopReason == "end_turn" || len(resp.ToolUses) == 0 {
			return lastText, nil
		}

		// Build the assistant message (with tool_use content blocks).
		assistantParts := []ContentPart{}
		if resp.Text != "" {
			assistantParts = append(assistantParts, ContentPart{Type: "text", Text: resp.Text})
		}
		for _, tu := range resp.ToolUses {
			assistantParts = append(assistantParts, ContentPart{
				Type:      "tool_use",
				ToolUseID: tu.ID,
				ToolName:  tu.Name,
				ToolInput: tu.Input,
			})
		}
		messages = append(messages, ConversationMessage{Role: "assistant", Content: assistantParts})

		// Execute all tool calls and collect results.
		toolResultParts := []ContentPart{}
		for _, tu := range resp.ToolUses {
			result, toolErr := a.executeTool(ctx, tu)

			activity := ToolActivity{
				ToolName: tu.Name,
				Input:    tu.Input,
			}
			ts.progress.recordToolUse(activity)

			_ = a.out.AppendString(fmt.Sprintf("\n[tool: %s]\n%s\n", tu.Name, describeInput(tu.Input)))
			if toolErr != nil {
				_ = a.out.AppendString(fmt.Sprintf("[tool error] %v\n", toolErr))
				toolResultParts = append(toolResultParts, ContentPart{
					Type:               "tool_result",
					ResultForToolUseID: tu.ID,
					ResultContent:      fmt.Sprintf("Error: %v", toolErr),
					IsError:            true,
				})
			} else {
				_ = a.out.AppendString(result + "\n")
				toolResultParts = append(toolResultParts, ContentPart{
					Type:               "tool_result",
					ResultForToolUseID: tu.ID,
					ResultContent:      result,
				})
			}
		}

		// Append the tool results as a user message.
		messages = append(messages, ConversationMessage{Role: "user", Content: toolResultParts})
	}

	return lastText, fmt.Errorf("agent exceeded max turns (%d)", maxTurns)
}

// executeTool finds and runs the named tool.
func (a *Agent) executeTool(ctx context.Context, tu ToolUseRequest) (string, error) {
	for _, t := range a.cfg.Tools {
		if t.Definition().Name == tu.Name {
			workDir := a.state.WorkDir
			if workDir == "" {
				workDir = "."
			}
			return t.Execute(ctx, workDir, tu.Input)
		}
	}
	return "", fmt.Errorf("unknown tool: %s", tu.Name)
}

// describeInput returns a short, human-readable summary of a tool's input.
func describeInput(input map[string]interface{}) string {
	if input == nil {
		return ""
	}
	var parts []string
	for k, v := range input {
		switch vs := v.(type) {
		case string:
			truncated := vs
			if len(truncated) > 80 {
				truncated = truncated[:80] + "…"
			}
			parts = append(parts, fmt.Sprintf("%s=%q", k, truncated))
		default:
			parts = append(parts, fmt.Sprintf("%s=%v", k, v))
		}
	}
	return strings.Join(parts, " ")
}

// SpawnTask creates a new TaskState, launches the agent loop in a goroutine,
// and returns the TaskState immediately for progress polling.
func SpawnTask(ctx context.Context, description, prompt, workDir string, cfg AgentConfig) (*TaskState, error) {
	id, err := generateTaskID()
	if err != nil {
		return nil, fmt.Errorf("spawn task: %w", err)
	}

	outputFile := taskOutputPath(id)
	ctx, cancel := context.WithCancel(ctx)

	state := newTaskState(id, description, prompt, workDir, outputFile, "", cancel)
	agent := NewAgent(state, cfg)

	go func() {
		defer cancel() // ensure the context is always cancelled when the goroutine exits
		agent.Run(ctx)
		now := time.Now()
		state.endAt.Store(now)
	}()

	return state, nil
}
