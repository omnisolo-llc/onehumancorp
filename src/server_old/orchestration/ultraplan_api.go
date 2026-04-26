package orchestration

import (
	"encoding/json"
	"net/http"
)

type UltraPlanAPI struct {
	manager *UltraPlanManager
}

func NewUltraPlanAPI(manager *UltraPlanManager) *UltraPlanAPI {
	return &UltraPlanAPI{
		manager: manager,
	}
}

func (api *UltraPlanAPI) RegisterRoutes(mux *http.ServeMux) {
	mux.HandleFunc("/api/ultraplan/create", api.HandleCreateUltraPlan)
	mux.HandleFunc("/api/ultraplan/critique", api.HandleSubmitCritique)
	mux.HandleFunc("/api/ultraplan/vote", api.HandleCastVote)
	mux.HandleFunc("/api/ultraplan/finalize", api.HandleFinalizeUltraPlan)
}

func (api *UltraPlanAPI) HandleCreateUltraPlan(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}
	var req struct {
		MissionID    string                 `json:"mission_id"`
		StateMachine map[string]interface{} `json:"state_machine"`
	}
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "Invalid request", http.StatusBadRequest)
		return
	}

	plan, err := api.manager.CreatePlan(r.Context(), req.MissionID, req.StateMachine)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	tx, err := api.manager.db.Begin(r.Context())
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer tx.Rollback(r.Context())

	if _, err := tx.Exec(r.Context(), `INSERT INTO ultraplan_proposals (id, plan_id, status) VALUES ($1, $2, 'PROPOSE')`, plan.ID, plan.ID); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	if err := tx.Commit(r.Context()); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(plan)
}

func (api *UltraPlanAPI) HandleSubmitCritique(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}
	var req struct {
		PlanID   string `json:"plan_id"`
		AgentID  string `json:"agent_id"`
		Critique string `json:"critique"`
	}
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "Invalid request", http.StatusBadRequest)
		return
	}

	if err := api.manager.SubmitCritique(r.Context(), req.PlanID, req.AgentID, req.Critique); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	tx, err := api.manager.db.Begin(r.Context())
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer tx.Rollback(r.Context())

	if _, err := tx.Exec(r.Context(), `UPDATE ultraplan_proposals SET status = 'CRITIQUE' WHERE plan_id = $1`, req.PlanID); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	if err := tx.Commit(r.Context()); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
	w.Write([]byte(`{"status":"success"}`))
}

func (api *UltraPlanAPI) HandleCastVote(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}
	var req struct {
		PlanID  string `json:"plan_id"`
		AgentID string `json:"agent_id"`
		Vote    string `json:"vote"` // APPROVE, REJECT
	}
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "Invalid request", http.StatusBadRequest)
		return
	}

	tx, err := api.manager.db.Begin(r.Context())
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer tx.Rollback(r.Context())

	_, err = tx.Exec(r.Context(), `INSERT INTO ultraplan_votes (plan_id, agent_id, vote) VALUES ($1, $2, $3)`, req.PlanID, req.AgentID, req.Vote)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	if err := tx.Commit(r.Context()); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
	w.Write([]byte(`{"status":"success"}`))
}

func (api *UltraPlanAPI) HandleFinalizeUltraPlan(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}
	var req struct {
		PlanID string `json:"plan_id"`
	}
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "Invalid request", http.StatusBadRequest)
		return
	}

	tx, err := api.manager.db.Begin(r.Context())
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer tx.Rollback(r.Context())

	var approveCount int
	if err := tx.QueryRow(r.Context(), `SELECT count(*) FROM ultraplan_votes WHERE plan_id = $1 AND vote = 'APPROVE'`, req.PlanID).Scan(&approveCount); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	status := "REVISE"
	if approveCount > 0 {
		status = "APPROVED"
	}

	if _, err := tx.Exec(r.Context(), `UPDATE ultraplan_proposals SET status = $1 WHERE plan_id = $2`, status, req.PlanID); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	if status == "APPROVED" {
		if _, err := tx.Exec(r.Context(), `UPDATE ultraplan_proposals SET status = 'EXECUTE' WHERE plan_id = $1`, req.PlanID); err != nil {
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}
	}

	if err := tx.Commit(r.Context()); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
	w.Write([]byte(`{"status":"` + status + `"}`))
}
