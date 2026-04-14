package builtin

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"time"
)


type anthropicCacheControl struct {
	Type string `json:"type"`
}

type anthropicSystem struct {
	Type         string                 `json:"type"`
	Text         string                 `json:"text"`
	CacheControl *anthropicCacheControl `json:"cache_control,omitempty"`
}

type anthropicRequest struct {
	Model     string               `json:"model"`
	MaxTokens int                  `json:"max_tokens"`
	System    []anthropicSystem    `json:"system,omitempty"`
	Messages  []anthropicMessage   `json:"messages"`
	Tools     []anthropicToolDef   `json:"tools,omitempty"`
}

type anthropicMessage struct {
	Role    string               `json:"role"`
	Content []anthropicContent   `json:"content"`
}

type anthropicContent struct {
	Type         string                 `json:"type"`
	Text         string                 `json:"text,omitempty"`
	ID           string                 `json:"id,omitempty"`
	Name         string                 `json:"name,omitempty"`
	Input        map[string]interface{} `json:"input,omitempty"`
	ToolUseID    string                 `json:"tool_use_id,omitempty"`
	Content      interface{}            `json:"content,omitempty"`
	IsError      bool                   `json:"is_error,omitempty"`
	CacheControl *anthropicCacheControl `json:"cache_control,omitempty"`
}

type anthropicToolDef struct {
	Name         string                 `json:"name"`
	Description  string                 `json:"description"`
	InputSchema  json.RawMessage        `json:"input_schema"`
	CacheControl *anthropicCacheControl `json:"cache_control,omitempty"`
}

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
	var messages []anthropicMessage
	for _, m := range req.Messages {
		if m.Role == RoleSystem {
			continue // Handled separately
		}

		role := string(m.Role)
		if role == string(RoleTool) {
			role = "user" // Tool results are often sent as user in some simplified layers
		}

		messages = append(messages, anthropicMessage{
			Role: role,
			Content: []anthropicContent{
				{
					Type: "text",
					Text: m.Content,
				},
			},
		})
	}

	// Add cache control to the last user message if any
	for i := len(messages) - 1; i >= 0; i-- {
		if messages[i].Role == "user" && len(messages[i].Content) > 0 {
			lastContentIdx := len(messages[i].Content) - 1
			messages[i].Content[lastContentIdx].CacheControl = &anthropicCacheControl{Type: "ephemeral"}
			break
		}
	}

	tools := make([]anthropicToolDef, 0, len(req.Tools))
	for i, t := range req.Tools {
		toolDef := anthropicToolDef{
			Name:        t.Name,
			Description: t.Description,
			InputSchema: t.Parameters,
		}
		if i == len(req.Tools)-1 {
			toolDef.CacheControl = &anthropicCacheControl{Type: "ephemeral"}
		}
		tools = append(tools, toolDef)
	}

	var systemBlocks []anthropicSystem
	if req.System != "" {
		systemBlocks = append(systemBlocks, anthropicSystem{
			Type: "text",
			Text: req.System,
			CacheControl: &anthropicCacheControl{Type: "ephemeral"},
		})
	}

	maxTokens := req.MaxTokens
	if maxTokens == 0 {
		maxTokens = 4096 // Default max tokens
	}

	bodyStruct := anthropicRequest{
		Model:     req.Model,
		MaxTokens: maxTokens,
		System:    systemBlocks,
		Messages:  messages,
		Tools:     tools,
	}

	body, _ := json.Marshal(bodyStruct)

	httpReq, err := http.NewRequestWithContext(ctx, "POST", "https://api.anthropic.com/v1/messages", bytes.NewReader(body))
	if err != nil {
		return ChatResponse{}, err
	}

	httpReq.Header.Set("x-api-key", c.APIKey)
	httpReq.Header.Set("anthropic-version", "2023-06-01")
	httpReq.Header.Set("anthropic-beta", "prompt-caching-2024-07-31")
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
