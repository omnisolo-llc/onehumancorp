package growth

import (
	"encoding/json"
	"net/http"

	"github.com/onehumancorp/mono/src/server/lib/analytics"
)

type ExperimentsAPI struct {
	manager *ExperimentManager
	tracker *analytics.Tracker
}

func NewExperimentsAPI(manager *ExperimentManager, tracker *analytics.Tracker) *ExperimentsAPI {
	return &ExperimentsAPI{
		manager: manager,
		tracker: tracker,
	}
}

func (api *ExperimentsAPI) GetVariantHandler(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	experimentID := r.URL.Query().Get("experiment_id")
	userID := r.URL.Query().Get("user_id")

	if experimentID == "" || userID == "" {
		http.Error(w, "Missing experiment_id or user_id", http.StatusBadRequest)
		return
	}

	variant := api.manager.GetVariant(experimentID, userID)

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]string{
		"variant": variant,
	})
}

type ExperimentEventRequest struct {
	ExperimentID string `json:"experiment_id"`
	Variant      string `json:"variant"`
}

func (api *ExperimentsAPI) RecordImpressionHandler(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var req ExperimentEventRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "Invalid request body", http.StatusBadRequest)
		return
	}

	if req.ExperimentID == "" || req.Variant == "" {
		http.Error(w, "Missing experiment_id or variant", http.StatusBadRequest)
		return
	}

	api.tracker.TrackEvent(r.Context(), "ab_test_impression", map[string]interface{}{
		"experiment_id": req.ExperimentID,
		"variant":       req.Variant,
	})

	w.WriteHeader(http.StatusOK)
}

func (api *ExperimentsAPI) RecordConversionHandler(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var req ExperimentEventRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "Invalid request body", http.StatusBadRequest)
		return
	}

	if req.ExperimentID == "" || req.Variant == "" {
		http.Error(w, "Missing experiment_id or variant", http.StatusBadRequest)
		return
	}

	api.tracker.TrackEvent(r.Context(), "ab_test_conversion", map[string]interface{}{
		"experiment_id": req.ExperimentID,
		"variant":       req.Variant,
	})

	w.WriteHeader(http.StatusOK)
}
