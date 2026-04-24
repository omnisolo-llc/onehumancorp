package utils

import (
	"os"
	"testing"
)

func TestEnvBoolDefault(t *testing.T) {
	os.Setenv("TEST_KEY_ENV", "true")
	defer os.Unsetenv("TEST_KEY_ENV")

	if !EnvBoolDefault("TEST_KEY_ENV", false) {
		t.Errorf("expected true")
	}

	if EnvBoolDefault("TEST_MISSING", false) {
		t.Errorf("expected false")
	}
}
