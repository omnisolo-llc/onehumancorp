package onboarding

import (
	"context"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var (
	meter                 = otel.Meter("github.com/onehumancorp/mono/srcs/server/onboarding")
	AuditAttemptsTotal, _ = meter.Int64Counter("onboarding_audit_attempts_total", metric.WithDescription("Total setup audit attempts"))
	AuditErrorsTotal, _   = meter.Int64Counter("onboarding_audit_errors_total", metric.WithDescription("Total setup audit errors"))
)

// SetupAuditService verifies the Day One setup environment.
type SetupAuditService interface {
	VerifySetup(ctx context.Context, targetDir string) (bool, error)
}

type setupAuditServiceImpl struct {
	requiredFiles []string
}

// NewSetupAuditService creates a new audit service.
func NewSetupAuditService(requiredFiles []string) SetupAuditService {
	return &setupAuditServiceImpl{
		requiredFiles: requiredFiles,
	}
}

func (s *setupAuditServiceImpl) VerifySetup(ctx context.Context, targetDir string) (bool, error) {
	AuditAttemptsTotal.Add(ctx, 1)

	cleanDir := filepath.Clean(targetDir)
	if filepath.IsAbs(cleanDir) {
		AuditErrorsTotal.Add(ctx, 1)
		return false, errors.New("absolute paths are not allowed")
	}

	missingFiles := []string{}
	for _, f := range s.requiredFiles {
		path := filepath.Clean(filepath.Join(cleanDir, f))

		// boundary check enforcing exact equality or prefix with trailing slash
		if path != cleanDir && !strings.HasPrefix(path, cleanDir+string(os.PathSeparator)) {
			// Special case for root directory `.`
			if cleanDir != "." {
				AuditErrorsTotal.Add(ctx, 1)
				return false, errors.New("path escapes directory boundary")
			}
		}

		if _, err := os.Stat(path); os.IsNotExist(err) {
			missingFiles = append(missingFiles, f)
		}
	}

	if len(missingFiles) > 0 {
		AuditErrorsTotal.Add(ctx, 1)
		return false, fmt.Errorf("missing required onboarding files: %v", missingFiles)
	}

	return true, nil
}
