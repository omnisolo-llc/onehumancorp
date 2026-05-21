package mcp_audit_sync

import (
    "context"
    "database/sql"
    "os"
    "testing"
    "time"

    _ "github.com/mattn/go-sqlite3"
)

func setupTestDB(t *testing.T) *sql.DB {
    db, err := sql.Open("sqlite3", ":memory:")
    if err != nil {
        t.Fatalf("failed to open sqlite DB: %v", err)
    }

    _, err = db.Exec(`
        CREATE TABLE mcp_audit_sync_log (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            tenant_id TEXT,
            agent_id TEXT,
            action TEXT,
            resource TEXT,
            status TEXT,
            metadata TEXT,
            timestamp INTEGER
        )
    `)
    if err != nil {
        t.Fatalf("failed to create table: %v", err)
    }

    return db
}

func TestSyncAuditLogsToCloud(t *testing.T) {
    db := setupTestDB(t)
    defer db.Close()

    logger := NewAuditLogger(db)

    tests := []struct {
        name     string
        payload  AuditSyncPayload
        spiffeID string
        wantErr  bool
    }{
        {
            name: "valid payload and valid spiffe ID",
            payload: AuditSyncPayload{
                TenantID:  "tenant-1",
                AgentID:   "agent-1",
                Action:    "read",
                Resource:  "file.txt",
                Status:    "success",
                Metadata:  "{}",
                Timestamp: time.Now().Unix(),
            },
            spiffeID: "spiffe://onehumancorp.io/tenant/tenant-1/agent/agent-1",
            wantErr:  false,
        },
        {
            name: "valid payload and admin spiffe ID",
            payload: AuditSyncPayload{
                TenantID:  "tenant-1",
                AgentID:   "agent-1",
                Action:    "read",
                Resource:  "file.txt",
                Status:    "success",
                Metadata:  "{}",
                Timestamp: time.Now().Unix(),
            },
            spiffeID: "spiffe://onehumancorp.io/admin",
            wantErr:  false,
        },
        {
            name: "invalid spiffe ID",
            payload: AuditSyncPayload{
                TenantID:  "tenant-1",
                AgentID:   "agent-1",
                Action:    "read",
                Resource:  "file.txt",
                Status:    "success",
                Metadata:  "{}",
                Timestamp: time.Now().Unix(),
            },
            spiffeID: "spiffe://onehumancorp.io/tenant/tenant-2/agent/agent-1",
            wantErr:  true,
        },
        {
            name: "missing tenant_id",
            payload: AuditSyncPayload{
                AgentID:   "agent-1",
                Action:    "read",
                Resource:  "file.txt",
                Status:    "success",
                Metadata:  "{}",
                Timestamp: time.Now().Unix(),
            },
            spiffeID: "spiffe://onehumancorp.io/tenant//agent/agent-1",
            wantErr:  true,
        },
        {
            name: "missing agent_id",
            payload: AuditSyncPayload{
                TenantID:  "tenant-1",
                Action:    "read",
                Resource:  "file.txt",
                Status:    "success",
                Metadata:  "{}",
                Timestamp: time.Now().Unix(),
            },
            spiffeID: "spiffe://onehumancorp.io/tenant/tenant-1/agent/",
            wantErr:  true,
        },
        {
            name: "missing action",
            payload: AuditSyncPayload{
                TenantID:  "tenant-1",
                AgentID:   "agent-1",
                Resource:  "file.txt",
                Status:    "success",
                Metadata:  "{}",
                Timestamp: time.Now().Unix(),
            },
            spiffeID: "spiffe://onehumancorp.io/tenant/tenant-1/agent/agent-1",
            wantErr:  true,
        },
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            err := logger.SyncAuditLogsToCloud(context.Background(), tt.payload, tt.spiffeID)
            if (err != nil) != tt.wantErr {
                t.Errorf("SyncAuditLogsToCloud() error = %v, wantErr %v", err, tt.wantErr)
            }
        })
    }
}

func TestSyncAuditLogsToCloud_MissingTimestamp(t *testing.T) {
    db := setupTestDB(t)
    defer db.Close()

    logger := NewAuditLogger(db)

    payload := AuditSyncPayload{
        TenantID:  "tenant-1",
        AgentID:   "agent-1",
        Action:    "read",
        Resource:  "file.txt",
        Status:    "success",
        Metadata:  "{}",
    }

    err := logger.SyncAuditLogsToCloud(context.Background(), payload, "spiffe://onehumancorp.io/tenant/tenant-1/agent/agent-1")
    if err != nil {
        t.Errorf("SyncAuditLogsToCloud() unexpected error: %v", err)
    }

    var timestamp int64
    err = db.QueryRow("SELECT timestamp FROM mcp_audit_sync_log LIMIT 1").Scan(&timestamp)
    if err != nil {
        t.Fatalf("failed to query db: %v", err)
    }

    if timestamp == 0 {
        t.Errorf("expected timestamp to be set, got 0")
    }
}

