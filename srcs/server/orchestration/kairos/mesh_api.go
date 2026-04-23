package kairos

import (
	"context"
	"encoding/json"
	"fmt"
	"net/http"

	"github.com/gorilla/websocket"
)

type MeshAPI struct {
	mesh TeammateMesh
	repo *SharedTaskRepo
}

func NewMeshAPI(mesh TeammateMesh, repo *SharedTaskRepo) *MeshAPI {
	return &MeshAPI{mesh: mesh, repo: repo}
}

func (api *MeshAPI) RegisterRoutes(mux *http.ServeMux) {
	mux.HandleFunc("/api/kairos/mesh/publish", api.HandlePublish)
	mux.HandleFunc("/api/kairos/mesh/subscribe", api.HandleSubscribe)
	mux.HandleFunc("/api/kairos/mesh/approvals", api.HandleApprovals)
}

type PublishRequest struct {
	Channel string          `json:"channel"`
	Message json.RawMessage `json:"message"`
}

func (api *MeshAPI) HandlePublish(w http.ResponseWriter, r *http.Request) {
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

	var req PublishRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	if req.Channel == "" {
		http.Error(w, "missing channel", http.StatusBadRequest)
		return
	}

	if err := api.mesh.Publish(r.Context(), req.Channel, req.Message); err != nil {
		http.Error(w, "failed to publish", http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
	w.Write([]byte(`{"status":"ok"}`))
}

func (api *MeshAPI) HandleSubscribe(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	channel := r.URL.Query().Get("channel")
	if channel == "" {
		http.Error(w, "missing channel", http.StatusBadRequest)
		return
	}

	// Use standard upgrader check (default allows same-origin)
	defaultUpgrader := websocket.Upgrader{}

	conn, err := defaultUpgrader.Upgrade(w, r, nil)
	if err != nil {
		return
	}
	defer conn.Close()

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	// Use our cancellable context
	sub, err := api.mesh.Subscribe(ctx, channel)
	if err != nil {
		conn.WriteMessage(websocket.TextMessage, []byte(fmt.Sprintf("Failed to subscribe: %v", err)))
		return
	}

	clientDone := make(chan struct{})

	// Read from websocket to handle disconnects (client closing connection)
	go func() {
		defer close(clientDone)
		for {
			if _, _, err := conn.ReadMessage(); err != nil {
				break
			}
		}
	}()

	for {
		select {
		case <-clientDone:
			// Connection was closed by client, or read error
			return
		case <-r.Context().Done():
			// Connection was dropped at HTTP level
			return
		case msg, ok := <-sub:
			if !ok {
				return
			}
			if err := conn.WriteMessage(websocket.TextMessage, msg); err != nil {
				return
			}
		}
	}
}

type ApprovalUpdateRequest struct {
	ID     string `json:"id"`
	Status string `json:"status"`
}

func (api *MeshAPI) HandleApprovals(w http.ResponseWriter, r *http.Request) {
	if api.repo == nil {
		http.Error(w, "repository not available", http.StatusInternalServerError)
		return
	}

	if r.Method == http.MethodGet {
		tasks, err := api.repo.ListPendingApprovals(r.Context())
		if err != nil {
			http.Error(w, "failed to fetch approvals", http.StatusInternalServerError)
			return
		}
		json.NewEncoder(w).Encode(tasks)
		return
	}

	if r.Method == http.MethodPut || r.Method == http.MethodPost {
		var req ApprovalUpdateRequest
		if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
			http.Error(w, "bad request", http.StatusBadRequest)
			return
		}

		if req.ID == "" || req.Status == "" {
			http.Error(w, "missing required fields", http.StatusBadRequest)
			return
		}

		ActionApprovalsTotal.WithLabelValues(req.Status, "high").Inc()
		if err := api.repo.UpdateApprovalStatus(r.Context(), req.ID, req.Status); err != nil {
			http.Error(w, "failed to update approval status", http.StatusInternalServerError)
			return
		}

		w.WriteHeader(http.StatusOK)
		w.Write([]byte(`{"status":"ok"}`))
		return
	}

	http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
}
