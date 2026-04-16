package orchestration

import (
    "os"
    "path/filepath"
)

// RunDesktopOnboarding bypasses cloud setup for Standalone Desktop mode
// by verifying local configuration paths.
func RunDesktopOnboarding() bool {
    home, err := os.UserHomeDir()
    if err != nil {
        return false
    }

    ohcDir := filepath.Join(home, ".ohc")
    if _, err := os.Stat(ohcDir); os.IsNotExist(err) {
        err := os.MkdirAll(ohcDir, 0755)
        if err != nil {
            return false
        }
    }

    return true
}

// RunCloudOnboarding bypasses or verifies setup for Cloud K8s mode
// by verifying Kubernetes environment variables.
func RunCloudOnboarding() bool {
    // KUBERNETES_SERVICE_HOST is standard in K8s pods
    host := os.Getenv("KUBERNETES_SERVICE_HOST")
    if host == "" {
        return false
    }
    return true
}
