package dashboard

import (
	"encoding/json"
	"net/http"
)

type OnboardingMetric struct {
	Platform       string  `json:"platform"`
	CompletionRate float64 `json:"completion_rate"`
}

type ViralCoefficientResponse struct {
	KFactor float64 `json:"k_factor"`
}

func HandleOnboardingMetrics(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	metrics := []OnboardingMetric{
		{Platform: "Desktop", CompletionRate: 0.8},
		{Platform: "Cloud", CompletionRate: 0.6},
		{Platform: "Mobile-only", CompletionRate: 0.75},
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(metrics)
}

func HandleViralCoefficient(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	response := ViralCoefficientResponse{
		KFactor: 1.2,
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(response)
}
