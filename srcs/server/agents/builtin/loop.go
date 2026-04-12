package builtin

import (
	"context"
	"encoding/json"
	"fmt"
	"time"
)

// Init initializes the session lifecycle.
func (a *BuiltinAgent) Init(ctx context.Context) error {
	a.TokensUsed = 0
	return nil
}

// Teardown cleans up the session lifecycle.
func (a *BuiltinAgent) Teardown(ctx context.Context) error {
	return nil
}

// isToolAllowed checks if a tool is permitted to run.
func (a *BuiltinAgent) isToolAllowed(toolName string) bool {
	if len(a.DenyList) > 0 {
		for _, deny := range a.DenyList {
			if deny == toolName {
				return false
			}
		}
	}
	if len(a.AllowList) > 0 {
		allowed := false
		for _, allow := range a.AllowList {
			if allow == toolName {
				allowed = true
				break
			}
		}
		if !allowed {
			return false
		}
	}
	return true
}

func (a *BuiltinAgent) estimateTokens(content string) int {
	return len(content) / 4 // naive estimation
}

func (a *BuiltinAgent) clipMessages(messages []Message) []Message {
	if a.ContextWindow <= 0 {
		return messages
	}

	totalTokens := 0
	var result []Message
	// keep from newest to oldest
	for i := len(messages) - 1; i >= 0; i-- {
		tokens := a.estimateTokens(messages[i].Content)
		if totalTokens+tokens > a.ContextWindow {
			break
		}
		totalTokens += tokens
		result = append([]Message{messages[i]}, result...)
	}
	return result
}

// Run executes the agent loop until completion or error.
func (a *BuiltinAgent) Run(ctx context.Context, initialMessages []Message) ([]Message, error) {
	if err := a.Init(ctx); err != nil {
		return nil, err
	}
	defer a.Teardown(ctx)

	messages := append([]Message(nil), initialMessages...)
	turns := 0

	maxTurns := a.MaxTurns
	if maxTurns <= 0 {
		maxTurns = 100 // default max turns
	}
	maxRetries := a.MaxRetries
	if maxRetries <= 0 {
		maxRetries = 3
	}

	for {
		if turns >= maxTurns {
			return messages, fmt.Errorf("max turns exceeded")
		}
		turns++

		if a.TokenBudget > 0 && a.TokensUsed >= a.TokenBudget {
			return messages, fmt.Errorf("token budget exceeded")
		}

		// Prepare request
		req := ChatRequest{
			Model:       a.Model,
			System:      a.System,
			Messages:    a.clipMessages(messages),
			Tools:       a.Tools,
			MaxTokens:   a.MaxTokens,
			Temperature: a.Temperature,
		}

		// Call LLM
		var resp ChatResponse
		var err error

		if a.Streaming {
			chunkCh, strErr := a.Client.ChatStream(ctx, req)
			if strErr != nil {
				err = strErr
			} else {
				resp = ChatResponse{Message: Message{Role: RoleAssistant}}
				for chunk := range chunkCh {
					resp.Message.Content += chunk.Message.Content
					for _, incTc := range chunk.Message.ToolCalls {
						found := false
						for j := range resp.Message.ToolCalls {
							if resp.Message.ToolCalls[j].ID == incTc.ID {
								// Merge partial tool call argument strings
								resp.Message.ToolCalls[j].Arguments = json.RawMessage(string(resp.Message.ToolCalls[j].Arguments) + string(incTc.Arguments))
								found = true
								break
							}
						}
						if !found {
							resp.Message.ToolCalls = append(resp.Message.ToolCalls, incTc)
						}
					}
				}
			}
		} else {
			for retry := 0; retry <= maxRetries; retry++ {
				resp, err = a.Client.Chat(ctx, req)
				if err == nil {
					break
				}
				time.Sleep(time.Duration(retry*100) * time.Millisecond) // exponential backoff
			}
		}

		if err != nil {
			return messages, fmt.Errorf("llm chat error: %w", err)
		}

		a.TokensUsed += a.estimateTokens(resp.Message.Content)
		messages = append(messages, resp.Message)

		if len(resp.Message.ToolCalls) == 0 {
			// No tool calls, we are done
			break
		}

		// Execute tool calls
		var toolResults []ToolResult
		for _, tc := range resp.Message.ToolCalls {
			if !a.isToolAllowed(tc.Name) {
				toolResults = append(toolResults, ToolResult{
					ToolCallID: tc.ID,
					Error:      fmt.Sprintf("tool %q is not allowed by permissions", tc.Name),
				})
				continue
			}

			result, err := a.executeToolCall(ctx, tc)
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
