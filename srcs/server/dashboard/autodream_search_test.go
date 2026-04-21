package dashboard

import (
	"bytes"
	"context"
	"database/sql"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/autodream"
	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

type MockEmbeddingClient struct{}

func (m *MockEmbeddingClient) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	return []float32{1.0, 0.0, 0.0}, nil
}

func TestHandleAutoDreamKnowledgeSearch(t *testing.T) {
	dbConn, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open memory sqlite: %v", err)
	}
	defer dbConn.Close()

	provider := db.NewSqliteProvider(dbConn)
	ctx := context.Background()

	_, err = provider.Exec(ctx, `
		CREATE TABLE knowledge_embeddings (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			metadata TEXT,
			embedding TEXT
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	store := autodream.NewSQLiteVectorStore(provider)
	metadata := map[string]any{"test": "meta"}
	err = store.Store(ctx, "test-1", []float32{1.0, 0.0, 0.0}, metadata, "test content")
	if err != nil {
		t.Fatalf("failed to store vector: %v", err)
	}

	handler := HandleAutoDreamKnowledgeSearch(provider, &MockEmbeddingClient{})

	reqBody := AutoDreamSearchRequest{
		QueryText: "test content",
		Limit:     5,
	}
	bodyBytes, _ := json.Marshal(reqBody)

	req := httptest.NewRequest("POST", "/api/v1/autodream/knowledge/search", bytes.NewBuffer(bodyBytes))
	req.Header.Set("Content-Type", "application/json")
	rr := httptest.NewRecorder()

	handler.ServeHTTP(rr, req)

	if rr.Code != http.StatusOK {
		t.Errorf("expected status 200, got %d. Body: %s", rr.Code, rr.Body.String())
	}

	var res AutoDreamSearchResult
	if err := json.NewDecoder(rr.Body).Decode(&res); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	if len(res.Results) != 1 {
		t.Fatalf("expected 1 result, got %d", len(res.Results))
	}
	if res.Results[0].ID != "test-1" {
		t.Errorf("expected id test-1, got %s", res.Results[0].ID)
	}
}
