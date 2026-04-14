package onboarding

import (
	"context"
	"errors"
)

type ValidationEndpoint struct{}

func (v *ValidationEndpoint) ValidateConfig(ctx context.Context, config map[string]string) error {
	mode, ok := config["mode"]
	if !ok {
		return errors.New("mode is required")
	}

	db := config["db"]
	cache := config["cache"]

	if mode == "cloud" {
		if db != "postgres" || cache != "redis" {
			return errors.New("invalid configuration for cloud mode: requires postgres and redis")
		}
	} else if mode == "standalone" {
		if db != "sqlite" || cache != "memory" {
			return errors.New("invalid configuration for standalone mode: requires sqlite and memory")
		}
	} else {
		return errors.New("unknown mode")
	}

	return nil
}
