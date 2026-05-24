use std::sync::Arc;
use crate::agent::{Agent, AgentRunConfig};
use ohc_builtin_agent_core::types::{ChatRequest, Message, Role, ChatResponse, Usage};
use futures::future::join_all;
use tokio::sync::Mutex;
use async_trait::async_trait;

#[async_trait]
pub trait RufloLlmClient: Send + Sync {
    async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>>;
}

pub struct SwarmAgent {
    pub name: String,
    pub role_description: String,
    pub llm: Arc<dyn RufloLlmClient>,
}

impl SwarmAgent {
    pub async fn prompt(&self, task: &str, context: &str) -> Result<String, String> {
        let system_prompt = format!("You are {}. Role: {}. {}", self.name, self.role_description, context);
        let req = ChatRequest {
            model: "default".to_string(),
            system: system_prompt,
            messages: vec![Message::user(task)],
            tools: vec![],
            max_tokens: 4000,
            temperature: 0.2,
        };

        match self.llm.chat(req).await {
            Ok(resp) => Ok(resp.message.content),
            Err(e) => Err(format!("LLM Error: {}", e)),
        }
    }
}

pub enum SwarmTopology {
    /// A leader agent delegates specific parts of a task to workers and synthesizes the results.
    Hierarchical {
        leader: SwarmAgent,
        workers: Vec<SwarmAgent>,
    },
    /// Agents communicate in a peer-to-peer fashion, broadcasting insights to a shared context pool before producing a final output.
    Mesh {
        agents: Vec<SwarmAgent>,
        rounds: usize,
    },
    /// Agents independently propose solutions to a task, followed by a consensus phase where they evaluate proposals to reach a majority/unanimous decision.
    AdaptiveWithConsensus {
        agents: Vec<SwarmAgent>,
    },
}

pub struct RufloSwarm {
    pub topology: SwarmTopology,
}

