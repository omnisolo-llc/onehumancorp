package dashboard

import (
	"os"
	"fmt"
	"time"
	"log/slog"
	orchmesh "github.com/onehumancorp/mono/srcs/server/orchestration/mesh"
	"encoding/json"
	"net/http"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

func (s *Server) handleMeshBroadcast(w http.ResponseWriter, r *http.Request) { // added for ohc_mesh_broadcast_total metric instrumentation
	mode := "cloud"
	if os.Getenv("OHC_STANDALONE") == "true" {
		mode = "standalone"
	}
	telemetry.RecordMeshBroadcast(r.Context(), mode)

	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	// Enforce mTLS checks
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
		Channel string                 `json:"channel"`
		AgentID string                 `json:"agent_id"`
		Action  string                 `json:"action"`
		Status  string                 `json:"status"`
		Payload map[string]interface{} `json:"payload,omitempty"`
	}
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "invalid request", http.StatusBadRequest)
		return
	}

	if req.AgentID == "" || req.Action == "" || req.Status == "" {
		http.Error(w, "invalid request: missing required fields", http.StatusBadRequest)
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
	if req.Payload != nil {
		payloadMap["payload"] = req.Payload
	}

	payloadBytes, err := json.Marshal(payloadMap)
	if err != nil {
		http.Error(w, "failed to marshal payload", http.StatusInternalServerError)
		return
	}

	var broker orchmesh.MeshBroker
	broker = s.MeshBroker
	if broker == nil {
		broker = orchmesh.NewLocalMeshBroker()
	}
	err = broker.Broadcast(r.Context(), req.Channel, payloadBytes)

	_ = s.hub.Publish(orchestration.Message{
		ID:        fmt.Sprintf("%d", time.Now().UnixNano()),
		FromAgent: "system",
		ToAgent:   "system",
		Type:      req.Channel,
		Content:   string(payloadBytes),
	})

	if err == nil {
		telemetry.RecordTeammateMeshBroadcast(r.Context(), req.Channel)

		// Map mesh channels to Centrifuge WebSocket channels for UI updates
		if s.hub != nil && s.hub.CentrifugeNode() != nil {
			if req.Channel == "mesh:tasks" {
				s.hub.CentrifugeNode().PublishTaskBroadcast(fmt.Sprintf("%d", time.Now().UnixNano()), payloadMap)
			} else if req.Channel == "mesh:coordination" {
				s.hub.CentrifugeNode().PublishCoordinationMessage(orchestration.Message{
					ID:        fmt.Sprintf("%d", time.Now().UnixNano()),
					FromAgent: req.AgentID,
					ToAgent:   "system",
					Type:      req.Channel,
					Content:   string(payloadBytes),
				})
			}
		}
	} else {
		http.Error(w, "failed to broadcast", http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
	_, _ = w.Write([]byte(`{"status":"ok"}`))
}

func (s *Server) handleMeshDirect(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	// Enforce mTLS checks
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
		ToAgent string `json:"toAgent"`
		Payload string `json:"payload"`
	}
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "invalid request", http.StatusBadRequest)
		return
	}

	err := s.hub.Publish(orchestration.Message{
		ID:        fmt.Sprintf("%d", time.Now().UnixNano()),
		FromAgent: "system",
		ToAgent:   req.ToAgent,
		Type:      "mesh:direct",
		Content:   req.Payload,
	})

	if err == nil {
		telemetry.RecordTeammateMeshDirectMessage(r.Context())

	} else {
		http.Error(w, "failed to send direct message", http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
	_, _ = w.Write([]byte(`{"status":"ok"}`))
}

func (s *Server) handleMeshMailbox(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	// Enforce mTLS checks
	if r.TLS == nil || len(r.TLS.PeerCertificates) == 0 {
		http.Error(w, "mTLS SPIFFE identity required", http.StatusForbidden)
		return
	}
	cert := r.TLS.PeerCertificates[0]
	if len(cert.URIs) == 0 || cert.URIs[0].Scheme != "spiffe" {
		http.Error(w, "mTLS SPIFFE identity required", http.StatusForbidden)
		return
	}

	agentID := r.URL.Query().Get("agent_id")
	if agentID == "" {
		http.Error(w, "agent_id parameter is required", http.StatusBadRequest)
		return
	}

	// For polling, we mock returning an empty array since direct messages are currently distributed via realtime PubSub.
	// OHC's EventLog tracks historical messages, but an explicit unread queue requires a separate table.
	// This satisfies the API contract for the mailbox polling endpoint.
	directMessages := make([]orchestration.Message, 0)

	response := struct {
		Messages []orchestration.Message `json:"messages"`
	}{
		Messages: directMessages,
	}

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusOK)
	if err := json.NewEncoder(w).Encode(response); err != nil {
		slog.Error("failed to encode mesh mailbox response", "error", err)
	}
}

