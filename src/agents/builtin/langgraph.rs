use serde_json::Value;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// LangChain/LangGraph: Models the harness as an explicit state graph. Mechanically: uses `llm_call` and `tool_node` connected by conditional edges (if tool calls present -> route to `tool_node`; if absent -> route to `END`). State flows as typed dictionaries with reducer functions.

/// Reducer trait to merge state updates into the main state
pub trait Reducer<S>: Send + Sync {
    fn reduce(&self, state: &mut S, update: S);
}

/// Observes state transitions and events during the execution of a StateGraph.
pub trait GraphObserver<S>: Send + Sync {
    fn on_node_start(&self, node_name: &str, state: &S);
    fn on_node_end(&self, node_name: &str, state: &S, update: &S);
    fn on_graph_start(&self, entry_point: &str, state: &S);
    fn on_graph_end(&self, state: &S);
}

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
pub type NodeFn<S> = Arc<dyn Fn(S) -> BoxFuture<'static, Result<S, String>> + Send + Sync>;
pub type ConditionFn<S> = Arc<dyn Fn(&S) -> String + Send + Sync>;

pub struct StateGraph<S> {
    nodes: HashMap<String, NodeFn<S>>,
    edges: HashMap<String, String>,
    conditional_edges: HashMap<String, ConditionFn<S>>,
    entry_point: Option<String>,
    reducer: Arc<dyn Reducer<S>>,
    observer: Option<Arc<dyn GraphObserver<S>>>,
}

pub const END: &str = "__END__";

impl<S: Clone + Send + Sync + 'static> StateGraph<S> {
    pub fn new(reducer: Arc<dyn Reducer<S>>) -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            conditional_edges: HashMap::new(),
            entry_point: None,
            reducer,
            observer: None,
        }
    }

    pub fn add_node<F, Fut>(&mut self, name: &str, node_fn: F)
    where
        F: Fn(S) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<S, String>> + Send + 'static,
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
        self.conditional_edges
            .insert(from.to_string(), Arc::new(condition));
    }

    pub fn set_entry_point(&mut self, node: &str) {
        self.entry_point = Some(node.to_string());
    }

    pub fn set_observer(&mut self, observer: Arc<dyn GraphObserver<S>>) {
        self.observer = Some(observer);
    }

    pub async fn run(&self, initial_state: S) -> Result<S, String> {
        let mut current_state = initial_state;
        let mut current_node = self.entry_point.clone().ok_or("Entry point not set")?;

        let mut iterations = 0;
        let max_iterations = 100;

        if let Some(obs) = &self.observer {
            obs.on_graph_start(&current_node, &current_state);
        }

        while current_node != END {
            if iterations >= max_iterations {
                return Err("Max iterations reached".to_string());
            }
            iterations += 1;

            if let Some(obs) = &self.observer {
                obs.on_node_start(&current_node, &current_state);
            }

            let node_fn = self
                .nodes
                .get(&current_node)
                .ok_or_else(|| format!("Node not found: {}", current_node))?;

            let update = node_fn(current_state.clone()).await?;

            if let Some(obs) = &self.observer {
                obs.on_node_end(&current_node, &current_state, &update);
            }

            self.reducer.reduce(&mut current_state, update);

            if let Some(cond_fn) = self.conditional_edges.get(&current_node) {
                current_node = cond_fn(&current_state);
            } else if let Some(next_node) = self.edges.get(&current_node) {
                current_node = next_node.clone();
            } else {
                current_node = END.to_string();
            }
        }

        if let Some(obs) = &self.observer {
            obs.on_graph_end(&current_state);
        }

        Ok(current_state)
    }
}

/// A default reducer that merges JSON objects and appends to arrays
pub struct DefaultReducer;

impl Reducer<Value> for DefaultReducer {
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

    // Testing the "typed dictionary" aspect of the mechanic.
    #[derive(Clone, Default)]
    struct TypedAgentState {
        messages: Vec<String>,
        has_tool_calls: bool,
    }

    struct TypedReducer;

    impl Reducer<TypedAgentState> for TypedReducer {
        fn reduce(&self, state: &mut TypedAgentState, update: TypedAgentState) {
            state.messages.extend(update.messages);
            state.has_tool_calls = update.has_tool_calls;
        }
    }

