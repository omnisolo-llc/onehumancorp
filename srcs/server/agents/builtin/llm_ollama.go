package builtin

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

// OllamaClient implements LLMClient for Ollama (local LLM).
type OllamaClient struct {
	Endpoint string
	Client   *http.Client
}

func NewOllamaClient(endpoint string) *OllamaClient {
	if endpoint == "" {
		endpoint = "http://localhost:11434/api/chat"
	}
	return &OllamaClient{
		Endpoint: endpoint,
		Client:   &http.Client{Timeout: 5 * time.Minute},
	}
}

func (c *OllamaClient) Chat(ctx context.Context, req ChatRequest) (ChatResponse, error) {
	type ollamaMessage struct {
		Role    string `json:"role"`
		Content string `json:"content"`
	}

	var messages []ollamaMessage
	if req.System != "" {
		messages = append(messages, ollamaMessage{Role: "system", Content: req.System})
	}

	for _, m := range req.Messages {
		messages = append(messages, ollamaMessage{
			Role:    string(m.Role),
			Content: m.Content,
		})
	}

	payload := map[string]interface{}{
		"model":    req.Model,
		"messages": messages,
		"stream":   false,
	}

	body, _ := json.Marshal(payload)

	httpReq, err := http.NewRequestWithContext(ctx, "POST", c.Endpoint, bytes.NewReader(body))
	if err != nil {
		return ChatResponse{}, err
	}

	httpReq.Header.Set("Content-Type", "application/json")

	start := time.Now()
	resp, err := c.Client.Do(httpReq)
	if err != nil {
		return ChatResponse{}, err
	}
	defer resp.Body.Close()
	telemetry.RecordLLMNetworkLatency(ctx, req.Model, time.Since(start).Seconds())

	if resp.StatusCode != 200 {
		return ChatResponse{}, fmt.Errorf("ollama api error: %s", resp.Status)
	}

	var result struct {
		Message struct {
			Content string `json:"content"`
		} `json:"message"`
	}

	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		return ChatResponse{}, err
	}

	return ChatResponse{
		Message: Message{
			Role:    RoleAssistant,
			Content: result.Message.Content,
		},
	}, nil
}
