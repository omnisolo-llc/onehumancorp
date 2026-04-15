package onboarding

import (
	"context"
	"fmt"
	"os"
	"sort"
)

// RunCLI instantiates the InteractiveWizard, runs the setup, and prints
// the configuration mapped to stdout.
func RunCLI(ctx context.Context, isCloud bool) error {
	wizard := NewInteractiveWizard()

	config, err := wizard.RunInteractiveSetup(ctx, isCloud)
	if err != nil {
		return fmt.Errorf("failed to run interactive setup: %w", err)
	}

	mode := "Standalone"
	if isCloud {
		mode = "Cloud-native"
	}

	fmt.Fprintf(os.Stdout, "OHC Interactive Setup (%s)\n", mode)
	fmt.Fprintf(os.Stdout, "Configuration Options:\n")

	keys := make([]string, 0, len(config))
	for k := range config {
		keys = append(keys, k)
	}
	sort.Strings(keys)

	for _, k := range keys {
		fmt.Fprintf(os.Stdout, "  %s: %s\n", k, config[k])
	}

	return nil
}
