package dashboard

import (
	"encoding/json"
	"net/http"
	"os"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type AutoDreamSyncRequest struct {
	ForceReindex bool `json:"force_reindex"`
}

type AutoDreamQueryRequest struct {
	QueryText string `json:"query_text"`
	Limit     int    `json:"limit"`
}

type AutoDreamQueryResult struct {
	Results []orchestration.TruthSearchResult `json:"results"`
}

func (s *Server) handleAutoDreamSync(w http.ResponseWriter, r *http.Request) {

	start := time.Now()
	defer func() {
		mode := "cloud"
		if os.Getenv("OHC_STANDALONE") == "true" {
			mode = "standalone"
		}
		telemetry.RecordAutoDreamSyncDuration(r.Context(), time.Since(start).Seconds(), mode)
	}()
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	claims := auth.ClaimsFromContext(r.Context())
	if claims == nil || claims.OrganizationID == "" {
		http.Error(w, "unauthorized: missing claims", http.StatusUnauthorized)
		return
	}

	var req AutoDreamSyncRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		req = AutoDreamSyncRequest{}
	}

	if s.hub == nil || s.hub.SIPDB() == nil {
		http.Error(w, "AutoDream Sync requires a configured Hub and SIPDB", http.StatusServiceUnavailable)
		return
	}

	worker := orchestration.NewAutoDreamWorker(s.hub.SIPDB().Provider())
	err := worker.ConsolidateEpoch(r.Context())
	if err != nil {
		http.Error(w, "failed to synchronize AutoDream: "+err.Error(), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]string{"status": "success"})
}

func (s *Server) handleAutoDreamQuery(w http.ResponseWriter, r *http.Request) {

	start := time.Now()
	defer func() {
		mode := "cloud"
		if os.Getenv("OHC_STANDALONE") == "true" {
			mode = "standalone"
		}
		telemetry.RecordAutoDreamQueryDuration(r.Context(), time.Since(start).Seconds(), mode)
	}()
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	claims := auth.ClaimsFromContext(r.Context())
	if claims == nil || claims.OrganizationID == "" {
		http.Error(w, "unauthorized: missing claims", http.StatusUnauthorized)
		return
	}

	var req AutoDreamQueryRequest
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

	minimaxKey := os.Getenv("MINIMAX_API_KEY")
	var embedding string
	if minimaxKey != "" {
		client := orchestration.NewMinimaxClient(minimaxKey)
		emb, err := client.GenerateEmbedding(r.Context(), req.QueryText)
		if err == nil {
			bytesEmb, _ := json.Marshal(emb)
			embedding = string(bytesEmb)
		} else {
			embedding = "[0.0, 0.0, 0.0]"
		}
	} else {
		embedding = "[0.0, 0.0, 0.0]"
	}

	if s.hub == nil || s.hub.SIPDB() == nil {
		http.Error(w, "AutoDream Query requires a configured Hub and SIPDB", http.StatusServiceUnavailable)
		return
	}

	worker := orchestration.NewAutoDreamWorker(s.hub.SIPDB().Provider())
	results, err := worker.SearchTruth(r.Context(), embedding, req.Limit)
	if err != nil {
		http.Error(w, "failed to search AutoDream memories: "+err.Error(), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(AutoDreamQueryResult{Results: results})
}
