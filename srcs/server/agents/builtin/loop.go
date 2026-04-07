package builtin

import (
	"context"
	"fmt"
	"log/slog"
	"time"
)

// Run executes the agent loop until completion or error.
func (a *BuiltinAgent) Run(ctx context.Context, initialMessages []Message) ([]Message, error) {
	messages := append([]Message(nil), initialMessages...)
	turnCount := 0
	maxTurns := 50

	for {
		if turnCount >= maxTurns {
			slog.Warn("BuiltinAgent: max turns reached", "turns", turnCount)
			break
		}
		turnCount++

		// Context sliding window (compaction) - robust implementation
		// Ensure we don't sever ToolCall <-> ToolResult relationships
		if len(messages) > 100 {
			// Find a safe boundary to compact (not in the middle of a tool call)
			// For this implementation, we keep the first 5 (system/initial context) and the last 40
			// Always ensure the cut point is on a user/assistant boundary
			safeBoundary := len(messages) - 40
			for safeBoundary > 5 && safeBoundary < len(messages) {
				if messages[safeBoundary].Role == RoleUser || messages[safeBoundary].Role == RoleAssistant {
					break
				}
				safeBoundary++
			}

			if safeBoundary < len(messages) {
				newMessages := make([]Message, 0, len(messages[:5])+len(messages[safeBoundary:]))
				newMessages = append(newMessages, messages[:5]...)
				newMessages = append(newMessages, messages[safeBoundary:]...)
				messages = newMessages
			}
		}

		req := ChatRequest{
			Model:       a.Model,
			System:      a.System,
			Messages:    messages,
			Tools:       a.Tools,
			MaxTokens:   a.MaxTokens,
			Temperature: a.Temperature,
		}

		var resp ChatResponse
		var err error

		// Retry loop for API calls
		maxRetries := 3
		for retry := 0; retry < maxRetries; retry++ {
			if streamer, ok := a.Client.(StreamingClient); ok {
				ch, streamErr := streamer.ChatStream(ctx, req)
				if streamErr != nil {
					err = streamErr
					time.Sleep(time.Second * time.Duration(retry+1))
					continue
				}

				resp.Message.Role = RoleAssistant
				for event := range ch {
					if event.Type == "content_block_delta" {
						resp.Message.Content += event.Delta
					} else if event.Type == "message_stop" {
						break
					}
				}
				err = nil
				break
			} else {
				resp, err = a.Client.Chat(ctx, req)
				if err == nil {
					break
				}
				time.Sleep(time.Second * time.Duration(retry+1))
			}
		}

		if err != nil {
			return messages, fmt.Errorf("llm chat error after retries: %w", err)
		}

		messages = append(messages, resp.Message)

		if len(resp.Message.ToolCalls) == 0 {
			break
		}

		// Execute tool calls
		var toolResults []ToolResult
		for _, tc := range resp.Message.ToolCalls {
			result, err := a.executeToolCall(ctx, tc)
			if err != nil {
				toolResults = append(toolResults, ToolResult{
					ToolCallID: tc.ID,
					Error:      err.Error(),
				})
			} else {
				toolResults = append(toolResults, result)
			}
		}

		messages = append(messages, Message{
			Role:        RoleTool,
			ToolResults: toolResults,
		})
	}

	return messages, nil
}

func (a *BuiltinAgent) executeToolCall(ctx context.Context, tc ToolCall) (ToolResult, error) {
	// Permission checks could be wired in here before tool execution
	// For example, checking if the tool is in an allowed list for this agent/tenant
	for _, tool := range a.Tools {
		if tool.Name == tc.Name {
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
