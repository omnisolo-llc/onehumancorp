package builtin

import "fmt"

// BudgetTracker mimics the tracking logic in claude-code query/tokenBudget.ts.
type BudgetTracker struct {
	ContinuationCount    int
	LastDeltaTokens      int
	LastGlobalTurnTokens int
}

const (
	completionThreshold float64 = 0.9
	diminishingThreshold int = 500
)

type TokenBudgetDecision struct {
	Action            string // "continue" or "stop"
	NudgeMessage      string
	ContinuationCount int
	Pct               int
	TurnTokens        int
	Budget            int
	Diminishing       bool
}

// CheckTokenBudget evaluates if the model has hit a budget constraint.
// It is designed to be called when the LLM stops generating due to length
// (e.g. hit max_tokens for a turn) and we want to nudge it to continue
// towards the MaxTaskBudget.
func CheckTokenBudget(tracker *BudgetTracker, budget int, globalTurnTokens int) TokenBudgetDecision {
	if budget <= 0 {
		return TokenBudgetDecision{Action: "stop"}
	}

	pct := int((float64(globalTurnTokens) / float64(budget)) * 100)
	deltaSinceLastCheck := globalTurnTokens - tracker.LastGlobalTurnTokens

	isDiminishing := tracker.ContinuationCount >= 3 &&
		deltaSinceLastCheck < diminishingThreshold &&
		tracker.LastDeltaTokens < diminishingThreshold

	if !isDiminishing && float64(globalTurnTokens) < float64(budget)*completionThreshold {
		tracker.ContinuationCount++
		tracker.LastDeltaTokens = deltaSinceLastCheck
		tracker.LastGlobalTurnTokens = globalTurnTokens
		nudge := fmt.Sprintf("Stopped at %d%% of token target (%d / %d). Keep working — do not summarize.", pct, globalTurnTokens, budget)
		return TokenBudgetDecision{
			Action:            "continue",
			NudgeMessage:      nudge,
			ContinuationCount: tracker.ContinuationCount,
			Pct:               pct,
			TurnTokens:        globalTurnTokens,
			Budget:            budget,
		}
	}

	return TokenBudgetDecision{
		Action:            "stop",
		ContinuationCount: tracker.ContinuationCount,
		Pct:               pct,
		TurnTokens:        globalTurnTokens,
		Budget:            budget,
		Diminishing:       isDiminishing,
	}
}
