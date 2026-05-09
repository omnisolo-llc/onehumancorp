package dashboard

import (
	"encoding/json"
	"net/http"
)

type OnboardingMetric struct {
	Platform       string  `json:"platform"`
	CompletionRate float64 `json:"completion_rate"`
}

// Pre-allocate the metrics slice and JSON payload to avoid allocations on every request
var (
	cachedMetrics = []OnboardingMetric{
		{Platform: "Desktop", CompletionRate: 0.8},
		{Platform: "Cloud", CompletionRate: 0.6},
		{Platform: "Mobile-only", CompletionRate: 0.75},
	}
	cachedMetricsJSON []byte
)

func init() {
	cachedMetricsJSON, _ = json.Marshal(cachedMetrics)
}

func HandleOnboardingMetrics(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	w.Write(cachedMetricsJSON)
}
