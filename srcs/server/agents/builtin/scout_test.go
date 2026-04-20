package builtin

import (
	"context"
	"database/sql"
	"strings"
	"testing"

	_ "modernc.org/sqlite"
)

func TestScoutAgent_RegisterParsedAPI(t *testing.T) {
	// Setup in-memory SQLite DB for testing
	db, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("Failed to open sqlite db: %v", err)
	}
	defer db.Close()

	scout := NewScoutAgent(db, "switchboard.local")
	ctx := context.Background()

	// Test case 1: Successful registration of a dummy API
	t.Run("Success_DummyAPI", func(t *testing.T) {
		err := scout.RegisterParsedAPI(ctx, "https://example.com/dummy-openapi.yaml")
		if err != nil {
			t.Errorf("Expected success, got error: %v", err)
		}
	})

	// Test case 2: Invalid URL format
	t.Run("Failure_InvalidURL", func(t *testing.T) {
		err := scout.RegisterParsedAPI(ctx, "invalid-url")
		if err == nil {
			t.Errorf("Expected error for invalid URL, got nil")
		}
	})

	// Test case 3: Safety Guardrail Failure
	t.Run("Failure_SafetyGuardrails", func(t *testing.T) {
		err := scout.RegisterParsedAPI(ctx, "https://example.com/malicious-api.yaml")
		if err == nil {
			t.Errorf("Expected error for malicious API failing guardrails, got nil")
		}
	})

	// Test case 4: Check System Prompt contains required keywords
	t.Run("SystemPrompt", func(t *testing.T) {
		prompt := scout.SystemPrompt()
		if prompt == "" {
			t.Errorf("SystemPrompt is empty")
		}

		keywords := []string{"Scout", "Dynamic Tool Discovery", "Agentic Guardrails", "SPIFFE/SPIRE", "OpenAPI"}
		for _, kw := range keywords {
			if !strings.Contains(prompt, kw) {
				t.Errorf("SystemPrompt missing expected keyword: %s", kw)
			}
		}
	})
}
