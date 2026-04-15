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

if config.Mode == "cloud" && !config.MultiTenant {
return nil, errors.New("cloud mode requires OHC_MULTITENANT to be true")
}

if config.Mode == "standalone" && config.MultiTenant {
return nil, errors.New("standalone mode cannot be multitenant")
}

return config, nil
}
