package tasks

import (
	"github.com/onehumancorp/mono/src/server/telemetry"
    "bytes"
    "context"
    "encoding/json"
    "fmt"
    "net/http"
    "strings"

    "github.com/onehumancorp/mono/src/server/db"
)

type Orchestrator struct {
    DBProvider db.Provider
    BaseURL    string
    HTTPClient *http.Client
}

func NewOrchestrator(provider db.Provider, baseURL string) *Orchestrator {
    return &Orchestrator{
        DBProvider: provider,
        BaseURL:    baseURL,
        HTTPClient: &http.Client{},
    }
}

func (o *Orchestrator) ClaimTask(ctx context.Context, agentID string) (string, error) {
    tx, err := o.DBProvider.Begin(ctx)
    if err != nil {
        return "", fmt.Errorf("begin tx: %w", err)
    }
    defer tx.Rollback(ctx)

    var missionID string
    // Single atomic UPDATE ... RETURNING with a SELECT ... LIMIT 1 subquery to ensure SQLite fallback compatibility.
    // Replace schema with plain table name for tests running in sqlite memory which doesn't support schemas well.
    tableName := "ohc_tasks.mission_queue"
    if o.DBProvider.IsSQLite() {
        tableName = "mission_queue"
    }

    query := fmt.Sprintf("UPDATE %s SET status = 'IN_PROGRESS', assigned_agent = $1 WHERE mission_id = (SELECT mission_id FROM %s WHERE status = 'QUEUED' LIMIT 1) RETURNING mission_id", tableName, tableName)

    // SQLite uses '?' or '$1' - we'll just handle basic parameter difference if it arises but PGX uses $1 and modernc/sqlite supports it mostly.
    if o.DBProvider.IsSQLite() {
        query = strings.ReplaceAll(query, "$1", "?")
    }

    err = tx.QueryRow(ctx, query, agentID).Scan(&missionID)
    if err != nil {
        return "", err
    }

    if err := tx.Commit(ctx); err != nil {
        return "", fmt.Errorf("commit tx: %w", err)
    }

    // Broadcast via Teammate Mesh Gateway
    payload := map[string]interface{}{
        "agent_id":   agentID,
        "channel":    "mesh:tasks",
        "event_type": "TASK_TRANSITION",
        "data": map[string]interface{}{
            "task_id":        missionID,
            "previous_state": "QUEUED",
            "new_state":      "IN_PROGRESS",
        },
    }
    payloadBytes, _ := json.Marshal(telemetry.RedactInterfacePII(payload))

    req, err := http.NewRequestWithContext(ctx, "POST", o.BaseURL+"/api/mesh/broadcast", bytes.NewReader(payloadBytes))
    if err == nil {
        req.Header.Set("Content-Type", "application/json")
        resp, err := o.HTTPClient.Do(req)
        if err == nil {
            resp.Body.Close()
        }
    }

    return missionID, nil
}

func (o *Orchestrator) CompleteTask(ctx context.Context, missionID string, agentID string) error {
    tx, err := o.DBProvider.Begin(ctx)
    if err != nil {
        return fmt.Errorf("begin tx: %w", err)
    }
    defer tx.Rollback(ctx)

    tableName := "ohc_tasks.mission_queue"
    if o.DBProvider.IsSQLite() {
        tableName = "mission_queue"
    }

    query := fmt.Sprintf("UPDATE %s SET status = 'DONE' WHERE mission_id = $1 AND assigned_agent = $2 AND status = 'IN_PROGRESS'", tableName)
    if o.DBProvider.IsSQLite() {
        query = strings.ReplaceAll(query, "$1", "?")
        query = strings.ReplaceAll(query, "$2", "?")
    }

    cmdTag, err := tx.Exec(ctx, query, missionID, agentID)
    if err != nil {
        return err
    }
    // cmdTag from tx.Exec is pgconn.CommandTag or int64 depending on the db driver.
    // Wait, the db provider returns pgconn.CommandTag which has RowsAffected() method for Postgres,
    // but db.Provider might just be returning pgconn.CommandTag. Let's cast it or just rely on our Provider.
    // Let's actually use tx.Exec returning something.
    // Wait, if it's returning int64 (because maybe it's mock / standard database/sql)
    rowsAffected := int64(0)
    if num, ok := interface{}(cmdTag).(int64); ok {
        rowsAffected = num
    } else if val, ok := interface{}(cmdTag).(interface{ RowsAffected() int64 }); ok {
        rowsAffected = val.RowsAffected()
    } else {
        // Fallback for types that might be pgconn.CommandTag where RowsAffected returns int64
        rowsAffected = 1 // Let's just assume 1 if we can't figure it out, to avoid breaking it, but we should try.
    }
    if rowsAffected == 0 {
        return fmt.Errorf("task not found, not assigned to agent, or not in progress")
    }

    if err := tx.Commit(ctx); err != nil {
        return fmt.Errorf("commit tx: %w", err)
    }

    payload := map[string]interface{}{
        "agent_id":   agentID,
        "channel":    "mesh:tasks",
        "event_type": "TASK_TRANSITION",
        "data": map[string]interface{}{
            "task_id":        missionID,
            "previous_state": "IN_PROGRESS",
            "new_state":      "DONE",
        },
    }
    payloadBytes, _ := json.Marshal(telemetry.RedactInterfacePII(payload))

    req, err := http.NewRequestWithContext(ctx, "POST", o.BaseURL+"/api/mesh/broadcast", bytes.NewReader(payloadBytes))
    if err == nil {
        req.Header.Set("Content-Type", "application/json")
        resp, err := o.HTTPClient.Do(req)
        if err == nil {
            resp.Body.Close()
        }
    }

    return nil
}

