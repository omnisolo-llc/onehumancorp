package builtin

import (
	"encoding/json"
	"fmt"
	"time"
	"context"
	"os"
)


// BuiltinAgent handles the core loop for the builtin agent.
type BuiltinAgent struct {
	Client      LLMClient
	Model       string
	System      string
	Tools       []Tool
	MaxTokens   int
	Temperature float32
	MaxTaskBudget int // Maximum output tokens permitted for an entire task
}

// LLMClient is the interface for talking to the LLM backend.
type LLMClient interface {
	Chat(ctx context.Context, req ChatRequest) (ChatResponse, error)
}


// AgentConfig holds the configuration for a builtin agent run.
type AgentConfig struct {
	LLM                LLMClient
	Tools              []Tool
	SystemPrompt       string
	SystemPromptSuffix string
	MaxTurns           int
	MaxTokensPerTurn   int
}

// SpawnTask launches a builtin task and returns state for polling.
func SpawnTask(ctx context.Context, description, prompt, workDir string, cfg AgentConfig) (*TaskState, error) {
	id, err := generateTaskID()
	if err != nil {
		return nil, fmt.Errorf("spawn task: %w", err)
	}

	outputFile := taskOutputPath(id)
	ctx, cancel := context.WithCancel(ctx)

	state := newTaskState(id, description, prompt, workDir, outputFile, "", cancel)

	if cfg.LLM == nil {
		// Just a placeholder until wiring is correct
		cfg.LLM = NewAnthropicClient(os.Getenv("ANTHROPIC_API_KEY"))
	}
	if cfg.Tools == nil {
		cfg.Tools = AllTools()
	}

	agent := &BuiltinAgent{
		Client:    cfg.LLM,
		Model:     "claude-3-7-sonnet-20250219", // Default
		System:    cfg.SystemPrompt + cfg.SystemPromptSuffix,
		Tools:     cfg.Tools,
		MaxTokens: cfg.MaxTokensPerTurn,
	}

	go func() {
		defer cancel()
		state.setStatus(TaskStatusRunning)

		out, err := newTaskOutput(state.OutputFile)
		if err == nil {
			defer out.Close()
			out.AppendString(fmt.Sprintf("=== Agent task started: %s ===\n", state.Description))
		}

		messages := []Message{{
			Role: RoleUser,
			Content: prompt,
		}}

		_, runErr := agent.RunWithCallback(ctx, messages, func(ev AgentEvent) {
			switch ev.Type {
			case AgentEventTypeLLMResponse:
				state.progress.recordTokens(int64(ev.Usage.InputTokens), int64(ev.Usage.OutputTokens))
			case AgentEventTypeToolCall:
				var input map[string]interface{}
				if ev.ToolArgsJSON != "" {
					_ = json.Unmarshal([]byte(ev.ToolArgsJSON), &input)
				}
				state.progress.recordToolUse(ToolActivity{
					ToolName: ev.ToolName,
					Input:    input,
				})
				if out != nil {
					out.AppendString(fmt.Sprintf("\n--- Tool Call: %s ---\nArgs: %s\nResult: %s\n", ev.ToolName, ev.ToolArgsJSON, ev.ToolResult))
				}
			case AgentEventTypeTaskComplete:
				if ev.Content != "" {
					state.result.Store(ev.Content)
					if out != nil {
						out.AppendString(fmt.Sprintf("\n--- Final Answer ---\n%s\n", ev.Content))
					}
				}
			case AgentEventTypeError:
				if out != nil && ev.Error != nil {
					out.AppendString(fmt.Sprintf("\n--- Error ---\n%v\n", ev.Error))
				}
			}
		})

		if runErr != nil {
			state.err.Store(runErr.Error())
			state.setStatus(TaskStatusFailed)
		} else {
			state.setStatus(TaskStatusCompleted)
		}

		now := time.Now()
		state.endAt.Store(now)
	}()

	return state, nil
}
