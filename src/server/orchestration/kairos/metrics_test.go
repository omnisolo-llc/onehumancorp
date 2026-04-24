package kairos

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestGetMode(t *testing.T) {
	// Test headless mode
	t.Setenv("OHC_HEADLESS", "true")
	assert.Equal(t, "headless", GetMode())

	// Test cloud mode
	t.Setenv("OHC_HEADLESS", "false")
	t.Setenv("OHC_MULTITENANT", "true")
	assert.Equal(t, "cloud", GetMode())

	// Test standalone mode
	t.Setenv("OHC_MULTITENANT", "false")
	assert.Equal(t, "standalone", GetMode())
}
