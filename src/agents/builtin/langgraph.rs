use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Master Catalog: Models the harness as an explicit state graph. Mechanically: uses `llm_call` and `tool_node` connected by conditional edges. State flows as typed dictionaries with reducer functions.

/// Reducer trait to merge state updates into the main typed state dictionary
pub trait Reducer<S, U>: Send + Sync {
    fn reduce(&self, state: &mut S, update: U);
}

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

pub struct StateGraph<S, U> {
    nodes: HashMap<String, Arc<dyn Fn(S) -> BoxFuture<'static, Result<U, String>> + Send + Sync>>,
    edges: HashMap<String, String>,
    conditional_edges: HashMap<String, Arc<dyn Fn(&S) -> String + Send + Sync>>,
    entry_point: Option<String>,
    reducer: Arc<dyn Reducer<S, U>>,
}

pub const END: &str = "__END__";

impl<S: Clone + Send + Sync + 'static, U: Send + Sync + 'static> StateGraph<S, U> {
    pub fn new(reducer: Arc<dyn Reducer<S, U>>) -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            conditional_edges: HashMap::new(),
            entry_point: None,
            reducer,
        }
    }

    pub fn add_node<F, Fut>(&mut self, name: &str, node_fn: F)
    where
        F: Fn(S) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<U, String>> + Send + 'static,
    {
        self.nodes.insert(
            name.to_string(),
            Arc::new(move |val| Box::pin(node_fn(val))),
        );
    }

    pub fn add_edge(&mut self, from: &str, to: &str) {
        self.edges.insert(from.to_string(), to.to_string());
    }

    pub fn add_conditional_edges<C>(&mut self, from: &str, condition: C)
    where
        C: Fn(&S) -> String + Send + Sync + 'static,
    {
        self.conditional_edges.insert(from.to_string(), Arc::new(condition));
    }

    pub fn set_entry_point(&mut self, node: &str) {
        self.entry_point = Some(node.to_string());
    }

    pub async fn run(&self, initial_state: S) -> Result<S, String> {
        let mut current_state = initial_state;
        let mut current_node = self.entry_point.clone().ok_or("Entry point not set")?;

        let mut iterations = 0;
        let max_iterations = 100;

        while current_node != END {
            if iterations >= max_iterations {
                return Err("Max iterations reached".to_string());
            }
            iterations += 1;

            let node_fn = self.nodes.get(&current_node).ok_or_else(|| format!("Node not found: {}", current_node))?;

            let update = node_fn(current_state.clone()).await?;
            self.reducer.reduce(&mut current_state, update);

            if let Some(cond_fn) = self.conditional_edges.get(&current_node) {
                current_node = cond_fn(&current_state);
            } else if let Some(next_node) = self.edges.get(&current_node) {
                current_node = next_node.clone();
            } else {
                current_node = END.to_string();
            }
        }

        Ok(current_state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug)]
    struct Message {
        role: String,
        content: String,
    }

    #[derive(Clone, Debug)]
    struct ToolCall {
        name: String,
        args: String,
    }

    #[derive(Clone, Debug)]
    struct AgentState {
        messages: Vec<Message>,
        tool_calls: Vec<ToolCall>,
        has_tool_calls: bool,
    }

    #[derive(Clone, Debug)]
    enum StateUpdate {
        AddMessage(Message),
        AddToolCalls(Vec<ToolCall>),
        ClearToolCalls,
    }

    struct AgentReducer;

    impl Reducer<AgentState, StateUpdate> for AgentReducer {
        fn reduce(&self, state: &mut AgentState, update: StateUpdate) {
            match update {
                StateUpdate::AddMessage(msg) => {
                    state.messages.push(msg);
                }
                StateUpdate::AddToolCalls(calls) => {
                    state.tool_calls.extend(calls);
                    state.has_tool_calls = true;
                }
                StateUpdate::ClearToolCalls => {
                    state.tool_calls.clear();
                    state.has_tool_calls = false;
                }
            }
        }
    }

    // A more complex state update that can handle multiple updates at once
    #[derive(Clone, Debug)]
    struct BatchStateUpdate {
        updates: Vec<StateUpdate>,
    }

    struct BatchAgentReducer;

    impl Reducer<AgentState, BatchStateUpdate> for BatchAgentReducer {
        fn reduce(&self, state: &mut AgentState, update: BatchStateUpdate) {
            for single_update in update.updates {
                match single_update {
                    StateUpdate::AddMessage(msg) => {
                        state.messages.push(msg);
                    }
                    StateUpdate::AddToolCalls(calls) => {
                        state.tool_calls.extend(calls);
                        state.has_tool_calls = !state.tool_calls.is_empty();
                    }
                    StateUpdate::ClearToolCalls => {
                        state.tool_calls.clear();
                        state.has_tool_calls = false;
                    }
                }
            }
        }
    }

    #[tokio::test]
    async fn test_langgraph_mechanic_typed_dictionaries() {
        let mut graph = StateGraph::new(Arc::new(BatchAgentReducer));

        // The llm_call node generates a response and possibly a tool call
        graph.add_node("llm_call", |state| async move {
            let turn = state.messages.len();
            if turn < 2 {
                // Simulate returning a tool call
                Ok(BatchStateUpdate {
                    updates: vec![
                        StateUpdate::AddMessage(Message { role: "assistant".to_string(), content: "".to_string() }),
                        StateUpdate::AddToolCalls(vec![ToolCall { name: "search".to_string(), args: "weather".to_string() }]),
                    ]
                })
            } else {
                // Simulate returning final text
                Ok(BatchStateUpdate {
                    updates: vec![
                        StateUpdate::AddMessage(Message { role: "assistant".to_string(), content: "The weather is sunny.".to_string() }),
                    ]
                })
            }
        });

        // The tool_node executes the tool
        graph.add_node("tool_node", |_state| async move {
            Ok(BatchStateUpdate {
                updates: vec![
                    StateUpdate::AddMessage(Message { role: "tool".to_string(), content: "Sunny".to_string() }),
                    StateUpdate::ClearToolCalls,
                ]
            })
        });

        // Edge from tool_node back to llm_call
        graph.add_edge("tool_node", "llm_call");

        // Conditional edge from llm_call
        graph.add_conditional_edges("llm_call", |state| {
            if state.has_tool_calls {
                "tool_node".to_string()
            } else {
                END.to_string()
            }
        });

        graph.set_entry_point("llm_call");

        let initial_state = AgentState {
            messages: vec![Message { role: "user".to_string(), content: "What is the weather?".to_string() }],
            tool_calls: vec![],
            has_tool_calls: false,
        };

        let final_state = graph.run(initial_state).await.unwrap();

        assert_eq!(final_state.messages.len(), 4);
        assert_eq!(final_state.messages[0].role, "user");
        assert_eq!(final_state.messages[1].role, "assistant");
        assert_eq!(final_state.messages[2].role, "tool");
        assert_eq!(final_state.messages[3].role, "assistant");
        assert_eq!(final_state.messages[3].content, "The weather is sunny.");
    }
}
