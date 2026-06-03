/// Token budget tracker — mirrors Go BudgetTracker.
#[derive(Debug, Default)]
pub struct BudgetTracker {
    pub continuation_count: i32,
    pub last_delta_tokens: i32,
    pub last_global_turn_tokens: i32,
    pub ema_delta_tokens: f64,
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
    pub anomaly_detected: bool,
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
            anomaly_detected: false,
        };
    }

    let pct = ((global_turn_tokens as f64 / budget as f64) * 100.0) as i32;
    let delta = global_turn_tokens - tracker.last_global_turn_tokens;

    // Real-time Anomaly Detection Layer
    let mut is_anomaly = false;
    if tracker.continuation_count >= 1 && tracker.ema_delta_tokens > 0.0 {
        if (delta as f64) > tracker.ema_delta_tokens * 3.0 && delta > 1000 {
            is_anomaly = true;
        }
    }

    if is_anomaly {
        return TokenBudgetDecision {
            action: BudgetAction::Stop,
            nudge_message: "Real-time cost anomaly detected: massive token usage spike. Stopping to prevent runaway costs.".to_string(),
            continuation_count: tracker.continuation_count,
            pct,
            turn_tokens: global_turn_tokens,
            budget,
            diminishing: false,
            anomaly_detected: true,
        };
    }

    // Update Exponential Moving Average (EMA) of token deltas (only if not an anomaly)
    if tracker.ema_delta_tokens == 0.0 {
        tracker.ema_delta_tokens = delta as f64;
    } else {
        tracker.ema_delta_tokens = 0.5 * (delta as f64) + 0.5 * tracker.ema_delta_tokens;
    }

    let is_diminishing = tracker.continuation_count >= 3
        && delta < DIMINISHING_THRESHOLD
        && tracker.last_delta_tokens < DIMINISHING_THRESHOLD;

    if !is_diminishing
        && (global_turn_tokens as f64) < (budget as f64) * COMPLETION_THRESHOLD
    {
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
            anomaly_detected: false,
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
        anomaly_detected: false,
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
    fn test_budget_anomaly_detection() {
        let mut tracker = BudgetTracker::default();
        let budget = 10000;

        // Turn 1: Normal usage (400 tokens)
        let decision1 = check_token_budget(&mut tracker, budget, 400);
        assert_eq!(decision1.action, BudgetAction::Continue);
        assert_eq!(decision1.anomaly_detected, false);

        // Turn 2: Normal usage (another 450 tokens, global = 850)
        let decision2 = check_token_budget(&mut tracker, budget, 850);
        assert_eq!(decision2.action, BudgetAction::Continue);
        assert_eq!(decision2.anomaly_detected, false);

        // Turn 3: MASSIVE spike indicating a runaway loop or prompt injection (5000 tokens, global = 5850)
        // Previous delta was 450, EMA is around 425. New delta is 5000. 5000 > 425 * 3 (1275) AND 5000 > 1000.
        // This should trigger anomaly detection and stop.
        let decision3 = check_token_budget(&mut tracker, budget, 5850);
        assert_eq!(decision3.action, BudgetAction::Stop);
        assert_eq!(decision3.anomaly_detected, true);
        assert!(decision3.nudge_message.contains("anomaly detected"));
    }
}
