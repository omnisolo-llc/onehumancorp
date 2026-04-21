package dashboard

import (
	"encoding/json"
	"net/http"

	"github.com/onehumancorp/mono/srcs/server/autodream"
	"github.com/onehumancorp/mono/srcs/server/db"
)

type AutoDreamSearchRequest struct {
	QueryText string `json:"query_text"`
	Limit     int    `json:"limit"`
}

type AutoDreamSearchResult struct {
	Results []*autodream.KnowledgeRecord `json:"results"`
}

func HandleAutoDreamKnowledgeSearch(dbProvider db.Provider, client autodream.EmbeddingClient) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
			return
		}

		var req AutoDreamSearchRequest
		if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
			http.Error(w, "invalid request body", http.StatusBadRequest)
			return
		}

		if req.QueryText == "" {
			http.Error(w, "query_text is required", http.StatusBadRequest)
			return
		}

		if req.Limit <= 0 {
			req.Limit = 5
		}

		vector, err := client.GenerateEmbedding(r.Context(), req.QueryText)
		if err != nil {
			http.Error(w, "failed to generate embedding: "+err.Error(), http.StatusInternalServerError)
			return
		}

		var store autodream.VectorStore
		if dbProvider.IsSQLite() {
			store = autodream.NewSQLiteVectorStore(dbProvider)
		} else {
			store = autodream.NewPGVectorStore(dbProvider)
		}

		results, err := store.Search(r.Context(), vector, req.Limit)
		if err != nil {
			http.Error(w, "failed to search knowledge embeddings: "+err.Error(), http.StatusInternalServerError)
			return
		}

		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(AutoDreamSearchResult{Results: results})
	}
}
