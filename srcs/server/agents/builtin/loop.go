package builtin

import (
	"context"
	"encoding/json"
	"fmt"
)

// Max error retries and output token limits for graceful degradation
const (
	MaxOutputTokensRecoveryLimit = 3
	MaxErrorRetries              = 3
)

// Run executes the agent loop until completion or error.
func (a *BuiltinAgent) Run(ctx context.Context, initialMessages []Message) ([]Message, error) {
	return a.RunWithCallback(ctx, initialMessages, nil)
}

// mergeToolCalls combines chunked tool call delta fragments from streams
func mergeToolCalls(existing []ToolCall, incoming []ToolCall) []ToolCall {
	for _, inc := range incoming {
		found := false
		for i, ext := range existing {
			// Streaming chunks either match exactly by ID, or if ID is missing in chunk,
			// assume we are appending to the most recent/last tool call in the array
			if ext.ID == inc.ID || (inc.ID == "" && i == len(existing)-1) {
				found = true
				if inc.ID != "" && ext.ID == "" {
					existing[i].ID = inc.ID
				}
				if inc.Name != "" && ext.Name == "" {
					existing[i].Name = inc.Name
				}
				if len(inc.Arguments) > 0 {
					existing[i].Arguments = append(existing[i].Arguments, inc.Arguments...)
				}
				break
			}
		}
		if !found {
			existing = append(existing, inc)
		}
	}
	return existing
}

// RunWithCallback executes the agent loop, calling cb for each event.
// cb may be nil, in which case the behaviour is identical to Run.
func (a *BuiltinAgent) RunWithCallback(ctx context.Context, initialMessages []Message, cb EventCallback) ([]Message, error) {
	messages := append([]Message(nil), initialMessages...)
	iteration := 0

	maxOutputTokensOverride := a.MaxTokens
	maxOutputTokensRecoveryCount := 0
	errorRetries := 0
	currentModel := a.Model

	for {
		iteration++
		if cb != nil {
			cb(AgentEvent{
				Type:         AgentEventTypeIterationStarted,
				Iteration:    iteration,
				MessageCount: len(messages),
			})
		}

		// Tool Allow/Deny List filtering
		filteredTools := a.Tools
		if len(a.AllowedTools) > 0 || len(a.DeniedTools) > 0 {
			var allowed []Tool
			for _, t := range a.Tools {
				denied := false
				for _, dt := range a.DeniedTools {
					if t.Name == dt {
						denied = true
						break
					}
				}
				if !denied {
					if len(a.AllowedTools) == 0 {
						allowed = append(allowed, t)
					} else {
						for _, at := range a.AllowedTools {
							if t.Name == at {
								allowed = append(allowed, t)
								break
							}
						}
					}
				}
			}
			filteredTools = allowed
		}

		// Prepare request
		req := ChatRequest{
			Model:       currentModel,
			System:      a.System,
			Messages:    messages,
			Tools:       filteredTools,
			MaxTokens:   maxOutputTokensOverride,
			Temperature: a.Temperature,
			Stream:      a.UseStreaming,
		}

		var resp ChatResponse
		var err error

		if a.UseStreaming {
			chunkChan := make(chan ChatResponseChunk, 100)
			go func() {
				defer close(chunkChan)
				err = a.Client.ChatStream(ctx, req, chunkChan)
			}()

			var fullContent string
			var finalStopReason string
			var toolCalls []ToolCall

			for chunk := range chunkChan {
				fullContent += chunk.Delta
				if chunk.StopReason != "" {
					finalStopReason = chunk.StopReason
				}
				if len(chunk.ToolCalls) > 0 {
					toolCalls = mergeToolCalls(toolCalls, chunk.ToolCalls)
				}
			}
			resp = ChatResponse{
				Message: Message{
					Role:      RoleAssistant,
					Content:   fullContent,
					ToolCalls: toolCalls,
				},
				StopReason: finalStopReason,
			}
		} else {
			resp, err = a.Client.Chat(ctx, req)
		}

		if err != nil {
			errorRetries++
			if errorRetries > MaxErrorRetries {
				// Try Fallback Model if available and all retries exhausted
				if a.FallbackModel != "" && currentModel != a.FallbackModel {
					currentModel = a.FallbackModel
					errorRetries = 0 // Reset for fallback
					continue
				}
				return messages, fmt.Errorf("llm chat error after retries: %w", err)
			}

			// Retry without modifying messages context (for temporary server errors)
			continue
		}
		// Reset retries after success
		errorRetries = 0

		messages = append(messages, resp.Message)

		// Max tokens escalation logic
		if resp.StopReason == "max_tokens" || resp.StopReason == "length" {
			if maxOutputTokensRecoveryCount < MaxOutputTokensRecoveryLimit {
				maxOutputTokensRecoveryCount++
				if a.MaxOutputEscalate > 0 && maxOutputTokensOverride < a.MaxOutputEscalate {
					maxOutputTokensOverride = a.MaxOutputEscalate
				}

				// Fix context window constraints
				messages = append(messages, Message{
					Role: RoleUser,
					Content: "Output token limit hit. Resume directly — no apology, no recap of what you were doing. " +
							"Pick up mid-thought if that is where the cut happened. Break remaining work into smaller pieces.",
				})
				continue
			}
		} else {
			// Reset recovery count if stopped naturally
			maxOutputTokensRecoveryCount = 0
		}

		// Context window / Auto-compact logic estimation based on string length
		if a.MaxContextTokens > 0 {
			estimatedTokens := 0
			for _, m := range messages {
				estimatedTokens += len(m.Content) / 4
			}

			if estimatedTokens > a.MaxContextTokens && len(messages) > 5 {
				// Avoid double Assistant or User messages to maintain strict sequence

				// Keep first message (System/Initial prompt)
				compacted := append([]Message(nil), messages[0])

				// Identify tail to keep
				tail := messages[len(messages)-4:]
				firstTailMessage := tail[0]

				if messages[0].Role == RoleUser {
				    // First message is user, so we MUST inject an Assistant message to break the User-User sequence
				    compacted = append(compacted, Message{
						Role:    RoleAssistant,
						Content: "Understood, context preserved.",
					})
				}

				compacted = append(compacted, Message{
					Role:    RoleUser,
					Content: "[Earlier context compacted to save tokens]",
				})

				if firstTailMessage.Role == RoleUser {
					// We need an assistant message between them to break the User-User sequence
					compacted = append(compacted, Message{
						Role:    RoleAssistant,
						Content: "Understood, proceeding with truncated context.",
					})
				}

				compacted = append(compacted, tail...)
				messages = compacted
			}
		}

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