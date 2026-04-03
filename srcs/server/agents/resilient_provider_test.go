package agents

import (
	"context"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

)

// mockPrimaryProvider fails deliberately to test the fallback.
type mockPrimaryProvider struct {
	failReason bool
	failEmbed  bool
}

func (m *mockPrimaryProvider) Reason(ctx context.Context, prompt string) (string, error) {
	if m.failReason {
		return "", http.ErrHandlerTimeout
	}
	return "primary-reason", nil
}

func (m *mockPrimaryProvider) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	if m.failEmbed {
		return nil, http.ErrHandlerTimeout
	}
	return []float32{1.0, 2.0}, nil
}

func TestResilientProvider_Reason_Fallback(t *testing.T) {
	primary := &mockPrimaryProvider{failReason: true}

	// Create a mock local LLM endpoint that succeeds
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
		w.Write([]byte(`{"response": "local-fallback-reason"}`))
	}))
	defer ts.Close()

	local := NewLocalLLMProvider(ts.URL, "llama-test")

	resilient := NewResilientProvider(primary, local)

	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	resp, err := resilient.Reason(ctx, "test prompt")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if resp != "local-fallback-reason" {
		t.Errorf("expected fallback response 'local-fallback-reason', got '%s'", resp)
	}
}

func TestResilientProvider_GenerateEmbedding_Fallback(t *testing.T) {
	primary := &mockPrimaryProvider{failEmbed: true}

	// Create a mock local LLM endpoint that succeeds
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
		w.Write([]byte(`{"embedding": [3.0, 4.0]}`))
	}))
	defer ts.Close()

	local := NewLocalLLMProvider(ts.URL, "llama-test")

	resilient := NewResilientProvider(primary, local)

	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	emb, err := resilient.GenerateEmbedding(ctx, "test text")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if len(emb) != 2 || emb[0] != 3.0 || emb[1] != 4.0 {
		t.Errorf("expected fallback embedding [3.0, 4.0], got %v", emb)
	}
}
