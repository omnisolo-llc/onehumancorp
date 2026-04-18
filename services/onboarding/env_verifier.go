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
	BootstrapOrgID     string
	BootstrapOrgName   string
	BootstrapCEOName   string
	DefaultAgentName   string
	DefaultAgentRole   string
	DefaultAgentRegion string
}

// VerifyEnvironment checks the provided environment variables for Day One setup validity.
func VerifyEnvironment(envVars map[string]string) (*EnvConfig, error) {
	config := &EnvConfig{}

	mode, ok := envVars["OHC_SOURCE_MODE"]
	if !ok || mode == "" {
		return nil, errors.New("OHC_SOURCE_MODE is required (e.g. standalone, cloud, headless, thin_client)")
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

	if config.Mode == "standalone" && config.MultiTenant {
		return nil, errors.New("standalone mode cannot be multitenant")
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

	if val, ok := envVars["OHC_BOOTSTRAP_ORG_ID"]; ok {
		config.BootstrapOrgID = val
	}
	if val, ok := envVars["OHC_BOOTSTRAP_ORG_NAME"]; ok {
		config.BootstrapOrgName = val
	}
	if val, ok := envVars["OHC_BOOTSTRAP_CEO_NAME"]; ok {
		config.BootstrapCEOName = val
	}
	if val, ok := envVars["OHC_DEFAULT_AGENT_NAME"]; ok {
		config.DefaultAgentName = val
	}
	if val, ok := envVars["OHC_DEFAULT_AGENT_ROLE"]; ok {
		config.DefaultAgentRole = val
	}
	if val, ok := envVars["OHC_DEFAULT_AGENT_REGION"]; ok {
		config.DefaultAgentRegion = val
	} else {
		config.DefaultAgentRegion = "docker"
	}

	return config, nil
}
