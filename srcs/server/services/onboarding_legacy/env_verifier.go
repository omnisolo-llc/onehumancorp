package onboarding

import (
	"errors"
	"strings"
)

// EnvConfig represents a parsed day-one configuration.
type EnvConfig struct {
	Mode             string
	MultiTenant      bool
	Headless         bool
	TelemetryEnabled bool
	ApiEndpoint      string
	DatabaseURL      string
}

// VerifyEnvironment checks the provided environment variables for Day One setup validity.
func VerifyEnvironment(envVars map[string]string) (*EnvConfig, error) {
	config := &EnvConfig{}

	mode, ok := envVars["OHC_SOURCE_MODE"]
	if !ok || mode == "" {
		if _, isK8s := envVars["KUBERNETES_SERVICE_HOST"]; isK8s {
			mode = "cloud"
			envVars["OHC_MULTITENANT"] = "true"
		} else if endpoint, hasEndpoint := envVars["OHC_API_ENDPOINT"]; hasEndpoint && endpoint != "" {
			mode = "thin_client"
		} else {
			mode = "standalone"
		}
	}
	config.Mode = strings.ToLower(mode)

	if mt, ok := envVars["OHC_MULTITENANT"]; ok && strings.ToLower(mt) == "true" {
		config.MultiTenant = true
	}

	if hl, ok := envVars["OHC_HEADLESS"]; ok && strings.ToLower(hl) == "true" {
		config.Headless = true
	}
	if config.Mode == "cloud" && !config.MultiTenant {
		return nil, errors.New("cloud mode requires OHC_MULTITENANT to be true")
	}

	if config.Mode == "cloud" {
		dbUrl, ok := envVars["DATABASE_URL"]
		if !ok || dbUrl == "" {
			// Auto-detected cloud mode shouldn't strictly error if DATABASE_URL is initially missing
			// BUT if OHC_MULTITENANT is set to "true" explicitly and not via auto detect, we error
			if _, isK8s := envVars["KUBERNETES_SERVICE_HOST"]; isK8s {
				config.DatabaseURL = ""
			} else {
				return nil, errors.New("cloud mode requires DATABASE_URL")
			}
		} else {
            config.DatabaseURL = dbUrl
        }
	}

	if config.Mode == "standalone" && config.MultiTenant {
		return nil, errors.New("standalone mode cannot be multitenant")
	}

	if config.Mode == "standalone" {
		dbUrl, ok := envVars["DATABASE_URL"]
		if !ok || dbUrl == "" {
			config.DatabaseURL = "sqlite://local.db"
		} else {
			config.DatabaseURL = dbUrl
		}
	}

	if config.Mode == "thin_client" {
		endpoint, ok := envVars["OHC_API_ENDPOINT"]
		if !ok || endpoint == "" {
			return nil, errors.New("thin_client mode requires OHC_API_ENDPOINT")
		}
		config.ApiEndpoint = endpoint
	}

	telemetryEnabled := false
	if tel, ok := envVars["OHC_TELEMETRY_ENABLED"]; ok && strings.ToLower(tel) == "true" {
		telemetryEnabled = true
	}

	isStandalone := false
	if sa, ok := envVars["OHC_STANDALONE"]; ok && strings.ToLower(sa) == "true" {
		isStandalone = true
	}
	if config.Mode == "standalone" {
		isStandalone = true
	}

	if isStandalone {
		config.TelemetryEnabled = telemetryEnabled
	} else {
		config.TelemetryEnabled = true
		if tel, ok := envVars["OHC_TELEMETRY_ENABLED"]; ok && strings.ToLower(tel) == "false" {
			config.TelemetryEnabled = false
		}
	}

	return config, nil
}
