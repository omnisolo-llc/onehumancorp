package autodream

import (
	"context"
	"os"
	"path/filepath"
	"testing"
	"time"
	"fmt"
	"database/sql"
	"database/sql/driver"

	"github.com/onehumancorp/mono/src/server/auth"
	"github.com/onehumancorp/mono/src/server/db"
	"github.com/onehumancorp/mono/src/server/memory"
	import_sqlite "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) db.Provider {
	dbConn, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}

	_ = import_sqlite.RegisterDeterministicScalarFunction("vec_distance_cosine", 2, func(ctx *import_sqlite.FunctionContext, args []driver.Value) (driver.Value, error) {
		return 0.01, nil
	})

	provider := db.NewSqliteProvider(dbConn)
	ctx := context.Background()
	_, err = provider.Exec(ctx, `CREATE TABLE IF NOT EXISTS consolidated_memory (
		id TEXT PRIMARY KEY,
		organization_id TEXT NOT NULL,
		agent_id TEXT,
		content TEXT NOT NULL,
		embedding TEXT,
		source_type TEXT NOT NULL,
		created_at DATETIME DEFAULT CURRENT_TIMESTAMP
	);`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}
	return provider
}

type mockRow struct {
	id string
}
func (r *mockRow) Next() bool {
	if r.id == "sync1" {
		r.id = ""
		return true
	}
	return false
}
func (r *mockRow) Scan(dest ...any) error {
	*dest[0].(*string) = "sync1"
	*dest[1].(*string) = "org1"
	*dest[2].(*string) = "TASK_SUMMARY"
	*dest[3].(*string) = "content"
	*dest[4].(*[]byte) = []byte("[0.1, 0.2]")
	*dest[5].(*time.Time) = time.Now()
	*dest[6].(*string) = "task1"
	return nil
}
func (r *mockRow) Close() {}

type mockDbProvider struct {
	queryCalled bool
	execCalled  bool
	failQuery   bool
	failScan    bool
	failJSON    bool
}
func (m *mockDbProvider) Query(ctx context.Context, sql string, optionsAndArgs ...any) (interface{
	Next() bool
	Scan(dest ...any) error
	Close()
}, error) {
	m.queryCalled = true
	if m.failQuery {
		return nil, fmt.Errorf("query failed")
	}
	if m.failScan {
		return &mockRowFailScan{id: "sync1"}, nil
	}
	if m.failJSON {
		return &mockRowFailJSON{id: "sync1"}, nil
	}
	return &mockRow{id: "sync1"}, nil
}

func (m *mockDbProvider) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	m.execCalled = true
	if len(arguments) > 0 && arguments[0] == "sync2" {
		return 0, fmt.Errorf("exec failed")
	}
	return 1, nil
}

type mockRowFailScan struct {
	id string
}
func (r *mockRowFailScan) Next() bool {
	if r.id == "sync1" {
		r.id = ""
		return true
	}
	return false
}
func (r *mockRowFailScan) Scan(dest ...any) error {
	return fmt.Errorf("scan failed")
}
func (r *mockRowFailScan) Close() {}

type mockRowFailJSON struct {
	id string
}
func (r *mockRowFailJSON) Next() bool {
	if r.id == "sync1" {
		r.id = ""
		return true
	}
	return false
}
func (r *mockRowFailJSON) Scan(dest ...any) error {
	*dest[0].(*string) = "sync1"
	*dest[1].(*string) = "org1"
	*dest[2].(*string) = "TASK_SUMMARY"
	*dest[3].(*string) = "content"
	*dest[4].(*[]byte) = []byte("invalid json")
	*dest[5].(*time.Time) = time.Now()
	*dest[6].(*string) = "task1"
	return nil
}
func (r *mockRowFailJSON) Close() {}

type myMockLLM struct {
	failReason bool
	failEmbed  bool
}
func (m *myMockLLM) Reason(ctx context.Context, prompt string) (string, error) {
	if m.failReason {
		return "", fmt.Errorf("reason failed")
	}
	return "summary", nil
}
func (m *myMockLLM) GenerateEmbedding(ctx context.Context, content string) ([]float32, error) {
	if m.failEmbed {
		return nil, fmt.Errorf("embed failed")
	}
	return []float32{0.1, 0.2}, nil
}
func (m *myMockLLM) ParseYAML(ctx context.Context, input string, dest interface{}) error {
	return nil
}

