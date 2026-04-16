package orchestration

import (
    "os"
    "path/filepath"
    "testing"
)

func TestRunDesktopOnboarding(t *testing.T) {
    // Create a temporary HOME directory to avoid mutating the real one
    tempHome := t.TempDir()
    t.Setenv("HOME", tempHome)

    success := RunDesktopOnboarding()
    if !success {
        t.Errorf("Expected RunDesktopOnboarding to return true, got false")
    }

    // Verify the directory and file were created
    ohcDir := filepath.Join(tempHome, ".ohc")
    if _, err := os.Stat(ohcDir); os.IsNotExist(err) {
        t.Errorf("Expected directory %s to be created, but it does not exist", ohcDir)
    }
}

func TestRunCloudOnboarding(t *testing.T) {
    // Should fail when KUBERNETES_SERVICE_HOST is not set
    t.Setenv("KUBERNETES_SERVICE_HOST", "")
    if RunCloudOnboarding() {
        t.Errorf("Expected RunCloudOnboarding to return false when env is not set")
    }

    // Should succeed when KUBERNETES_SERVICE_HOST is set
    t.Setenv("KUBERNETES_SERVICE_HOST", "10.0.0.1")
    if !RunCloudOnboarding() {
        t.Errorf("Expected RunCloudOnboarding to return true when env is set")
    }
}
