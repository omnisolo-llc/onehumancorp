package billing

import (
	"testing"
)

func TestTracker_BurnRates(t *testing.T) {
	catalog := map[string]Price{
		"test-model": {InputPerMillionUSD: 10.0, OutputPerMillionUSD: 20.0},
	}
	tracker := NewTracker(catalog)
	defer tracker.Close()

	orgID := "org-burn-test"

	// 1. Initial burn rates should be 0
	tTokens, tUSD := tracker.GetBurnRates(orgID)
	if tTokens != 0 || tUSD != 0 {
		t.Errorf("Initial burn rates should be 0, got tokens=%v, usd=%v", tTokens, tUSD)
	}

	// 2. Track some usage
	_, _ = tracker.Track(Usage{
		OrganizationID:   orgID,
		AgentID:          "agent-1",
		Model:            "test-model",
		PromptTokens:     1000000,
		CompletionTokens: 500000,
	})

	// 3. Manually trigger burn rate calculation
	tracker.recordTokenBurnRates()

	// 4. Verify burn rates
	// Tokens = 1.5M, USD = 10 + 10 = 20
	tTokens, tUSD = tracker.GetBurnRates(orgID)
	if tTokens != 1500000 {
		t.Errorf("Expected token burn rate 1500000, got %v", tTokens)
	}
	if tUSD != 20.0 {
		t.Errorf("Expected USD burn rate 20.0, got %v", tUSD)
	}

	// 5. Track more usage
	_, _ = tracker.Track(Usage{
		OrganizationID:   orgID,
		AgentID:          "agent-1",
		Model:            "test-model",
		PromptTokens:     500000,
		CompletionTokens: 0,
	})

	// 6. Record again
	tracker.recordTokenBurnRates()

	// 7. Verify new burn rates (it's a delta per recording cycle)
	// New tokens = 500k, New USD = 5.0
	tTokens, tUSD = tracker.GetBurnRates(orgID)
	if tTokens != 500000 {
		t.Errorf("Expected token burn rate 500000, got %v", tTokens)
	}
	if tUSD != 5.0 {
		t.Errorf("Expected USD burn rate 5.0, got %v", tUSD)
	}
}
