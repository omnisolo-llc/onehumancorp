package builtin

import (
	"context"
	"fmt"
	"log/slog"
	"math"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/agents/local"
)

const (
	maxAgentTurns    = 50
	defaultMaxTokens = 8192
	maxContextTokens = 150000 // A realistic upper bound to prevent context overflow

	systemPrompt = `You are an agent for Claude Code, Anthropic's official CLI for Claude.
Given the user's message, use the tools available to complete the task fully.
Complete the task — don't gold-plate, but don't leave it half-done.

Important guidelines:
- Work autonomously without asking for confirmation
- Use bash, file_read, file_write, file_edit, grep, glob, and other tools freely
- When writing code, ensure it is complete and correct
- Read existing code before modifying it
- Run tests if available to verify your changes
- Provide a clear summary when done`
)

// AgentConfig configures the Builtin agent.
type AgentConfig struct {
	SystemPrompt string
	MaxTurns     int
	MaxTokens    int
	LLM          local.LLMClient
	Tools        []local.Tool
}

// Agent represents the running Builtin agent loop.
type Agent struct {
	cfg AgentConfig
}

// NewAgent creates a new Builtin agent.
func NewAgent(cfg AgentConfig) *Agent {
	if cfg.SystemPrompt == "" {
		cfg.SystemPrompt = systemPrompt
	}
	if cfg.MaxTurns <= 0 {
		cfg.MaxTurns = maxAgentTurns
	}
	if cfg.MaxTokens <= 0 {
		cfg.MaxTokens = defaultMaxTokens
	}
	if cfg.Tools == nil {
		cfg.Tools = local.DefaultTools()
	}
	return &Agent{cfg: cfg}
}

// Run executes the agentic loop until completion or error.
func (a *Agent) Run(ctx context.Context, task string) (string, error) {
	var toolDefs []local.ToolDefinition
	for _, t := range a.cfg.Tools {
		toolDefs = append(toolDefs, t.Definition())
	}

	messages := []local.ConversationMessage{
		{
			Role: "user",
			Content: []local.ContentPart{
				{Type: "text", Text: task},
			},
		},
	}

	var finalText string
	var totalInputTokens, totalOutputTokens int64

	for turn := 0; turn < a.cfg.MaxTurns; turn++ {
		if err := ctx.Err(); err != nil {
			return finalText, err
		}

		// Implement Token Budget Management by truncating oldest non-system messages if needed.
		// Note: A real tokenizer should be used, but here we estimate conservatively.
		messages = a.truncateContextIfNeeded(messages, maxContextTokens)

		req := local.CompletionRequest{
			SystemPrompt: a.cfg.SystemPrompt,
			Messages:     messages,
			Tools:        toolDefs,
			MaxTokens:    a.cfg.MaxTokens,
		}

		slog.Info("agent turn", "turn", turn, "messages", len(messages))

		// Implement Retries with exponential backoff and graceful degradation
		resp, err := a.executeWithRetry(ctx, req)
		if err != nil {
			// Graceful degradation: if LLM fails repeatedly, return the last known text and the error.
			return finalText, fmt.Errorf("LLM error after retries on turn %d: %w", turn, err)
		}

		totalInputTokens += resp.InputTokens
		totalOutputTokens += resp.OutputTokens

		slog.Info("LLM response",
			"turn", turn,
			"stop_reason", resp.StopReason,
			"tool_count", len(resp.ToolUses),
			"input_tokens", resp.InputTokens,
			"output_tokens", resp.OutputTokens,
		)

		if resp.Text != "" {
			finalText = resp.Text
		}

		var assistantContent []local.ContentPart
		if resp.Text != "" {
			assistantContent = append(assistantContent, local.ContentPart{
				Type: "text",
				Text: resp.Text,
			})
		}

		for _, tu := range resp.ToolUses {
			assistantContent = append(assistantContent, local.ContentPart{
				Type:      "tool_use",
				ToolUseID: tu.ID,
				ToolName:  tu.Name,
				ToolInput: tu.Input,
			})
		}

		messages = append(messages, local.ConversationMessage{
			Role:    "assistant",
			Content: assistantContent,
		})

		if len(resp.ToolUses) == 0 && (resp.StopReason == "end_turn" || resp.StopReason == "stop") {
			slog.Info("agent complete", "turn", turn, "total_input_tokens", totalInputTokens, "total_output_tokens", totalOutputTokens)
			break
		}

		if len(resp.ToolUses) == 0 {
			slog.Warn("stopping agent loop (e.g. max_tokens reached)", "stop_reason", resp.StopReason)
			break
		}

		var toolResultParts []local.ContentPart
		for _, tu := range resp.ToolUses {
			// Check permission model before execution
			if !a.isToolAllowed(tu.Name) {
				slog.Warn("tool blocked by permission model", "tool", tu.Name, "id", tu.ID)
				toolResultParts = append(toolResultParts, local.ContentPart{
					Type:               "tool_result",
					ResultForToolUseID: tu.ID,
					ResultContent:      "Error: Tool execution blocked by permission model",
					IsError:            true,
				})
				continue
			}

			slog.Info("executing tool", "tool", tu.Name, "id", tu.ID)

			result, err := a.executeTool(ctx, tu)
			if err != nil {
				slog.Warn("tool error", "tool", tu.Name, "error", err)
				toolResultParts = append(toolResultParts, local.ContentPart{
					Type:               "tool_result",
					ResultForToolUseID: tu.ID,
					ResultContent:      fmt.Sprintf("Error: %v", err),
					IsError:            true,
				})
			} else {
				slog.Info("tool success", "tool", tu.Name)
				toolResultParts = append(toolResultParts, local.ContentPart{
					Type:               "tool_result",
					ResultForToolUseID: tu.ID,
					ResultContent:      result,
				})
			}
		}

		messages = append(messages, local.ConversationMessage{
			Role:    "user",
			Content: toolResultParts,
		})
	}

	return finalText, nil
}

