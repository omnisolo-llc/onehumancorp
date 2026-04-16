package builtin

import (
	"context"
	"encoding/json"
	"fmt"
	"math"
	"math/rand"
	"time"
	"log/slog"
	"strings"
	"regexp"
	"strconv"
)

// getRetryDelay returns a backoff delay, similar to getRetryDelay in CC-Source.
func getRetryDelay(attempt int) time.Duration {
	baseDelay := float64(2000) * math.Pow(2, float64(attempt-1)) // 2s base delay
	if baseDelay > 32000 {
		baseDelay = 32000
	}
	jitter := rand.Float64() * 0.25 * baseDelay
	return time.Duration(baseDelay+jitter) * time.Millisecond
}

var maxTokensContextOverflowRegex = regexp.MustCompile(`input length and \x60max_tokens\x60 exceed context limit: (\d+) \+ (\d+) > (\d+)`)

func parseMaxTokensContextOverflowError(err error) (inputTokens, maxTokens, contextLimit int, ok bool) {
	if err == nil {
		return
	}
	msg := err.Error()
	matches := maxTokensContextOverflowRegex.FindStringSubmatch(msg)
	if len(matches) == 4 {
		i, err1 := strconv.Atoi(matches[1])
		m, err2 := strconv.Atoi(matches[2])
		c, err3 := strconv.Atoi(matches[3])
		if err1 == nil && err2 == nil && err3 == nil {
			return i, m, c, true
		}
	}
	return 0, 0, 0, false
}

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

	// Set a reasonable fallback for max loops if not defined, to prevent infinite loops (like maxAgentTurns)
	maxIterations := 50

	for {
		if iteration >= maxIterations {
			return messages, fmt.Errorf("agent exceeded max iterations (%d)", maxIterations)
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

		// Call LLM with retry logic and potential streaming support
		var resp ChatResponse
		var err error
		maxRetries := 3
		for attempt := 1; attempt <= maxRetries; attempt++ {
			if streamClient, ok := a.Client.(StreamingLLMClient); ok && cb != nil {
				resp, err = streamClient.ChatStream(ctx, req, func(chunk string) {
					cb(AgentEvent{
						Type:    AgentEventTypeStreamChunk,
						Content: chunk,
					})
				})
			} else {
				resp, err = a.Client.Chat(ctx, req)
			}

			if err == nil {
				break
			}

			if attempt < maxRetries {
				// Handle max tokens context overflow errors by adjusting max_tokens for the next attempt
				if inputTokens, _, contextLimit, ok := parseMaxTokensContextOverflowError(err); ok {
					safetyBuffer := 1000
					availableContext := contextLimit - inputTokens - safetyBuffer
					if availableContext < 100 { // FLOOR_OUTPUT_TOKENS ~ 100
						// Not enough context to continue
					} else {
						// Adjust max tokens to fit within context
						req.MaxTokens = availableContext
						slog.Warn("Adjusted max_tokens due to context overflow", "new_max_tokens", req.MaxTokens)
					}
				}

				delay := getRetryDelay(attempt)
				slog.Warn("llm chat error, retrying", "attempt", attempt, "maxRetries", maxRetries, "delay", delay, "error", err)
				select {
				case <-ctx.Done():
					return messages, fmt.Errorf("context cancelled during retry: %w", ctx.Err())
				case <-time.After(delay):
					// Continue to the next attempt
				}
			}
		}

		if err != nil {
			return messages, fmt.Errorf("llm chat error after %d retries: %w", maxRetries, err)
		}

		messages = append(messages, resp.Message)

		totalTurnTokens += resp.Usage.OutputTokens

		if len(resp.Message.ToolCalls) == 0 {
			// Check if we stopped due to max length and should continue under budget
			// We check for length/max_tokens as StopReason based on standard provider outputs
			if a.MaxTaskBudget > 0 && (resp.StopReason == "max_tokens" || resp.StopReason == "length" || strings.Contains(resp.StopReason, "model_context_window_exceeded")) {
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

		// Append tool results to messages
		messages = append(messages, Message{
			Role:        RoleTool,
			ToolResults: toolResults,
		})
	}

	return messages, nil
}

func (a *BuiltinAgent) executeToolCall(ctx context.Context, tc ToolCall) (ToolResult, error) {
	// First check tool permissions
	if a.ToolPermissions != nil {
		if err := a.ToolPermissions.CanExecute(tc.Name); err != nil {
			return ToolResult{}, err
		}
	}

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
