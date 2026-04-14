package agentgrpc

import (
	"context"
	"encoding/json"
	"fmt"
	"iter"
	"os"
	"strings"
	"time"

	"google.golang.org/genai"

	"google.golang.org/adk/model"
)

// ── model.LLM adapters ───────────────────────────────────────────────────────
//
// Each adapter wraps one of our existing HTTP-based LLM clients and
// translates between ADK's genai format and the respective API wire format.
// All adapters implement model.LLM and are non-streaming (they return a single
// complete LLMResponse with Partial=false, TurnComplete=true).

// adkAnthropicModel implements model.LLM backed by Anthropic's Messages API.
type adkAnthropicModel struct {
	apiKey     string
	modelName  string
	httpClient interface {
		Do(*httpReq) (*httpResp, error)
	}
}

// We use a minimal http helper type to avoid import cycles; the real
// implementation uses net/http directly in each adapter file.

// NewAnthropicModel creates an adk model.LLM for Anthropic Claude.
func NewAnthropicModel(modelName, apiKey string) model.LLM {
	return &adkAnthropicModel{
		apiKey:    apiKey,
		modelName: modelName,
	}
}

// Name implements model.LLM.
func (m *adkAnthropicModel) Name() string { return m.modelName }

// GenerateContent implements model.LLM.
func (m *adkAnthropicModel) GenerateContent(ctx context.Context, req *model.LLMRequest, _ bool) iter.Seq2[*model.LLMResponse, error] {
	return func(yield func(*model.LLMResponse, error) bool) {
		resp, err := m.generate(ctx, req)
		yield(resp, err)
	}
}

func (m *adkAnthropicModel) generate(ctx context.Context, req *model.LLMRequest) (*model.LLMResponse, error) {
	type antMessage struct {
		Role    string `json:"role"`
		Content any    `json:"content"`
	}
	type antTool struct {
		Name        string `json:"name"`
		Description string `json:"description,omitempty"`
		InputSchema any    `json:"input_schema"`
	}

	var messages []antMessage
	for _, c := range req.Contents {
		role := c.Role
		if role == "model" {
			role = "assistant"
		}
		content := genaiPartsToAnthropic(c.Parts)
		messages = append(messages, antMessage{Role: role, Content: content})
	}

	var tools []antTool
	var systemPrompt string
	if req.Config != nil {
		if req.Config.SystemInstruction != nil {
			for _, p := range req.Config.SystemInstruction.Parts {
				systemPrompt += p.Text
			}
		}
		for _, t := range req.Config.Tools {
			for _, fd := range t.FunctionDeclarations {
				tools = append(tools, antTool{
					Name:        fd.Name,
					Description: fd.Description,
					InputSchema: functionDeclToSchema(fd),
				})
			}
		}
	}

	maxTokens := 2048
	if req.Config != nil && req.Config.MaxOutputTokens > 0 {
		maxTokens = int(req.Config.MaxOutputTokens)
	}

	payload := map[string]any{
		"model":      m.modelName,
		"max_tokens": maxTokens,
		"messages":   messages,
	}
	if systemPrompt != "" {
		payload["system"] = systemPrompt
	}
	if len(tools) > 0 {
		payload["tools"] = tools
	}

	body, err := json.Marshal(payload)
	if err != nil {
		return nil, err
	}

	respBody, err := doHTTP(ctx, "POST", "https://api.anthropic.com/v1/messages", body, map[string]string{
		"x-api-key":         m.apiKey,
		"anthropic-version": "2023-06-01",
		"content-type":      "application/json",
	}, 2*time.Minute)
	if err != nil {
		return nil, err
	}

	var result struct {
		Content []struct {
			Type  string         `json:"type"`
			Text  string         `json:"text"`
			ID    string         `json:"id"`
			Name  string         `json:"name"`
			Input map[string]any `json:"input"`
		} `json:"content"`
		StopReason string `json:"stop_reason"`
	}
	if err := json.Unmarshal(respBody, &result); err != nil {
		return nil, fmt.Errorf("anthropic: decode response: %w", err)
	}

	var parts []*genai.Part
	for _, block := range result.Content {
		switch block.Type {
		case "text":
			parts = append(parts, genai.NewPartFromText(block.Text))
		case "tool_use":
			parts = append(parts, &genai.Part{
				FunctionCall: &genai.FunctionCall{
					Name: block.Name,
					Args: block.Input,
				},
			})
		}
	}

	return &model.LLMResponse{
		Content:      &genai.Content{Role: "model", Parts: parts},
		TurnComplete: result.StopReason == "end_turn" || result.StopReason == "stop_sequence",
	}, nil
}

