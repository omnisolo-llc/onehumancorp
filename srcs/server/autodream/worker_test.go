package autodream

import (
	"context"
	"errors"
	"testing"
	"time"
)

type MockEmbeddingClient struct {
	err error
}

func (m *MockEmbeddingClient) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	if m.err != nil {
		return nil, m.err
	}
	return []float32{0.5, 0.5}, nil
}

type MockVectorStore struct {
	stored []string
	err    error
}

func (m *MockVectorStore) Store(ctx context.Context, id string, vector []float32, metadata map[string]any, content string) error {
	if m.err != nil {
		return m.err
	}
	m.stored = append(m.stored, id)
	return nil
}

func (m *MockVectorStore) Search(ctx context.Context, vector []float32, limit int) ([]*KnowledgeRecord, error) {
	return nil, nil
}

type MockTraceQueue struct {
	id       string
	content  string
	metadata map[string]any
	fetchErr error
	compErr  error
	complete bool
}

func (m *MockTraceQueue) FetchNextTrace(ctx context.Context) (string, string, map[string]any, error) {
	return m.id, m.content, m.metadata, m.fetchErr
}

func (m *MockTraceQueue) MarkTraceComplete(ctx context.Context, id string) error {
	m.complete = true
	return m.compErr
}

func TestAutoDreamWorkerProcessTrace(t *testing.T) {
	store := &MockVectorStore{}
	client := &MockEmbeddingClient{}
	queue := &MockTraceQueue{}
	worker := NewAutoDreamWorker(store, client, queue)

	ctx := context.Background()
	err := worker.ProcessTrace(ctx, "trace-1", "my trace", map[string]any{})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if len(store.stored) != 1 || store.stored[0] != "trace-1" {
		t.Errorf("expected trace-1 to be stored, got %v", store.stored)
	}
}

func TestAutoDreamWorkerProcessTrace_ClientError(t *testing.T) {
	store := &MockVectorStore{}
	client := &MockEmbeddingClient{err: errors.New("client error")}
	queue := &MockTraceQueue{}
	worker := NewAutoDreamWorker(store, client, queue)

	ctx := context.Background()
	err := worker.ProcessTrace(ctx, "trace-1", "my trace", map[string]any{})
	if err == nil {
		t.Fatal("expected error, got nil")
	}
}

func TestAutoDreamWorkerProcessTrace_StoreError(t *testing.T) {
	store := &MockVectorStore{err: errors.New("store error")}
	client := &MockEmbeddingClient{}
	queue := &MockTraceQueue{}
	worker := NewAutoDreamWorker(store, client, queue)

	ctx := context.Background()
	err := worker.ProcessTrace(ctx, "trace-1", "my trace", map[string]any{})
	if err == nil {
		t.Fatal("expected error, got nil")
	}
}

func TestAutoDreamWorkerStart(t *testing.T) {
	store := &MockVectorStore{}
	client := &MockEmbeddingClient{}
	queue := &MockTraceQueue{
		id:      "trace-2",
		content: "content-2",
	}
	worker := NewAutoDreamWorker(store, client, queue)

	ctx, cancel := context.WithCancel(context.Background())
	worker.Start(ctx, 10*time.Millisecond)

	time.Sleep(50 * time.Millisecond)
	cancel()
	time.Sleep(20 * time.Millisecond)

	if !queue.complete {
		t.Errorf("expected queue complete to be true")
	}
}

func TestAutoDreamWorkerProcessNext_QueueNil(t *testing.T) {
	store := &MockVectorStore{}
	client := &MockEmbeddingClient{}
	worker := NewAutoDreamWorker(store, client, nil)

	ctx := context.Background()
	// Should not panic
	worker.processNext(ctx)
}

func TestAutoDreamWorkerProcessNext_QueueFetchError(t *testing.T) {
	store := &MockVectorStore{}
	client := &MockEmbeddingClient{}
	queue := &MockTraceQueue{
		fetchErr: errors.New("fetch error"),
	}
	worker := NewAutoDreamWorker(store, client, queue)

	ctx := context.Background()
	worker.processNext(ctx)

	if len(store.stored) != 0 {
		t.Errorf("expected nothing stored, got %v", store.stored)
	}
}

func TestAutoDreamWorkerProcessNext_QueueEmptyId(t *testing.T) {
	store := &MockVectorStore{}
	client := &MockEmbeddingClient{}
	queue := &MockTraceQueue{
		id: "",
	}
	worker := NewAutoDreamWorker(store, client, queue)

	ctx := context.Background()
	worker.processNext(ctx)

	if len(store.stored) != 0 {
		t.Errorf("expected nothing stored, got %v", store.stored)
	}
}
