use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentTemplate {
    pub id: String,
    pub role: String,
    pub mission: String,
    pub handoff_to: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgentTurn {
    pub agent_id: String,
    pub role: String,
    pub contribution: String,
    pub handoff_to: Vec<String>,
    pub confidence: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgentWorkspaceTranscript {
    pub task: String,
    pub turns: Vec<AgentTurn>,
    pub final_brief: String,
}

#[async_trait]
pub trait AgentLlm: Send + Sync {
    async fn reason(&self, prompt: &str) -> Result<String, String>;
}

#[async_trait]
impl AgentLlm for crate::minimax::MinimaxClient {
    async fn reason(&self, prompt: &str) -> Result<String, String> {
        self.reason(prompt).await
    }
}

pub fn agent_templates() -> Vec<AgentTemplate> {
    vec![
        AgentTemplate {
            id: "strategy_lead".to_string(),
            role: "Strategy Lead".to_string(),
            mission: "Break the task into a practical plan, define success criteria, and name the needed workstreams.".to_string(),
            handoff_to: vec!["market_researcher".to_string(), "offer_designer".to_string()],
        },
        AgentTemplate {
            id: "market_researcher".to_string(),
            role: "Market Researcher".to_string(),
            mission: "Identify audience, competitor pressure, demand signals, and risks that should shape the plan.".to_string(),
            handoff_to: vec!["operations_planner".to_string()],
        },
        AgentTemplate {
            id: "offer_designer".to_string(),
            role: "Offer Designer".to_string(),
            mission: "Turn the strategy into concrete positioning, package, pricing, and customer-facing copy.".to_string(),
            handoff_to: vec!["operations_planner".to_string()],
        },
        AgentTemplate {
            id: "operations_planner".to_string(),
            role: "Operations Planner".to_string(),
            mission: "Convert prior work into execution steps, owners, systems, dependencies, and launch checks.".to_string(),
            handoff_to: vec!["quality_reviewer".to_string()],
        },
        AgentTemplate {
            id: "quality_reviewer".to_string(),
            role: "Quality Reviewer".to_string(),
            mission: "Review the previous agents, resolve conflicts, and produce the final coordinated brief.".to_string(),
            handoff_to: vec![],
        },
    ]
}

pub struct MinimaxAgentWorkspace<L> {
    llm: L,
    templates: Vec<AgentTemplate>,
    turn_delay: std::time::Duration,
}

impl<L> MinimaxAgentWorkspace<L>
where
    L: AgentLlm,
{
    pub fn new(llm: L, templates: Vec<AgentTemplate>) -> Self {
        Self {
            llm,
            templates,
            turn_delay: std::time::Duration::from_millis(0),
        }
    }

    pub fn with_turn_delay(mut self, turn_delay: std::time::Duration) -> Self {
        self.turn_delay = turn_delay;
        self
    }

    pub async fn run(&self, task: &str) -> Result<AgentWorkspaceTranscript, String> {
        if self.templates.len() < 5 {
            return Err("agent workspace requires at least five agent templates".to_string());
        }

        let mut turns = Vec::new();
        for template in &self.templates {
            let prompt = agent_prompt(template, task, &turns)?;
            let raw = self.llm.reason(&prompt).await?;
            let turn = match parse_and_validate_agent_turn(&raw, template, &turns) {
                Ok(turn) => turn,
                Err(parse_err) => {
                    let repair_prompt = repair_prompt(template, &raw, &parse_err)?;
                    let repaired = self.llm.reason(&repair_prompt).await?;
                    parse_and_validate_agent_turn(&repaired, template, &turns)?
                }
            };
            turns.push(turn);
            if !self.turn_delay.is_zero() {
                tokio::time::sleep(self.turn_delay).await;
            }
        }

        let final_brief = final_brief_from_turns(&turns);

        Ok(AgentWorkspaceTranscript {
            task: task.to_string(),
            turns,
            final_brief,
        })
    }
}

pub fn minimax_agent_workspace_from_env() -> Result<MinimaxAgentWorkspace<crate::minimax::MinimaxClient>, String> {
    let api_key = std::env::var("MINIMAX_API_KEY")
        .map_err(|_| "MINIMAX_API_KEY is required for the Minimax agent workspace".to_string())?;
    if api_key.trim().is_empty() {
        return Err("MINIMAX_API_KEY is empty".to_string());
    }
    validate_minimax_api_key(&api_key)?;

    Ok(MinimaxAgentWorkspace::new(
        crate::minimax::MinimaxClient::new(api_key),
        agent_templates(),
    ))
}

fn validate_minimax_api_key(api_key: &str) -> Result<(), String> {
    let normalized = api_key.trim().to_ascii_lowercase();
    if normalized.is_empty()
        || normalized.contains("fake")
        || normalized.contains("mock")
        || normalized.contains("dummy")
    {
        return Err("MINIMAX_API_KEY must be a real MiniMax API key".to_string());
    }
    Ok(())
}

fn agent_prompt(
    template: &AgentTemplate,
    task: &str,
    prior_turns: &[AgentTurn],
) -> Result<String, String> {
    let prior_json = serde_json::to_string(prior_turns).map_err(|err| err.to_string())?;
    let handoff_json = serde_json::to_string(&template.handoff_to).map_err(|err| err.to_string())?;

    Ok(format!(
        "You are an OHC agent in a five-agent workspace. Agent id: {agent_id}. Role: {role}. Mission: {mission}. Task: {task}. Prior agent transcript JSON: {prior_json}. You must collaborate with the prior agents and hand off to exactly these next agents: {handoff_json} (do not include any other agents or omit any, ensure the list is exactly as provided). Return strict JSON only with keys agent_id, role, contribution, handoff_to, confidence. contribution must mention at least one prior agent when prior transcript is non-empty, and must be concise but specific.",
        agent_id = template.id,
        role = template.role,
        mission = template.mission,
        task = task,
        prior_json = prior_json,
        handoff_json = handoff_json,
    ))
}

fn final_brief_from_turns(turns: &[AgentTurn]) -> String {
    let Some(last) = turns.last() else {
        return String::new();
    };
    if last.contribution.len() >= 80 {
        return last.contribution.clone();
    }

    turns
        .iter()
        .map(|turn| format!("{}: {}", turn.role, turn.contribution))
        .collect::<Vec<_>>()
        .join("\n")
}

fn repair_prompt(
    template: &AgentTemplate,
    raw_response: &str,
    parse_error: &str,
) -> Result<String, String> {
    let handoff_json = serde_json::to_string(&template.handoff_to).map_err(|err| err.to_string())?;
    Ok(format!(
        "Convert this agent response into strict JSON only. Do not add markdown. Required schema: {{\"agent_id\":\"{agent_id}\",\"role\":\"{role}\",\"contribution\":\"short specific text\",\"handoff_to\":{handoff_json},\"confidence\":0.0}}. The previous parse error was: {parse_error}. Raw response: {raw_response}",
        agent_id = template.id,
        role = template.role,
        handoff_json = handoff_json,
        parse_error = parse_error,
        raw_response = raw_response,
    ))
}

pub fn parse_agent_turn(raw: &str) -> Result<AgentTurn, String> {
    let value = parse_json_object(raw)?;
    let turn: AgentTurn = serde_json::from_value(value)
        .map_err(|err| format!("failed to parse agent turn JSON: {err}"))?;

    if turn.agent_id.trim().is_empty() {
        return Err("agent turn missing agent_id".to_string());
    }
    if turn.role.trim().is_empty() {
        return Err("agent turn missing role".to_string());
    }
    if !has_substantive_contribution(&turn.contribution) {
        return Err("agent turn contribution is empty or placeholder text".to_string());
    }
    if !(0.0..=1.0).contains(&turn.confidence) {
        return Err("agent confidence must be between 0 and 1".to_string());
    }

    Ok(turn)
}

fn parse_and_validate_agent_turn(
    raw: &str,
    template: &AgentTemplate,
    prior_turns: &[AgentTurn],
) -> Result<AgentTurn, String> {
    let turn = parse_agent_turn(raw)?;
    validate_agent_turn_contract(&turn, template, prior_turns)?;
    Ok(turn)
}

fn validate_agent_turn_contract(
    turn: &AgentTurn,
    template: &AgentTemplate,
    prior_turns: &[AgentTurn],
) -> Result<(), String> {
    if turn.agent_id != template.id {
        return Err(format!(
            "agent '{}' returned mismatched agent_id '{}'",
            template.id, turn.agent_id
        ));
    }
    if turn.role != template.role {
        return Err(format!(
            "agent '{}' returned mismatched role '{}'",
            template.id, turn.role
        ));
    }
    if turn.handoff_to != template.handoff_to {
        return Err(format!(
            "agent '{}' returned handoff {:?}, expected {:?}",
            template.id, turn.handoff_to, template.handoff_to
        ));
    }
    if !prior_turns.is_empty() && !references_prior_agent(&turn.contribution, prior_turns) {
        return Err(format!(
            "agent '{}' contribution must reference prior workspace context",
            template.id
        ));
    }
    Ok(())
}

fn references_prior_agent(contribution: &str, prior_turns: &[AgentTurn]) -> bool {
    let contribution = contribution.to_ascii_lowercase();
    if ["prior", "previous", "earlier", "transcript", "handoff"]
        .iter()
        .any(|marker| contribution.contains(marker))
    {
        return true;
    }

    prior_turns.iter().any(|turn| {
        contribution.contains(&turn.agent_id.to_ascii_lowercase())
            || contribution.contains(&turn.role.to_ascii_lowercase())
            || turn
                .role
                .split_whitespace()
                .next()
                .map(|word| contribution.contains(&word.to_ascii_lowercase()))
                .unwrap_or(false)
    })
}

fn has_substantive_contribution(contribution: &str) -> bool {
    let trimmed = contribution.trim();
    trimmed.len() >= 24
        && trimmed
            .chars()
            .any(|ch| ch.is_ascii_alphabetic() || ch.is_alphabetic())
        && !matches!(trimmed, "..." | "…" | "n/a" | "N/A")
}

fn parse_json_object(raw: &str) -> Result<Value, String> {
    match serde_json::from_str::<Value>(raw) {
        Ok(value) if value.is_object() => Ok(value),
        Ok(_) => Err("agent response was JSON but not an object".to_string()),
        Err(_) => {
            for (idx, ch) in raw.char_indices() {
                if ch != '{' {
                    continue;
                }
                let mut stream = serde_json::Deserializer::from_str(&raw[idx..]).into_iter::<Value>();
                if let Some(Ok(value)) = stream.next() {
                    if value.is_object() {
                        return Ok(value);
                    }
                }
            }
            Err("failed to extract agent JSON object".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;
    use std::sync::Mutex;

    struct ScriptedLlm {
        responses: Mutex<VecDeque<Result<String, String>>>,
    }

    impl ScriptedLlm {
        fn new(responses: Vec<Result<String, String>>) -> Self {
            Self {
                responses: Mutex::new(responses.into()),
            }
        }
    }

    #[async_trait]
    impl AgentLlm for ScriptedLlm {
        async fn reason(&self, _prompt: &str) -> Result<String, String> {
            self.responses
                .lock()
                .unwrap()
                .pop_front()
                .expect("scripted response should exist")
        }
    }

    #[test]
    fn templates_define_five_collaborating_agents() {
        let templates = agent_templates();

        assert_eq!(templates.len(), 5);
        assert_eq!(templates[0].id, "strategy_lead");
        assert_eq!(templates[4].id, "quality_reviewer");
        assert!(templates.iter().any(|template| {
            template.role == "Operations Planner"
                && template.handoff_to == vec!["quality_reviewer".to_string()]
        }));
    }

    #[test]
    fn parses_strict_agent_turn_json() {
        let turn = parse_agent_turn(
            r#"{"agent_id":"strategy_lead","role":"Strategy Lead","contribution":"Use three coordinated workstreams.","handoff_to":["market_researcher"],"confidence":0.86}"#,
        )
        .unwrap();

        assert_eq!(turn.agent_id, "strategy_lead");
        assert_eq!(turn.handoff_to, vec!["market_researcher"]);
    }

    #[test]
    fn parses_first_json_object_when_model_adds_trailing_text() {
        let turn = parse_agent_turn(
            r#"{"agent_id":"strategy_lead","role":"Strategy Lead","contribution":"Use three coordinated workstreams for launch planning.","handoff_to":["market_researcher"],"confidence":0.86}
            Explanation: I used the requested schema."#,
        )
        .unwrap();

        assert_eq!(turn.agent_id, "strategy_lead");
    }

    #[test]
    fn rejects_placeholder_agent_contribution() {
        let err = parse_agent_turn(
            r#"{"agent_id":"operations_planner","role":"Operations Planner","contribution":"...","handoff_to":["quality_reviewer"],"confidence":0.86}"#,
        )
        .unwrap_err();

        assert!(err.contains("placeholder"));
    }

    #[test]
    fn rejects_fake_minimax_api_key_for_workspace_env() {
        let err = validate_minimax_api_key("fake-key").unwrap_err();

        assert!(err.contains("real MiniMax"));
    }

    #[test]
    fn validates_exact_handoff_contract() {
        let template = AgentTemplate {
            id: "strategy_lead".to_string(),
            role: "Strategy Lead".to_string(),
            mission: "Plan".to_string(),
            handoff_to: vec!["market_researcher".to_string(), "offer_designer".to_string()],
        };
        let err = parse_and_validate_agent_turn(
            r#"{"agent_id":"strategy_lead","role":"Strategy Lead","contribution":"Define concrete launch workstreams for this project.","handoff_to":["market_researcher"],"confidence":0.86}"#,
            &template,
            &[],
        )
        .unwrap_err();

        assert!(err.contains("expected"));
    }

    #[test]
    fn validates_prior_agent_reference_after_first_turn() {
        let template = AgentTemplate {
            id: "market_researcher".to_string(),
            role: "Market Researcher".to_string(),
            mission: "Research".to_string(),
            handoff_to: vec!["operations_planner".to_string()],
        };
        let prior = vec![AgentTurn {
            agent_id: "strategy_lead".to_string(),
            role: "Strategy Lead".to_string(),
            contribution: "Define launch workstreams.".to_string(),
            handoff_to: vec!["market_researcher".to_string()],
            confidence: 0.8,
        }];
        let err = parse_and_validate_agent_turn(
            r#"{"agent_id":"market_researcher","role":"Market Researcher","contribution":"Audience demand is strongest around weekday commuters.","handoff_to":["operations_planner"],"confidence":0.86}"#,
            &template,
            &prior,
        )
        .unwrap_err();

        assert!(err.contains("prior workspace"));
    }

    #[test]
    fn final_brief_uses_full_transcript_when_reviewer_is_terse() {
        let turns = vec![
            AgentTurn {
                agent_id: "strategy_lead".to_string(),
                role: "Strategy Lead".to_string(),
                contribution: "Define subscription launch workstreams.".to_string(),
                handoff_to: vec![],
                confidence: 0.8,
            },
            AgentTurn {
                agent_id: "quality_reviewer".to_string(),
                role: "Quality Reviewer".to_string(),
                contribution: "Approved.".to_string(),
                handoff_to: vec![],
                confidence: 0.8,
            },
        ];

        let brief = final_brief_from_turns(&turns);

        assert!(brief.contains("Strategy Lead"));
        assert!(brief.contains("Quality Reviewer"));
    }

    #[tokio::test]
    async fn workspace_repairs_non_strict_agent_json_before_accepting_turn() {
        let templates = agent_templates();
        let llm = ScriptedLlm::new(vec![
            Ok("agent_id: strategy_lead, contribution: plan".to_string()),
            Ok(r#"{"agent_id":"strategy_lead","role":"Strategy Lead","contribution":"Plan accepted after repair with launch workstreams defined.","handoff_to":["market_researcher","offer_designer"],"confidence":0.8}"#.to_string()),
            Ok(r#"{"agent_id":"market_researcher","role":"Market Researcher","contribution":"Research builds on the Strategy Lead handoff with audience and demand signals.","handoff_to":["operations_planner"],"confidence":0.8}"#.to_string()),
            Ok(r#"{"agent_id":"offer_designer","role":"Offer Designer","contribution":"Offer design uses the prior research to shape tiers and copy.","handoff_to":["operations_planner"],"confidence":0.8}"#.to_string()),
            Ok(r#"{"agent_id":"operations_planner","role":"Operations Planner","contribution":"Operations planning uses the previous offer and research to define launch steps.","handoff_to":["quality_reviewer"],"confidence":0.8}"#.to_string()),
            Ok(r#"{"agent_id":"quality_reviewer","role":"Quality Reviewer","contribution":"Final review resolves the prior agent contributions into launch steps.","handoff_to":[],"confidence":0.8}"#.to_string()),
        ]);
        let workspace = MinimaxAgentWorkspace::new(llm, templates);

        let transcript = workspace.run("Launch a bakery subscription").await.unwrap();

        assert_eq!(transcript.turns.len(), 5);
        assert_eq!(
            transcript.turns[0].contribution,
            "Plan accepted after repair with launch workstreams defined."
        );
    }

    #[tokio::test]
    async fn live_minimax_five_agent_workspace_collaborates() {
        // Skip this test in normal runs to make the test suite hermetic
        if std::env::var("OHC_RUN_LIVE_MINIMAX_TESTS").is_err() {
            return;
        }
        let maybe_workspace = minimax_agent_workspace_from_env();
        if maybe_workspace.is_err() {
            tracing::info!("Skipping live_minimax_five_agent_workspace_collaborates: MINIMAX_API_KEY not set");
            return;
        }
        let workspace = maybe_workspace.unwrap()

            .with_turn_delay(std::time::Duration::from_millis(
                std::env::var("OHC_MINIMAX_SWARM_TURN_DELAY_MS")
                    .ok()
                    .and_then(|value| value.parse::<u64>().ok())
                    .unwrap_or(0),
            ));
        let transcript = workspace
            .run("Create a launch plan for a neighborhood bakery adding subscription pastry boxes.")
            .await
            .expect("live Minimax agent workspace should complete");
        tracing::info!("{}", serde_json::to_string_pretty(&transcript).unwrap());

        assert_eq!(transcript.turns.len(), 5);
        assert!(transcript
            .turns
            .iter()
            .all(|turn| has_substantive_contribution(&turn.contribution)));
        for (turn, template) in transcript.turns.iter().zip(agent_templates()) {
            assert_eq!(turn.handoff_to, template.handoff_to);
        }
        for idx in 1..transcript.turns.len() {
            assert!(references_prior_agent(
                &transcript.turns[idx].contribution,
                &transcript.turns[..idx],
            ));
        }
        assert!(transcript.final_brief.len() > 80);
    }
}
