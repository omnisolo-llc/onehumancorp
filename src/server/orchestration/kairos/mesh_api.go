package kairos

import (
	"context"
	"encoding/json"
	"fmt"
	"net/http"

	"github.com/onehumancorp/mono/src/server/auth"
	"github.com/gorilla/websocket"
)

type ApprovalWorkflowEngine interface {
	GetPendingApprovalTasks(ctx context.Context, orgID string) (interface{}, error)
	ApproveTask(ctx context.Context, taskID string, agentID string) error
	RejectTask(ctx context.Context, taskID string, agentID string) error
}

type MeshAPI struct {
	mesh TeammateMesh
	workflowEngine ApprovalWorkflowEngine
}

func NewMeshAPI(mesh TeammateMesh, workflowEngine ApprovalWorkflowEngine) *MeshAPI {
	return &MeshAPI{mesh: mesh, workflowEngine: workflowEngine}
}

func (api *MeshAPI) RegisterRoutes(mux *http.ServeMux) {

	mux.HandleFunc("/api/kairos/actions/pending", api.HandleGetPendingActions)
	mux.HandleFunc("/api/kairos/actions/approve", api.HandleApproveAction)
	mux.HandleFunc("/api/kairos/actions/reject", api.HandleRejectAction)

	mux.HandleFunc("/api/kairos/mesh/publish", api.HandlePublish)
	mux.HandleFunc("/api/kairos/mesh/subscribe", api.HandleSubscribe)
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

type approvalRequest struct {
	TaskID string `json:"task_id"`
}

func (api *MeshAPI) HandleGetPendingActions(w http.ResponseWriter, r *http.Request) {
	claims := auth.ClaimsFromContext(r.Context())
	if claims == nil || claims.OrganizationID == "" {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	tasks, err := api.workflowEngine.GetPendingApprovalTasks(r.Context(), claims.OrganizationID)
	if err != nil {
		http.Error(w, "failed to fetch pending actions", http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(tasks)
}

func (api *MeshAPI) HandleApproveAction(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	claims := auth.ClaimsFromContext(r.Context())
	if claims == nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	var req approvalRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "invalid JSON payload", http.StatusBadRequest)
		return
	}

	if err := api.workflowEngine.ApproveTask(r.Context(), req.TaskID, claims.Subject); err != nil {
		http.Error(w, "failed to approve action", http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
	w.Write([]byte(`{"status":"approved"}`))
}

func (api *MeshAPI) HandleRejectAction(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	claims := auth.ClaimsFromContext(r.Context())
	if claims == nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	var req approvalRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "invalid JSON payload", http.StatusBadRequest)
		return
	}

	if err := api.workflowEngine.RejectTask(r.Context(), req.TaskID, claims.Subject); err != nil {
		http.Error(w, "failed to reject action", http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
	w.Write([]byte(`{"status":"rejected"}`))
}
