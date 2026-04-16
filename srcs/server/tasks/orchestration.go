package tasks

import (
    "bytes"
    "context"
    "encoding/json"
    "fmt"
    "net/http"
    "strings"

    "github.com/onehumancorp/mono/srcs/server/db"
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
    payloadBytes, _ := json.Marshal(payload)

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
