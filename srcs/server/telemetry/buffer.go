package telemetry

import (
	"context"
	"encoding/hex"
	"fmt"
	"math/rand"
	"os"

	"github.com/onehumancorp/mono/srcs/server/db"
)

var (
	localDB db.Provider
)

// SetLocalDB injects the local database provider for standalone telemetry buffering.
func SetLocalDB(provider db.Provider) {
	localDB = provider
}

func init() {
	BufferMetricFunc = defaultBufferMetricFunc
}

func generateID() string {
	b := make([]byte, 16)
	_, _ = rand.Read(b)
	return hex.EncodeToString(b[0:4]) + "-" + hex.EncodeToString(b[4:6]) + "-" + hex.EncodeToString(b[6:8]) + "-" + hex.EncodeToString(b[8:10]) + "-" + hex.EncodeToString(b[10:])
}

func defaultBufferMetricFunc(ctx context.Context, metricType string, payload string) error {
	if os.Getenv("OHC_STANDALONE") != "true" {
		return nil
	}
	if os.Getenv("OHC_TELEMETRY_ENABLED") != "true" {
		return nil // Opt-in privacy guardrail
	}
	if localDB == nil {
		return fmt.Errorf("localDB not initialized")
	}

	id := generateID()
	query := "INSERT INTO telemetry_buffer (id, metric_type, payload) VALUES ($1, $2, $3)"
	if localDB.IsSQLite() {
		query = "INSERT INTO telemetry_buffer (id, metric_type, payload) VALUES (?, ?, ?)"
	}

	_, err := localDB.Exec(ctx, query, id, metricType, payload)
	if err != nil {
		return fmt.Errorf("failed to insert telemetry metric into buffer: %w", err)
	}
	return nil
}
