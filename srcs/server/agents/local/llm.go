package local

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"log/slog"
	"net/http"
	"os"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"github.com/onehumancorp/mono/srcs/server/utils"
)

// LLMClient is the abstraction used by the agent loop to interact with a language model.
// It returns a sequence of content blocks (text and/or tool-use) for each turn.
type LLMClient interface {
	// Complete sends a conversation (system + messages) to the LLM and returns the
	// assistant reply.  It supports tool-use: if the model requests tool calls they
	// are returned in the AssistantMessage.ToolUses slice.
	Complete(ctx context.Context, req CompletionRequest) (*AssistantMessage, error)
}

// CompletionRequest is the input to a single LLM turn.
type CompletionRequest struct {
	SystemPrompt string
	Messages     []ConversationMessage
	Tools        []ToolDefinition
	MaxTokens    int
	Temperature  float64
}

// ConversationMessage represents one turn in the conversation.
type ConversationMessage struct {
	Role    string        // "user" or "assistant"
	Content []ContentPart // text and/or tool-result blocks
}

// ContentPart is a discriminated union of text, tool-use, and tool-result parts.
type ContentPart struct {
	Type string // "text", "tool_use", "tool_result"

	// text
	Text string

	// tool_use
	ToolUseID string
	ToolName  string
	ToolInput map[string]interface{}

	// tool_result
	ResultForToolUseID string
	ResultContent      string
	IsError            bool
}

// ToolDefinition describes a tool available to the agent.
type ToolDefinition struct {
	Name        string
	Description string
	InputSchema map[string]interface{} // JSON Schema object
}

// AssistantMessage is the model's reply for one turn.
type AssistantMessage struct {
	Text         string
	ToolUses     []ToolUseRequest
	InputTokens  int64
	OutputTokens int64
	StopReason   string // "end_turn", "tool_use", "max_tokens", etc.
}

// ToolUseRequest is a single tool-call from the model.
type ToolUseRequest struct {
	ID    string
	Name  string
	Input map[string]interface{}
}

// ─── Anthropic Messages API client ───────────────────────────────────────────

const defaultAnthropicModel = "claude-sonnet-4-5"

type anthropicClient struct {
	apiKey   string
	model    string
	endpoint string
	hc       *http.Client
}

// NewAnthropicClient creates a client that calls the Anthropic Messages API.
// Set ANTHROPIC_API_KEY in the environment or supply apiKey directly.
func NewAnthropicClient(apiKey, model, endpoint string) LLMClient {
	if apiKey == "" {
		apiKey = os.Getenv("ANTHROPIC_API_KEY")
	}
	if model == "" {
		model = os.Getenv("OHC_LOCAL_AGENT_MODEL")
	}
	if model == "" {
		model = defaultAnthropicModel
	}
	if endpoint == "" {
		endpoint = os.Getenv("ANTHROPIC_API_BASE_URL")
	}
	if endpoint == "" {
		endpoint = "https://api.anthropic.com"
	}
	endpoint = strings.TrimRight(endpoint, "/")
	return &anthropicClient{
		apiKey:   apiKey,
		model:    model,
		endpoint: endpoint + "/v1/messages",
		hc:       &http.Client{Timeout: 120 * time.Second},
	}
}

// anthropicRequest is the JSON body for POST /v1/messages.
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
	InputSchema  map[string]interface{} `json:"input_schema"`
	CacheControl *anthropicCacheControl `json:"cache_control,omitempty"`
}

type anthropicResponse struct {
	Content    []anthropicContent `json:"content"`
	StopReason string             `json:"stop_reason"`
	Usage      struct {
		InputTokens              int64 `json:"input_tokens"`
		OutputTokens             int64 `json:"output_tokens"`
		CacheCreationInputTokens int64 `json:"cache_creation_input_tokens"`
		CacheReadInputTokens     int64 `json:"cache_read_input_tokens"`
	} `json:"usage"`
}

