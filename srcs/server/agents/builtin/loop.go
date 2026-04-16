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

	budgetTracker := &BudgetTracker{}
	totalTurnTokens := 0
	hasAttemptedReactiveCompact := false
	maxOutputTokensRecoveryCount := 0
	maxTurns := 50 // Enforce max turns limit to prevent infinite loops.

	for {
		if iteration >= maxTurns {
			if cb != nil {
				cb(AgentEvent{
					Type:    AgentEventTypeError,
					Error:   fmt.Errorf("agent exceeded max turns (%d)", maxTurns),
				})
			}
			return messages, fmt.Errorf("agent exceeded max turns (%d)", maxTurns)
		}

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
		totalTurnTokens += resp.Usage.OutputTokens

		// Handle error responses or max tokens reach (Prompt too long / length stop reason)
		if resp.StopReason == "max_tokens" || resp.StopReason == "length" {
			if a.MaxTaskBudget > 0 {
				decision := CheckTokenBudget(budgetTracker, a.MaxTaskBudget, totalTurnTokens)
				if decision.Action == "continue" {
					messages = append(messages, Message{
						Role:    RoleUser,
						Content: decision.NudgeMessage,
					})
					continue
				} else if decision.Action == "stop" && decision.Diminishing {
					if cb != nil {
						cb(AgentEvent{
							Type:    AgentEventTypeTaskComplete,
							Content: "Stopped due to token budget limit or diminishing returns.\n" + resp.Message.Content,
						})
					}
					break
				}
			} else {
				// Fallback generic recovery for max tokens if budget isn't strictly used.
				if maxOutputTokensRecoveryCount < 3 {
					maxOutputTokensRecoveryCount++
					messages = append(messages, Message{
						Role:    RoleUser,
						Content: "Output token limit hit. Resume directly — no apology, no recap of what you were doing. Pick up mid-thought if that is where the cut happened. Break remaining work into smaller pieces.",
					})
					continue
				}
			}
		} else if resp.StopReason == "prompt_too_long" {
			if !hasAttemptedReactiveCompact {
				if len(messages) > 4 {
					hasAttemptedReactiveCompact = true
					compactedMessages := append([]Message{messages[0]}, messages[len(messages)-3:]...)
					messages = compactedMessages
					continue
				}
			}
			return messages, fmt.Errorf("prompt_too_long: unable to recover")
		}

		if len(resp.Message.ToolCalls) == 0 {
			// No tool calls and not forced to continue — the agent produced a final response.
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
