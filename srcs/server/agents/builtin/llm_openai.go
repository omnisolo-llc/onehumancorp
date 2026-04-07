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
	var messages []map[string]interface{}
	if req.System != "" {
		messages = append(messages, map[string]interface{}{"role": "system", "content": req.System})
	}

	for _, m := range req.Messages {
		role := string(m.Role)

        if role == string(RoleTool) {
            // OpenAI Tool Results
            for _, tr := range m.ToolResults {
                content := tr.Content
                if tr.Error != "" {
                    content = "Error: " + tr.Error
                }
                messages = append(messages, map[string]interface{}{
                    "role": "tool",
                    "tool_call_id": tr.ToolCallID,
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
                toolCalls = append(toolCalls, map[string]interface{}{
                    "id": tc.ID,
                    "type": "function",
                    "function": map[string]interface{}{
                        "name": tc.Name,
                        "arguments": string(tc.Arguments),
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
		"model":       req.Model,
		"messages":    messages,
	}
    if len(tools) > 0 {
        payload["tools"] = tools
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
        bodyBytes, _ := io.ReadAll(resp.Body)
		return ChatResponse{}, fmt.Errorf("openai api error: %s - %s", resp.Status, string(bodyBytes))
	}

	var result struct {
		Choices []struct {
			Message struct {
				Content string `json:"content"`
                ToolCalls []struct {
                    ID string `json:"id"`
                    Function struct {
                        Name string `json:"name"`
                        Arguments string `json:"arguments"`
                    } `json:"function"`
                } `json:"tool_calls"`
			} `json:"message"`
		} `json:"choices"`
	}

	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		return ChatResponse{}, err
	}

	content := ""
    var toolCalls []ToolCall

	if len(result.Choices) > 0 {
        msg := result.Choices[0].Message
		content = msg.Content
        for _, tc := range msg.ToolCalls {
            toolCalls = append(toolCalls, ToolCall{
                ID: tc.ID,
                Name: tc.Function.Name,
                Arguments: json.RawMessage(tc.Function.Arguments),
            })
        }
	}

	return ChatResponse{
		Message: Message{
			Role:    RoleAssistant,
			Content: content,
            ToolCalls: toolCalls,
		},
	}, nil
}
