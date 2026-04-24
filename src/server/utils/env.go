package utils

import (
	"os"
	"strings"
)

// EnvBoolDefault returns the boolean value of an environment variable,
// or a default value if the variable is not set or empty.
func EnvBoolDefault(key string, fallback bool) bool {
	val := os.Getenv(key)
	if val == "" {
		return fallback
	}
	return strings.ToLower(val) == "true"
}