func (c *anthropicClient) Complete(ctx context.Context, req CompletionRequest) (*AssistantMessage, error) {
	start := time.Now()
	defer func() {
		telemetry.RecordLLMNetworkLatency(ctx, c.model, time.Since(start).Seconds())
	}()
	maxTok := req.MaxTokens
	if maxTok <= 0 {
		maxTok = 2048
	} else if maxTok > 4096 {
		maxTok = 4096
	}

	msgs := make([]anthropicMessage, 0, len(req.Messages))
	for _, m := range req.Messages {
		am := anthropicMessage{Role: m.Role}
		for _, p := range m.Content {
			switch p.Type {
			case "text":
				am.Content = append(am.Content, anthropicContent{Type: "text", Text: p.Text})
			case "tool_use":
				am.Content = append(am.Content, anthropicContent{
					Type:  "tool_use",
					ID:    p.ToolUseID,
					Name:  p.ToolName,
					Input: p.ToolInput,
				})
			case "tool_result":
				var cont interface{} = p.ResultContent
				ac := anthropicContent{
					Type:      "tool_result",
					ToolUseID: p.ResultForToolUseID,
					Content:   cont,
					IsError:   p.IsError,
				}
				am.Content = append(am.Content, ac)
			}
		}
		msgs = append(msgs, am)
	}

	// Add cache control to the last user message if any to cache the multi-turn conversation
	for i := len(msgs) - 1; i >= 0; i-- {
		if msgs[i].Role == "user" && len(msgs[i].Content) > 0 {
			lastContentIdx := len(msgs[i].Content) - 1
			msgs[i].Content[lastContentIdx].CacheControl = &anthropicCacheControl{Type: "ephemeral"}
			break
		}
	}

	tools := make([]anthropicToolDef, 0, len(req.Tools))
	for i, t := range req.Tools {
		schema := t.InputSchema
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

	var systemBlocks []anthropicSystem
	if req.SystemPrompt != "" {
		systemBlocks = append(systemBlocks, anthropicSystem{
			Type: "text",
			Text: utils.MinifyJSONString(req.SystemPrompt),
			// Cache the system prompt
			CacheControl: &anthropicCacheControl{Type: "ephemeral"},
		})
	}

	body := anthropicRequest{
		Model:     c.model,
		MaxTokens: maxTok,
		System:    systemBlocks,
		Messages:  msgs,
		Tools:     tools,
	}

	raw, err := json.Marshal(body)
	if err != nil {
		return nil, fmt.Errorf("anthropic: marshal request: %w", err)
	}

	httpReq, err := http.NewRequestWithContext(ctx, http.MethodPost, c.endpoint, bytes.NewReader(raw))
	if err != nil {
		return nil, fmt.Errorf("anthropic: build request: %w", err)
	}
	httpReq.Header.Set("Content-Type", "application/json")
	httpReq.Header.Set("x-api-key", c.apiKey)
	httpReq.Header.Set("anthropic-version", "2023-06-01")
	httpReq.Header.Set("anthropic-beta", "prompt-caching-2024-07-31") // Ensure prompt caching is active

	resp, err := c.hc.Do(httpReq)
	if err != nil {
		return nil, fmt.Errorf("anthropic: http: %w", err)
	}
	defer resp.Body.Close()

	respBody, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, fmt.Errorf("anthropic: read body: %w", err)
	}

	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("anthropic: status %d: %s", resp.StatusCode, string(respBody))
	}

	var ar anthropicResponse
	if err := json.Unmarshal(respBody, &ar); err != nil {
		return nil, fmt.Errorf("anthropic: unmarshal response: %w", err)
	}

	out := &AssistantMessage{
		StopReason:   ar.StopReason,
		InputTokens:  ar.Usage.InputTokens, // This usually includes the total input tokens billed (non-cached + cache_creation)
		OutputTokens: ar.Usage.OutputTokens,
	}

	// Add telemetry for caching efficiency
	if ar.Usage.CacheReadInputTokens > 0 {
		telemetry.RecordCacheHit(ctx, "anthropic_prompt_cache", "api")
		slog.DebugContext(ctx, "Anthropic Prompt Caching", "cache_read_input_tokens", ar.Usage.CacheReadInputTokens)
	}
	if ar.Usage.CacheCreationInputTokens > 0 {
		telemetry.RecordCacheMiss(ctx, "anthropic_prompt_cache", "api")
		slog.DebugContext(ctx, "Anthropic Prompt Caching", "cache_creation_input_tokens", ar.Usage.CacheCreationInputTokens)
	}

	for _, c := range ar.Content {
		switch c.Type {
		case "text":
			out.Text += c.Text
		case "tool_use":
			out.ToolUses = append(out.ToolUses, ToolUseRequest{
				ID:    c.ID,
				Name:  c.Name,
				Input: c.Input,
			})
		}
	}
	return out, nil
}

