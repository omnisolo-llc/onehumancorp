package api

import (
    "encoding/json"
    "net/http"

    "github.com/onehumancorp/mono/srcs/server/db"
    "github.com/pgvector/pgvector-go"
)

type AutoDreamAPI struct {
    store *db.AutoDreamStore
}

func NewAutoDreamAPI(store *db.AutoDreamStore) *AutoDreamAPI {
    return &AutoDreamAPI{store: store}
}

type QueryRequest struct {
    Limit     int       `json:"limit"`
    Embedding []float32 `json:"embedding"`
}

func (a *AutoDreamAPI) HandleQuery(w http.ResponseWriter, r *http.Request) {
    if r.Method != http.MethodPost {
        http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
        return
    }

    var req QueryRequest
    if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
        http.Error(w, "Invalid request body", http.StatusBadRequest)
        return
    }

    limit := 10
    if req.Limit > 0 {
        limit = req.Limit
    }

    if len(req.Embedding) == 0 {
        req.Embedding = make([]float32, 1536)
    }

    embedding := pgvector.NewVector(req.Embedding)

    findings, err := a.store.QuerySimilarFindings(r.Context(), embedding, limit)
    if err != nil {
        http.Error(w, err.Error(), http.StatusInternalServerError)
        return
    }

    w.Header().Set("Content-Type", "application/json")
    json.NewEncoder(w).Encode(findings)
}
