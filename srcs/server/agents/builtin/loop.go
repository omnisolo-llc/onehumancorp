package builtin

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"
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
	maxOutputTokensRecoveryCount := 0

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

		var resp ChatResponse
		var err error
		for {
			resp, err = a.Client.Chat(ctx, req)
			if err != nil {
				errMsg := err.Error()
				// Retry logic mimicking claude-code max_output_tokens escalation
				if (contains(errMsg, "prompt is too long") || contains(errMsg, "max_output_tokens")) && maxOutputTokensRecoveryCount < 3 {
					maxOutputTokensRecoveryCount++
					continue
				}
				return messages, fmt.Errorf("llm chat error: %w", err)
			}
			break
		}

		messages = append(messages, resp.Message)

		totalTurnTokens += resp.Usage.OutputTokens

		if len(resp.Message.ToolCalls) == 0 {
			// Check if we stopped due to max length and should continue under budget
			// We check for length/max_tokens as StopReason based on standard provider outputs
			if a.MaxTaskBudget > 0 && (resp.StopReason == "max_tokens" || resp.StopReason == "length") {
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
			} else if resp.StopReason == "max_tokens" || resp.StopReason == "length" {
				// If max task budget is not set, or we hit limits outside budget checking
				// implement the standard max_output_tokens recovery from claude-code
				if maxOutputTokensRecoveryCount < 3 {
					maxOutputTokensRecoveryCount++
					messages = append(messages, Message{
						Role:    RoleUser,
						Content: "Output token limit hit. Resume directly — no apology, no recap of what you were doing. Pick up mid-thought if that is where the cut happened. Break remaining work into smaller pieces.",
					})
					continue
				}
			}

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

		// claude-code handles missing tool calls in stream errors.
		// if a tool error causes an early return or crash, we ensure
		// all requested tools get an explicit error injection if they fail execution.
		// (Above loop already captures standard execution errors)

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

func contains(s, substr string) bool {
	return strings.Contains(s, substr)
}
