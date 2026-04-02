package orchestration

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestMinimaxClientReasonSuccess(t *testing.T) {
	// Start a local HTTP server
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.Header.Get("Authorization") != "Bearer valid-key" {
			w.WriteHeader(http.StatusUnauthorized)
			return
		}
		resp := map[string]interface{}{
			"choices": []map[string]interface{}{
				{
					"message": map[string]string{
						"content": "42",
					},
				},
			},
		}
		json.NewEncoder(w).Encode(resp)
	}))
	defer ts.Close()

	// Use our own internal package variable URL to point to the test server
	originalURL := MinimaxAPIURL
	MinimaxAPIURL = ts.URL
	defer func() { MinimaxAPIURL = originalURL }()

	client := NewMinimaxClient("valid-key")
	res, err := client.Reason(context.Background(), "What is 6x7?")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if res != "42" {
		t.Fatalf("expected 42, got %v", res)
	}
}

func TestMinimaxClientReasonFailure(t *testing.T) {
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusInternalServerError)
		w.Write([]byte("internal error"))
	}))
	defer ts.Close()

	originalURL := MinimaxAPIURL
	MinimaxAPIURL = ts.URL
	defer func() { MinimaxAPIURL = originalURL }()

	client := NewMinimaxClient("valid-key")
	_, err := client.Reason(context.Background(), "test")
	if err == nil {
		t.Fatalf("expected error on 500 response")
	}
}

func TestMinimaxClientReasonEmptyResponse(t *testing.T) {
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		resp := map[string]interface{}{
			"choices": []map[string]interface{}{},
		}
		json.NewEncoder(w).Encode(resp)
	}))
	defer ts.Close()

	originalURL := MinimaxAPIURL
	MinimaxAPIURL = ts.URL
	defer func() { MinimaxAPIURL = originalURL }()

	client := NewMinimaxClient("valid-key")
	_, err := client.Reason(context.Background(), "test")
	if err == nil {
		t.Fatalf("expected error on empty choices")
	}
}

func TestMinimaxClientGenerateEmbeddingSuccess(t *testing.T) {
	// Reset global circuit breaker state before testing
	ResetCircuitBreakerForTest()

	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.Header.Get("Authorization") != "Bearer valid-key" {
			w.WriteHeader(http.StatusUnauthorized)
			return
		}
		resp := map[string]interface{}{
			"vectors": [][]float32{
				{0.1, 0.2, 0.3},
			},
		}
		json.NewEncoder(w).Encode(resp)
	}))
	defer ts.Close()

	originalURL := MinimaxEmbeddingAPIURL
	MinimaxEmbeddingAPIURL = ts.URL
	defer func() { MinimaxEmbeddingAPIURL = originalURL }()

	client := NewMinimaxClient("valid-key")
	res, err := client.GenerateEmbedding(context.Background(), "test embedding")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(res) != 3 {
		t.Fatalf("expected 3 floats, got %d", len(res))
	}
	if res[0] != 0.1 || res[1] != 0.2 || res[2] != 0.3 {
		t.Fatalf("expected [0.1, 0.2, 0.3], got %v", res)
	}
}

func TestMinimaxClientGenerateEmbeddingFailure(t *testing.T) {
	// Reset global circuit breaker state before testing
	ResetCircuitBreakerForTest()

	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusInternalServerError)
		w.Write([]byte("internal error"))
	}))
	defer ts.Close()

	originalURL := MinimaxEmbeddingAPIURL
	MinimaxEmbeddingAPIURL = ts.URL
	defer func() { MinimaxEmbeddingAPIURL = originalURL }()

	client := NewMinimaxClient("valid-key")
	_, err := client.GenerateEmbedding(context.Background(), "test")
	if err == nil {
		t.Fatalf("expected error on 500 response")
	}
}
