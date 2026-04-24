package weaviate

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestWeaviateTool_WeaviateQuery(t *testing.T) {
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/graphql" {
			t.Errorf("Expected path /v1/graphql, got %s", r.URL.Path)
		}
		if r.Header.Get("Authorization") != "Bearer test-key" {
			t.Errorf("Expected Authorization header 'Bearer test-key', got '%s'", r.Header.Get("Authorization"))
		}

		var payload QueryPayload
		if err := json.NewDecoder(r.Body).Decode(&payload); err != nil {
			t.Fatal("Failed to decode request body")
		}
		if payload.Query != "{ Get { Article { title } } }" {
			t.Errorf("Unexpected query: %s", payload.Query)
		}

		w.WriteHeader(http.StatusOK)
		w.Write([]byte(`{"data": {"Get": {"Article": [{"title": "Test"}]}}}`))
	}))
	defer ts.Close()

	tool := NewWeaviateTool()
	result, err := tool.WeaviateQuery(context.Background(), ts.URL, "test-key", "{ Get { Article { title } } }")

	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	if result == nil {
		t.Fatalf("Expected result, got nil")
	}
}

func TestWeaviateTool_WeaviateInsert(t *testing.T) {
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/objects" {
			t.Errorf("Expected path /v1/objects, got %s", r.URL.Path)
		}

		var payload InsertPayload
		if err := json.NewDecoder(r.Body).Decode(&payload); err != nil {
			t.Fatal("Failed to decode request body")
		}
		if payload.Class != "Article" {
			t.Errorf("Expected Class 'Article', got '%s'", payload.Class)
		}

		w.WriteHeader(http.StatusOK)
		w.Write([]byte(`{"id": "12345"}`))
	}))
	defer ts.Close()

	tool := NewWeaviateTool()
	result, err := tool.WeaviateInsert(context.Background(), ts.URL, "", "Article", map[string]interface{}{"title": "Test"}, []float32{0.1, 0.2})

	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	if result == nil {
		t.Fatalf("Expected result, got nil")
	}
}

func TestWeaviateTool_WeaviateSchema(t *testing.T) {
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/schema" {
			t.Errorf("Expected path /v1/schema, got %s", r.URL.Path)
		}

		w.WriteHeader(http.StatusOK)
		w.Write([]byte(`{"classes": [{"class": "Article"}]}`))
	}))
	defer ts.Close()

	tool := NewWeaviateTool()
	result, err := tool.WeaviateSchema(context.Background(), ts.URL, "")

	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	if result == nil {
		t.Fatalf("Expected result, got nil")
	}
}
