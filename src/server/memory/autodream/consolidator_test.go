package autodream

import (
	"context"
	"os"
	"path/filepath"
	"testing"
	"time"
	"fmt"

	"github.com/onehumancorp/mono/src/server/auth"
	"github.com/onehumancorp/mono/src/server/db"
	"github.com/onehumancorp/mono/src/server/memory"
)

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
func (r *mockRow) Err() error { return nil }
func (r *mockRow) Columns() ([]string, error) { return nil, nil }

type mockDbProvider struct {
	queryCalled bool
	execCalled  bool
	failQuery   bool
	failScan    bool
	failJSON    bool
}
func (m *mockDbProvider) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
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
func (m *mockDbProvider) AcquireTask(ctx context.Context, organizationID, agentID string) (*db.TaskRecord, error) {
	return nil, nil
}
func (m *mockDbProvider) QueryRow(ctx context.Context, sql string, optionsAndArgs ...any) db.Row {
	return nil
}
func (m *mockDbProvider) Begin(ctx context.Context) (db.Tx, error) {
	return nil, nil
}
func (m *mockDbProvider) Close() {}
func (m *mockDbProvider) Ping(ctx context.Context) error { return nil }
func (m *mockDbProvider) GetDB() interface{} { return nil }
func (m *mockDbProvider) Dialect() string { return "sqlite" }
func (m *mockDbProvider) IsSQLite() bool { return true }
func (m *mockDbProvider) SearchMemories(ctx context.Context, organizationID string, queryText string, limit int) ([]string, error) {
	return nil, nil
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
func (r *mockRowFailScan) Err() error { return nil }
func (r *mockRowFailScan) Columns() ([]string, error) { return nil, nil }

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
func (r *mockRowFailJSON) Err() error { return nil }
func (r *mockRowFailJSON) Columns() ([]string, error) { return nil, nil }

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

type mockDbExecFailing struct {
	mockDbProvider
}
func (m *mockDbExecFailing) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	return 0, fmt.Errorf("forced exec error")
}
func (m *mockDbExecFailing) AcquireTask(ctx context.Context, organizationID, agentID string) (*db.TaskRecord, error) {
	return nil, nil
}
func (m *mockDbExecFailing) IsSQLite() bool { return true }
func (m *mockDbExecFailing) SearchMemories(ctx context.Context, organizationID string, queryText string, limit int) ([]string, error) {
	return nil, nil
}

