package kairos

import (
    "context"
    "database/sql"
    "testing"
    "time"

    "github.com/stretchr/testify/assert"
    "github.com/onehumancorp/mono/srcs/server/db"
    "github.com/onehumancorp/mono/srcs/server/telemetry"
    _ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) db.Provider {
    t.Helper()
    telemetry.InitTelemetry() // needed for metrics
    dbConn, err := sql.Open("sqlite", "file:kairos-test?mode=memory&cache=shared")
    assert.NoError(t, err)
    prov := db.NewSqliteProvider(dbConn)

    _, err = prov.Exec(context.Background(), `
    CREATE TABLE IF NOT EXISTS shared_tasks_decomposition (
        id TEXT PRIMARY KEY,
        organization_id TEXT NOT NULL,
        title TEXT NOT NULL,
        status TEXT NOT NULL DEFAULT 'PENDING',
        payload TEXT
    )`)
    assert.NoError(t, err)

    _, err = prov.Exec(context.Background(), `
    CREATE TABLE IF NOT EXISTS autodream_memories (
        id TEXT PRIMARY KEY,
        task_id TEXT REFERENCES shared_tasks_decomposition(id),
        content TEXT NOT NULL,
        embedding TEXT,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP
    )`)
    assert.NoError(t, err)

    return prov
}

type mockLLMClient struct{
    fail bool
}
func (m *mockLLMClient) ChatCompletion(ctx context.Context, payload map[string]interface{}) (map[string]interface{}, error) {
    return nil, nil
}
func (m *mockLLMClient) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
    if m.fail {
        return nil, assert.AnError
    }
    emb := make([]float32, 1536)
    emb[0] = 0.5
    return emb, nil
}
func (m *mockLLMClient) Reason(ctx context.Context, prompt string) (string, error) {
    return "", nil
}

func TestAutoDreamWorker_Consolidate(t *testing.T) {
    prov := setupTestDB(t)

    _, err := prov.Exec(context.Background(), "INSERT INTO shared_tasks_decomposition (id, organization_id, title, status, payload) VALUES ('task-1', 'org-1', 'Title', 'COMPLETED', '{\"data\":\"test\"}')")
    assert.NoError(t, err)

    worker := NewAutoDreamWorker(prov, &mockLLMClient{})
    worker.Consolidate(context.Background())

    var count int
    err = prov.QueryRow(context.Background(), "SELECT COUNT(*) FROM autodream_memories WHERE task_id = 'task-1'").Scan(&count)
    assert.NoError(t, err)
    assert.Equal(t, 1, count)

    var status string
    err = prov.QueryRow(context.Background(), "SELECT status FROM shared_tasks_decomposition WHERE id = 'task-1'").Scan(&status)
    assert.NoError(t, err)
    assert.Equal(t, "ARCHIVED", status)
}

func TestAutoDreamWorker_Consolidate_LLMFailure(t *testing.T) {
    prov := setupTestDB(t)

    _, err := prov.Exec(context.Background(), "INSERT INTO shared_tasks_decomposition (id, organization_id, title, status, payload) VALUES ('task-2', 'org-1', 'Title', 'COMPLETED', '{\"data\":\"test\"}')")
    assert.NoError(t, err)

    worker := NewAutoDreamWorker(prov, &mockLLMClient{fail: true})
    worker.Consolidate(context.Background())

    var count int
    err = prov.QueryRow(context.Background(), "SELECT COUNT(*) FROM autodream_memories WHERE task_id = 'task-2'").Scan(&count)
    assert.NoError(t, err)
    assert.Equal(t, 0, count)

    var status string
    err = prov.QueryRow(context.Background(), "SELECT status FROM shared_tasks_decomposition WHERE id = 'task-2'").Scan(&status)
    assert.NoError(t, err)
    assert.Equal(t, "COMPLETED", status)
}

func TestAutoDreamWorker_Start(t *testing.T) {
    prov := setupTestDB(t)
    worker := NewAutoDreamWorker(prov, &mockLLMClient{})
    ctx, cancel := context.WithCancel(context.Background())
    done := make(chan struct{})
    go func() {
        worker.Start(ctx, 1*time.Millisecond)
        close(done)
    }()
    time.Sleep(10 * time.Millisecond)
    cancel()
    <-done
}
