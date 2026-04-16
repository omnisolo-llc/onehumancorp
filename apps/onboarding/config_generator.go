package onboarding

import (
	"context"
	"errors"
)

type Config struct {
	Mode        string
	DatabaseURL string
	RedisURL    string
}

func GenerateDayOneConfig(ctx context.Context, mode string) (*Config, error) {
	if mode == "cloud" {
		return &Config{
			Mode:        "cloud",
			DatabaseURL: "postgres://ohc-cloud-spiffe-user@localhost:5432/ohc",
			RedisURL:    "redis://localhost:6379",
		}, nil
	} else if mode == "standalone" {
		return &Config{
			Mode:        "standalone",
			DatabaseURL: "sqlite:///.ohc-local-data/db/ohc.db",
			RedisURL:    "",
		}, nil
	}
	return nil, errors.New("invalid mode, must be 'cloud' or 'standalone'")
}
