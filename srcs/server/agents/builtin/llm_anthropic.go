package builtin

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"time"
)

// AnthropicClient implements LLMClient for Anthropic's Claude API.

const MaxHistoryMessages = 40

func truncateMessages(msgs []Message) []Message {
	if len(msgs) <= MaxHistoryMessages {
		return msgs
	}

	// For Anthropic, system prompt is sent separately, so we just need to ensure
	// we don't orphan tool results from their tool calls.
	// Anthropic requires alternating roles in older APIs but allows adjacent same roles sometimes,
	// but strictly requires the first message to be "user" and tool results to follow tool calls.

	// Start by finding a safe boundary near the target length.
	targetCut := len(msgs) - MaxHistoryMessages

	// Walk forward to find a safe cut point where we don't start in the middle of a tool use sequence
	safeCut := targetCut
	for i := targetCut; i < len(msgs); i++ {
		// A safe cut point is a user message that does NOT contain tool results (unless we kept the assistant's tool call)
		if msgs[i].Role == RoleUser && len(msgs[i].ToolResults) == 0 {
			safeCut = i
			break
		}
	}

	if safeCut == targetCut && msgs[safeCut].Role != RoleUser {
		// If we didn't find a clean break, just walk forward to the next user message
		for i := targetCut; i < len(msgs); i++ {
			if msgs[i].Role == RoleUser {
				safeCut = i
				break
			}
		}
	}

	return msgs[safeCut:]
}

type AnthropicClient struct {
	APIKey string
	Client *http.Client
}

func NewAnthropicClient(apiKey string) *AnthropicClient {
	return &AnthropicClient{
		APIKey: apiKey,
		Client: &http.Client{Timeout: 2 * time.Minute},
	}
}

func (c *AnthropicClient) Chat(ctx context.Context, req ChatRequest) (ChatResponse, error) {
	req.Messages = truncateMessages(req.Messages)
	// Anthropic Messages API expects system prompt as a separate field, not in messages
	// For simplicity, we assume req structure aligns with a translation layer, or we map it here.

	// Map our ChatRequest to Anthropic's payload
	type antMessage struct {
		Role    string `json:"role"`
		Content string `json:"content"`
	}

	var messages []antMessage
	for _, m := range req.Messages {
		if m.Role == RoleSystem {
			continue // Handled separately
		}

		role := string(m.Role)
		if role == string(RoleTool) {
			role = "user" // Tool results are often sent as user in some simplified layers
		}

		messages = append(messages, antMessage{
			Role:    role,
			Content: m.Content,
		})
	}

	payload := map[string]interface{}{
		"model":      req.Model,
		"max_tokens": req.MaxTokens,
		"system":     req.System,
		"messages":   messages,
	}

	body, _ := json.Marshal(payload)

	httpReq, err := http.NewRequestWithContext(ctx, "POST", "https://api.anthropic.com/v1/messages", bytes.NewReader(body))
	if err != nil {
		return ChatResponse{}, err
	}

	httpReq.Header.Set("x-api-key", c.APIKey)
	httpReq.Header.Set("anthropic-version", "2023-06-01")
	httpReq.Header.Set("content-type", "application/json")

	resp, err := c.Client.Do(httpReq)
	if err != nil {
		return ChatResponse{}, err
	}
	defer resp.Body.Close()

	if resp.StatusCode != 200 {
		return ChatResponse{}, fmt.Errorf("anthropic api error: %s", resp.Status)
	}

	var result struct {
		Content []struct {
			Text string `json:"text"`
		} `json:"content"`
	}

	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		return ChatResponse{}, err
	}

	content := ""
	if len(result.Content) > 0 {
		content = result.Content[0].Text
	}

	return ChatResponse{
		Message: Message{
			Role:    RoleAssistant,
			Content: content,
		},
	}, nil
}