impl RufloSwarm {
    pub async fn execute(&self, task: &str) -> Result<String, String> {
        match &self.topology {
            SwarmTopology::Hierarchical { leader, workers } => {
                // 1. Leader breaks down task (simulate by passing the task to all workers directly in this simple model,
                // but ideally the leader would emit sub-tasks. We'll just ask workers to solve their part).
                let mut futures = Vec::new();
                for worker in workers {
                    let w_task = format!("Subtask derived from main task: {}", task);
                    let w_fut = async move {
                        worker.prompt(&w_task, "Focus on your specific domain.").await
                    };
                    futures.push(w_fut);
                }

                let results = join_all(futures).await;
                let mut valid_results = Vec::new();
                for r in results {
                    valid_results.push(r?);
                }

                // 2. Leader synthesizes
                let synthesis_context = format!("Worker results:\n{}", valid_results.join("\n\n"));
                leader.prompt(task, &format!("Synthesize the following worker outputs into a final cohesive response: {}", synthesis_context)).await
            }
            SwarmTopology::Mesh { agents, rounds } => {
                let shared_pool = Arc::new(Mutex::new(Vec::new()));

                for _ in 0..*rounds {
                    let mut futures = Vec::new();
                    for agent in agents {
                        let pool = shared_pool.clone();
                        let task_clone = task.to_string();
                        let fut = async move {
                            let current_context = {
                                let lock = pool.lock().await;
                                lock.join("\n")
                            };
                            let prompt_ctx = format!("Current shared knowledge pool:\n{}", current_context);
                            let output = agent.prompt(&task_clone, &prompt_ctx).await?;
                            let mut lock = pool.lock().await;
                            lock.push(format!("{}: {}", agent.name, output));
                            Ok::<(), String>(())
                        };
                        futures.push(fut);
                    }
                    let res = join_all(futures).await;
                    for r in res {
                        r?;
                    }
                }

                // Gather final pool as output
                let final_pool = shared_pool.lock().await;
                Ok(final_pool.join("\n\n"))
            }
            SwarmTopology::AdaptiveWithConsensus { agents } => {
                // 1. Propose
                let mut futures = Vec::new();
                for agent in agents {
                    let task_clone = task.to_string();
                    let fut = async move {
                        agent.prompt(&task_clone, "Propose a complete solution.").await
                    };
                    futures.push(fut);
                }
                let proposals_res = join_all(futures).await;
                let mut proposals = Vec::new();
                for (i, res) in proposals_res.into_iter().enumerate() {
                    proposals.push(format!("Proposal {}: {}", i + 1, res?));
                }

                // 2. Consensus Evaluation
                let consensus_context = format!("Evaluate these proposals and agree on the best one or synthesize a new consensus:\n{}", proposals.join("\n\n"));

                let mut consensus_futures = Vec::new();
                for agent in agents {
                    let task_clone = task.to_string();
                    let ctx_clone = consensus_context.clone();
                    let fut = async move {
                        agent.prompt(&task_clone, &ctx_clone).await
                    };
                    consensus_futures.push(fut);
                }

                let consensus_results = join_all(consensus_futures).await;
                let mut valid_consensus = Vec::new();
                for c in consensus_results {
                    valid_consensus.push(c?);
                }

                // Final string: we could do another round of voting, or just return the aggregated consensus thoughts.
                Ok(format!("Proposals Phase:\n{}\n\nConsensus Phase:\n{}", proposals.join("\n"), valid_consensus.join("\n\n")))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockRufloClient {
        resp_text: String,
    }

    #[async_trait::async_trait]
    impl RufloLlmClient for MockRufloClient {
        async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Ok(ChatResponse {
                message: Message::assistant(self.resp_text.clone()),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            })
        }
    }

    #[tokio::test]
    async fn test_ruflo_hierarchical_topology() {
        let leader = SwarmAgent {
            name: "Leader".to_string(),
            role_description: "Synthesize".to_string(),
            llm: Arc::new(MockRufloClient { resp_text: "Final synthesized output".to_string() }),
        };

        let workers = vec![
            SwarmAgent {
                name: "Worker1".to_string(),
                role_description: "Part 1".to_string(),
                llm: Arc::new(MockRufloClient { resp_text: "Worker 1 output".to_string() }),
            },
            SwarmAgent {
                name: "Worker2".to_string(),
                role_description: "Part 2".to_string(),
                llm: Arc::new(MockRufloClient { resp_text: "Worker 2 output".to_string() }),
            },
        ];

        let swarm = RufloSwarm {
            topology: SwarmTopology::Hierarchical { leader, workers },
        };

        let result = swarm.execute("Solve a complex problem").await.unwrap();
        assert_eq!(result, "Final synthesized output");
    }

    #[tokio::test]
    async fn test_ruflo_mesh_topology() {
        let agents = vec![
            SwarmAgent {
                name: "Peer1".to_string(),
                role_description: "Peer".to_string(),
                llm: Arc::new(MockRufloClient { resp_text: "Insight A".to_string() }),
            },
            SwarmAgent {
                name: "Peer2".to_string(),
                role_description: "Peer".to_string(),
                llm: Arc::new(MockRufloClient { resp_text: "Insight B".to_string() }),
            },
        ];

        let swarm = RufloSwarm {
            topology: SwarmTopology::Mesh { agents, rounds: 1 },
        };

        let result = swarm.execute("Discuss problem").await.unwrap();
        // Since order of completion in join_all for Mesh is deterministic based on Vec order,
        // it should be Peer1 then Peer2.
        assert!(result.contains("Peer1: Insight A"));
        assert!(result.contains("Peer2: Insight B"));
    }

    #[tokio::test]
    async fn test_ruflo_adaptive_consensus_topology() {
        let agents = vec![
            SwarmAgent {
                name: "Agent1".to_string(),
                role_description: "Voter".to_string(),
                llm: Arc::new(MockRufloClient { resp_text: "I vote for Proposal X".to_string() }),
            },
        ];

        let swarm = RufloSwarm {
            topology: SwarmTopology::AdaptiveWithConsensus { agents },
        };

        let result = swarm.execute("Reach a decision").await.unwrap();
        assert!(result.contains("Proposals Phase:\nProposal 1: I vote for Proposal X"));
        assert!(result.contains("Consensus Phase:\nI vote for Proposal X"));
    }
}