// ─── Ollama / OpenAI-compatible client ───────────────────────────────────────

type openAICompatClient struct {
	endpoint string
	apiKey   string
	model    string
	hc       *http.Client
}

// NewOllamaClient creates a client for a local Ollama server.
func NewOllamaClient(endpoint, model string) LLMClient {
	if endpoint == "" {
		endpoint = os.Getenv("OHC_LOCAL_LLM_ENDPOINT")
	}
	if endpoint == "" {
		endpoint = "http://127.0.0.1:11434/v1"
	}
	if model == "" {
		model = os.Getenv("OHC_LOCAL_MODEL_NAME")
	}
	if model == "" {
		model = "llama3"
	}
	return &openAICompatClient{
		endpoint: strings.TrimRight(endpoint, "/") + "/chat/completions",
		model:    model,
		hc:       &http.Client{Timeout: 120 * time.Second},
	}
}

// NewOpenAICompatClient creates a client for any OpenAI-compatible endpoint.
func NewOpenAICompatClient(endpoint, apiKey, model string) LLMClient {
	if endpoint == "" {
		endpoint = "https://api.openai.com/v1"
	}
	if apiKey == "" {
		apiKey = os.Getenv("OPENAI_API_KEY")
	}
	if model == "" {
		model = "gpt-4o"
	}
	return &openAICompatClient{
		endpoint: strings.TrimRight(endpoint, "/") + "/chat/completions",
		apiKey:   apiKey,
		model:    model,
		hc:       &http.Client{Timeout: 120 * time.Second},
	}
}

type openAIRequest struct {
	Model    string          `json:"model"`
	Messages []openAIMessage `json:"messages"`
	Tools    []openAITool    `json:"tools,omitempty"`
	MaxTokens int            `json:"max_tokens,omitempty"`
}

type openAIMessage struct {
	Role       string            `json:"role"`
	Content    interface{}       `json:"content"`
	ToolCallID string            `json:"tool_call_id,omitempty"`
	ToolCalls  []openAIToolCall  `json:"tool_calls,omitempty"`
}

type openAIToolCall struct {
	ID       string `json:"id"`
	Type     string `json:"type"`
	Function struct {
		Name      string `json:"name"`
		Arguments string `json:"arguments"`
	} `json:"function"`
}

type openAITool struct {
	Type     string `json:"type"`
	Function struct {
		Name        string                 `json:"name"`
		Description string                 `json:"description"`
		Parameters  map[string]interface{} `json:"parameters"`
	} `json:"function"`
}

type openAIResponse struct {
	Choices []struct {
		Message    openAIMessage `json:"message"`
		FinishReason string      `json:"finish_reason"`
	} `json:"choices"`
	Usage struct {
		PromptTokens     int64 `json:"prompt_tokens"`
		CompletionTokens int64 `json:"completion_tokens"`
	} `json:"usage"`
}

