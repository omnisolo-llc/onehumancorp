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
		Usage struct {
			InputTokens  int `json:"input_tokens"`
			OutputTokens int `json:"output_tokens"`
		} `json:"usage"`
		StopReason string `json:"stop_reason"`
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
		Usage: Usage{
			InputTokens:  result.Usage.InputTokens,
			OutputTokens: result.Usage.OutputTokens,
		},
		StopReason: result.StopReason,
	}, nil
}
