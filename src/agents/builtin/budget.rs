/// Token budget tracker — mirrors Go BudgetTracker.
#[derive(Debug, Default)]
pub struct BudgetTracker {
    pub continuation_count: i32,
    pub last_delta_tokens: i32,
    pub last_global_turn_tokens: i32,
}

const COMPLETION_THRESHOLD: f64 = 0.9;
const DIMINISHING_THRESHOLD: i32 = 500;

pub struct TokenBudgetDecision {
    pub action: BudgetAction,
    pub nudge_message: String,
    pub continuation_count: i32,
    pub pct: i32,
    pub turn_tokens: i32,
    pub budget: i32,
    pub diminishing: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub enum BudgetAction {
    Continue,
    Stop,
}

/// Evaluate token budget state when the LLM stops due to length.
/// Mirrors Go's CheckTokenBudget.
pub fn check_token_budget(
    tracker: &mut BudgetTracker,
    budget: i32,
    global_turn_tokens: i32,
) -> TokenBudgetDecision {
    if budget <= 0 {
        return TokenBudgetDecision {
            action: BudgetAction::Stop,
            nudge_message: String::new(),
            continuation_count: tracker.continuation_count,
            pct: 0,
            turn_tokens: global_turn_tokens,
            budget,
            diminishing: false,
        };
    }

    let pct = ((global_turn_tokens as f64 / budget as f64) * 100.0) as i32;
    let delta = global_turn_tokens - tracker.last_global_turn_tokens;

    let is_diminishing = tracker.continuation_count >= 3
        && delta < DIMINISHING_THRESHOLD
        && tracker.last_delta_tokens < DIMINISHING_THRESHOLD;

    if !is_diminishing && (global_turn_tokens as f64) < (budget as f64) * COMPLETION_THRESHOLD {
        tracker.continuation_count += 1;
        tracker.last_delta_tokens = delta;
        tracker.last_global_turn_tokens = global_turn_tokens;
        let nudge = format!(
            "Stopped at {}% of token target ({} / {}). Keep working — do not summarize.",
            pct, global_turn_tokens, budget
        );
        return TokenBudgetDecision {
            action: BudgetAction::Continue,
            nudge_message: nudge,
            continuation_count: tracker.continuation_count,
            pct,
            turn_tokens: global_turn_tokens,
            budget,
            diminishing: false,
        };
    }

    TokenBudgetDecision {
        action: BudgetAction::Stop,
        nudge_message: String::new(),
        continuation_count: tracker.continuation_count,
        pct,
        turn_tokens: global_turn_tokens,
        budget,
        diminishing: is_diminishing,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_budget_continue_below_threshold() {
        let mut tracker = BudgetTracker::default();
        let decision = check_token_budget(&mut tracker, 1000, 400);
        assert_eq!(decision.action, BudgetAction::Continue);
        assert!(decision.nudge_message.contains("40%"));
    }

    #[test]
    fn test_budget_stop_above_threshold() {
        let mut tracker = BudgetTracker::default();
        let decision = check_token_budget(&mut tracker, 1000, 950);
        assert_eq!(decision.action, BudgetAction::Stop);
    }

    #[test]
    fn test_budget_zero() {
        let mut tracker = BudgetTracker::default();
        let decision = check_token_budget(&mut tracker, 0, 100);
        assert_eq!(decision.action, BudgetAction::Stop);
    }
    #[test]
    fn test_budget_diminishing_returns() {
        let mut tracker = BudgetTracker {
            continuation_count: 3,
            last_delta_tokens: 400,
            last_global_turn_tokens: 0,
        };
        let decision = check_token_budget(&mut tracker, 10000, 400); // delta is 400
        assert_eq!(decision.action, BudgetAction::Stop);
        assert!(decision.diminishing);
    }

}
