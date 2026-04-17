package telemetry

import (
	"context"
	"encoding/json"
	"log/slog"
	"os"
)

// InitUnifiedLogging initializes a structured slog logger that behaves
// consistently across Cloud and Standalone environments.
// It outputs JSON logs to stdout.
func InitUnifiedLogging(mode string) {
	opts := &slog.HandlerOptions{
		Level: slog.LevelInfo,
	}

	// Ensure that all environments use structured JSON logging.
	handler := slog.NewJSONHandler(os.Stdout, opts)

	logger := slog.New(handler).With(
		slog.String("component", "ohc-backend"),
		slog.String("deployment_mode", mode),
	)

	slog.SetDefault(logger)
}

// LogCloudEvent is a helper for unified structured logging for Cloud events.
func LogCloudEvent(ctx context.Context, event string, attrs map[string]interface{}) {
	slog.InfoContext(ctx, event, "details", formatMap(attrs))
}

func formatMap(m map[string]interface{}) string {
	b, _ := json.Marshal(m)
	return string(b)
}
