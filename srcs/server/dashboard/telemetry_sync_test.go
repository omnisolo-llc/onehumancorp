package dashboard

import (
    "bytes"
    "encoding/json"
    "net/http"
    "net/http/httptest"
    "testing"
    "github.com/onehumancorp/mono/srcs/server/telemetry"
)

func TestHandleTelemetrySync_NewMetrics(t *testing.T) {
    s := &Server{}

    payloads := []map[string]interface{}{
        {
            "metric_type": "cache_hit",
            "payload": `{"operation":"op","cache_type":"type"}`,
        },
        {
            "metric_type": "sqlite_lock_contention",
            "payload": `{"operation":"op"}`,
        },
        {
            "metric_type": "task_queue_length",
            "payload": `{"amount":5}`,
        },
        {
            "metric_type": "sub_agent_queue_length",
            "payload": `5`,
        },
    }
    body, _ := json.Marshal(payloads)

    req := httptest.NewRequest(http.MethodPost, "/api/telemetry/sync", bytes.NewBuffer(body))
    req.Header.Set("Content-Type", "application/json")
    w := httptest.NewRecorder()

    // Initialize global meter in telemetry package to avoid nil pointers
    _, _ = telemetry.InitTelemetry()

    s.handleTelemetrySync(w, req)

    if w.Code != http.StatusOK {
        t.Errorf("expected 200 OK, got %d", w.Code)
    }
}
