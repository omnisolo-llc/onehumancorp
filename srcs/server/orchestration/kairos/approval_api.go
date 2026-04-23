package kairos

import (
	"context"
	"encoding/json"
	"net/http"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)



type Mutex interface {
	Lock(ctx context.Context, ttl time.Duration) error
	Unlock(ctx context.Context) error
}

type MutexProvider interface {
	NewMutex(key string) Mutex
}

type ApprovalAPI struct {
	repo         *SharedTaskRepo
	mesh         TeammateMesh
	lockProvider MutexProvider
}

func NewApprovalAPI(provider db.Provider, mesh TeammateMesh, lockProvider MutexProvider) *ApprovalAPI {
	return &ApprovalAPI{
		repo:         NewSharedTaskRepo(provider),
		mesh:         mesh,
		lockProvider: lockProvider,
	}
}

func (api *ApprovalAPI) RegisterRoutes(mux *http.ServeMux) {
	mux.HandleFunc("/api/kairos/approvals", api.HandleGetApprovals)
	mux.HandleFunc("/api/kairos/approvals/decide", api.HandleDecideApproval)
}

func (api *ApprovalAPI) HandleGetApprovals(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	// mTLS check
	if r.TLS == nil || len(r.TLS.PeerCertificates) == 0 {
		http.Error(w, "mTLS SPIFFE identity required", http.StatusForbidden)
		return
	}
	cert := r.TLS.PeerCertificates[0]
	if len(cert.URIs) == 0 || cert.URIs[0].Scheme != "spiffe" {
		http.Error(w, "mTLS SPIFFE identity required", http.StatusForbidden)
		return
	}

	tasks, err := api.repo.GetPendingApprovals(r.Context())
	if err != nil {
		http.Error(w, "failed to fetch approvals", http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(tasks)
}

type DecideRequest struct {
	TaskID   string `json:"task_id"`
	Decision string `json:"decision"` // "APPROVED" or "REJECTED"
}

func (api *ApprovalAPI) HandleDecideApproval(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	// mTLS check
	if r.TLS == nil || len(r.TLS.PeerCertificates) == 0 {
		http.Error(w, "mTLS SPIFFE identity required", http.StatusForbidden)
		return
	}
	cert := r.TLS.PeerCertificates[0]
	if len(cert.URIs) == 0 || cert.URIs[0].Scheme != "spiffe" {
		http.Error(w, "mTLS SPIFFE identity required", http.StatusForbidden)
		return
	}

	var req DecideRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	if req.TaskID == "" || (req.Decision != "APPROVED" && req.Decision != "REJECTED") {
		http.Error(w, "invalid request", http.StatusBadRequest)
		return
	}

	// Use ohc:lock lock structure for thread-safety
	lock := api.lockProvider.NewMutex("ohc:lock:kairos_approval:" + req.TaskID)
	if err := lock.Lock(r.Context(), 10*time.Second); err != nil { // 10s ttl
		http.Error(w, "failed to acquire lock", http.StatusConflict)
		return
	}
	defer lock.Unlock(r.Context())

	// Fetch task to verify it is PENDING
	task, err := api.repo.Get(r.Context(), req.TaskID)
	if err != nil {
		http.Error(w, "task not found", http.StatusNotFound)
		return
	}

	if task.ApprovalStatus != "PENDING" {
		http.Error(w, "task is not pending approval", http.StatusConflict)
		return
	}

	if err := api.repo.UpdateApprovalStatus(r.Context(), req.TaskID, req.Decision); err != nil {
		http.Error(w, "failed to update approval status", http.StatusInternalServerError)
		return
	}

	// Update metrics
	mode := GetMode()
	ApprovalActionsTotal.WithLabelValues(mode, req.Decision, task.ActionRisk).Inc()

	// Broadcast over mesh:tasks
	msg := map[string]interface{}{
		"action":   "approval_decided",
		"task_id":  req.TaskID,
		"decision": req.Decision,
	}
	msgBytes, _ := json.Marshal(msg)
	_ = api.mesh.Publish(r.Context(), "mesh:tasks", msgBytes)

	w.WriteHeader(http.StatusOK)
	w.Write([]byte(`{"status":"ok"}`))
}

func (api *ApprovalAPI) CreatePendingApproval(ctx context.Context, task *SharedTask) error {
	if task.ApprovalStatus == "" {
		task.ApprovalStatus = "PENDING"
	}
	if err := api.repo.Insert(ctx, task); err != nil {
		return err
	}

	if task.ApprovalStatus == "PENDING" && task.ActionRisk == "High" {
		msg := map[string]interface{}{
			"action":  "pending_approval",
			"task_id": task.ID,
		}
		msgBytes, _ := json.Marshal(msg)
		_ = api.mesh.Publish(ctx, "mesh:tasks", msgBytes)
	}

	return nil
}
