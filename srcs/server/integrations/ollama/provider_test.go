package ollama

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"
)

func TestOllamaProvider_ID(t *testing.T) {
	p := NewOllamaProvider("")
	if p.ID() != "ollama_mcp" {
		t.Errorf("expected ID 'ollama_mcp', got '%s'", p.ID())
	}
}

func TestOllamaProvider_Tools(t *testing.T) {
	p := NewOllamaProvider("")
	tools := p.Tools()
	if len(tools) != 3 {
		t.Errorf("expected 3 tools, got %d", len(tools))
	}
}

func TestOllamaProvider_Initialize(t *testing.T) {
	p := NewOllamaProvider("")
	err := p.Initialize()
	if err != nil {
		t.Errorf("expected nil error, got %v", err)
	}
}

func TestOllamaProvider_ListOllamaModels(t *testing.T) {
	// Create a mock server
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/api/tags" {
			t.Errorf("expected path /api/tags, got %s", r.URL.Path)
		}

		resp := OllamaTagsResponse{
			Models: []struct {
				Name string `json:"name"`
			}{
				{Name: "llama3"},
				{Name: "mistral"},
			},
		}
		json.NewEncoder(w).Encode(resp)
	}))
	defer ts.Close()

	p := NewOllamaProvider(ts.URL)
	models, err := p.ListOllamaModels(context.Background())
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(models) != 2 {
		t.Fatalf("expected 2 models, got %d", len(models))
	}
	if models[0] != "llama3" || models[1] != "mistral" {
		t.Errorf("unexpected models: %v", models)
	}
}

func TestOllamaProvider_ListOllamaModels_Error(t *testing.T) {
	// Create a mock server returning an error
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusInternalServerError)
	}))
	defer ts.Close()

	p := NewOllamaProvider(ts.URL)
	_, err := p.ListOllamaModels(context.Background())
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
}

func TestOllamaProvider_ListOllamaModels_InvalidJSON(t *testing.T) {
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Write([]byte("invalid json"))
	}))
	defer ts.Close()

	p := NewOllamaProvider(ts.URL)
	_, err := p.ListOllamaModels(context.Background())
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
}

func TestOllamaProvider_ListOllamaModels_RequestError(t *testing.T) {
	p := NewOllamaProvider("http://\x00")
	_, err := p.ListOllamaModels(context.Background())
	if err == nil {
		t.Fatalf("expected error on invalid URL")
	}
}

func TestOllamaProvider_PullOllamaModel(t *testing.T) {
	// Create a mock server
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/api/pull" {
			t.Errorf("expected path /api/pull, got %s", r.URL.Path)
		}
		w.WriteHeader(http.StatusOK)
	}))
	defer ts.Close()

	p := NewOllamaProvider(ts.URL)
	err := p.PullOllamaModel(context.Background(), "llama3")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
}

func TestOllamaProvider_PullOllamaModel_Error(t *testing.T) {
	// Create a mock server returning an error
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusInternalServerError)
	}))
	defer ts.Close()

	p := NewOllamaProvider(ts.URL)
	err := p.PullOllamaModel(context.Background(), "llama3")
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
}

func TestOllamaProvider_PullOllamaModel_RequestError(t *testing.T) {
	p := NewOllamaProvider("http://\x00")
	err := p.PullOllamaModel(context.Background(), "llama3")
	if err == nil {
		t.Fatalf("expected error on invalid URL")
	}
}


func TestOllamaProvider_CheckOllamaHealth_Success(t *testing.T) {
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
		w.Write([]byte("Ollama is running"))
	}))
	defer ts.Close()

	p := NewOllamaProvider(ts.URL)
	isHealthy, err := p.CheckOllamaHealth(context.Background())
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if !isHealthy {
		t.Errorf("expected true, got false")
	}
}

func TestOllamaProvider_CheckOllamaHealth_Failure(t *testing.T) {
	// Create a provider pointing to a port that's definitely not listening
	// or close the server immediately
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {}))
	url := ts.URL
	ts.Close() // Close immediately to force a connection refused

	p := NewOllamaProvider(url)

	// Create context with short timeout to ensure fast failure
	ctx, cancel := context.WithTimeout(context.Background(), 100*time.Millisecond)
	defer cancel()

	isHealthy, err := p.CheckOllamaHealth(ctx)
	if err != nil {
		t.Fatalf("unexpected error: %v (should handle connection refused gracefully)", err)
	}
	if isHealthy {
		t.Errorf("expected false, got true")
	}
}

func TestOllamaProvider_CheckOllamaHealth_StatusError(t *testing.T) {
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusInternalServerError)
	}))
	defer ts.Close()

	p := NewOllamaProvider(ts.URL)
	isHealthy, err := p.CheckOllamaHealth(context.Background())
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if isHealthy {
		t.Errorf("expected false, got true")
	}
}

func TestOllamaProvider_CheckOllamaHealth_RequestError(t *testing.T) {
	p := NewOllamaProvider("http://\x00")
	_, err := p.CheckOllamaHealth(context.Background())
	if err == nil {
		t.Fatalf("expected error on invalid URL")
	}
}

func TestOllamaProvider_ContextCancel(t *testing.T) {
	p := NewOllamaProvider("http://localhost:11434")

	ctx, cancel := context.WithCancel(context.Background())
	cancel() // Cancel immediately

	_, err := p.ListOllamaModels(ctx)
	if err == nil {
		t.Fatalf("expected error on cancelled context")
	}

	err = p.PullOllamaModel(ctx, "llama3")
	if err == nil {
		t.Fatalf("expected error on cancelled context")
	}

	isHealthy, err := p.CheckOllamaHealth(ctx)
	if err != nil {
		t.Fatalf("expected nil error on health check failure, got: %v", err)
	}
	if isHealthy {
		t.Fatalf("expected false on cancelled context")
	}
}
