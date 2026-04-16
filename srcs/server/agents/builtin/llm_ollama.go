package builtin

import (
	"bufio"
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"time"
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

	resp, err := c.Client.Do(httpReq)
	if err != nil {
		return ChatResponse{}, err
	}
	defer resp.Body.Close()

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

// ChatStream implements real SSE streaming for Ollama.
func (c *OllamaClient) ChatStream(ctx context.Context, req ChatRequest, chunkChan chan<- ChatResponseChunk) error {
	req.Stream = true
	b, err := json.Marshal(req)
	if err != nil {
		return err
	}

	httpReq, err := http.NewRequestWithContext(ctx, "POST", c.Endpoint+"/api/chat", bytes.NewReader(b))
	if err != nil {
		return err
	}
	httpReq.Header.Set("Content-Type", "application/json")
	httpReq.Header.Set("Accept", "text/event-stream")

	resp, err := c.Client.Do(httpReq)
	if err != nil {
		return err
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return fmt.Errorf("ollama streaming error: %s", resp.Status)
	}

	scanner := bufio.NewScanner(resp.Body)
	for scanner.Scan() {
		line := scanner.Text()

		var chunk struct {
			Message struct {
				Content   string `json:"content"`
				ToolCalls []struct {
					Function struct {
						Name      string `json:"name,omitempty"`
						Arguments map[string]interface{} `json:"arguments,omitempty"`
					} `json:"function,omitempty"`
				} `json:"tool_calls,omitempty"`
			} `json:"message"`
			DoneReason string `json:"done_reason"`
		}
		if err := json.Unmarshal([]byte(line), &chunk); err == nil {
			var tcs []ToolCall
			for _, t := range chunk.Message.ToolCalls {
				bArgs, _ := json.Marshal(t.Function.Arguments)
				tcs = append(tcs, ToolCall{Name: t.Function.Name, Arguments: bArgs})
			}
			chunkChan <- ChatResponseChunk{
				Delta:      chunk.Message.Content,
				StopReason: chunk.DoneReason,
				ToolCalls:  tcs,
			}
		}
	}
	return scanner.Err()
}
