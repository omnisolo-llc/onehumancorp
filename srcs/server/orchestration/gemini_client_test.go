package orchestration

import (
"context"
"encoding/json"
"net/http"
"net/http/httptest"
"os"
"strings"
"testing"
)

// mockGeminiServer creates a mock Gemini API server for unit tests.
func mockGeminiServer(t *testing.T, responseText string) *httptest.Server {
t.Helper()
ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
w.Header().Set("Content-Type", "application/json")
resp := map[string]interface{}{
"candidates": []map[string]interface{}{
{
"content": map[string]interface{}{
"parts": []map[string]interface{}{
{"text": responseText},
},
},
},
},
}
json.NewEncoder(w).Encode(resp)
}))
return ts
}

func TestNewGeminiClient(t *testing.T) {
c := NewGeminiClient("test-key", "")
if c == nil {
t.Fatal("NewGeminiClient returned nil")
}
if c.APIKey != "test-key" {
t.Errorf("APIKey = %q, want %q", c.APIKey, "test-key")
}
if c.Model != DefaultGeminiModel {
t.Errorf("Model = %q, want %q", c.Model, DefaultGeminiModel)
}
}

func TestNewGeminiClientCustomModel(t *testing.T) {
c := NewGeminiClient("key", "gemini-1.5-pro")
if c.Model != "gemini-1.5-pro" {
t.Errorf("Model = %q, want gemini-1.5-pro", c.Model)
}
}

func TestGeminiClientReasonMock(t *testing.T) {
expectedText := "This is a mock Gemini response."
ts := mockGeminiServer(t, expectedText)
defer ts.Close()

// Override global URL for this test
origURL := GeminiAPIURL
GeminiAPIURL = ts.URL + "/%s?key=%s"
defer func() { GeminiAPIURL = origURL }()

c := NewGeminiClient("test-key", "gemini-2.0-flash")
result, err := c.Reason(context.Background(), "Hello, Gemini!")
if err != nil {
t.Fatalf("Reason returned unexpected error: %v", err)
}
if result != expectedText {
t.Errorf("Reason = %q, want %q", result, expectedText)
}
}

func TestGeminiClientReasonEmptyKey(t *testing.T) {
c := NewGeminiClient("", "")
_, err := c.Reason(context.Background(), "test")
if err == nil {
t.Fatal("expected error for empty API key, got nil")
}
if !strings.Contains(err.Error(), "API key") {
t.Errorf("error message = %q, expected to contain 'API key'", err.Error())
}
}

func TestGeminiClientReasonAPIError(t *testing.T) {
ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
w.WriteHeader(http.StatusUnauthorized)
w.Write([]byte(`{"error":{"code":401,"message":"Invalid API key"}}`))
}))
defer ts.Close()

origURL := GeminiAPIURL
GeminiAPIURL = ts.URL + "/%s?key=%s"
defer func() { GeminiAPIURL = origURL }()

c := NewGeminiClient("bad-key", "")
_, err := c.Reason(context.Background(), "test")
if err == nil {
t.Fatal("expected error for bad API key, got nil")
}
}

func TestGeminiClientReasonEmptyCandidates(t *testing.T) {
ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
w.Header().Set("Content-Type", "application/json")
w.Write([]byte(`{"candidates":[]}`))
}))
defer ts.Close()

origURL := GeminiAPIURL
GeminiAPIURL = ts.URL + "/%s?key=%s"
defer func() { GeminiAPIURL = origURL }()

c := NewGeminiClient("test-key", "")
_, err := c.Reason(context.Background(), "test")
if err == nil {
t.Fatal("expected error for empty candidates, got nil")
}
}

func TestGeminiClientGenerateEmbeddingNotImplemented(t *testing.T) {
c := NewGeminiClient("key", "")
_, err := c.GenerateEmbedding(context.Background(), "test")
if err == nil {
t.Fatal("expected error, got nil")
}
}

// TestGeminiClientReasonLive tests with a real Gemini API key if GEMINI_API_KEY is set.
func TestGeminiClientReasonLive(t *testing.T) {
key := os.Getenv("GEMINI_API_KEY")
if key == "" {
t.Skip("GEMINI_API_KEY not set, skipping live Gemini test")
}

c := NewGeminiClient(key, "gemini-2.0-flash")
ctx := context.Background()
result, err := c.Reason(ctx, "Say exactly 'Hello, World!' and nothing else.")
if err != nil {
t.Fatalf("Live Gemini Reason failed: %v", err)
}
if strings.TrimSpace(result) == "" {
t.Fatal("Live Gemini returned empty response")
}
t.Logf("Gemini response: %s", result)
}
