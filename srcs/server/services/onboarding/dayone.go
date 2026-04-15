package onboarding

import (
	"context"
	"fmt"
)

// RunDayOneSetup orchestrates the Day One setup process for OHC.
func RunDayOneSetup(ctx context.Context, isCloud bool) (string, error) {
	// 1. Provision environment
	if err := ProvisionEnvironment(ctx, isCloud); err != nil {
		return "", fmt.Errorf("provisioning failed: %w", err)
	}

	// 2. Interactive setup
	wizard := NewInteractiveWizard()
	config, err := wizard.RunInteractiveSetup(ctx, isCloud)
	if err != nil {
		return "", fmt.Errorf("interactive setup failed: %w", err)
	}

	// 3. Validate config
	validator := &ValidationEndpoint{}
	if err := validator.ValidateConfig(ctx, config); err != nil {
		return "", fmt.Errorf("config validation failed: %w", err)
	}

	// 4. Generate audit report
	report := GenerateAuditReport(isCloud)
	return report, nil
}
