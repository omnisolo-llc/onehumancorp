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

    // Conversation loop state
    maxTurns := 100
    turnCount := 0
    consecutiveErrors := 0
    maxConsecutiveErrors := 3

	for turnCount < maxTurns {
		turnCount++

        // Prepare request
		req := ChatRequest{
			Model:       a.Model,
			System:      a.System,
			Messages:    messages,
			Tools:       a.Tools,
			MaxTokens:   a.MaxTokens,
			Temperature: a.Temperature,
		}

		// Call LLM
		resp, err := a.Client.Chat(ctx, req)
		if err != nil {
            consecutiveErrors++
            if consecutiveErrors >= maxConsecutiveErrors {
			    return messages, fmt.Errorf("llm chat error (exceeded max retries): %w", err)
            }
            slog.Warn("llm chat error, retrying", "err", err, "turn", turnCount)
            time.Sleep(time.Second * time.Duration(consecutiveErrors))
            continue
		}
        consecutiveErrors = 0 // Reset on success

		messages = append(messages, resp.Message)

		if len(resp.Message.ToolCalls) == 0 {
			// No tool calls, we are done
			break
		}

		// Execute tool calls
		var toolResults []ToolResult
		for _, tc := range resp.Message.ToolCalls {
			result, err := a.executeToolCall(ctx, tc)
			if err != nil {
				// We return the error as a tool result instead of failing the whole loop,
                // allowing the LLM to gracefully recover.
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

    if turnCount >= maxTurns {
        return messages, fmt.Errorf("exceeded maximum turns (%d)", maxTurns)
    }

	return messages, nil
}

func (a *BuiltinAgent) executeToolCall(ctx context.Context, tc ToolCall) (ToolResult, error) {
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
