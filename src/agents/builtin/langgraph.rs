use serde_json::Value;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Reducer trait to merge state updates into the main state
pub trait Reducer: Send + Sync {
    fn reduce(&self, state: &mut Value, update: Value);
}

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
pub type NodeFn = Arc<dyn Fn(Value) -> BoxFuture<'static, Result<Value, String>> + Send + Sync>;
pub type ConditionFn = Arc<dyn Fn(&Value) -> String + Send + Sync>;

pub struct StateGraph {
    nodes: HashMap<String, NodeFn>,
    edges: HashMap<String, String>,
    conditional_edges: HashMap<String, ConditionFn>,
    entry_point: Option<String>,
    reducer: Arc<dyn Reducer>,
}

pub const END: &str = "__END__";

impl StateGraph {
    pub fn new(reducer: Arc<dyn Reducer>) -> Self {
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
        F: Fn(Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Value, String>> + Send + 'static,
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
        C: Fn(&Value) -> String + Send + Sync + 'static,
    {
        self.conditional_edges.insert(from.to_string(), Arc::new(condition));
    }

    pub fn set_entry_point(&mut self, node: &str) {
        self.entry_point = Some(node.to_string());
    }

    pub async fn run(&self, initial_state: Value) -> Result<Value, String> {
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

/// A default reducer that merges JSON objects and appends to arrays
pub struct DefaultReducer;

impl Reducer for DefaultReducer {
    fn reduce(&self, state: &mut Value, update: Value) {
        if let (Value::Object(state_map), Value::Object(update_map)) = (state, update) {
            for (k, v) in update_map {
                match v {
                    Value::Array(mut new_arr) => {
                        if let Some(Value::Array(existing_arr)) = state_map.get_mut(&k) {
                            existing_arr.append(&mut new_arr);
                        } else {
                            state_map.insert(k, Value::Array(new_arr));
                        }
                    }
                    _ => {
                        state_map.insert(k, v);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_langgraph_mechanic() {
        let mut graph = StateGraph::new(Arc::new(DefaultReducer));

        // The llm_call node generates a response and possibly a tool call
        graph.add_node("llm_call", |state| async move {
            let messages = state.get("messages").unwrap().as_array().unwrap();
            let turn = messages.len();
            if turn < 2 {
                // Simulate returning a tool call
                Ok(json!({
                    "messages": [{"role": "assistant", "content": "", "tool_calls": [{"name": "search", "args": "weather"}]}],
                    "has_tool_calls": true
                }))
            } else {
                // Simulate returning final text
                Ok(json!({
                    "messages": [{"role": "assistant", "content": "The weather is sunny."}],
                    "has_tool_calls": false
                }))
            }
        });

        // The tool_node executes the tool
        graph.add_node("tool_node", |_state| async move {
            Ok(json!({
                "messages": [{"role": "tool", "content": "Sunny"}],
                "has_tool_calls": false
            }))
        });

        // Edge from tool_node back to llm_call
        graph.add_edge("tool_node", "llm_call");

        // Conditional edge from llm_call
        graph.add_conditional_edges("llm_call", |state| {
            if state.get("has_tool_calls").and_then(|v| v.as_bool()).unwrap_or(false) {
                "tool_node".to_string()
            } else {
                END.to_string()
            }
        });

        graph.set_entry_point("llm_call");

        let initial_state = json!({
            "messages": [{"role": "user", "content": "What is the weather?"}],
            "has_tool_calls": false
        });

        let final_state = graph.run(initial_state).await.unwrap();

        let messages = final_state.get("messages").unwrap().as_array().unwrap();
        assert_eq!(messages.len(), 4);
        assert_eq!(messages[0]["role"], "user");
        assert_eq!(messages[1]["role"], "assistant");
        assert_eq!(messages[2]["role"], "tool");
        assert_eq!(messages[3]["role"], "assistant");
        assert_eq!(messages[3]["content"], "The weather is sunny.");
    }
}
