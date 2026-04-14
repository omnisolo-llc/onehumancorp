package onboarding

import (
	"os"
	"testing"
)

func TestGetStatusStandalone(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	status := GetStatus()
	if !status.IsStandalone {
		t.Errorf("Expected IsStandalone to be true")
	}
	if status.Mode != "Standalone Desktop" {
		t.Errorf("Expected Mode to be Standalone Desktop, got %s", status.Mode)
	}
}

func TestGetStatusCloudNative(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "false")
	defer os.Unsetenv("OHC_STANDALONE")

	status := GetStatus()
	if status.IsStandalone {
		t.Errorf("Expected IsStandalone to be false")
	}
	if status.Mode != "Cloud-Native K8s" {
		t.Errorf("Expected Mode to be Cloud-Native K8s, got %s", status.Mode)
	}
}
