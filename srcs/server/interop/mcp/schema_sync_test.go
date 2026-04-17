package mcp

import (
    "context"
    "encoding/json"
    "net/http"
    "net/http/httptest"
    "testing"

    "github.com/onehumancorp/mono/srcs/server/db"
)

type syncMockTx struct {
    execCalls int
    shouldErr bool
    shouldErrCommit bool
}

func (m *syncMockTx) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
    m.execCalls++
    if m.shouldErr {
        return 0, context.DeadlineExceeded
    }
    return 1, nil
}
func (m *syncMockTx) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) { return nil, nil }
func (m *syncMockTx) QueryRow(ctx context.Context, sql string, optionsAndArgs ...any) db.Row { return nil }
func (m *syncMockTx) Commit(ctx context.Context) error {
    if m.shouldErrCommit {
        return context.DeadlineExceeded
    }
    return nil
}
func (m *syncMockTx) Rollback(ctx context.Context) error { return nil }


type syncMockDB struct {
    mockDBProvider
    tx *syncMockTx
    shouldFailTx bool
    shouldFailExec bool
    shouldFailCommit bool
    isSqlite bool
}

func (m *syncMockDB) Begin(ctx context.Context) (db.Tx, error) {
    if m.shouldFailTx {
        return nil, context.DeadlineExceeded
    }
    m.tx = &syncMockTx{shouldErr: m.shouldFailExec, shouldErrCommit: m.shouldFailCommit}
    return m.tx, nil
}

func (m *syncMockDB) IsSQLite() bool {
    return m.isSqlite
}


func TestSchemaSyncTool_Execute(t *testing.T) {
    ctx := context.Background()

    migrations := []Migration{
        {Filename: "001.sql", Content: "CREATE TABLE test (id BIGSERIAL PRIMARY KEY, data JSONB);"},
    }

    server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
        if r.URL.Path != "/api/mcp/schema" || r.URL.Query().Get("target_version") != "latest" {
            w.WriteHeader(http.StatusBadRequest)
            return
        }
        w.WriteHeader(http.StatusOK)
        json.NewEncoder(w).Encode(migrations)
    }))
    defer server.Close()

    mockDB := &syncMockDB{}
    proxy := NewMcpSyncProxy(mockDB, nil, server.URL)
    tool := NewSchemaSyncTool(proxy)

    err := tool.Execute(ctx, "latest")
    if err != nil {
        t.Fatalf("Expected no error, got %v", err)
    }
    if mockDB.tx == nil || mockDB.tx.execCalls != 1 {
        t.Errorf("Expected 1 transaction execution")
    }
}

func TestSchemaSyncTool_Execute_SQLite(t *testing.T) {
    ctx := context.Background()

    migrations := []Migration{
        {Filename: "001.sql", Content: "CREATE TABLE test (id BIGSERIAL PRIMARY KEY, data JSONB);"},
    }

    server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
        w.WriteHeader(http.StatusOK)
        json.NewEncoder(w).Encode(migrations)
    }))
    defer server.Close()

    mockDB := &syncMockDB{isSqlite: true}
    proxy := NewMcpSyncProxy(mockDB, nil, server.URL)
    tool := NewSchemaSyncTool(proxy)

    err := tool.Execute(ctx, "latest")
    if err != nil {
        t.Fatalf("Expected no error, got %v", err)
    }
    if mockDB.tx == nil || mockDB.tx.execCalls != 1 {
        t.Errorf("Expected 1 transaction execution")
    }
}


func TestSchemaSyncTool_Execute_HTTPError(t *testing.T) {
    ctx := context.Background()

    server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
        w.WriteHeader(http.StatusInternalServerError)
    }))
    defer server.Close()

    mockDB := &syncMockDB{}
    proxy := NewMcpSyncProxy(mockDB, nil, server.URL)
    tool := NewSchemaSyncTool(proxy)

    err := tool.Execute(ctx, "latest")
    if err == nil {
        t.Fatalf("Expected error, got nil")
    }
}