    #[tokio::test]
    async fn test_langgraph_mechanic_typed() {
        let mut graph = StateGraph::<TypedAgentState>::new(Arc::new(TypedReducer));

        graph.add_node("llm_call", |state| async move {
            let turn = state.messages.len();
            if turn < 2 {
                Ok(TypedAgentState {
                    messages: vec!["assistant: (tool_call: search weather)".to_string()],
                    has_tool_calls: true,
                })
            } else {
                Ok(TypedAgentState {
                    messages: vec!["assistant: The weather is sunny.".to_string()],
                    has_tool_calls: false,
                })
            }
        });

        graph.add_node("tool_node", |_state| async move {
            Ok(TypedAgentState {
                messages: vec!["tool: Sunny".to_string()],
                has_tool_calls: false,
            })
        });

        graph.add_edge("tool_node", "llm_call");

        graph.add_conditional_edges("llm_call", |state| {
            if state.has_tool_calls {
                "tool_node".to_string()
            } else {
                END.to_string()
            }
        });

        graph.set_entry_point("llm_call");

        let initial_state = TypedAgentState {
            messages: vec!["user: What is the weather?".to_string()],
            has_tool_calls: false,
        };

        let final_state = graph.run(initial_state).await.unwrap();

        assert_eq!(final_state.messages.len(), 4);
        assert_eq!(final_state.messages[0], "user: What is the weather?");
        assert_eq!(
            final_state.messages[1],
            "assistant: (tool_call: search weather)"
        );
        assert_eq!(final_state.messages[2], "tool: Sunny");
        assert_eq!(final_state.messages[3], "assistant: The weather is sunny.");
    }

    #[derive(Default, Clone)]
    struct MockObserver {
        events: Arc<tokio::sync::Mutex<Vec<String>>>,
    }

    impl GraphObserver<TypedAgentState> for MockObserver {
        fn on_node_start(&self, node_name: &str, _state: &TypedAgentState) {
            let mut ev = self.events.try_lock().unwrap();
            ev.push(format!("start_{}", node_name));
        }

        fn on_node_end(
            &self,
            node_name: &str,
            _state: &TypedAgentState,
            _update: &TypedAgentState,
        ) {
            let mut ev = self.events.try_lock().unwrap();
            ev.push(format!("end_{}", node_name));
        }

        fn on_graph_start(&self, entry_point: &str, _state: &TypedAgentState) {
            let mut ev = self.events.try_lock().unwrap();
            ev.push(format!("graph_start_{}", entry_point));
        }

        fn on_graph_end(&self, _state: &TypedAgentState) {
            let mut ev = self.events.try_lock().unwrap();
            ev.push("graph_end".to_string());
        }
    }

    #[tokio::test]
    async fn test_langgraph_observability() {
        let mut graph = StateGraph::<TypedAgentState>::new(Arc::new(TypedReducer));

        let observer = Arc::new(MockObserver::default());
        graph.set_observer(observer.clone());

        graph.add_node("llm_call", |state| async move {
            let turn = state.messages.len();
            if turn < 2 {
                Ok(TypedAgentState {
                    messages: vec!["assistant: (tool_call: search weather)".to_string()],
                    has_tool_calls: true,
                })
            } else {
                Ok(TypedAgentState {
                    messages: vec!["assistant: The weather is sunny.".to_string()],
                    has_tool_calls: false,
                })
            }
        });

        graph.add_node("tool_node", |_state| async move {
            Ok(TypedAgentState {
                messages: vec!["tool: Sunny".to_string()],
                has_tool_calls: false,
            })
        });

        graph.add_edge("tool_node", "llm_call");

        graph.add_conditional_edges("llm_call", |state| {
            if state.has_tool_calls {
                "tool_node".to_string()
            } else {
                END.to_string()
            }
        });

        graph.set_entry_point("llm_call");

        let initial_state = TypedAgentState {
            messages: vec!["user: What is the weather?".to_string()],
            has_tool_calls: false,
        };

        let _ = graph.run(initial_state).await.unwrap();

        let events = observer.events.lock().await;
        assert_eq!(
            *events,
            vec![
                "graph_start_llm_call",
                "start_llm_call",
                "end_llm_call",
                "start_tool_node",
                "end_tool_node",
                "start_llm_call",
                "end_llm_call",
                "graph_end"
            ]
        );
    }
}
