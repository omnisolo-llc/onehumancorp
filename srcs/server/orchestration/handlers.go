package orchestration

import (
	"encoding/json"
	"net/http"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/google/uuid"
)

type SharedTaskHandler struct {
	db db.Provider
}

func NewSharedTaskHandler(provider db.Provider) *SharedTaskHandler {
	return &SharedTaskHandler{db: provider}
}

func (h *SharedTaskHandler) CreateTask(w http.ResponseWriter, r *http.Request) {
	claims := auth.ClaimsFromContext(r.Context())
	if claims == nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	var req struct {
		Title       string `json:"title"`
		Description string `json:"description"`
	}
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	query := "INSERT INTO shared_tasks (id, organization_id, title, description) VALUES ($1, $2, $3, $4) "
	if h.db.IsSQLite() {
		query = "INSERT INTO shared_tasks (id, organization_id, title, description) VALUES (?, ?, ?, ?) "
	}

	var id string
	id = uuid.New().String()
	_, err := h.db.Exec(r.Context(), query, id, claims.OrganizationID, req.Title, req.Description)
	if err != nil {
		http.Error(w, "internal error", http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusCreated)
	json.NewEncoder(w).Encode(map[string]string{"id": id})
}

func (h *SharedTaskHandler) UpdateTask(w http.ResponseWriter, r *http.Request) {
	claims := auth.ClaimsFromContext(r.Context())
	if claims == nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	parts := strings.Split(strings.Trim(r.URL.Path, "/"), "/")
	if len(parts) < 2 {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}
	id := parts[1]

	var req struct {
		Status string `json:"status"`
	}
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	query := "UPDATE shared_tasks SET status = $1 WHERE id = $2 AND organization_id = $3"
	if h.db.IsSQLite() {
		query = "UPDATE shared_tasks SET status = ? WHERE id = ? AND organization_id = ?"
	}

	_, err := h.db.Exec(r.Context(), query, req.Status, id, claims.OrganizationID)
	if err != nil {
		http.Error(w, "internal error", http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
}

func (h *SharedTaskHandler) ListTasks(w http.ResponseWriter, r *http.Request) {
	claims := auth.ClaimsFromContext(r.Context())
	if claims == nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	query := "SELECT id, title, status FROM shared_tasks WHERE organization_id = $1"
	if h.db.IsSQLite() {
		query = "SELECT id, title, status FROM shared_tasks WHERE organization_id = ?"
	}

	rows, err := h.db.Query(r.Context(), query, claims.OrganizationID)
	if err != nil {
		http.Error(w, "internal error", http.StatusInternalServerError)
		return
	}
	defer rows.Close()

	var tasks []map[string]interface{}
	for rows.Next() {
		var id, title, status string
		if err := rows.Scan(&id, &title, &status); err == nil {
			tasks = append(tasks, map[string]interface{}{"id": id, "title": title, "status": status})
		}
	}

	json.NewEncoder(w).Encode(tasks)
}

func (h *SharedTaskHandler) LockTask(w http.ResponseWriter, r *http.Request) {
	claims := auth.ClaimsFromContext(r.Context())
	if claims == nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	parts := strings.Split(strings.Trim(r.URL.Path, "/"), "/")
	if len(parts) < 2 {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}
	id := parts[1]

	query := "UPDATE shared_tasks SET status = 'IN_PROGRESS' WHERE id = $1 AND organization_id = $2 AND status = 'PENDING'"
	if h.db.IsSQLite() {
		query = "UPDATE shared_tasks SET status = 'IN_PROGRESS' WHERE id = ? AND organization_id = ? AND status = 'PENDING'"
	}

	_, err := h.db.Exec(r.Context(), query, id, claims.OrganizationID)
	if err != nil {
		http.Error(w, "internal error", http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
}
