package builtin

import (
	"context"
	"fmt"
	"log/slog"
	"time"
)

// Run executes the agent loop until completion or error.
func (a *BuiltinAgent) Run(ctx context.Context, initialMessages []Message) ([]Message, error) {
	config := GetConfig()
	messages := append([]Message(nil), initialMessages...)

	for turn := 0; turn < config.MaxTurns; turn++ {
		// Check context window and compact if necessary
		messages = CompactMessages(messages, config.TokenThreshold)

		// Prepare request
		req := ChatRequest{
			Model:       a.Model,
			System:      a.System,
			Messages:    messages,
			Tools:       a.Tools,
			MaxTokens:   a.MaxTokens,
			Temperature: a.Temperature,
			Stream:      config.StreamingEnabled,
			MaxRetries:  config.MaxRetries,
		}

		var resp ChatResponse
		var err error

		// Call LLM with retries
		for attempt := 0; attempt <= config.MaxRetries; attempt++ {
			resp, err = a.Client.Chat(ctx, req)
			if err == nil {
				break
			}
			slog.Warn("llm chat error, retrying", "attempt", attempt, "error", err)
			time.Sleep(time.Duration(1<<attempt) * time.Second) // exponential backoff
		}

		if err != nil {
			return messages, fmt.Errorf("llm chat error after retries: %w", err)
		}

		messages = append(messages, resp.Message)

		if len(resp.Message.ToolCalls) == 0 {
			// No tool calls, we are done
			break
		}

		// Execute tool calls
		var toolResults []ToolResult
		for _, tc := range resp.Message.ToolCalls {
			result, err := a.executeToolCall(ctx, tc, config.PermissionMode)
			if err != nil {
				// We can return the error as a tool result instead of failing the whole loop
				toolResults = append(toolResults, ToolResult{
					ToolCallID: tc.ID,
					Error:      err.Error(),
				})
			} else {
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

func (a *BuiltinAgent) executeToolCall(ctx context.Context, tc ToolCall, permissionMode string) (ToolResult, error) {
	for _, tool := range a.Tools {
		if tool.Name == tc.Name {

			if tool.RequiresApproval && permissionMode == "strict" {
				return ToolResult{}, fmt.Errorf("tool %q requires approval in strict mode", tc.Name)
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