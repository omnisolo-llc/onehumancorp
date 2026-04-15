package onboarding

import (
	"fmt"
	"os"
	"path/filepath"
)

// DiagnosticsResult holds the results of the environment health check.
type DiagnosticsResult struct {
	Passed  bool
	Details []string
}

// RunDiagnostics checks for the existence of required Day One paths programmatically.
func RunDiagnostics(isCloud bool) DiagnosticsResult {
	result := DiagnosticsResult{
		Passed:  true,
		Details: make([]string, 0),
	}
	baseDir := ".ohc-local-data"
	if isCloud {
		baseDir = ".ohc-cloud-data"
	}

	requiredPaths := []string{
		filepath.Join(baseDir, "db"),
		filepath.Join(baseDir, "blob"),
		filepath.Join(baseDir, "config"),
	}

	for _, path := range requiredPaths {
		if _, err := os.Stat(path); os.IsNotExist(err) {
			result.Passed = false
			result.Details = append(result.Details, fmt.Sprintf("Missing required path: %s", path))
		} else {
			result.Details = append(result.Details, fmt.Sprintf("Found required path: %s", path))
		}
	}

	return result
}