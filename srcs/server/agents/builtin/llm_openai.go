package builtin

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"time"
)

// OpenAIClient implements LLMClient for OpenAI API.
type OpenAIClient struct {
	APIKey string
	Client *http.Client
}

func NewOpenAIClient(apiKey string) *OpenAIClient {
	return &OpenAIClient{
		APIKey: apiKey,
		Client: &http.Client{Timeout: 2 * time.Minute},
	}
}

func (c *OpenAIClient) Chat(ctx context.Context, req ChatRequest) (ChatResponse, error) {
	// Map our ChatRequest to OpenAI's payload
	type openaiMessage struct {
		Role    string `json:"role"`
		Content string `json:"content"`
	}

	var messages []openaiMessage
	if req.System != "" {
		messages = append(messages, openaiMessage{Role: "system", Content: req.System})
	}

	for _, m := range req.Messages {
		messages = append(messages, openaiMessage{
			Role:    string(m.Role),
			Content: m.Content,
		})
	}

	payload := map[string]interface{}{
		"model":       req.Model,
		"messages":    messages,
	}

	body, _ := json.Marshal(payload)

	httpReq, err := http.NewRequestWithContext(ctx, "POST", "https://api.openai.com/v1/chat/completions", bytes.NewReader(body))
	if err != nil {
		return ChatResponse{}, err
	}

	httpReq.Header.Set("Authorization", "Bearer "+c.APIKey)
	httpReq.Header.Set("Content-Type", "application/json")

	resp, err := c.Client.Do(httpReq)
	if err != nil {
		return ChatResponse{}, err
	}
	defer resp.Body.Close()

	if resp.StatusCode != 200 {
		return ChatResponse{}, fmt.Errorf("openai api error: %s", resp.Status)
	}

	var result struct {
		Choices []struct {
			Message struct {
				Content string `json:"content"`
			} `json:"message"`
		} `json:"choices"`
	}

	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		return ChatResponse{}, err
	}

	content := ""
	if len(result.Choices) > 0 {
		content = result.Choices[0].Message.Content
	}

	return ChatResponse{
		Message: Message{
			Role:    RoleAssistant,
			Content: content,
		},
	}, nil
}
