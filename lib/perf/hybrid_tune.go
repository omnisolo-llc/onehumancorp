package perf

import (
	"context"
)

// TuneConfig holds configuration for tuning the hybrid environment.
type TuneConfig struct {
	MaxWorkers int
	BatchSize  int
}

// DefaultTuneConfig returns the default tuning configuration.
func DefaultTuneConfig() TuneConfig {
	return TuneConfig{
		MaxWorkers: 10,
		BatchSize:  100,
	}
}

type tuneConfigKeyType struct{}

var tuneConfigKey = tuneConfigKeyType{}

// ApplyTuning applies the tuning configuration to the given context.
func ApplyTuning(ctx context.Context, config TuneConfig) context.Context {
	return context.WithValue(ctx, tuneConfigKey, config)
}

// GetTuneConfig retrieves the tuning configuration from the context.
func GetTuneConfig(ctx context.Context) TuneConfig {
	if config, ok := ctx.Value(tuneConfigKey).(TuneConfig); ok {
		return config
	}
	return DefaultTuneConfig()
}
