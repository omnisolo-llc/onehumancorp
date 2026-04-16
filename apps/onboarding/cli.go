package onboarding

import (
	"context"
	"fmt"
	"os"
	"sort"
)

// RunCLI instantiates the InteractiveWizard, runs the setup, and prints
// the configuration mapped to stdout.
func RunCLI(ctx context.Context, mode string) error {
	config, err := GenerateDayOneConfig(ctx, mode)
	if err != nil {
		return fmt.Errorf("failed to run interactive setup: %w", err)
	}

	displayMode := "Standalone"
	if mode == "cloud" {
		displayMode = "Cloud-native"
	}

	fmt.Fprintf(os.Stdout, "OHC Interactive Setup (%s)\n", displayMode)
	fmt.Fprintf(os.Stdout, "Configuration Options:\n")

	configMap := map[string]string{
		"mode": config.Mode,
		"database": config.DatabaseURL,
		"redis": config.RedisURL,
	}

	keys := make([]string, 0, len(configMap))
	for k := range configMap {
		keys = append(keys, k)
	}
	sort.Strings(keys)

	for _, k := range keys {
		fmt.Fprintf(os.Stdout, "  %s: %s\n", k, configMap[k])
	}

	return nil
}
