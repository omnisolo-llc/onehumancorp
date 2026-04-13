package onboarding

import (
	"context"
	"fmt"
	"encoding/json"
	"net/http"
	"os"
	"path/filepath"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var provisionsCounter metric.Int64Counter

func init() {
	meter := otel.Meter("github.com/onehumancorp/mono/ohc")
	provisionsCounter, _ = meter.Int64Counter("provisions_total")
}

// ProvisionEnvironment sets up the necessary folders for Day One Hybrid OS onboarding.
func ProvisionEnvironment(ctx context.Context, isCloud bool) error {
	if provisionsCounter != nil {
		provisionsCounter.Add(ctx, 1)
	}

	baseDir := ".ohc-local-data"
	if isCloud {
		baseDir = ".ohc-cloud-data"
	}

	dirs := []string{
		filepath.Join(baseDir, "db"),
		filepath.Join(baseDir, "blob"),
		filepath.Join(baseDir, "config"),
	}

	for _, dir := range dirs {
		if err := os.MkdirAll(dir, 0755); err != nil {
			return fmt.Errorf("failed to create directory %s: %w", dir, err)
		}
	}

	return nil
}

// CheckEnvironment verifies that the necessary folders for Day One Hybrid OS onboarding exist.
func CheckEnvironment(isCloud bool) error {
	baseDir := ".ohc-local-data"
	if isCloud {
		baseDir = ".ohc-cloud-data"
	}

	dirs := []string{
		filepath.Join(baseDir, "db"),
		filepath.Join(baseDir, "blob"),
		filepath.Join(baseDir, "config"),
	}

	for _, dir := range dirs {
		if _, err := os.Stat(dir); os.IsNotExist(err) {
			return fmt.Errorf("directory %s does not exist", dir)
		}
	}

	return nil
}


// HealthHandler responds to HTTP health check requests for the environment.
func HealthHandler(w http.ResponseWriter, r *http.Request) {
	isCloud := r.URL.Query().Get("cloud") == "true"

	err := CheckEnvironment(isCloud)
	w.Header().Set("Content-Type", "application/json")

	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		json.NewEncoder(w).Encode(map[string]string{
			"status": "error",
			"error":  err.Error(),
		})
		return
	}

	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(map[string]string{
		"status": "ok",
	})
}