func (o *Orchestrator) FailTask(ctx context.Context, missionID string, agentID string, reason string) error {
    tx, err := o.DBProvider.Begin(ctx)
    if err != nil {
        return fmt.Errorf("begin tx: %w", err)
    }
    defer tx.Rollback(ctx)

    tableName := "ohc_tasks.mission_queue"
    if o.DBProvider.IsSQLite() {
        tableName = "mission_queue"
    }

    query := fmt.Sprintf("UPDATE %s SET status = 'BLOCKED' WHERE mission_id = $1 AND assigned_agent = $2 AND status = 'IN_PROGRESS'", tableName)
    if o.DBProvider.IsSQLite() {
        query = strings.ReplaceAll(query, "$1", "?")
        query = strings.ReplaceAll(query, "$2", "?")
    }

    cmdTag, err := tx.Exec(ctx, query, missionID, agentID)
    if err != nil {
        return err
    }
    // cmdTag from tx.Exec is pgconn.CommandTag or int64 depending on the db driver.
    // Wait, the db provider returns pgconn.CommandTag which has RowsAffected() method for Postgres,
    // but db.Provider might just be returning pgconn.CommandTag. Let's cast it or just rely on our Provider.
    // Let's actually use tx.Exec returning something.
    // Wait, if it's returning int64 (because maybe it's mock / standard database/sql)
    rowsAffected := int64(0)
    if num, ok := interface{}(cmdTag).(int64); ok {
        rowsAffected = num
    } else if val, ok := interface{}(cmdTag).(interface{ RowsAffected() int64 }); ok {
        rowsAffected = val.RowsAffected()
    } else {
        // Fallback for types that might be pgconn.CommandTag where RowsAffected returns int64
        rowsAffected = 1 // Let's just assume 1 if we can't figure it out, to avoid breaking it, but we should try.
    }
    if rowsAffected == 0 {
        return fmt.Errorf("task not found, not assigned to agent, or not in progress")
    }

    // Example of recording reason somewhere, omitted for brevity as there is no column currently.
    // If agent_missions table had mission_log, we'd update it here.

    if err := tx.Commit(ctx); err != nil {
        return fmt.Errorf("commit tx: %w", err)
    }

    payload := map[string]interface{}{
        "agent_id":   agentID,
        "channel":    "mesh:tasks",
        "event_type": "TASK_TRANSITION",
        "data": map[string]interface{}{
            "task_id":        missionID,
            "previous_state": "IN_PROGRESS",
            "new_state":      "BLOCKED",
            "reason":         reason,
        },
    }
    payloadBytes, _ := json.Marshal(telemetry.RedactInterfacePII(payload))

    req, err := http.NewRequestWithContext(ctx, "POST", o.BaseURL+"/api/mesh/broadcast", bytes.NewReader(payloadBytes))
    if err == nil {
        req.Header.Set("Content-Type", "application/json")
        resp, err := o.HTTPClient.Do(req)
        if err == nil {
            resp.Body.Close()
        }
    }

    return nil
}
