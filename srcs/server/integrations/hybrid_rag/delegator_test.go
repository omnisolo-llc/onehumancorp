package hybrid_rag

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestLocalDelegator_DelegateToCloud(t *testing.T) {
	mockServer := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			t.Errorf("Expected POST request, got %s", r.Method)
		}
		if r.Header.Get("Authorization") != "Bearer test-api-key" {
			t.Errorf("Expected Authorization header 'Bearer test-api-key', got '%s'", r.Header.Get("Authorization"))
		}

		var payload MissionPayload
		if err := json.NewDecoder(r.Body).Decode(&payload); err != nil {
			t.Errorf("Failed to decode payload: %v", err)
		}
		if payload.OriginalQuery != "test query" {
			t.Errorf("Expected original_query 'test query', got '%s'", payload.OriginalQuery)
		}
		if payload.Content != "raw private info (sanitized)" {
			t.Errorf("Expected content 'raw private info (sanitized)', got '%s'", payload.Content)
		}

		w.WriteHeader(http.StatusOK)
		json.NewEncoder(w).Encode(CloudDelegationResponse{
			Status:    "success",
			MissionID: "mock-mission-id",
		})
	}))
	defer mockServer.Close()

	delegator := NewLocalDelegator(mockServer.URL, "test-api-key")

	ctx := context.Background()
	localCtx := RAGContext{
		OriginalQuery: "test query",
		RawContent:    "raw private info",
	}

	missionID, err := delegator.DelegateToCloud(ctx, localCtx)
	if err != nil {
		t.Fatalf("Unexpected error: %v", err)
	}

	if missionID != "mock-mission-id" {
		t.Errorf("Expected mission ID 'mock-mission-id', got '%s'", missionID)
	}
}

func TestMockLocalDelegator(t *testing.T) {
	mock := &MockLocalDelegator{
		MockResponse: "test-mission",
	}
	ctx := context.Background()
	missionID, err := mock.DelegateToCloud(ctx, RAGContext{
		OriginalQuery: "q",
		RawContent:    "c",
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if missionID != "test-mission" {
		t.Errorf("Expected test-mission, got %s", missionID)
	}
	if mock.LastPayload == nil {
		t.Fatal("Expected LastPayload to be populated")
	}
	if mock.LastPayload.Content != "c (sanitized)" {
		t.Errorf("Expected 'c (sanitized)', got '%s'", mock.LastPayload.Content)
	}
}