func TestSchemaSyncTool_Execute_DecodeError(t *testing.T) {
    ctx := context.Background()

    server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
        w.WriteHeader(http.StatusOK)
        w.Write([]byte("invalid json"))
    }))
    defer server.Close()

    mockDB := &syncMockDB{}
    proxy := NewMcpSyncProxy(mockDB, nil, server.URL)
    tool := NewSchemaSyncTool(proxy)

    err := tool.Execute(ctx, "latest")
    if err == nil {
        t.Fatalf("Expected error, got nil")
    }
}

func TestSchemaSyncTool_Execute_DBTxError(t *testing.T) {
    ctx := context.Background()
    migrations := []Migration{ {Filename: "001.sql", Content: "CREATE TABLE test (id BIGSERIAL PRIMARY KEY);"} }
    server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
        w.WriteHeader(http.StatusOK)
        json.NewEncoder(w).Encode(migrations)
    }))
    defer server.Close()

    mockDB := &syncMockDB{shouldFailTx: true}
    proxy := NewMcpSyncProxy(mockDB, nil, server.URL)
    tool := NewSchemaSyncTool(proxy)

    err := tool.Execute(ctx, "latest")
    if err == nil {
        t.Fatalf("Expected error when DB Begin fails, got nil")
    }
}

func TestSchemaSyncTool_Execute_DBExecError(t *testing.T) {
    ctx := context.Background()
    migrations := []Migration{ {Filename: "001.sql", Content: "CREATE TABLE test (id BIGSERIAL PRIMARY KEY);"} }
    server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
        w.WriteHeader(http.StatusOK)
        json.NewEncoder(w).Encode(migrations)
    }))
    defer server.Close()

    mockDB := &syncMockDB{shouldFailExec: true}
    proxy := NewMcpSyncProxy(mockDB, nil, server.URL)
    tool := NewSchemaSyncTool(proxy)

    err := tool.Execute(ctx, "latest")
    if err == nil {
        t.Fatalf("Expected error when DB Exec fails, got nil")
    }
}

func TestSchemaSyncTool_Execute_DBCommitError(t *testing.T) {
    ctx := context.Background()
    migrations := []Migration{ {Filename: "001.sql", Content: "CREATE TABLE test (id BIGSERIAL PRIMARY KEY);"} }
    server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
        w.WriteHeader(http.StatusOK)
        json.NewEncoder(w).Encode(migrations)
    }))
    defer server.Close()

    mockDB := &syncMockDB{shouldFailCommit: true}
    proxy := NewMcpSyncProxy(mockDB, nil, server.URL)
    tool := NewSchemaSyncTool(proxy)

    err := tool.Execute(ctx, "latest")
    if err == nil {
        t.Fatalf("Expected error when DB Commit fails, got nil")
    }
}

func TestSchemaSyncTool_Execute_BadReqError(t *testing.T) {
    ctx := context.Background()

    mockDB := &syncMockDB{}
    // Invalid URL schema to fail http.NewRequestWithContext
    proxy := NewMcpSyncProxy(mockDB, nil, "http://192.168.0.%31/")
    tool := NewSchemaSyncTool(proxy)

    err := tool.Execute(ctx, "latest")
    if err == nil {
        t.Fatalf("Expected error, got nil")
    }
}

func TestSchemaSyncTool_Execute_ReqExecError(t *testing.T) {
    ctx := context.Background()

    mockDB := &syncMockDB{}
    // Simulate Do error (server down/unreachable)
    proxy := NewMcpSyncProxy(mockDB, nil, "http://localhost:12345/nonexistent")
    tool := NewSchemaSyncTool(proxy)

    err := tool.Execute(ctx, "latest")
    if err == nil {
        t.Fatalf("Expected error, got nil")
    }
}
