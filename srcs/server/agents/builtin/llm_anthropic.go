package builtin

import (
	"bufio"
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"strings"
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

// ChatStream implements real SSE streaming for Anthropic.
func (c *AnthropicClient) ChatStream(ctx context.Context, req ChatRequest, chunkChan chan<- ChatResponseChunk) error {
	req.Stream = true
	b, err := json.Marshal(req)
	if err != nil {
		return err
	}

	httpReq, err := http.NewRequestWithContext(ctx, "POST", "https://api.anthropic.com/v1/messages", bytes.NewReader(b))
	if err != nil {
		return err
	}
	httpReq.Header.Set("x-api-key", c.APIKey)
	httpReq.Header.Set("anthropic-version", "2023-06-01")
	httpReq.Header.Set("Content-Type", "application/json")

	resp, err := c.Client.Do(httpReq)
	if err != nil {
		return err
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return fmt.Errorf("anthropic streaming error: %s", resp.Status)
	}

	scanner := bufio.NewScanner(resp.Body)
	for scanner.Scan() {
		line := scanner.Text()
		if !strings.HasPrefix(line, "data: ") {
			continue
		}
		data := strings.TrimPrefix(line, "data: ")
		if data == "[DONE]" {
			break
		}

		var chunk struct {
			Type  string `json:"type"`
			Delta struct {
				Text       string `json:"text,omitempty"`
				StopReason string `json:"stop_reason,omitempty"`
				Partial    string `json:"partial_json,omitempty"`
			} `json:"delta"`
			ContentBlock struct {
				Type string `json:"type,omitempty"`
				ID   string `json:"id,omitempty"`
				Name string `json:"name,omitempty"`
			} `json:"content_block"`
		}
		if err := json.Unmarshal([]byte(data), &chunk); err == nil {
			var tc []ToolCall
			if chunk.Type == "content_block_start" && chunk.ContentBlock.Type == "tool_use" {
				tc = append(tc, ToolCall{ID: chunk.ContentBlock.ID, Name: chunk.ContentBlock.Name})
			}
			if chunk.Type == "content_block_delta" && chunk.Delta.Partial != "" {
				tc = append(tc, ToolCall{Arguments: []byte(chunk.Delta.Partial)})
			}

			if chunk.Type == "content_block_delta" || chunk.Type == "message_delta" || len(tc) > 0 {
				chunkChan <- ChatResponseChunk{
					Delta:      chunk.Delta.Text,
					StopReason: chunk.Delta.StopReason,
					ToolCalls:  tc,
				}
			}
		}
	}
	return scanner.Err()
}
