package main

import (
	"fmt"
	"os"

	"github.com/onehumancorp/mono/services/onboarding"
)

func RunCLI(endpoint string) error {
	fmt.Printf("🔍 Verifying Day One environment via %s...\n", endpoint)
	resp, err := onboarding.VerifyEnvironment(endpoint)
	if err != nil {
		return fmt.Errorf("Verification failed: %w", err)
	}

	fmt.Printf("✅ Mode: %s\n", resp.Mode)
	fmt.Printf("✅ Overall Status: %s\n", resp.Status)
	for _, d := range resp.Diagnostics {
		if d.Status == "ok" {
			fmt.Printf("  [OK] %s: %s\n", d.Check, d.Message)
		} else {
			fmt.Printf("  [ERR] %s: %s\n", d.Check, d.Message)
		}
	}
	return nil
}

func main() {
	endpoint := os.Getenv("OHC_BACKEND_URL")
	if endpoint == "" {
		endpoint = "http://localhost:8080"
	}
	if err := RunCLI(endpoint); err != nil {
		fmt.Fprintf(os.Stderr, "%v\n", err)
		os.Exit(1)
	}
}