func TestConsolidator_ProcessMemoryFile(t *testing.T) {
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

	c := NewConsolidator(nil, mockLLMClient, tempDir)
	claims := &auth.Claims{OrganizationID: "test-org"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	err = c.processMemoryFile(ctx, filePath)
	if err != nil {
		t.Errorf("Expected nil, got %v", err)
	}

	// Wait, we need to test vectorRepo Upsert fail
	mockRepoDB := &mockDbProvider{}
	vr := memory.NewVectorRepository(mockRepoDB)
	cWithRepo := NewConsolidator(vr, mockLLMClient, tempDir)
	err = cWithRepo.processMemoryFile(ctx, filePath)
	if err != nil {
		t.Errorf("Expected nil when upsert works but uses mock db, got %v", err)
	}

	// invalid yaml
	badPath := filepath.Join(tempDir, "bad.yml")
	os.WriteFile(badPath, []byte("bad yaml content: :"), 0644)
	err = c.processMemoryFile(ctx, badPath)
	if err == nil {
		t.Errorf("Expected error for invalid yaml")
	}

	// missing fields
	missingPath := filepath.Join(tempDir, "missing.yml")
	os.WriteFile(missingPath, []byte("task_id: ''\ncontent: ''"), 0644)
	err = c.processMemoryFile(ctx, missingPath)
	if err == nil {
		t.Errorf("Expected error for missing fields")
	}

	// file not found
	err = c.processMemoryFile(ctx, filepath.Join(tempDir, "notfound.yml"))
	if err == nil {
		t.Errorf("Expected error for not found file")
	}

	// generate embedding error
	mockLLMFail := &myMockLLM{failEmbed: true}
	cFail := NewConsolidator(nil, mockLLMFail, tempDir)
	err = cFail.processMemoryFile(ctx, filePath)
	if err == nil {
		t.Errorf("Expected error for embedding fail")
	}
}

func TestConsolidator_Consolidate(t *testing.T) {
	c := &Consolidator{
		llm: &myMockLLM{},
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "test-org"})

	err := c.Consolidate(ctx, "task1", []string{})
	if err != nil {
		t.Errorf("Expected nil error for empty logs, got %v", err)
	}

	err = c.Consolidate(ctx, "task1", []string{"log1", "log2"})
	if err != nil {
		t.Errorf("Expected nil error, got %v", err)
	}

	// Test upsert failure by supplying a failing DB
	vr := memory.NewVectorRepository(&mockDbExecFailing{})
	cWithVR := &Consolidator{llm: &myMockLLM{}, vectorRepo: vr}
	err = cWithVR.Consolidate(ctx, "task1", []string{"log1", "log2"})
	if err == nil {
		t.Errorf("Expected error for upsert fail")
	}

	// Error missing org
	err = c.Consolidate(context.Background(), "task1", []string{"log1"})
	if err == nil {
		t.Errorf("Expected error for missing org")
	}

	mockLLMFail := &myMockLLM{failReason: true}
	cFail := &Consolidator{llm: mockLLMFail}
	err = cFail.Consolidate(ctx, "task1", []string{"log1"})
	if err == nil {
		t.Errorf("Expected reason fail")
	}

	mockLLMFail2 := &myMockLLM{failEmbed: true}
	cFail2 := &Consolidator{llm: mockLLMFail2}
	err = cFail2.Consolidate(ctx, "task1", []string{"log1"})
	if err == nil {
		t.Errorf("Expected embed fail")
	}
}

func TestPushToCloud(t *testing.T) {
	c := &Consolidator{}
	err := c.PushToCloud(context.Background(), nil)
	if err == nil {
		t.Errorf("Expected err when dbprovider is nil")
	}

	mockDb := &mockDbProvider{}
	err = c.PushToCloud(context.Background(), mockDb)
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
	err = c.PushToCloud(context.Background(), mockDbFail)
	if err == nil {
		t.Errorf("Expected error")
	}

	mockDbScanFail := &mockDbProvider{failScan: true}
	err = c.PushToCloud(context.Background(), mockDbScanFail)
	if err != nil {
		t.Errorf("Expected nil, got %v", err)
	}

	mockDbJSONFail := &mockDbProvider{failJSON: true}
	err = c.PushToCloud(context.Background(), mockDbJSONFail)
	if err != nil {
		t.Errorf("Expected nil, got %v", err)
	}
}

func TestStartWatcher(t *testing.T) {
	c := NewConsolidator(nil, &myMockLLM{}, "")
	err := c.StartWatcher(context.Background())
	if err != nil {
		t.Errorf("Expected nil for empty dir, got %v", err)
	}

	cFailMake := NewConsolidator(nil, &myMockLLM{}, "/root/impossible/dir/to/make")
	err = cFailMake.StartWatcher(context.Background())
	if err == nil {
		t.Errorf("Expected error for impossible dir")
	}

	tempDir := t.TempDir()
	c = NewConsolidator(nil, &myMockLLM{}, tempDir)
	ctx, cancel := context.WithCancel(context.Background())
	err = c.StartWatcher(ctx)
	if err != nil {
		t.Errorf("Expected nil, got %v", err)
	}

	// Write a file to trigger watcher
	os.WriteFile(filepath.Join(tempDir, "watch.yml"), []byte("task_id: w1\ncontent: test"), 0644)
	time.Sleep(100 * time.Millisecond) // Let watcher process

	cancel()
	time.Sleep(50 * time.Millisecond) // Let goroutine exit
}