// ── OpenAI model ─────────────────────────────────────────────────────────────

// adkOpenAIModel implements model.LLM backed by OpenAI's Chat Completions API.
type adkOpenAIModel struct {
	apiKey    string
	modelName string
	endpoint  string
}

// NewOpenAIModel creates an adk model.LLM for OpenAI.
// endpoint defaults to https://api.openai.com/v1/chat/completions; pass a
// custom URL to use compatible providers (e.g. Azure OpenAI, Ollama).
func NewOpenAIModel(modelName, apiKey, endpoint string) model.LLM {
	if endpoint == "" {
		endpoint = "https://api.openai.com/v1/chat/completions"
	}
	return &adkOpenAIModel{
		apiKey:    apiKey,
		modelName: modelName,
		endpoint:  endpoint,
	}
}

// NewOllamaModel creates an adk model.LLM backed by a local Ollama instance
// using the OpenAI-compatible endpoint.
func NewOllamaModel(modelName, endpoint string) model.LLM {
	if endpoint == "" {
		endpoint = "http://localhost:11434/v1/chat/completions"
	}
	return &adkOpenAIModel{
		apiKey:    "ollama", // Ollama ignores the key
		modelName: modelName,
		endpoint:  endpoint,
	}
}

func (m *adkOpenAIModel) Name() string { return m.modelName }

func (m *adkOpenAIModel) GenerateContent(ctx context.Context, req *model.LLMRequest, _ bool) iter.Seq2[*model.LLMResponse, error] {
	return func(yield func(*model.LLMResponse, error) bool) {
		resp, err := m.generate(ctx, req)
		yield(resp, err)
	}
}

