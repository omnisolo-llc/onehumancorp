package orchestration

import (
"bytes"
"context"
"encoding/json"
"fmt"
"io"
"net/http"
"time"
)

// GeminiAPIURL is the endpoint for Google Gemini API (generateContent).
// Can be overridden in tests.
var GeminiAPIURL = "https://generativelanguage.googleapis.com/v1beta/models/%s:generateContent?key=%s"

// DefaultGeminiModel is the default Gemini model to use.
const DefaultGeminiModel = "gemini-2.0-flash"

// GeminiClient is an implementation of MinimaxClient backed by Google Gemini.
// It satisfies the same MinimaxClient interface so it can be used as a drop-in
// replacement for Minimax-based reasoning in tests and production alike.
type GeminiClient struct {
APIKey    string
Model     string
httpClient *http.Client
}

// NewGeminiClient creates a new GeminiClient with the given API key.
// The model defaults to DefaultGeminiModel when empty.
func NewGeminiClient(apiKey, model string) *GeminiClient {
if model == "" {
model = DefaultGeminiModel
}
return &GeminiClient{
APIKey: apiKey,
Model:  model,
httpClient: &http.Client{
Timeout: 120 * time.Second,
},
}
}

// Reason sends a prompt to Gemini and returns the generated text response.
// This satisfies the MinimaxClient interface.
func (g *GeminiClient) Reason(ctx context.Context, prompt string) (string, error) {
if g.APIKey == "" {
return "", fmt.Errorf("gemini API key is not configured")
}

url := fmt.Sprintf(GeminiAPIURL, g.Model, g.APIKey)

payload := map[string]interface{}{
"contents": []map[string]interface{}{
{
"parts": []map[string]string{
{"text": prompt},
},
},
},
"generationConfig": map[string]interface{}{
"temperature":     0.7,
"maxOutputTokens": 512,
},
}

body, err := json.Marshal(payload)
if err != nil {
return "", fmt.Errorf("gemini marshal payload: %w", err)
}

req, err := http.NewRequestWithContext(ctx, http.MethodPost, url, bytes.NewReader(body))
if err != nil {
return "", fmt.Errorf("gemini create request: %w", err)
}
req.Header.Set("Content-Type", "application/json")

resp, err := g.httpClient.Do(req)
if err != nil {
return "", fmt.Errorf("gemini HTTP request: %w", err)
}
defer resp.Body.Close()

respBody, err := io.ReadAll(resp.Body)
if err != nil {
return "", fmt.Errorf("gemini read response: %w", err)
}

if resp.StatusCode != http.StatusOK {
return "", fmt.Errorf("gemini API error (status %d): %s", resp.StatusCode, string(respBody))
}

var result struct {
Candidates []struct {
Content struct {
Parts []struct {
Text string `json:"text"`
} `json:"parts"`
} `json:"content"`
} `json:"candidates"`
Error *struct {
Message string `json:"message"`
Code    int    `json:"code"`
} `json:"error,omitempty"`
}

if err := json.Unmarshal(respBody, &result); err != nil {
return "", fmt.Errorf("gemini decode response: %w", err)
}

if result.Error != nil {
return "", fmt.Errorf("gemini API error %d: %s", result.Error.Code, result.Error.Message)
}

if len(result.Candidates) == 0 || len(result.Candidates[0].Content.Parts) == 0 {
return "", fmt.Errorf("gemini returned empty candidates")
}

return result.Candidates[0].Content.Parts[0].Text, nil
}

// GenerateEmbedding is not implemented for the Gemini client in this context.
// It returns an error to satisfy the MinimaxClient interface.
func (g *GeminiClient) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
return nil, fmt.Errorf("GeminiClient.GenerateEmbedding not implemented")
}