func TestSyncAuditLogsToCloud_TelemetryEnabled(t *testing.T) {
    os.Setenv("OHC_TELEMETRY_ENABLED", "true")
    defer os.Unsetenv("OHC_TELEMETRY_ENABLED")

    db := setupTestDB(t)
    defer db.Close()

    logger := NewAuditLogger(db)

    payload := AuditSyncPayload{
        TenantID:  "tenant-1",
        AgentID:   "agent-1",
        Action:    "read",
        Resource:  "file.txt",
        Status:    "success",
        Metadata:  "{}",
    }

    err := logger.SyncAuditLogsToCloud(context.Background(), payload, "spiffe://onehumancorp.io/tenant/tenant-1/agent/agent-1")
    if err != nil {
        t.Errorf("SyncAuditLogsToCloud() unexpected error: %v", err)
    }
}

func TestSyncAuditLogsToCloud_TelemetryEnabledError(t *testing.T) {
    os.Setenv("OHC_TELEMETRY_ENABLED", "true")
    defer os.Unsetenv("OHC_TELEMETRY_ENABLED")

    db := setupTestDB(t)
    defer db.Close()

    logger := NewAuditLogger(db)

    payload := AuditSyncPayload{
        TenantID:  "tenant-1",
        AgentID:   "agent-1",
        Action:    "read",
        Resource:  "file.txt",
        Status:    "success",
        Metadata:  "{}",
    }

    // Pass invalid SPIFFE to trigger an error while telemetry is enabled
    err := logger.SyncAuditLogsToCloud(context.Background(), payload, "invalid")
    if err == nil {
        t.Errorf("SyncAuditLogsToCloud() expected error, got nil")
    }

    // Pass empty payload to trigger another error branch
    err = logger.SyncAuditLogsToCloud(context.Background(), AuditSyncPayload{}, "invalid")
    if err == nil {
        t.Errorf("SyncAuditLogsToCloud() expected error, got nil")
    }
}

func TestSyncAuditLogsToCloud_DBError(t *testing.T) {
    db := setupTestDB(t)
    // Close the DB immediately to force an error
    db.Close()

    logger := NewAuditLogger(db)

    payload := AuditSyncPayload{
        TenantID:  "tenant-1",
        AgentID:   "agent-1",
        Action:    "read",
        Resource:  "file.txt",
        Status:    "success",
        Metadata:  "{}",
    }

    os.Setenv("OHC_TELEMETRY_ENABLED", "true")
    defer os.Unsetenv("OHC_TELEMETRY_ENABLED")

    err := logger.SyncAuditLogsToCloud(context.Background(), payload, "spiffe://onehumancorp.io/tenant/tenant-1/agent/agent-1")
    if err == nil {
        t.Errorf("expected error due to closed db, got nil")
    }
}


func TestExecute(t *testing.T) {
    db := setupTestDB(t)
    defer db.Close()

    logger := NewAuditLogger(db)

    if logger.Name() != "sync_audit_logs_to_cloud" {
        t.Errorf("expected name sync_audit_logs_to_cloud, got %s", logger.Name())
    }

    if logger.Description() == "" {
        t.Errorf("expected non-empty description")
    }

    params := map[string]interface{}{
        "tenant_id": "tenant-1",
        "agent_id":  "agent-1",
        "action":    "read",
        "resource":  "file.txt",
        "status":    "success",
        "metadata":  "{}",
        "timestamp": float64(time.Now().Unix()),
    }

    res, err := logger.Execute(context.Background(), params, "spiffe://onehumancorp.io/tenant/tenant-1/agent/agent-1")
    if err != nil {
        t.Errorf("Execute() unexpected error: %v", err)
    }

    resMap, ok := res.(map[string]string)
    if !ok || resMap["status"] != "success" {
        t.Errorf("Execute() unexpected result: %v", res)
    }

    // test error path
    params["action"] = ""
    _, err = logger.Execute(context.Background(), params, "spiffe://onehumancorp.io/tenant/tenant-1/agent/agent-1")
    if err == nil {
        t.Errorf("Execute() expected error due to missing action, got nil")
    }
}