type failingSQLiteProvider struct {
	db.Provider
}
func (f *failingSQLiteProvider) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	return 0, fmt.Errorf("forced exec error")
}

func TestService_ProcessMemoryFile(t *testing.T) {
	tempDir := t.TempDir()
	filePath := filepath.Join(tempDir, "test.yml")

	yamlContent := `
task_id: "test-task"
agent_role: "Operations"
content: "test content"
`
	err := os.WriteFile(filePath, []byte(yamlContent), 0644)
	if err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	mockLLMClient := &myMockLLM{}

	s := NewService(nil, mockLLMClient, tempDir)
	claims := &auth.Claims{OrganizationID: "test-org"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	err = s.processMemoryFile(ctx, filePath)
	if err != nil {
		t.Errorf("Expected nil, got %v", err)
	}

	// invalid yaml
	badPath := filepath.Join(tempDir, "bad.yml")
	os.WriteFile(badPath, []byte("bad yaml content: :"), 0644)
	err = s.processMemoryFile(ctx, badPath)
	if err == nil {
		t.Errorf("Expected error for invalid yaml")
	}

	// missing fields
	missingPath := filepath.Join(tempDir, "missing.yml")
	os.WriteFile(missingPath, []byte("task_id: ''\ncontent: ''"), 0644)
	err = s.processMemoryFile(ctx, missingPath)
	if err == nil {
		t.Errorf("Expected error for missing fields")
	}

	// file not found
	err = s.processMemoryFile(ctx, filepath.Join(tempDir, "notfound.yml"))
	if err == nil {
		t.Errorf("Expected error for not found file")
	}

	// generate embedding error
	mockLLMFail := &myMockLLM{failEmbed: true}
	sFail := NewService(nil, mockLLMFail, tempDir)
	err = sFail.processMemoryFile(ctx, filePath)
	if err == nil {
		t.Errorf("Expected error for embedding fail")
	}
}

func TestService_Consolidate(t *testing.T) {
	s := &Service{
		llm: &myMockLLM{},
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "test-org"})

	err := s.Consolidate(ctx, "task1", []string{})
	if err != nil {
		t.Errorf("Expected nil error for empty logs, got %v", err)
	}

	err = s.Consolidate(ctx, "task1", []string{"log1", "log2"})
	if err != nil {
		t.Errorf("Expected nil error, got %v", err)
	}

	// Error missing org
	err = s.Consolidate(context.Background(), "task1", []string{"log1"})
	if err == nil {
		t.Errorf("Expected error for missing org")
	}

	mockLLMFail := &myMockLLM{failReason: true}
	sFail := &Service{llm: mockLLMFail}
	err = sFail.Consolidate(ctx, "task1", []string{"log1"})
	if err == nil {
		t.Errorf("Expected reason fail")
	}

	mockLLMFail2 := &myMockLLM{failEmbed: true}
	sFail2 := &Service{llm: mockLLMFail2}
	err = sFail2.Consolidate(ctx, "task1", []string{"log1"})
	if err == nil {
		t.Errorf("Expected embed fail")
	}
}

func TestAutoDreamConsolidationWithDB(t *testing.T) {
	provider := setupTestDB(t)
	claims := &auth.Claims{OrganizationID: "test-tenant-123"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	repo := memory.NewVectorRepository(provider)
	llm := &myMockLLM{}
	service := NewService(repo, llm, "")

	err := service.Consolidate(ctx, "task-123", []string{"log 1", "log 2"})
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}

	// Test upsert failure by supplying a failing DB
	failingProvider := &failingSQLiteProvider{Provider: provider}
	vr := memory.NewVectorRepository(failingProvider)
	sWithVR := &Service{llm: &myMockLLM{}, vectorRepo: vr}
	err = sWithVR.Consolidate(ctx, "task1", []string{"log1", "log2"})
	if err == nil {
		t.Errorf("Expected error for upsert fail")
	}
}


