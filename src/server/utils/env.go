package utils

import (
	"os"
	"strings"
)

// EnvBoolDefault returns the boolean value of an environment variable, or a fallback if not set.
func EnvBoolDefault(key string, fallback bool) bool {
	val := os.Getenv(key)
	if val == "" {
		return fallback
	}
	return strings.ToLower(val) == "true"
}
