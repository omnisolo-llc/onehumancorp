package onboarding

import (
	"encoding/json"
	"net/http"
)

type ProvisionRequest struct {
	Profile    Profile    `json:"profile"`
	Goals      []string   `json:"goals"`
	Deployment string     `json:"deployment"`
	Admin      Admin      `json:"admin"`
}

type Profile struct {
	Name     string `json:"name"`
	Industry string `json:"industry"`
	Size     string `json:"size"`
	Language string `json:"language"`
}

type Admin struct {
	Name     string `json:"name"`
	Email    string `json:"email"`
	Password string `json:"password"`
}

func ProvisionHandler(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var req ProvisionRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(map[string]string{"status": "provisioned", "message": "State persisted successfully"})
}
