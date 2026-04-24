package utils

import (
	"os"
	"testing"
)

func TestEnvBoolDefault(t *testing.T) {
	t.Run("default true", func(t *testing.T) {
		os.Unsetenv("TEST_KEY")
		if !EnvBoolDefault("TEST_KEY", true) {
			t.Errorf("expected true")
		}
	})

	t.Run("default false", func(t *testing.T) {
		os.Unsetenv("TEST_KEY")
		if EnvBoolDefault("TEST_KEY", false) {
			t.Errorf("expected false")
		}
	})

	t.Run("set true", func(t *testing.T) {
		os.Setenv("TEST_KEY", "true")
		if !EnvBoolDefault("TEST_KEY", false) {
			t.Errorf("expected true")
		}
	})

	t.Run("set false", func(t *testing.T) {
		os.Setenv("TEST_KEY", "false")
		if EnvBoolDefault("TEST_KEY", true) {
			t.Errorf("expected false")
		}
	})
}
