package main

import (
	"fmt"
	"net/http"
	"os"
	"strings"

	"github.com/onehumancorp/mono/services/onboarding"
)

func main() {
	envVars := make(map[string]string)
	for _, env := range os.Environ() {
		parts := strings.SplitN(env, "=", 2)
		if len(parts) == 2 {
			envVars[parts[0]] = parts[1]
		}
	}

	config, err := onboarding.VerifyEnvironment(envVars)
	if err != nil {
		fmt.Printf("Day One Environment Verification Failed: %v\n", err)
		os.Exit(1)
	}

	fmt.Printf("Day One Environment Verification Passed!\nMode: %s\nMultiTenant: %v\nHeadless: %v\n", config.Mode, config.MultiTenant, config.Headless)

	http.HandleFunc("/api/provision", onboarding.ProvisionHandler)
	http.HandleFunc("/api/wizard/state/save", onboarding.SaveWizardStateHandler)
	http.HandleFunc("/api/wizard/state", onboarding.GetWizardStateHandler)
	http.HandleFunc("/api/verify-environment", onboarding.VerifyEnvironmentHandler)
	http.HandleFunc("/api/audit-setup", onboarding.AuditSetupHandler)
	http.HandleFunc("/api/generate-config", onboarding.GenerateConfigHandler)

	fmt.Println("Starting onboarding service on :8080...")
	if err := http.ListenAndServe(":8080", nil); err != nil {
		fmt.Printf("Failed to start server: %v\n", err)
		os.Exit(1)
	}
}
