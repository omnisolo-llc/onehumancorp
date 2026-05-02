use crate::agent::{Agent, AgentEvent, AgentRunConfig};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::fs;

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct RalphProgress {
    pub task_description: String,
    pub features: Vec<Feature>,
    pub current_feature_index: usize,
    pub is_complete: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Feature {
    pub name: String,
    pub status: String, // "pending", "in_progress", "completed"
}

/// The Ralph Loop for long-running asynchronous tasks spanning multiple context windows.
pub struct RalphLoop {
    pub agent: Arc<Agent>,
    pub config: AgentRunConfig,
    pub progress_file_path: String,
}

impl RalphLoop {
    pub fn new(agent: Arc<Agent>, config: AgentRunConfig, progress_file_path: &str) -> Self {
        Self {
            agent,
            config,
            progress_file_path: progress_file_path.to_string(),
        }
    }

    /// Run the full Ralph Loop
    pub async fn run(&self, initial_task: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Phase 1: Initializer
        let mut progress = self.initialize(initial_task).await?;
        
        // Phase 2: Coding Agent Loop
        while !progress.is_complete {
            if progress.current_feature_index >= progress.features.len() {
                progress.is_complete = true;
                self.save_progress(&progress).await?;
                break;
            }

            let feature = &progress.features[progress.current_feature_index];
            if feature.status == "completed" {
                progress.current_feature_index += 1;
                continue;
            }

            tracing::info!("Ralph Loop: Starting work on feature: {}", feature.name);
            
            // Execute the agent run for this specific feature
            let feature_prompt = format!(
                "You are continuing a long-running task.\nOverall Task: {}\nFeature to implement now: {}\nExecute steps to complete this feature, verify it, and then stop.",
                progress.task_description, feature.name
            );

            // We use a fresh config to keep the context window small (compaction/reset)
            let mut feature_config = self.config.clone();
            feature_config.user_instructions = feature_prompt.clone();
            
            let mut on_event = |event: AgentEvent| {
                // In production, we could stream these to a UI or log them
                if let AgentEvent::TaskError { error } = event {
                    tracing::error!("Ralph Loop Feature Error: {}", error);
                }
            };

            match self.agent.run(&feature_config, &feature_prompt, &mut on_event).await {
                Ok(result) => {
                    tracing::info!("Ralph Loop: Feature {} completed. Result: {}", feature.name, result);
                    progress.features[progress.current_feature_index].status = "completed".to_string();
                    progress.current_feature_index += 1;
                    self.save_progress(&progress).await?;
                }
                Err(e) => {
                    tracing::error!("Ralph Loop failed on feature {}: {}", feature.name, e);
                    // For demo purposes, we break on error. A robust system would retry or mark failed.
                    break;
                }
            }
        }

        tracing::info!("Ralph Loop completely finished.");
        Ok(())
    }

    async fn initialize(&self, task: &str) -> Result<RalphProgress, Box<dyn std::error::Error + Send + Sync>> {
        if let Ok(data) = fs::read_to_string(&self.progress_file_path).await {
            if let Ok(progress) = serde_json::from_str::<RalphProgress>(&data) {
                tracing::info!("Ralph Loop: Resuming from existing progress file.");
                return Ok(progress);
            }
        }

        tracing::info!("Ralph Loop: Initializing new progress file.");
        
        // Use the agent itself to break down the task
        let breakdown_prompt = format!("Break down the following task into 3 distinct, manageable features to implement sequentially. Respond strictly with a JSON array of strings representing the feature names. Task: {}", task);
        
        let mut on_event = |_| {};
        let result = self.agent.run(&self.config, &breakdown_prompt, &mut on_event).await?;
        
        // Simplistic extraction (assuming the model output valid JSON)
        let mut features = vec![];
        if let Ok(parsed) = serde_json::from_str::<Vec<String>>(&result) {
            for name in parsed {
                features.push(Feature { name, status: "pending".to_string() });
            }
        } else {
            // Fallback
            features.push(Feature { name: "Step 1".to_string(), status: "pending".to_string() });
            features.push(Feature { name: "Step 2".to_string(), status: "pending".to_string() });
        }

        let progress = RalphProgress {
            task_description: task.to_string(),
            features,
            current_feature_index: 0,
            is_complete: false,
        };

        self.save_progress(&progress).await?;
        Ok(progress)
    }

    async fn save_progress(&self, progress: &RalphProgress) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let json = serde_json::to_string_pretty(progress)?;
        fs::write(&self.progress_file_path, json).await?;
        Ok(())
    }
}
