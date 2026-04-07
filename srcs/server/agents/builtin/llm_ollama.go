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
	var messages []map[string]interface{}
	if req.System != "" {
		messages = append(messages, map[string]interface{}{"role": "system", "content": req.System})
	}

	for _, m := range req.Messages {
		role := string(m.Role)

        if role == string(RoleTool) {
            for _, tr := range m.ToolResults {
                content := tr.Content
                if tr.Error != "" {
                    content = "Error: " + tr.Error
                }
                messages = append(messages, map[string]interface{}{
                    "role": "tool",
                    "content": content,
                })
            }
            continue
        }

        msg := map[string]interface{}{
            "role":    role,
            "content": m.Content,
        }

        if len(m.ToolCalls) > 0 {
            var toolCalls []map[string]interface{}
            for _, tc := range m.ToolCalls {
                var args map[string]interface{}
                if len(tc.Arguments) > 0 {
                    _ = json.Unmarshal(tc.Arguments, &args)
                }
                toolCalls = append(toolCalls, map[string]interface{}{
                    "function": map[string]interface{}{
                        "name": tc.Name,
                        "arguments": args,
                    },
                })
            }
            msg["tool_calls"] = toolCalls
        }

		messages = append(messages, msg)
	}

    var tools []map[string]interface{}
    for _, t := range req.Tools {
        tools = append(tools, map[string]interface{}{
            "type": "function",
            "function": map[string]interface{}{
                "name": t.Name,
                "description": t.Description,
                "parameters": t.Parameters,
            },
        })
    }

	payload := map[string]interface{}{
		"model":    req.Model,
		"messages": messages,
		"stream":   false,
	}

    if len(tools) > 0 {
        payload["tools"] = tools
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
        bodyBytes, _ := io.ReadAll(resp.Body)
		return ChatResponse{}, fmt.Errorf("ollama api error: %s - %s", resp.Status, string(bodyBytes))
	}

	var result struct {
		Message struct {
			Content string `json:"content"`
            ToolCalls []struct {
                Function struct {
                    Name string `json:"name"`
                    Arguments map[string]interface{} `json:"arguments"`
                } `json:"function"`
            } `json:"tool_calls"`
		} `json:"message"`
	}

	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		return ChatResponse{}, err
	}

    content := result.Message.Content
    var toolCalls []ToolCall

    for i, tc := range result.Message.ToolCalls {
        argsJSON, _ := json.Marshal(tc.Function.Arguments)
        toolCalls = append(toolCalls, ToolCall{
            ID: fmt.Sprintf("call_%d", i),
            Name: tc.Function.Name,
            Arguments: argsJSON,
        })
    }

	return ChatResponse{
		Message: Message{
			Role:    RoleAssistant,
			Content: content,
            ToolCalls: toolCalls,
		},
	}, nil
}
