package agents

import (
	"context"
	"database/sql"
	"net/http"
	"net/http/httptest"
	"os"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

type mockPrimary struct {
	fail bool
	err  error
}

func (m *mockPrimary) Reason(ctx context.Context, prompt string) (string, error) {
	if m.fail {
		return "", m.err
	}
	return "primary reason", nil
}

func (m *mockPrimary) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	if m.fail {
		return nil, m.err
	}
	return []float32{1.0}, nil
}

func TestResilientProvider_Success(t *testing.T) {
	primary := &mockPrimary{fail: false}
	resilient := NewResilientProvider(primary, nil)

	resp, err := resilient.Reason(context.Background(), "test")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if resp != "primary reason" {
		t.Fatalf("expected 'primary reason', got '%s'", resp)
	}
}

func TestResilientProvider_Fallback(t *testing.T) {
	// Setup a fake local LLM server
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path == "/api/generate" {
			w.WriteHeader(http.StatusOK)
			w.Write([]byte(`{"response": "fallback reason"}`))
			return
		}
		if r.URL.Path == "/api/embeddings" {
			w.WriteHeader(http.StatusOK)
			w.Write([]byte(`{"embedding": [2.0]}`))
			return
		}
		w.WriteHeader(http.StatusNotFound)
	}))
	defer server.Close()

	os.Setenv("OHC_LOCAL_LLM_ENDPOINT", server.URL+"/api/generate")
	os.Setenv("OHC_LOCAL_LLM_EMBED_ENDPOINT", server.URL+"/api/embeddings")
	defer os.Unsetenv("OHC_LOCAL_LLM_ENDPOINT")
	defer os.Unsetenv("OHC_LOCAL_LLM_EMBED_ENDPOINT")

	// Primary always fails
	primary := &mockPrimary{fail: true, err: http.ErrServerClosed}
	resilient := NewResilientProvider(primary, nil)

	resp, err := resilient.Reason(context.Background(), "test")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if resp != "fallback reason" {
		t.Fatalf("expected 'fallback reason', got '%s'", resp)
	}

	emb, err := resilient.GenerateEmbedding(context.Background(), "test")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(emb) == 0 || emb[0] != 2.0 {
		t.Fatalf("expected [2.0], got %v", emb)
	}
}

func TestResilientProvider_MockDBLocalFallback(t *testing.T) {
	// Provide high-coverage unit tests utilizing the existing db.NewSqliteProvider(sqlDB) for local mocking.
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to create sqlite provider: %v", err)
	}
	defer sqlDB.Close()

	prov := db.NewSqliteProvider(sqlDB)
	defer prov.Close()

	primary := &mockPrimary{fail: true, err: http.ErrServerClosed}
	// just a simple wrapper check
	resilient := NewResilientProvider(primary, &mockPrimary{fail: false})

	// mock DB context wrapper test
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	_, err = prov.Exec(ctx, "CREATE TABLE dummy (id INT);")
	if err != nil {
		t.Fatalf("db exec failed: %v", err)
	}

	resp, err := resilient.Reason(ctx, "test with db")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if resp != "primary reason" {
		t.Fatalf("expected fallback mock 'primary reason', got '%s'", resp)
	}
}
