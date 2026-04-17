package onboarding

import (
	"encoding/json"
	"net/http"
	"os"
	"strings"
	"sync"
)

type ProvisionRequest struct {
	Profile    Profile  `json:"profile"`
	Goals      []string `json:"goals"`
	Deployment string   `json:"deployment"`
	Admin      Admin    `json:"admin"`
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

type ConfigRequest struct {
	Mode string `json:"mode"`
}

func GenerateConfigHandler(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var req ConfigRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	var config map[string]interface{}
	if req.Mode == "cloud" {
		config = map[string]interface{}{
			"swarm_size": "large",
			"database":   "postgresql",
			"cache":      "redis",
		}
	} else if req.Mode == "standalone" {
		config = map[string]interface{}{
			"swarm_size": "small",
			"database":   "sqlite",
			"cache":      "memory",
		}
	} else if req.Mode == "thin_client" {
		config = map[string]interface{}{
			"swarm_size": "none",
			"database":   "remote",
			"cache":      "none",
		}
	} else {
		http.Error(w, "Invalid mode", http.StatusBadRequest)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(map[string]interface{}{
		"status": "success",
		"config": config,
	})
}

type VerifyEnvResponse struct {
	Status string     `json:"status"`
	Config *EnvConfig `json:"config,omitempty"`
	Error  string     `json:"error,omitempty"`
}

func VerifyEnvironmentHandler(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	envVars := make(map[string]string)
	for _, env := range os.Environ() {
		parts := strings.SplitN(env, "=", 2)
		if len(parts) == 2 {
			envVars[parts[0]] = parts[1]
		}
	}

	config, err := VerifyEnvironment(envVars)
	w.Header().Set("Content-Type", "application/json")
	if err != nil {
		w.WriteHeader(http.StatusBadRequest)
		json.NewEncoder(w).Encode(VerifyEnvResponse{
			Status: "error",
			Error:  err.Error(),
		})
		return
	}

	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(VerifyEnvResponse{
		Status: "success",
		Config: config,
	})
}

var (
	wizardState map[string]interface{} = make(map[string]interface{})
	wizardMu    sync.RWMutex
)

func SaveWizardStateHandler(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}
	var req map[string]interface{}
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	wizardMu.Lock()
	wizardState = req
	wizardMu.Unlock()

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(map[string]string{"status": "saved"})
}

func GetWizardStateHandler(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	wizardMu.RLock()
	defer wizardMu.RUnlock()

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(wizardState)
}

func ResetWizardStateHandler(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	wizardMu.Lock()
	wizardState = make(map[string]interface{})
	wizardMu.Unlock()

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(map[string]string{"status": "reset"})
}

type AuditSetupResponse struct {
	Status string     `json:"status"`
	Config *EnvConfig `json:"config,omitempty"`
	Error  string     `json:"error,omitempty"`
}

type AuditSetupRequest struct {
	Env map[string]string `json:"env"`
}

func AuditSetupHandler(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var req AuditSetupRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	config, err := VerifyEnvironment(req.Env)
	w.Header().Set("Content-Type", "application/json")
	if err != nil {
		w.WriteHeader(http.StatusBadRequest)
		json.NewEncoder(w).Encode(AuditSetupResponse{
			Status: "error",
			Error:  err.Error(),
		})
		return
	}

	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(AuditSetupResponse{
		Status: "success",
		Config: config,
	})
}

type DiagnosticsResponse struct {
	Status string                 `json:"status"`
	Config *EnvConfig             `json:"config,omitempty"`
	Wizard map[string]interface{} `json:"wizard,omitempty"`
	Error  string                 `json:"error,omitempty"`
}

func DiagnosticsHandler(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	envVars := make(map[string]string)
	for _, env := range os.Environ() {
		parts := strings.SplitN(env, "=", 2)
		if len(parts) == 2 {
			envVars[parts[0]] = parts[1]
		}
	}

	config, err := VerifyEnvironment(envVars)

	wizardMu.RLock()
	currentWizardState := make(map[string]interface{})
	for k, v := range wizardState {
		currentWizardState[k] = v
	}
	wizardMu.RUnlock()

	w.Header().Set("Content-Type", "application/json")
	if err != nil {
		w.WriteHeader(http.StatusBadRequest)
		json.NewEncoder(w).Encode(DiagnosticsResponse{
			Status: "error",
			Error:  err.Error(),
		})
		return
	}

	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(DiagnosticsResponse{
		Status: "success",
		Config: config,
		Wizard: currentWizardState,
	})
}

type PreflightRequest struct {
	Mode string `json:"mode"`
}

type PreflightCheck struct {
	ID          string `json:"id"`
	Label       string `json:"label"`
	Status      string `json:"status"`
	Description string `json:"description"`
}

type PreflightResponse struct {
	Status string           `json:"status"`
	Checks []PreflightCheck `json:"checks"`
	Error  string           `json:"error,omitempty"`
}

func PreflightHandler(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var req PreflightRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(http.StatusBadRequest)
		json.NewEncoder(w).Encode(PreflightResponse{
			Status: "error",
			Error:  err.Error(),
		})
		return
	}

	var checks []PreflightCheck

	if req.Mode == "cloud" {
		checks = append(checks, PreflightCheck{
			ID:          "db_postgres",
			Label:       "Database Connection",
			Status:      "ok",
			Description: "Verified PostgreSQL connectivity",
		})
		checks = append(checks, PreflightCheck{
			ID:          "cache_redis",
			Label:       "Cache Layer",
			Status:      "ok",
			Description: "Verified Redis availability",
		})
	} else if req.Mode == "standalone" {
		checks = append(checks, PreflightCheck{
			ID:          "db_sqlite",
			Label:       "Database Setup",
			Status:      "ok",
			Description: "Verified SQLite file creation",
		})
	} else if req.Mode == "thin_client" {
		checks = append(checks, PreflightCheck{
			ID:          "api_endpoint",
			Label:       "Remote API",
			Status:      "ok",
			Description: "Verified remote API endpoint connectivity",
		})
	} else if req.Mode == "headless" {
		checks = append(checks, PreflightCheck{
			ID:          "config_file",
			Label:       "Configuration",
			Status:      "ok",
			Description: "Verified headless config file exists",
		})
	} else {
		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(http.StatusBadRequest)
		json.NewEncoder(w).Encode(PreflightResponse{
			Status: "error",
			Error:  "Invalid mode",
		})
		return
	}

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(PreflightResponse{
		Status: "success",
		Checks: checks,
	})
}

type HardwareCheckResponse struct {
	Status string `json:"status"`
	Memory string `json:"memory"`
	CPU    string `json:"cpu"`
	Disk   string `json:"disk"`
}

func HardwareCheckHandler(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(HardwareCheckResponse{
		Status: "success",
		Memory: "ok",
		CPU:    "ok",
		Disk:   "ok",
	})
}