func (m *adkOpenAIModel) generate(ctx context.Context, req *model.LLMRequest) (*model.LLMResponse, error) {
	type oaiMsg struct {
		Role       string `json:"role"`
		Content    any    `json:"content,omitempty"`
		ToolCalls  any    `json:"tool_calls,omitempty"`
		ToolCallID string `json:"tool_call_id,omitempty"`
		Name       string `json:"name,omitempty"`
	}
	type oaiFD struct {
		Name        string `json:"name"`
		Description string `json:"description,omitempty"`
		Parameters  any    `json:"parameters"`
	}
	type oaiTool struct {
		Type     string `json:"type"`
		Function oaiFD  `json:"function"`
	}

	var messages []oaiMsg

	if req.Config != nil && req.Config.SystemInstruction != nil {
		sys := ""
		for _, p := range req.Config.SystemInstruction.Parts {
			sys += p.Text
		}
		if sys != "" {
			messages = append(messages, oaiMsg{Role: "system", Content: sys})
		}
	}

	for _, c := range req.Contents {
		switch c.Role {
		case "user":
			text := ""
			var toolResults []any
			for _, p := range c.Parts {
				if p.FunctionResponse != nil {
					toolResults = append(toolResults, map[string]any{
						"role":         "tool",
						"tool_call_id": p.FunctionResponse.Name, // adk uses Name as the call ID
						"content":      jsonStringify(p.FunctionResponse.Response),
					})
				} else {
					text += p.Text
				}
			}
			if len(toolResults) > 0 {
				for _, tr := range toolResults {
					m2 := tr.(map[string]any)
					messages = append(messages, oaiMsg{
						Role:       "tool",
						Content:    m2["content"],
						ToolCallID: m2["tool_call_id"].(string),
					})
				}
			} else {
				messages = append(messages, oaiMsg{Role: "user", Content: text})
			}
		case "model":
			text := ""
			var toolCalls []any
			for _, p := range c.Parts {
				if p.FunctionCall != nil {
					argsJSON, _ := json.Marshal(p.FunctionCall.Args)
					toolCalls = append(toolCalls, map[string]any{
						"id":   p.FunctionCall.Name,
						"type": "function",
						"function": map[string]any{
							"name":      p.FunctionCall.Name,
							"arguments": string(argsJSON),
						},
					})
				} else {
					text += p.Text
				}
			}
			msg := oaiMsg{Role: "assistant"}
			if len(toolCalls) > 0 {
				msg.ToolCalls = toolCalls
			} else {
				msg.Content = text
			}
			messages = append(messages, msg)
		}
	}

	maxTokens := 2048
	if req.Config != nil && req.Config.MaxOutputTokens > 0 {
		maxTokens = int(req.Config.MaxOutputTokens)
	}

	payload := map[string]any{
		"model":      m.modelName,
		"messages":   messages,
		"max_tokens": maxTokens,
	}

	if req.Config != nil && len(req.Config.Tools) > 0 {
		var tools []oaiTool
		for _, t := range req.Config.Tools {
			for _, fd := range t.FunctionDeclarations {
				tools = append(tools, oaiTool{
					Type: "function",
					Function: oaiFD{
						Name:        fd.Name,
						Description: fd.Description,
						Parameters:  functionDeclToSchema(fd),
					},
				})
			}
		}
		if len(tools) > 0 {
			payload["tools"] = tools
		}
	}

	body, err := json.Marshal(payload)
	if err != nil {
		return nil, err
	}

	headers := map[string]string{
		"Authorization": "Bearer " + m.apiKey,
		"Content-Type":  "application/json",
	}

	respBody, err := doHTTP(ctx, "POST", m.endpoint, body, headers, 5*time.Minute)
	if err != nil {
		return nil, err
	}

	var result struct {
		Choices []struct {
			Message struct {
				Content   *string `json:"content"`
				ToolCalls []struct {
					ID       string `json:"id"`
					Function struct {
						Name      string `json:"name"`
						Arguments string `json:"arguments"`
					} `json:"function"`
				} `json:"tool_calls"`
			} `json:"message"`
			FinishReason string `json:"finish_reason"`
		} `json:"choices"`
	}
	if err := json.Unmarshal(respBody, &result); err != nil {
		return nil, fmt.Errorf("openai: decode response: %w", err)
	}
	if len(result.Choices) == 0 {
		return nil, fmt.Errorf("openai: no choices in response")
	}

	choice := result.Choices[0]
	var parts []*genai.Part
	if choice.Message.Content != nil && *choice.Message.Content != "" {
		parts = append(parts, genai.NewPartFromText(*choice.Message.Content))
	}
	for _, tc := range choice.Message.ToolCalls {
		var args map[string]any
		if err := json.Unmarshal([]byte(tc.Function.Arguments), &args); err != nil {
			args = map[string]any{"_raw": tc.Function.Arguments}
		}
		parts = append(parts, &genai.Part{
			FunctionCall: &genai.FunctionCall{
				Name: tc.Function.Name,
				Args: args,
			},
		})
	}

	return &model.LLMResponse{
		Content:      &genai.Content{Role: "model", Parts: parts},
		TurnComplete: choice.FinishReason == "stop",
	}, nil
}

// ── helper utilities ─────────────────────────────────────────────────────────

