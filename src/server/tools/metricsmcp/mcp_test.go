package metricsmcp

import (
	"context"
	"os"
	"testing"
    "database/sql"

	"github.com/onehumancorp/mono/src/server/auth"
	"github.com/onehumancorp/mono/src/server/db"
)

func TestLocalMetricsProvider(t *testing.T) {
	provider := NewLocalMetricsProvider(&MockDBProvider{})
	ctx := context.Background()

	metrics, err := provider.QueryMetrics(ctx, "cpu_usage")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if metrics["source"] != "local_buffer" {
		t.Errorf("expected source to be local_buffer, got %v", metrics["source"])
	}

	health, err := provider.GetHealthStatus(ctx)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if health["mode"] != "standalone" {
		t.Errorf("expected mode to be standalone, got %v", health["mode"])
	}
}

func TestCloudMetricsProvider_Unauthorized(t *testing.T) {
	provider := NewCloudMetricsProvider(&MockDBProvider{})
	ctx := context.Background()

	_, err := provider.QueryMetrics(ctx, "cpu_usage")
	if err == nil {
		t.Fatalf("expected error for unauthorized request")
	}

	_, err = provider.GetHealthStatus(ctx)
	if err == nil {
		t.Fatalf("expected error for unauthorized request")
	}
}

func TestCloudMetricsProvider_Authorized(t *testing.T) {
	provider := NewCloudMetricsProvider(&MockDBProvider{})

	claims := &auth.Claims{
		OrganizationID: "tenant-123",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	metrics, err := provider.QueryMetrics(ctx, "cpu_usage")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if metrics["tenant_id"] != "tenant-123" {
		t.Errorf("expected tenant_id to be tenant-123, got %v", metrics["tenant_id"])
	}

	health, err := provider.GetHealthStatus(ctx)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if health["tenant_id"] != "tenant-123" {
		t.Errorf("expected tenant_id to be tenant-123, got %v", health["tenant_id"])
	}
}

func TestHybridMetricsMCP(t *testing.T) {
	provider := NewLocalMetricsProvider(&MockDBProvider{})
	mcp := NewHybridMetricsMCP(provider)

	tools := mcp.ListTools()
	if len(tools) != 2 {
		t.Fatalf("expected 2 tools, got %d", len(tools))
	}

	ctx := context.Background()

	// Test query_metrics
	res, err := mcp.CallTool(ctx, "query_metrics", map[string]interface{}{"query": "test_query"})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	resMap, ok := res.(map[string]interface{})
	if !ok {
		t.Fatalf("expected map result")
	}
	if resMap["source"] != "local_buffer" {
		t.Errorf("expected source local_buffer")
	}

	// Test get_health_status
	res, err = mcp.CallTool(ctx, "get_health_status", map[string]interface{}{})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	resMap, ok = res.(map[string]interface{})
	if !ok {
		t.Fatalf("expected map result")
	}
	if resMap["status"] != "healthy" {
		t.Errorf("expected status healthy")
	}

	// Test unknown tool
	_, err = mcp.CallTool(ctx, "unknown_tool", map[string]interface{}{})
	if err == nil {
		t.Fatalf("expected error for unknown tool")
	}

	// Test query_metrics missing query
	_, err = mcp.CallTool(ctx, "query_metrics", map[string]interface{}{})
	if err == nil {
		t.Fatalf("expected error for missing query")
	}
}


func TestNewProviderFactory(t *testing.T) {
	origMulti := os.Getenv("OHC_MULTITENANT")
	origStandalone := os.Getenv("OHC_STANDALONE")
	t.Cleanup(func() {
		os.Setenv("OHC_MULTITENANT", origMulti)
		os.Setenv("OHC_STANDALONE", origStandalone)
	})

	// Test Cloud
	os.Setenv("OHC_MULTITENANT", "true")
	os.Setenv("OHC_STANDALONE", "false")
	p1 := NewProviderFactory(&MockDBProvider{})
	if _, ok := p1.(*CloudMetricsProvider); !ok {
		t.Errorf("expected CloudMetricsProvider")
	}

	// Test Local (Standalone)
	os.Setenv("OHC_STANDALONE", "true")
	p2 := NewProviderFactory(&MockDBProvider{})
	if _, ok := p2.(*LocalMetricsProvider); !ok {
		t.Errorf("expected LocalMetricsProvider")
	}

	// Test Local (Not Multitenant)
	os.Setenv("OHC_MULTITENANT", "false")
	os.Setenv("OHC_STANDALONE", "false")
	p3 := NewProviderFactory(&MockDBProvider{})
	if _, ok := p3.(*LocalMetricsProvider); !ok {
		t.Errorf("expected LocalMetricsProvider")
	}
}


type MockDBProvider struct {}

func (m *MockDBProvider) Query(ctx context.Context, query string, args ...interface{}) (db.Rows, error) {
    return &MockDBRows{}, nil
}

type MockDBRows struct {}
func (m *MockDBRows) Next() bool {
    return false
}
func (m *MockDBRows) Scan(dest ...interface{}) error {
    return nil
}
func (m *MockDBRows) Close() {}
func (m *MockDBRows) Columns() ([]string, error) {
    return nil, nil
}
func (m *MockDBRows) Err() error {
    return nil
}


func (m *MockDBProvider) Exec(ctx context.Context, query string, args ...interface{}) (int64, error) {
    return 0, nil
}
func (m *MockDBProvider) Ping(ctx context.Context) error {
    return nil
}

func (m *MockDBProvider) AcquireTask(ctx context.Context, namespace, agentID string) (*db.TaskRecord, error) {
    return nil, nil
}
func (m *MockDBProvider) ReleaseTask(ctx context.Context, id int64) error {
    return nil
}
func (m *MockDBProvider) CompleteTask(ctx context.Context, id int64, output map[string]interface{}) error {
    return nil
}
func (m *MockDBProvider) FailTask(ctx context.Context, id int64, errStr string) error {
    return nil
}
func (m *MockDBProvider) MarkTaskUnprocessable(ctx context.Context, id int64, errStr string) error {
    return nil
}
func (m *MockDBProvider) DeleteTask(ctx context.Context, id int64) error {
    return nil
}
func (m *MockDBProvider) BeginTx(ctx context.Context) (db.Tx, error) {
    return nil, nil
}
func (m *MockDBProvider) QueryRow(ctx context.Context, query string, args ...interface{}) db.Row {
    return nil
}
func (m *MockDBProvider) EnqueueTask(ctx context.Context, namespace, taskType string, payload map[string]interface{}) (int64, error) {
    return 0, nil
}
func (m *MockDBProvider) IsSQLite() bool {
    return false
}
func (m *MockDBProvider) DB() *sql.DB {
    return nil
}

func (m *MockDBProvider) Begin(ctx context.Context) (db.Tx, error) {
    return nil, nil
}
func (m *MockDBProvider) AcquireTaskWithDependencies(ctx context.Context, namespace, agentID string) (*db.TaskRecord, error) {
    return nil, nil
}
func (m *MockDBProvider) QueryContext(ctx context.Context, query string, args ...interface{}) (db.Rows, error) {
    return nil, nil
}

func (m *MockDBProvider) Close() {
}

func (m *MockDBProvider) SearchMemories(ctx context.Context, organizationID string, queryText string, limit int) ([]string, error) {
    return nil, nil
}
