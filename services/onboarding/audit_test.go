package onboarding

import (
	"os"
	"testing"
)

func TestRunAudit(t *testing.T) {
	// Temporarily override env for test
	os.Setenv("OHC_STANDALONE", "true")
	os.Setenv("OHC_MULTITENANT", "false")
	defer os.Unsetenv("OHC_STANDALONE")
	defer os.Unsetenv("OHC_MULTITENANT")

	audit := RunAudit()

	if !audit.IsStandalone {
		t.Errorf("Expected IsStandalone to be true")
	}
	if audit.IsCloudMode {
		t.Errorf("Expected IsCloudMode to be false")
	}

	// Check if dependencies list has expected core binaries
	foundGo := false
	for _, dep := range audit.Dependencies {
		if dep.Name == "go" {
			foundGo = true
		}
	}
	if !foundGo {
		t.Errorf("Expected 'go' in dependency list")
	}
}