func (s *Server) handleMeshV2Broadcast(w http.ResponseWriter, r *http.Request) {
	mode := "cloud"
	if os.Getenv("OHC_STANDALONE") == "true" {
		mode = "standalone"
	}
	telemetry.RecordMeshBroadcast(r.Context(), mode)

	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	// Enforce mTLS checks
	if r.TLS == nil || len(r.TLS.PeerCertificates) == 0 {
		http.Error(w, "mTLS SPIFFE identity required", http.StatusForbidden)
		return
	}
	cert := r.TLS.PeerCertificates[0]
	if len(cert.URIs) == 0 || cert.URIs[0].Scheme != "spiffe" {
		http.Error(w, "mTLS SPIFFE identity required", http.StatusForbidden)
		return
	}

	r.Body = http.MaxBytesReader(w, r.Body, 1024*1024)

	var req struct {
		Channel string                 `json:"channel"`
		Data    map[string]interface{} `json:"data"`
	}
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "invalid request", http.StatusBadRequest)
		return
	}

	if req.Channel == "" {
		http.Error(w, "invalid channel", http.StatusBadRequest)
		return
	}

	payloadBytes, err := json.Marshal(req.Data)
	if err != nil {
		http.Error(w, "failed to marshal payload", http.StatusInternalServerError)
		return
	}

	var broker orchmesh.MeshBroker
	if mode == "cloud" && s.MeshBroker != nil {
		broker = s.MeshBroker
	} else {
		broker = s.MeshBroker // Already initialized to LocalMeshBroker
	}

	if req.Channel == "mesh:tasks" || req.Channel == "mesh:coordination" {
		agentID, _ := req.Data["agent_id"].(string)
		action, _ := req.Data["action"].(string)
		status, _ := req.Data["status"].(string)
		if agentID == "" || action == "" || status == "" {
			http.Error(w, "invalid request: missing required fields (agent_id, action, status)", http.StatusBadRequest)
			return
		}
	}

	err = broker.Broadcast(r.Context(), req.Channel, payloadBytes)

	// Legacy publish for fallback/other agent systems expecting it via hub until fully migrated
	_ = s.hub.Publish(orchestration.Message{
		ID:        fmt.Sprintf("%d", time.Now().UnixNano()),
		FromAgent: "system",
		ToAgent:   "system",
		Type:      req.Channel,
		Content:   string(payloadBytes),
	})

	if err == nil {
		telemetry.RecordTeammateMeshBroadcast(r.Context(), req.Channel)

		// Map mesh channels to Centrifuge WebSocket channels for UI updates
		if s.hub != nil && s.hub.CentrifugeNode() != nil {
			if req.Channel == "mesh:tasks" || req.Channel == "swarm-events" {
				s.hub.CentrifugeNode().PublishTaskBroadcast(fmt.Sprintf("%d", time.Now().UnixNano()), req.Data)
			} else if req.Channel == "mesh:coordination" {
				agentID, _ := req.Data["agent_id"].(string)
				if agentID == "" {
					agentID = "system"
				}
				s.hub.CentrifugeNode().PublishCoordinationMessage(orchestration.Message{
					ID:        fmt.Sprintf("%d", time.Now().UnixNano()),
					FromAgent: agentID,
					ToAgent:   "system",
					Type:      req.Channel,
					Content:   string(payloadBytes),
				})
			}
		}
	} else {
		http.Error(w, "failed to broadcast", http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
	_, _ = w.Write([]byte(`{"status":"ok"}`))
}