func TestPushToCloud(t *testing.T) {
	s := &Service{}
	err := s.PushToCloud(context.Background(), nil)
	if err == nil {
		t.Errorf("Expected err when dbprovider is nil")
	}

	mockDb := &mockDbProvider{}
	err = s.PushToCloud(context.Background(), mockDb)
	if err != nil {
		t.Errorf("Expected nil, got %v", err)
	}
	if !mockDb.queryCalled {
		t.Errorf("Expected Query to be called")
	}
	if !mockDb.execCalled {
		t.Errorf("Expected Exec to be called")
	}

	mockDbFail := &mockDbProvider{failQuery: true}
	err = s.PushToCloud(context.Background(), mockDbFail)
	if err == nil {
		t.Errorf("Expected error")
	}

	mockDbScanFail := &mockDbProvider{failScan: true}
	err = s.PushToCloud(context.Background(), mockDbScanFail)
	if err != nil {
		t.Errorf("Expected nil, got %v", err)
	}

	mockDbJSONFail := &mockDbProvider{failJSON: true}
	err = s.PushToCloud(context.Background(), mockDbJSONFail)
	if err != nil {
		t.Errorf("Expected nil, got %v", err)
	}
}

func TestStartWatcher(t *testing.T) {
	s := NewService(nil, &myMockLLM{}, "")
	err := s.StartWatcher(context.Background())
	if err != nil {
		t.Errorf("Expected nil for empty dir, got %v", err)
	}

	sFailMake := NewService(nil, &myMockLLM{}, "/root/impossible/dir/to/make")
	err = sFailMake.StartWatcher(context.Background())
	if err == nil {
		t.Errorf("Expected error for impossible dir")
	}

	tempDir := t.TempDir()
	s = NewService(nil, &myMockLLM{}, tempDir)
	ctx, cancel := context.WithCancel(context.Background())
	err = s.StartWatcher(ctx)
	if err != nil {
		t.Errorf("Expected nil, got %v", err)
	}

	// Write a file to trigger watcher
	os.WriteFile(filepath.Join(tempDir, "watch.yml"), []byte("task_id: w1\ncontent: test"), 0644)
	time.Sleep(100 * time.Millisecond) // Let watcher process

	cancel()
	time.Sleep(50 * time.Millisecond) // Let goroutine exit
}

func TestResolveConflicts(t *testing.T) {
	provider := setupTestDB(t)
	claims := &auth.Claims{OrganizationID: "test-tenant-123"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	repo := memory.NewVectorRepository(provider)
	llm := &myMockLLM{}
	s := NewService(repo, llm, "")

	// Insert mock data
	repo.Upsert(ctx, &memory.EmbeddingRecord{
		ID: "m1", OrganizationID: "test-tenant-123", Content: "A", Embedding: []float32{0.1},
	})
	repo.Upsert(ctx, &memory.EmbeddingRecord{
		ID: "m2", OrganizationID: "test-tenant-123", Content: "B", Embedding: []float32{0.1},
	})

	err := s.ResolveConflicts(ctx, "test-tenant-123")
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}
}

func TestPruneStaleContext(t *testing.T) {
	provider := setupTestDB(t)
	claims := &auth.Claims{OrganizationID: "test-tenant-123"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	repo := memory.NewVectorRepository(provider)
	llm := &myMockLLM{}
	s := NewService(repo, llm, "")

	// Insert mock data with old date
	repo.Upsert(ctx, &memory.EmbeddingRecord{
		ID: "m1", OrganizationID: "test-tenant-123", Content: "A", Embedding: []float32{0.1}, CreatedAt: time.Now().Add(-48 * time.Hour),
	})

	err := s.PruneStaleContext(ctx, "test-tenant-123", 24*time.Hour)
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}

	// Verify it was deleted
	records, _ := repo.SemanticSearch(ctx, "test-tenant-123", []float32{0.1}, 10)
	if len(records) != 0 {
		t.Errorf("expected 0 records, got %d", len(records))
	}
}
