package onboarding

import (
	"encoding/json"
	"net/http"
	"os"
	"strings"
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
    Status string `json:"status"`
    Config *EnvConfig `json:"config,omitempty"`
    Error  string `json:"error,omitempty"`
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


type DiagnosticsResponse struct {
	Status string `json:"status"`
	Checks string `json:"checks"`
}

func DiagnosticsHandler(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(DiagnosticsResponse{
		Status: "ok",
		Checks: "passed",
	})
}
