package builtin_test

import (
	"os"
	"strings"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/agents/builtin"
)

func TestGetSystemPrompt_Standalone(t *testing.T) {
	// Backup and restore environment
	orig := os.Getenv("OHC_STANDALONE")
	defer os.Setenv("OHC_STANDALONE", orig)

	// Test Cloud Mode (should not contain fallback)
	os.Setenv("OHC_STANDALONE", "false")
	promptCloud := builtin.GetSystemPrompt()
	if strings.Contains(promptCloud, ".ohc/memory/auto/") {
		t.Errorf("Cloud mode prompt should not contain standalone memory directories")
	}

	// Test Standalone Mode (should contain fallback)
	os.Setenv("OHC_STANDALONE", "true")
	promptStandalone := builtin.GetSystemPrompt()
	if !strings.Contains(promptStandalone, ".ohc/memory/auto/") {
		t.Errorf("Standalone mode prompt must contain .ohc/memory/auto/ instructions")
	}
	if !strings.Contains(promptStandalone, ".ohc/memory/team/") {
		t.Errorf("Standalone mode prompt must contain .ohc/memory/team/ instructions")
	}
}
