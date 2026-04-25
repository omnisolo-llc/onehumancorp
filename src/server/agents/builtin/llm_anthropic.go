package builtin

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"log/slog"
	"net/http"
	"time"

	"github.com/onehumancorp/mono/src/server/telemetry"
	"github.com/onehumancorp/mono/src/server/utils"
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

type anthropicCacheControl struct {
	Type string `json:"type"`
}

type anthropicSystem struct {
	Type         string                 `json:"type"`
	Text         string                 `json:"text"`
	CacheControl *anthropicCacheControl `json:"cache_control,omitempty"`
}

type anthropicMessage struct {
	Role    string             `json:"role"`
	Content []anthropicContent `json:"content"`
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
	InputSchema  map[string]interface{} `json:"input_schema"`
	CacheControl *anthropicCacheControl `json:"cache_control,omitempty"`
}

type anthropicRequest struct {
	Model     string             `json:"model"`
	MaxTokens int                `json:"max_tokens"`
	System    []anthropicSystem  `json:"system,omitempty"`
	Messages  []anthropicMessage `json:"messages"`
	Tools     []anthropicToolDef `json:"tools,omitempty"`
}

type anthropicResponse struct {
	Content []struct {
		Type  string                 `json:"type"`
		Text  string                 `json:"text,omitempty"`
		ID    string                 `json:"id,omitempty"`
		Name  string                 `json:"name,omitempty"`
		Input map[string]interface{} `json:"input,omitempty"`
	} `json:"content"`
	Usage struct {
		InputTokens              int `json:"input_tokens"`
		OutputTokens             int `json:"output_tokens"`
		CacheCreationInputTokens int `json:"cache_creation_input_tokens"`
		CacheReadInputTokens     int `json:"cache_read_input_tokens"`
	} `json:"usage"`
	StopReason string `json:"stop_reason"`
}

func (c *AnthropicClient) Chat(ctx context.Context, req ChatRequest) (ChatResponse, error) {
	var messages []anthropicMessage
	for _, m := range req.Messages {
		if m.Role == RoleSystem {
			continue
		}

		role := string(m.Role)
		if role == string(RoleTool) {
			role = "user"
		}

		messages = append(messages, anthropicMessage{
			Role: role,
			Content: []anthropicContent{
				{Type: "text", Text: utils.MinifyJSONString(m.Content)},
			},
		})
	}

	// Prompt Caching: Cache the last user message
	for i := len(messages) - 1; i >= 0; i-- {
		if messages[i].Role == "user" && len(messages[i].Content) > 0 {
			lastIdx := len(messages[i].Content) - 1
			messages[i].Content[lastIdx].CacheControl = &anthropicCacheControl{Type: "ephemeral"}
			break
		}
	}

	var systemBlocks []anthropicSystem
	if req.System != "" {
		systemBlocks = append(systemBlocks, anthropicSystem{
			Type: "text",
			Text: utils.MinifyJSONString(req.System),
			// Prompt Caching: Cache the system prompt
			CacheControl: &anthropicCacheControl{Type: "ephemeral"},
		})
	}

	var tools []anthropicToolDef
	for i, t := range req.Tools {
		var schema map[string]interface{}
		if len(t.Parameters) > 0 {
			_ = json.Unmarshal(t.Parameters, &schema)
		}
		if schema == nil {
			schema = map[string]interface{}{"type": "object", "properties": map[string]interface{}{}}
		}
		toolDef := anthropicToolDef{
			Name:        t.Name,
			Description: t.Description,
			InputSchema: schema,
		}
		// Add cache control to the last tool definition
		if i == len(req.Tools)-1 {
			toolDef.CacheControl = &anthropicCacheControl{Type: "ephemeral"}
		}
		tools = append(tools, toolDef)
	}

	payload := anthropicRequest{
		Model:     req.Model,
		MaxTokens: req.MaxTokens,
		System:    systemBlocks,
		Messages:  messages,
		Tools:     tools,
	}

	if payload.MaxTokens == 0 {
		payload.MaxTokens = 2048
	}

	body, _ := json.Marshal(payload)

	httpReq, err := http.NewRequestWithContext(ctx, "POST", "https://api.anthropic.com/v1/messages", bytes.NewReader(body))
	if err != nil {
		return ChatResponse{}, err
	}

	httpReq.Header.Set("x-api-key", c.APIKey)
	httpReq.Header.Set("anthropic-version", "2023-06-01")
	httpReq.Header.Set("content-type", "application/json")
	httpReq.Header.Set("anthropic-beta", "prompt-caching-2024-07-31")

	resp, err := c.Client.Do(httpReq)
	if err != nil {
		return ChatResponse{}, err
	}
	defer resp.Body.Close()

	respBody, _ := io.ReadAll(resp.Body)
	if resp.StatusCode != 200 {
		return ChatResponse{}, fmt.Errorf("anthropic api error (status %d): %s", resp.StatusCode, string(respBody))
	}

	var result anthropicResponse
	if err := json.Unmarshal(respBody, &result); err != nil {
		return ChatResponse{}, err
	}

	// Telemetry for prompt caching
	if result.Usage.CacheReadInputTokens > 0 {
		telemetry.RecordCacheHit(ctx, "anthropic_prompt_cache", "api")
		slog.DebugContext(ctx, "Anthropic Prompt Caching hit", "tokens", result.Usage.CacheReadInputTokens)
	}
	if result.Usage.CacheCreationInputTokens > 0 {
		telemetry.RecordCacheMiss(ctx, "anthropic_prompt_cache", "api")
		slog.DebugContext(ctx, "Anthropic Prompt Caching miss (creation)", "tokens", result.Usage.CacheCreationInputTokens)
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