// genaiPartsToAnthropic converts genai Parts to Anthropic content blocks.
func genaiPartsToAnthropic(parts []*genai.Part) any {
	type textBlock struct {
		Type string `json:"type"`
		Text string `json:"text"`
	}
	type toolUseBlock struct {
		Type  string `json:"type"`
		ID    string `json:"id"`
		Name  string `json:"name"`
		Input any    `json:"input"`
	}
	type toolResultBlock struct {
		Type      string `json:"type"`
		ToolUseID string `json:"tool_use_id"`
		Content   string `json:"content"`
	}

	var blocks []any
	for _, p := range parts {
		switch {
		case p.FunctionCall != nil:
			blocks = append(blocks, toolUseBlock{
				Type:  "tool_use",
				ID:    p.FunctionCall.Name,
				Name:  p.FunctionCall.Name,
				Input: p.FunctionCall.Args,
			})
		case p.FunctionResponse != nil:
			blocks = append(blocks, toolResultBlock{
				Type:      "tool_result",
				ToolUseID: p.FunctionResponse.Name,
				Content:   jsonStringify(p.FunctionResponse.Response),
			})
		default:
			if p.Text != "" {
				blocks = append(blocks, textBlock{Type: "text", Text: p.Text})
			}
		}
	}
	if len(blocks) == 1 {
		if tb, ok := blocks[0].(textBlock); ok {
			return tb.Text
		}
	}
	return blocks
}

// functionDeclToSchema converts a genai FunctionDeclaration's parameters to a
// JSON-schema-compatible map suitable for Anthropic / OpenAI tool definitions.
func functionDeclToSchema(fd *genai.FunctionDeclaration) any {
	if fd.ParametersJsonSchema != nil {
		return fd.ParametersJsonSchema
	}
	if fd.Parameters != nil {
		return fd.Parameters
	}
	return map[string]any{"type": "object", "properties": map[string]any{}}
}

func jsonStringify(v any) string {
	if v == nil {
		return ""
	}
	if s, ok := v.(string); ok {
		return s
	}
	b, _ := json.Marshal(v)
	return string(b)
}

// newADKModel selects and constructs the appropriate adk model.LLM.
func newADKModel(cfg AgentConfig) (model.LLM, error) {
	provider := cfg.LLMProvider
	if provider == "" {
		if os.Getenv("ANTHROPIC_API_KEY") != "" {
			provider = "anthropic"
		} else if os.Getenv("OPENAI_API_KEY") != "" {
			provider = "openai"
		} else {
			provider = "ollama"
		}
	}

	modelName := cfg.Model
	switch provider {
	case "anthropic":
		key := os.Getenv("ANTHROPIC_API_KEY")
		if key == "" {
			return nil, fmt.Errorf("ANTHROPIC_API_KEY is required for provider %q", provider)
		}
		if modelName == "" {
			modelName = "claude-3-5-sonnet-20241022"
		}
		return NewAnthropicModel(modelName, key), nil
	case "openai":
		key := os.Getenv("OPENAI_API_KEY")
		if key == "" {
			return nil, fmt.Errorf("OPENAI_API_KEY is required for provider %q", provider)
		}
		if modelName == "" {
			modelName = "gpt-4o"
		}
		return NewOpenAIModel(modelName, key, cfg.LLMEndpoint), nil
	case "ollama":
		if modelName == "" {
			modelName = "llama3"
		}
		endpoint := cfg.LLMEndpoint
		if endpoint == "" {
			endpoint = os.Getenv("OHC_LOCAL_LLM_ENDPOINT")
		}
		return NewOllamaModel(modelName, endpoint), nil
	default:
		return nil, fmt.Errorf("unknown LLM provider %q", provider)
	}
}

// extractFinalText walks a genai.Content and returns concatenated text.
func extractFinalText(c *genai.Content) string {
	if c == nil {
		return ""
	}
	var sb strings.Builder
	for _, p := range c.Parts {
		sb.WriteString(p.Text)
	}
	return sb.String()
}