func (c *openAICompatClient) Complete(ctx context.Context, req CompletionRequest) (*AssistantMessage, error) {
	start := time.Now()
	defer func() {
		telemetry.RecordLLMNetworkLatency(ctx, c.model, time.Since(start).Seconds())
	}()
	maxTok := req.MaxTokens
	if maxTok <= 0 {
		maxTok = 2048
	} else if maxTok > 4096 {
		maxTok = 4096
	}

	var msgs []openAIMessage
	if req.SystemPrompt != "" {
		msgs = append(msgs, openAIMessage{Role: "system", Content: utils.MinifyJSONString(req.SystemPrompt)})
	}
	for _, m := range req.Messages {
		switch m.Role {
		case "user":
			// Collect text parts (tool-results become separate messages in OpenAI format)
			var textParts []string
			for _, p := range m.Content {
				switch p.Type {
				case "text":
					textParts = append(textParts, utils.MinifyJSONString(p.Text))
				case "tool_result":
					// Tool results are separate messages with role "tool"
					msgs = append(msgs, openAIMessage{
						Role:       "tool",
						Content:    p.ResultContent,
						ToolCallID: p.ResultForToolUseID,
					})
				}
			}
			if len(textParts) > 0 {
				msgs = append(msgs, openAIMessage{Role: "user", Content: strings.Join(textParts, "\n")})
			}
		case "assistant":
			am := openAIMessage{Role: "assistant"}
			var textParts []string
			for _, p := range m.Content {
				switch p.Type {
				case "text":
					textParts = append(textParts, utils.MinifyJSONString(p.Text))
				case "tool_use":
					argBytes, _ := json.Marshal(p.ToolInput)
					am.ToolCalls = append(am.ToolCalls, openAIToolCall{
						ID:   p.ToolUseID,
						Type: "function",
						Function: struct {
							Name      string `json:"name"`
							Arguments string `json:"arguments"`
						}{Name: p.ToolName, Arguments: string(argBytes)},
					})
				}
			}
			if len(textParts) > 0 {
				am.Content = strings.Join(textParts, "\n")
			}
			msgs = append(msgs, am)
		}
	}

	var tools []openAITool
	for _, t := range req.Tools {
		schema := t.InputSchema
		if schema == nil {
			schema = map[string]interface{}{"type": "object", "properties": map[string]interface{}{}}
		}
		ot := openAITool{Type: "function"}
		ot.Function.Name = t.Name
		ot.Function.Description = t.Description
		ot.Function.Parameters = schema
		tools = append(tools, ot)
	}

	body := openAIRequest{Model: c.model, Messages: msgs, Tools: tools, MaxTokens: maxTok}
	raw, err := json.Marshal(body)
	if err != nil {
		return nil, fmt.Errorf("openai-compat: marshal: %w", err)
	}

	httpReq, err := http.NewRequestWithContext(ctx, http.MethodPost, c.endpoint, bytes.NewReader(raw))
	if err != nil {
		return nil, fmt.Errorf("openai-compat: build request: %w", err)
	}
	httpReq.Header.Set("Content-Type", "application/json")
	if c.apiKey != "" {
		httpReq.Header.Set("Authorization", "Bearer "+c.apiKey)
	}

	resp, err := c.hc.Do(httpReq)
	if err != nil {
		return nil, fmt.Errorf("openai-compat: http: %w", err)
	}
	defer resp.Body.Close()

	respBody, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, fmt.Errorf("openai-compat: read body: %w", err)
	}

	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("openai-compat: status %d: %s", resp.StatusCode, string(respBody))
	}

	var or openAIResponse
	if err := json.Unmarshal(respBody, &or); err != nil {
		return nil, fmt.Errorf("openai-compat: unmarshal: %w", err)
	}
	if len(or.Choices) == 0 {
		return nil, fmt.Errorf("openai-compat: empty choices")
	}

	choice := or.Choices[0]
	out := &AssistantMessage{
		StopReason:   choice.FinishReason,
		InputTokens:  or.Usage.PromptTokens,
		OutputTokens: or.Usage.CompletionTokens,
	}
	if s, ok := choice.Message.Content.(string); ok {
		out.Text = s
	}
	for _, tc := range choice.Message.ToolCalls {
		var input map[string]interface{}
		_ = json.Unmarshal([]byte(tc.Function.Arguments), &input)
		out.ToolUses = append(out.ToolUses, ToolUseRequest{
			ID:    tc.ID,
			Name:  tc.Function.Name,
			Input: input,
		})
	}
	return out, nil
}

// defaultLLMClient selects an LLM client based on environment variables.
// Priority:
//  1. ANTHROPIC_API_KEY → Anthropic Messages API
//  2. OPENAI_API_KEY    → OpenAI-compatible (endpoint from OPENAI_API_BASE or default)
//  3. OHC_LOCAL_LLM_ENDPOINT → Ollama / local OpenAI-compat
func defaultLLMClient() LLMClient {
	var client LLMClient
	if key := os.Getenv("ANTHROPIC_API_KEY"); key != "" {
		client = NewAnthropicClient(key, "", "")
	} else if key := os.Getenv("OPENAI_API_KEY"); key != "" {
		endpoint := os.Getenv("OPENAI_API_BASE")
		model := os.Getenv("OHC_LOCAL_AGENT_MODEL")
		client = NewOpenAICompatClient(endpoint, key, model)
	} else {
		client = NewOllamaClient("", "")
	}

	// We wrap the selected client in a CachedLLMClient, but since we don't have DB/Redis
	// injected at this level, we just return the client. In a real environment,
	// the agent constructor should inject the db provider and wrap it.
	return client
}
