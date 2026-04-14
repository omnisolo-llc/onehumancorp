package builtin

import (
	"context"
	"encoding/json"
	"fmt"
)

// Run executes the agent loop until completion or error.
func (a *BuiltinAgent) Run(ctx context.Context, initialMessages []Message) ([]Message, error) {
	return a.RunWithCallback(ctx, initialMessages, nil)
}

// RunWithCallback executes the agent loop, calling cb for each event.
// cb may be nil, in which case the behaviour is identical to Run.
func (a *BuiltinAgent) RunWithCallback(ctx context.Context, initialMessages []Message, cb EventCallback) ([]Message, error) {
	messages := append([]Message(nil), initialMessages...)
	iteration := 0

	for {
		iteration++
		if cb != nil {
			cb(AgentEvent{
				Type:         AgentEventTypeIterationStarted,
				Iteration:    iteration,
				MessageCount: len(messages),
			})
		}

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
			return messages, fmt.Errorf("llm chat error: %w", err)
		}

		messages = append(messages, resp.Message)

		if len(resp.Message.ToolCalls) == 0 {
			// No tool calls — the agent produced a final response.
			if cb != nil {
				cb(AgentEvent{
					Type:    AgentEventTypeTaskComplete,
					Content: resp.Message.Content,
				})
			}
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

			if cb != nil {
				argsJSON := ""
				if raw, ok := any(tc.Arguments).(json.RawMessage); ok {
					argsJSON = string(raw)
				}
				content := result.Content
				if result.Error != "" {
					content = result.Error
				}
				cb(AgentEvent{
					Type:         AgentEventTypeToolCall,
					ToolName:     tc.Name,
					ToolArgsJSON: argsJSON,
					ToolResult:   content,
				})
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