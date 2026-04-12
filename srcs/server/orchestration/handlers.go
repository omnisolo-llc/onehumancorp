package orchestration

import (
    "encoding/json"
    "fmt"
    "net/http"
    "database/sql"

    "github.com/onehumancorp/mono/srcs/server/auth"
    "github.com/onehumancorp/mono/srcs/server/db"
)

// Handlers provides HTTP handlers for the shared tasks API.
type Handlers struct {
    db db.Provider
}

// NewHandlers creates a new Handlers instance.
func NewHandlers(db db.Provider) *Handlers {
    return &Handlers{db: db}
}

// CreateSharedTask creates a new task.
func (h *Handlers) CreateSharedTask(w http.ResponseWriter, r *http.Request) {
    claims := auth.ClaimsFromContext(r.Context())
    if claims == nil || claims.OrganizationID == "" {
        http.Error(w, "Unauthorized", http.StatusUnauthorized)
        return
    }

    var task SharedTask
    if err := json.NewDecoder(r.Body).Decode(&task); err != nil {
        http.Error(w, "Invalid request", http.StatusBadRequest)
        return
    }

    task.OrganizationID = claims.OrganizationID
    task.Status = "PENDING"
    if task.Priority == "" {
        task.Priority = "P2"
    }

    if task.Dependencies == nil {
        task.Dependencies = []string{}
    }

    depsJSON, _ := json.Marshal(task.Dependencies)

    query := `
        INSERT INTO shared_tasks (organization_id, title, description, status, agent_id, priority, payload, parent_plan_id, dependencies)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING id, created_at, updated_at
    `

    err := h.db.QueryRow(r.Context(), query,
        task.OrganizationID, task.Title, task.Description, task.Status,
        task.AssignedAgentID, task.Priority, task.Payload, task.ParentPlanID, depsJSON).
        Scan(&task.ID, &task.CreatedAt, &task.UpdatedAt)

    if err != nil {
        http.Error(w, fmt.Sprintf("Failed to create task: %v", err), http.StatusInternalServerError)
        return
    }

    w.Header().Set("Content-Type", "application/json")
    json.NewEncoder(w).Encode(task)
}

// ListSharedTasks lists all tasks for the tenant.
func (h *Handlers) ListSharedTasks(w http.ResponseWriter, r *http.Request) {
    claims := auth.ClaimsFromContext(r.Context())
    if claims == nil || claims.OrganizationID == "" {
        http.Error(w, "Unauthorized", http.StatusUnauthorized)
        return
    }

    query := `SELECT id, organization_id, title, description, status, agent_id, priority, payload, parent_plan_id, dependencies, created_at, updated_at FROM shared_tasks WHERE organization_id = $1`

    rows, err := h.db.Query(r.Context(), query, claims.OrganizationID)
    if err != nil {
        http.Error(w, fmt.Sprintf("Failed to list tasks: %v", err), http.StatusInternalServerError)
        return
    }
    defer rows.Close()

    var tasks []SharedTask
    for rows.Next() {
        var task SharedTask

        var agentID sql.NullString

        var created, updated sql.NullTime
        var depsJSON []byte
        if err := rows.Scan(&task.ID, &task.OrganizationID, &task.Title, &task.Description, &task.Status, &agentID, &task.Priority, &task.Payload, &task.ParentPlanID, &depsJSON, &created, &updated); err != nil {

            continue
        }
        if agentID.Valid {
            task.AssignedAgentID = agentID.String
        }
        if created.Valid {
            task.CreatedAt = created.Time
        }
        if updated.Valid {
            task.UpdatedAt = updated.Time
        }

        tasks = append(tasks, task)
    }

    w.Header().Set("Content-Type", "application/json")
    json.NewEncoder(w).Encode(tasks)
}

// UpdateSharedTask updates an existing task.
func (h *Handlers) UpdateSharedTask(w http.ResponseWriter, r *http.Request) {
    claims := auth.ClaimsFromContext(r.Context())
    if claims == nil || claims.OrganizationID == "" {
        http.Error(w, "Unauthorized", http.StatusUnauthorized)
        return
    }

    taskID := r.URL.Query().Get("id")
    if taskID == "" {
        http.Error(w, "Task ID required", http.StatusBadRequest)
        return
    }

    var update struct {
        Status  string `json:"status"`
        AgentID string `json:"agent_id"`
    }
    if err := json.NewDecoder(r.Body).Decode(&update); err != nil {
        http.Error(w, "Invalid request body", http.StatusBadRequest)
        return
    }

    query := `UPDATE shared_tasks SET status = $1, agent_id = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3 AND organization_id = $4`
    _, err := h.db.Exec(r.Context(), query, update.Status, update.AgentID, taskID, claims.OrganizationID)
    if err != nil {
        http.Error(w, fmt.Sprintf("Failed to update task: %v", err), http.StatusInternalServerError)
        return
    }

    w.WriteHeader(http.StatusOK)
}

// LockSharedTask locks a task for an agent.
func (h *Handlers) LockSharedTask(w http.ResponseWriter, r *http.Request) {
    claims := auth.ClaimsFromContext(r.Context())
    if claims == nil || claims.OrganizationID == "" {
        http.Error(w, "Unauthorized", http.StatusUnauthorized)
        return
    }

    taskID := r.URL.Query().Get("id")
    agentID := r.URL.Query().Get("agent_id")
    if taskID == "" || agentID == "" {
        http.Error(w, "Task ID and agent ID required", http.StatusBadRequest)
        return
    }

    query := `UPDATE shared_tasks SET status = 'IN_PROGRESS', agent_id = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2 AND organization_id = $3 AND status = 'PENDING'`

    rowsAffected, err := h.db.Exec(r.Context(), query, agentID, taskID, claims.OrganizationID)
    if err != nil {
        http.Error(w, fmt.Sprintf("Failed to lock task: %v", err), http.StatusInternalServerError)
        return
    }

    if rowsAffected == 0 {
        http.Error(w, "Task not found or already locked", http.StatusConflict)
        return
    }

    w.WriteHeader(http.StatusOK)
}
