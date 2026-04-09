package builtin

import (
	"os"
	"strconv"
)

type Config struct {
	MaxTurns           int
	MaxTokens          int
	TokenThreshold     int
	MaxRetries         int
	StreamingEnabled   bool
	PermissionMode     string
}

func GetConfig() Config {
	maxTurns, _ := strconv.Atoi(os.Getenv("OHC_BUILTIN_MAX_TURNS"))
	if maxTurns == 0 {
		maxTurns = 50
	}

	maxTokens, _ := strconv.Atoi(os.Getenv("OHC_BUILTIN_MAX_TOKENS"))
	if maxTokens == 0 {
		maxTokens = 4096
	}

	tokenThreshold, _ := strconv.Atoi(os.Getenv("OHC_BUILTIN_TOKEN_THRESHOLD"))
	if tokenThreshold == 0 {
		tokenThreshold = 180000 // A conservative default for 200k models
	}

	maxRetries, _ := strconv.Atoi(os.Getenv("OHC_BUILTIN_MAX_RETRIES"))
	if maxRetries == 0 {
		maxRetries = 3
	}

	streamingEnabled := os.Getenv("OHC_BUILTIN_STREAMING") == "true"

	permissionMode := os.Getenv("OHC_BUILTIN_PERMISSION_MODE")
	if permissionMode == "" {
		permissionMode = "auto"
	}

	return Config{
		MaxTurns:           maxTurns,
		MaxTokens:          maxTokens,
		TokenThreshold:     tokenThreshold,
		MaxRetries:         maxRetries,
		StreamingEnabled:   streamingEnabled,
		PermissionMode:     permissionMode,
	}
}
