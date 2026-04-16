package api

import (
	"encoding/json"
	"net/http"

	"github.com/onehumancorp/mono/srcs/server/services/growth"
)

type ABVariantRequest struct {
	Experiment string   `json:"experiment"`
	Variants   []string `json:"variants"`
	Weights    []int    `json:"weights"`
}

type ABVariantResponse struct {
	Variant string `json:"variant"`
}

func HandleABVariant(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var req ABVariantRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "Invalid request body", http.StatusBadRequest)
		return
	}

	variant := growth.AssignVariant(r.Context(), req.Experiment, req.Variants, req.Weights)

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(ABVariantResponse{Variant: variant})
}

type ABConversionRequest struct {
	Experiment string `json:"experiment"`
	Variant    string `json:"variant"`
}

func HandleABConversion(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var req ABConversionRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "Invalid request body", http.StatusBadRequest)
		return
	}

	growth.TrackConversion(r.Context(), req.Experiment, req.Variant)

	w.WriteHeader(http.StatusOK)
}
