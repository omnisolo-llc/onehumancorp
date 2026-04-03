package onboarding

import (
	"context"
	"os"

	"github.com/onehumancorp/mono/srcs/server/db"
)

// SetupStatus holds the diagnostic information for the Day One setup.
type SetupStatus struct {
	IsStandalone  bool     `json:"is_standalone"`
	DatabaseReady bool     `json:"database_ready"`
	MissingEnvVars []string `json:"missing_env_vars"`
}

// RunHealthCheck evaluates the current system environment and database connection
// to verify if the Day One setup for Standalone Desktop Mode is complete.
func RunHealthCheck(ctx context.Context, provider db.Provider) SetupStatus {
	status := SetupStatus{
		IsStandalone: os.Getenv("OHC_STANDALONE") == "true",
	}

	if status.IsStandalone {
		// SQLite check
		status.DatabaseReady = provider != nil && provider.IsSQLite()

		// Additional basic environment variables
		required := []string{"DATABASE_URL"}
		for _, req := range required {
			if os.Getenv(req) == "" {
				status.MissingEnvVars = append(status.MissingEnvVars, req)
			}
		}
	} else {
		// Cloud check
		status.DatabaseReady = provider != nil && !provider.IsSQLite()
		required := []string{"DATABASE_URL", "REDIS_URL"}
		for _, req := range required {
			if os.Getenv(req) == "" {
				status.MissingEnvVars = append(status.MissingEnvVars, req)
			}
		}
	}

	return status
}
