package onboarding

import (
	"errors"
	"strings"
)

// EnvConfig represents a parsed day-one configuration.
type EnvConfig struct {
	Mode        string
	MultiTenant bool
	Headless    bool
	DatabaseURL string
	RedisURL    string
}

// VerifyEnvironment checks the provided environment variables for Day One setup validity.
func VerifyEnvironment(envVars map[string]string) (*EnvConfig, error) {
	config := &EnvConfig{}

	mode, ok := envVars["OHC_SOURCE_MODE"]
	if !ok || mode == "" {
		return nil, errors.New("OHC_SOURCE_MODE is required (e.g. standalone, cloud, headless)")
	}
	config.Mode = strings.ToLower(mode)

	if mt, ok := envVars["OHC_MULTITENANT"]; ok && strings.ToLower(mt) == "true" {
		config.MultiTenant = true
	}

	if hl, ok := envVars["OHC_HEADLESS"]; ok && strings.ToLower(hl) == "true" {
		config.Headless = true
	}

	if config.Mode == "cloud" {
		if !config.MultiTenant {
			return nil, errors.New("cloud mode requires OHC_MULTITENANT to be true")
		}

		dbURL, ok := envVars["OHC_POSTGRES_URL"]
		if !ok || dbURL == "" {
			return nil, errors.New("cloud mode requires OHC_POSTGRES_URL")
		}
		// Zero Secrets: ensure spiffe compliant identity string is used
		if !strings.Contains(dbURL, "spiffe-") {
			return nil, errors.New("OHC_POSTGRES_URL must use a SPIFFE-compliant identity string (Zero Secrets)")
		}
		config.DatabaseURL = dbURL

		redisURL, ok := envVars["OHC_REDIS_URL"]
		if !ok || redisURL == "" {
			return nil, errors.New("cloud mode requires OHC_REDIS_URL")
		}
		if !strings.Contains(redisURL, "spiffe-") {
			return nil, errors.New("OHC_REDIS_URL must use a SPIFFE-compliant identity string (Zero Secrets)")
		}
		config.RedisURL = redisURL
	}

	if config.Mode == "standalone" {
		if config.MultiTenant {
			return nil, errors.New("standalone mode cannot be multitenant")
		}

		dbPath, ok := envVars["OHC_SQLITE_PATH"]
		if !ok || dbPath == "" {
			config.DatabaseURL = "file:./ohc.db?cache=shared" // Default for standalone
		} else {
			config.DatabaseURL = dbPath
		}
	}

	return config, nil
}
