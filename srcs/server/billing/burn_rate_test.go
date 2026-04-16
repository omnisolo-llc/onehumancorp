package billing

import (
	"testing"
	"time"
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
	// RecordUsage is no longer called in Track, so we need to trigger collection.

	// First collection (will record current state, which should have 1.5M tokens)
	tracker.forecaster.collectAndCalculate(tracker.ctx)

	// Manually backdate the first record to simulate time passing AND set tokens/cost to 0 to simulate rate from start
	now := time.Now()
	tracker.forecaster.mu.Lock()
	tracker.forecaster.usageHistory[orgID][0].timestamp = now.Add(-1 * time.Minute)
	tracker.forecaster.usageHistory[orgID][0].tokens = 0
	tracker.forecaster.usageHistory[orgID][0].costUSD = 0
	tracker.forecaster.mu.Unlock()

	// Second collection (will record same 1.5M tokens, but duration is now 1 min)
	tracker.forecaster.collectAndCalculate(tracker.ctx)

	// 4. Verify burn rates
	// Tokens = 1.5M over 1 min = 1.5M tokens/min, USD = 20 over 1 min = 20 USD/min
	tTokens, tUSD = tracker.GetBurnRates(orgID)
	if tTokens < 1400000 || tTokens > 1600000 {
		t.Errorf("Expected token burn rate ~1500000, got %v", tTokens)
	}
	if tUSD < 19 || tUSD > 21 {
		t.Errorf("Expected USD burn rate ~20.0, got %v", tUSD)
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
	tracker.forecaster.collectAndCalculate(tracker.ctx)

	// 7. Verify new burn rates (it's a moving average over the window)
	tTokens, tUSD = tracker.GetBurnRates(orgID)
	if tTokens <= 0 || tUSD <= 0 {
		t.Errorf("Expected positive burn rates, got tokens=%v, usd=%v", tTokens, tUSD)
	}
}
