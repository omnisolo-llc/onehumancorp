package mesh

import (
    "context"
    "encoding/json"
    "net/http"
    "os"

    "github.com/redis/rueidis"
    "github.com/onehumancorp/mono/srcs/server/telemetry"
)

type MeshEvent struct {
    AgentID string `json:"agent_id"`
    Action  string `json:"action"`
    Status  string `json:"status"`
}

type MeshPublisher interface {
    PublishMeshEvent(channel string, payload string) error
    PublishCentrifugeTask(taskID string, payload map[string]interface{})
}

type MeshServer struct {
    hub    MeshPublisher
    client rueidis.Client
}

func NewMeshServer(hub MeshPublisher) *MeshServer {
    var client rueidis.Client
    if os.Getenv("OHC_MULTITENANT") == "true" {
        c, err := rueidis.NewClient(rueidis.ClientOption{InitAddress: []string{"localhost:6379"}})
        if err == nil {
            client = c
        }
    }
    return &MeshServer{hub: hub, client: client}
}

func (s *MeshServer) HandleMeshBroadcast(w http.ResponseWriter, r *http.Request) {
    mode := "cloud"
    if os.Getenv("OHC_STANDALONE") == "true" {
        mode = "standalone"
    }
    telemetry.RecordMeshBroadcast(r.Context(), mode)

    if r.Method != http.MethodPost {
        http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
        return
    }

    if r.TLS == nil || len(r.TLS.PeerCertificates) == 0 {
        http.Error(w, "mTLS SPIFFE identity required", http.StatusForbidden)
        return
    }
    cert := r.TLS.PeerCertificates[0]
    if len(cert.URIs) == 0 || cert.URIs[0].Scheme != "spiffe" {
        http.Error(w, "mTLS SPIFFE identity required", http.StatusForbidden)
        return
    }

    var req struct {
        Channel string `json:"channel"`
        AgentID string `json:"agent_id"`
        Action  string `json:"action"`
        Status  string `json:"status"`
    }
    if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
        http.Error(w, "invalid request", http.StatusBadRequest)
        return
    }

    if req.AgentID == "" || req.Action == "" || req.Status == "" {
        http.Error(w, "invalid payload: missing agent_id, action, or status", http.StatusBadRequest)
        return
    }

    if req.Channel != "mesh:tasks" && req.Channel != "mesh:coordination" {
        http.Error(w, "invalid channel", http.StatusBadRequest)
        return
    }

    payloadMap := map[string]interface{}{
        "agent_id": req.AgentID,
        "action":   req.Action,
        "status":   req.Status,
    }

    payloadBytes, err := json.Marshal(payloadMap)
    if err != nil {
        http.Error(w, "failed to marshal payload", http.StatusInternalServerError)
        return
    }

    var publishErr error
    if s.client != nil {
        ctx := context.Background()
        cmd := s.client.B().Publish().Channel(req.Channel).Message(string(payloadBytes)).Build()
        publishErr = s.client.Do(ctx, cmd).Error()
    } else if s.hub != nil {
        publishErr = s.hub.PublishMeshEvent(req.Channel, string(payloadBytes))
    }

    if publishErr == nil {
        telemetry.RecordTeammateMeshBroadcast(r.Context(), req.Channel)

        if s.hub != nil && req.Channel == "mesh:tasks" {
            s.hub.PublishCentrifugeTask("task-update", payloadMap)
        }

        w.WriteHeader(http.StatusOK)
        w.Write([]byte(`{"status":"ok"}`))
    } else {
        http.Error(w, "failed to publish", http.StatusInternalServerError)
    }
}
