package builtin

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
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
	var messages []map[string]interface{}
	for _, m := range req.Messages {
		if m.Role == RoleSystem {
			continue // Handled separately
		}

		role := string(m.Role)

		// Handle tool interactions
		if role == string(RoleTool) {
			role = "user"

			// Map our tool results format to Anthropic format
			var contentParts []map[string]interface{}
			for _, tr := range m.ToolResults {
				content := tr.Content
				if tr.Error != "" {
					content = "Error: " + tr.Error
				}

				contentParts = append(contentParts, map[string]interface{}{
					"type":        "tool_result",
					"tool_use_id": tr.ToolCallID,
					"content":     content,
				})
			}
			messages = append(messages, map[string]interface{}{
				"role":    role,
				"content": contentParts,
			})
			continue
		}

		if len(m.ToolCalls) > 0 {
			// Assistant message with tool calls
			var contentParts []map[string]interface{}
			if m.Content != "" {
				contentParts = append(contentParts, map[string]interface{}{
					"type": "text",
					"text": m.Content,
				})
			}
			for _, tc := range m.ToolCalls {
				var args map[string]interface{}
				if len(tc.Arguments) > 0 {
					_ = json.Unmarshal(tc.Arguments, &args)
				}
				if args == nil {
					args = make(map[string]interface{})
				}
				contentParts = append(contentParts, map[string]interface{}{
					"type":  "tool_use",
					"id":    tc.ID,
					"name":  tc.Name,
					"input": args,
				})
			}
			messages = append(messages, map[string]interface{}{
				"role":    role,
				"content": contentParts,
			})
			continue
		}

		messages = append(messages, map[string]interface{}{
			"role":    role,
			"content": m.Content,
		})
	}

	// Map tools to Anthropic tool schema
	var tools []map[string]interface{}
	for _, t := range req.Tools {
		tools = append(tools, map[string]interface{}{
			"name":         t.Name,
			"description":  t.Description,
			"input_schema": t.Parameters,
		})
	}

	payload := map[string]interface{}{
		"model":      req.Model,
		"max_tokens": req.MaxTokens,
		"system":     req.System,
		"messages":   messages,
	}
	if len(tools) > 0 {
		payload["tools"] = tools
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
		bodyBytes, _ := io.ReadAll(resp.Body)
		return ChatResponse{}, fmt.Errorf("anthropic api error: %s - %s", resp.Status, string(bodyBytes))
	}

	var result struct {
		StopReason string `json:"stop_reason"`
		Content    []struct {
			Type  string          `json:"type"`
			Text  string          `json:"text"`
			ID    string          `json:"id"`
			Name  string          `json:"name"`
			Input json.RawMessage `json:"input"`
		} `json:"content"`
	}

	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		return ChatResponse{}, err
	}

	content := ""
	var toolCalls []ToolCall

	for _, c := range result.Content {
		if c.Type == "text" {
			content += c.Text
		} else if c.Type == "tool_use" {
			toolCalls = append(toolCalls, ToolCall{
				ID:        c.ID,
				Name:      c.Name,
				Arguments: c.Input,
			})
		}
	}

	return ChatResponse{
		Message: Message{
			Role:      RoleAssistant,
			Content:   content,
			ToolCalls: toolCalls,
		},
	}, nil
}
