package builtin

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"
)

const (
	MaxTurnsDefault         = 50
	MaxTokensDefault        = 4096
	MaxBudgetUSDDefault     = 5.0
	MaxToolResultLength     = 100000 // Approximate characters before truncation
	MaxContextTokens        = 8192   // Hard threshold for context window before truncation
)

// RunConfig provides configuration for the agent run.
type RunConfig struct {
	MaxTurns             int
	MaxTokens            int
	MaxBudgetUSD         float64
	CanUseTool           func(toolName string, args json.RawMessage) bool
	OnEvent              func(event AgentEvent)
	TotalCostUSD         float64
	MaxRetries           int
}

// DefaultRunConfig returns a sensible default configuration.
func DefaultRunConfig() RunConfig {
	return RunConfig{
		MaxTurns:     MaxTurnsDefault,
		MaxTokens:    MaxTokensDefault,
		MaxBudgetUSD: MaxBudgetUSDDefault,
		MaxRetries:   3,
		CanUseTool: func(toolName string, args json.RawMessage) bool { return true }, // Allow all by default
		OnEvent:    func(event AgentEvent) {},
	}
}

// Run executes the agent loop until completion, error, or max turns.
func (a *BuiltinAgent) Run(ctx context.Context, initialMessages []Message, config *RunConfig) ([]Message, error) {
	if config == nil {
		c := DefaultRunConfig()
		config = &c
	}
	messages := append([]Message(nil), initialMessages...)
	turnCount := 0

	if config.MaxTurns <= 0 {
		config.MaxTurns = MaxTurnsDefault
	}

	for {
		if turnCount >= config.MaxTurns {
			config.OnEvent(AgentEvent{Type: EventError, Data: "max_turns_reached"})
			return messages, fmt.Errorf("reached maximum turns (%d)", config.MaxTurns)
		}

		if config.MaxBudgetUSD > 0 && config.TotalCostUSD >= config.MaxBudgetUSD {
			config.OnEvent(AgentEvent{Type: EventError, Data: "max_budget_reached"})
			return messages, fmt.Errorf("reached maximum budget ($%.2f)", config.MaxBudgetUSD)
		}

		// Context Window Management (Truncation)
		messages = a.compactHistory(messages, MaxContextTokens)

		turnCount++
		config.OnEvent(AgentEvent{Type: EventTurnStart, Data: map[string]interface{}{"turn": turnCount}})

		// Prepare request
		systemPrompt := a.System
		if systemPrompt == "" {
			systemPrompt = GetSystemPrompt()
		}

		req := ChatRequest{
			Model:       a.Model,
			System:      systemPrompt,
			Messages:    messages,
			Tools:       a.Tools,
			MaxTokens:   a.MaxTokens,
			Temperature: a.Temperature,
		}

		// Call LLM
		config.OnEvent(AgentEvent{Type: EventLLMCallStart, Data: req})
		var resp ChatResponse
		var err error

		// Simple retry mechanism for network/API errors
		for r := 0; r < config.MaxRetries; r++ {
			resp, err = a.Client.Chat(req)
			if err == nil {
				break
			}
		}

		if err != nil {
			config.OnEvent(AgentEvent{Type: EventError, Data: err.Error()})
			return messages, fmt.Errorf("llm chat error: %w", err)
		}
		config.OnEvent(AgentEvent{Type: EventLLMCallComplete, Data: resp})

		// Update cost based on usage
		if resp.Usage.TotalTokens > 0 {
			config.TotalCostUSD += float64(resp.Usage.TotalTokens) * 0.00001
		}

		messages = append(messages, resp.Message)

		if len(resp.Message.ToolCalls) == 0 {
			// Streaming / Non-streaming completion event
			config.OnEvent(AgentEvent{Type: EventSuccess, Data: resp.Message.Content})
			break
		}

		// Execute tool calls
		var toolResults []ToolResult
		for _, tc := range resp.Message.ToolCalls {
			config.OnEvent(AgentEvent{Type: EventToolStart, Data: tc})
			result, err := a.executeToolCall(ctx, tc, config.CanUseTool)
			if err != nil {
				config.OnEvent(AgentEvent{Type: EventToolError, Data: err.Error()})
				toolResults = append(toolResults, ToolResult{
					ToolCallID: tc.ID,
					Error:      err.Error(),
				})
			} else {
				// Truncate overly long outputs to manage context window limit
				if len(result.Content) > MaxToolResultLength {
					result.Content = result.Content[:MaxToolResultLength] + "\n...[truncated due to length]"
				}
				config.OnEvent(AgentEvent{Type: EventToolComplete, Data: result})
				toolResults = append(toolResults, result)
			}
		}

		// Append tool results to messages
		messages = append(messages, Message{
			Role:        RoleTool,
			ToolResults: toolResults,
		})
	}

	return messages, nil
}

// compactHistory is a simple context window truncation strategy.
func (a *BuiltinAgent) compactHistory(messages []Message, maxTokens int) []Message {
	// A naive estimation: 1 word ~ 1 token for truncation.
	// We preserve the latest messages and remove older middle messages if limit is breached.
	estimatedTotal := 0
	for _, m := range messages {
		estimatedTotal += len(strings.Fields(m.Content))
		for _, tr := range m.ToolResults {
			estimatedTotal += len(strings.Fields(tr.Content))
		}
	}

	if estimatedTotal <= maxTokens || len(messages) <= 3 {
		return messages
	}

	// Truncate the oldest message that isn't the first prompt, if needed.
	// We keep first message, and drop from index 1.
	newMessages := make([]Message, 0, len(messages)-1)
	newMessages = append(newMessages, messages[0])
	newMessages = append(newMessages, messages[2:]...) // Drop message index 1

	// Recursive call to ensure we're under limit
	return a.compactHistory(newMessages, maxTokens)
}

func (a *BuiltinAgent) executeToolCall(ctx context.Context, tc ToolCall, canUseTool func(string, json.RawMessage) bool) (ToolResult, error) {
	for _, tool := range a.Tools {
		if tool.Name == tc.Name {
			if tool.RequiresAuth && canUseTool != nil && !canUseTool(tc.Name, tc.Arguments) {
				return ToolResult{
					ToolCallID: tc.ID,
					Error:      "Permission denied by user",
				}, nil
			}

			result, err := tool.Execute(ctx, tc.Arguments)
			if err != nil {
				return ToolResult{
					ToolCallID: tc.ID,
					Error:      err.Error(),
				}, nil
			}
			return ToolResult{
				ToolCallID: tc.ID,
				Content:    result,
			}, nil
		}
	}
	return ToolResult{}, fmt.Errorf("tool %q not found", tc.Name)
}