// executeWithRetry wraps the LLM complete call with exponential backoff retries.
func (a *Agent) executeWithRetry(ctx context.Context, req local.CompletionRequest) (*local.AssistantMessage, error) {
	const maxRetries = 3
	var lastErr error
	baseDelay := 1 * time.Second

	for attempt := 0; attempt < maxRetries; attempt++ {
		if err := ctx.Err(); err != nil {
			return nil, err
		}

		if a.cfg.LLM == nil {
			return nil, fmt.Errorf("LLM implementation is missing/nil")
		}

		resp, err := a.cfg.LLM.Complete(ctx, req)
		if err == nil {
			return resp, nil
		}

		lastErr = err
		slog.Warn("LLM API error", "attempt", attempt+1, "error", err)

		if attempt < maxRetries-1 {
			// Exponential backoff
			delay := time.Duration(math.Pow(2, float64(attempt))) * baseDelay
			select {
			case <-time.After(delay):
			case <-ctx.Done():
				return nil, ctx.Err()
			}
		}
	}
	return nil, fmt.Errorf("exhausted %d retries, last error: %w", maxRetries, lastErr)
}

// isToolAllowed acts as a conceptual permission model for tool execution.
func (a *Agent) isToolAllowed(toolName string) bool {
	// For example, an explicit deny-list or allow-list checks.
	return true
}

// truncateContextIfNeeded performs token budget management.
func (a *Agent) truncateContextIfNeeded(messages []local.ConversationMessage, maxTokens int) []local.ConversationMessage {
	estimateTokens := func(m local.ConversationMessage) int {
		tokens := 0
		for _, part := range m.Content {
			tokens += len(part.Text) / 4
			if part.ToolInput != nil {
				tokens += 50
			}
			if part.Type == "tool_result" {
				tokens += len(part.ResultContent) / 4
			}
		}
		return tokens
	}

	total := 0
	for _, m := range messages {
		total += estimateTokens(m)
	}

	if total <= maxTokens || len(messages) <= 3 {
		return messages
	}

	newMessages := []local.ConversationMessage{messages[0]}
	keepIdx := 1
	for keepIdx < len(messages)-2 && total > maxTokens {
		droppedTokens := estimateTokens(messages[keepIdx])
		total -= droppedTokens
		keepIdx++
	}

	newMessages = append(newMessages, messages[keepIdx:]...)
	return newMessages
}

func (a *Agent) executeTool(ctx context.Context, tu local.ToolUseRequest) (string, error) {
	for _, t := range a.cfg.Tools {
		if t.Definition().Name == tu.Name {
			return t.Execute(ctx, ".", tu.Input)
		}
	}
	return "", fmt.Errorf("unknown tool: %s", tu.Name)
}

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
