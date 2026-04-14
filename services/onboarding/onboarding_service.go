package onboarding

import (
	"encoding/json"
	"net/http"
)

type Status struct {
	SetupComplete bool `json:"setup_complete"`
}

func CheckStatusHandler(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "application/json")
	status := Status{SetupComplete: true}
	json.NewEncoder(w).Encode(status)
}
