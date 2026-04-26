package ollama

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestOllamaTool_ListOllamaModels(t *testing.T) {
	mockResponse := ModelList{}
	mockResponse.Models = append(mockResponse.Models, struct {
		Name       string `json:"name"`
		ModifiedAt string `json:"modified_at"`
		Size       int64  `json:"size"`
	}{Name: "llama3:latest", Size: 4700000000})

	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/api/tags" {
			t.Errorf("Expected path /api/tags, got %s", r.URL.Path)
		}
		w.WriteHeader(http.StatusOK)
		json.NewEncoder(w).Encode(mockResponse)
	}))
	defer ts.Close()

	tool := NewOllamaTool()
	models, err := tool.ListOllamaModels(context.Background(), ts.URL)

	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	if len(models.Models) != 1 {
		t.Fatalf("Expected 1 model, got %d", len(models.Models))
	}

	if models.Models[0].Name != "llama3:latest" {
		t.Errorf("Expected model name 'llama3:latest', got '%s'", models.Models[0].Name)
	}
}

func TestOllamaTool_PullOllamaModel(t *testing.T) {
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/api/pull" {
			t.Errorf("Expected path /api/pull, got %s", r.URL.Path)
		}
		var payload PullPayload
		if err := json.NewDecoder(r.Body).Decode(&payload); err != nil {
			t.Fatal("Failed to decode request body")
		}
		if payload.Name != "mistral:latest" {
			t.Errorf("Expected model name 'mistral:latest', got '%s'", payload.Name)
		}
		w.WriteHeader(http.StatusOK)
	}))
	defer ts.Close()

	tool := NewOllamaTool()
	err := tool.PullOllamaModel(context.Background(), ts.URL, "mistral:latest")

	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}
}

func TestOllamaTool_CheckOllamaHealth(t *testing.T) {
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/api/generate" {
			t.Errorf("Expected path /api/generate, got %s", r.URL.Path)
		}
		var payload GeneratePayload
		if err := json.NewDecoder(r.Body).Decode(&payload); err != nil {
			t.Fatal("Failed to decode request body")
		}
		if payload.Model != "llama3:latest" {
			t.Errorf("Expected model 'llama3:latest', got '%s'", payload.Model)
		}
		w.WriteHeader(http.StatusOK)
		w.Write([]byte(`{"response":"Hi there"}`))
	}))
	defer ts.Close()

	tool := NewOllamaTool()
	ok, err := tool.CheckOllamaHealth(context.Background(), ts.URL, "llama3:latest")

	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	if !ok {
		t.Errorf("Expected health check to pass, got false")
	}
}
