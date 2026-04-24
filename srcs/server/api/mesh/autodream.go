package mesh

import (
	"encoding/json"
	"net/http"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type AutoDreamHandler struct {
	repo *db.AutoDreamRepository
}

func NewAutoDreamHandler(repo *db.AutoDreamRepository) *AutoDreamHandler {
	return &AutoDreamHandler{repo: repo}
}

func (h *AutoDreamHandler) Search(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var req struct {
		Embedding []float32 `json:"embedding"`
		Limit     int       `json:"limit"`
	}

	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "Bad request", http.StatusBadRequest)
		return
	}

	if req.Limit <= 0 {
		req.Limit = 10
	}

	findings, err := h.repo.Search(r.Context(), req.Embedding, req.Limit)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(findings)
}

func (h *AutoDreamHandler) Store(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var finding db.Finding
	if err := json.NewDecoder(r.Body).Decode(&finding); err != nil {
		http.Error(w, "Bad request", http.StatusBadRequest)
		return
	}

	if err := h.repo.Upsert(r.Context(), &finding); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusCreated)
}
