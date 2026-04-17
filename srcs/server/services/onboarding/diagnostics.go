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

func defaultRuntimeDir() string {
	if stateDir := os.Getenv("XDG_STATE_HOME"); stateDir != "" {
		return filepath.Join(stateDir, "ohc", "runtime")
	}
	if home := os.Getenv("HOME"); home != "" {
		return filepath.Join(home, ".local", "state", "ohc", "runtime")
	}
	return filepath.Join(os.TempDir(), "ohc", "runtime")
}

// RunDiagnostics checks for the existence of required Day One paths programmatically.
func RunDiagnostics() DiagnosticsResult {
	result := DiagnosticsResult{
		Passed:  true,
		Details: make([]string, 0),
	}
	runtimeDir := os.Getenv("OHC_RUNTIME_DIR")
	if runtimeDir == "" {
		runtimeDir = defaultRuntimeDir()
	}
	memoryDir := os.Getenv("OHC_MEMORY_DIR")
	if memoryDir == "" {
		memoryDir = filepath.Join(runtimeDir, "memory")
	}
	statusDir := os.Getenv("OHC_STATUS_DIR")
	if statusDir == "" {
		statusDir = filepath.Join(runtimeDir, "status")
	}

	requiredPaths := []string{
		runtimeDir,
		memoryDir,
		statusDir,
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
