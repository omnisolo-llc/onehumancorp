package onboarding

import (
	"os"
	"os/exec"
)

// DependencyStatus represents the readiness of a single onboarding dependency.
type DependencyStatus struct {
	Name      string `json:"name"`
	Installed bool   `json:"installed"`
	ErrorMsg  string `json:"errorMsg,omitempty"`
}

// SetupAudit represents the comprehensive state of Day One environment.
type SetupAudit struct {
	Dependencies []DependencyStatus `json:"dependencies"`
	EnvConfigured bool               `json:"envConfigured"`
	IsCloudMode   bool               `json:"isCloudMode"`
	IsStandalone  bool               `json:"isStandalone"`
}

// RunAudit performs the automated Day One setup flow audit.
func RunAudit() SetupAudit {
	audit := SetupAudit{}

	// Verify .env file
	if _, err := os.Stat("../../.env"); err == nil {
		audit.EnvConfigured = true
	} else if _, err := os.Stat(".env"); err == nil {
		audit.EnvConfigured = true
	}

	// Verify dependencies
	binaries := []string{"bazelisk", "docker", "go"}

	// Determine Mode
	audit.IsCloudMode = os.Getenv("OHC_MULTITENANT") == "true"
	audit.IsStandalone = os.Getenv("OHC_STANDALONE") == "true"

	if audit.IsCloudMode {
		binaries = append(binaries, "redis-cli")
	}
	if audit.IsStandalone {
		binaries = append(binaries, "sqlite3")
	}

	for _, bin := range binaries {
		status := DependencyStatus{Name: bin}
		if _, err := exec.LookPath(bin); err == nil {
			status.Installed = true
		} else {
			status.Installed = false
			status.ErrorMsg = "Binary not found in PATH"
		}
		audit.Dependencies = append(audit.Dependencies, status)
	}

	return audit
}
