#![allow(clippy::empty_line_after_doc_comments)]
use serde_json::Value;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Master Catalog A: Framework Implementation Archetypes: LangChain/LangGraph. Models the harness as an explicit state graph. Mechanically: uses `llm_call` and `tool_node` connected by conditional edges (if tool calls present -> route to `tool_node`; if absent -> route to `END`). State flows as typed dictionaries with reducer functions.

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

    /// Compiles the StateGraph into a CompiledStateGraph that can be executed.
    /// This locks the graph topology and prevents further modifications.
    pub fn compile(self) -> Result<CompiledStateGraph<S>, String> {
        if self.entry_point.is_none() {
            return Err("Cannot compile: Entry point not set".to_string());
        }
        Ok(CompiledStateGraph {
            nodes: self.nodes,
            edges: self.edges,
            conditional_edges: self.conditional_edges,
            entry_point: self.entry_point.unwrap(),
            reducer: self.reducer,
            observer: self.observer,
        })
    }
}

pub trait AgentState: Clone + Send + Sync + 'static {
    fn has_tool_calls(&self) -> bool;
}

impl<S: AgentState> StateGraph<S> {
    /// Mechanically: uses `llm_call` and `tool_node` connected by conditional edges
    /// (if tool calls present -> route to `tool_node`; if absent -> route to `END`).
    pub fn build_standard_agent_harness<F1, Fut1, F2, Fut2>(&mut self, llm_call: F1, tool_node: F2)
    where
        F1: Fn(S) -> Fut1 + Send + Sync + 'static,
        Fut1: Future<Output = Result<S, String>> + Send + 'static,
        F2: Fn(S) -> Fut2 + Send + Sync + 'static,
        Fut2: Future<Output = Result<S, String>> + Send + 'static,
    {
        self.add_node("llm_call", llm_call);
        self.add_node("tool_node", tool_node);

        // if tool calls present -> route to `tool_node`; if absent -> route to `END`
        self.add_conditional_edges("llm_call", |state: &S| {
            if state.has_tool_calls() {
                "tool_node".to_string()
            } else {
                END.to_string()
            }
        });

        // Loop back to llm_call after tool execution
        self.add_edge("tool_node", "llm_call");
        self.set_entry_point("llm_call");
    }
}

/// A compiled state graph that is ready to be executed.
pub struct CompiledStateGraph<S> {
    nodes: HashMap<String, NodeFn<S>>,
    edges: HashMap<String, String>,
    conditional_edges: HashMap<String, ConditionFn<S>>,
    entry_point: String,
    reducer: Arc<dyn Reducer<S>>,
    observer: Option<Arc<dyn GraphObserver<S>>>,
}

impl<S: Clone + Send + Sync + 'static> CompiledStateGraph<S> {
    pub async fn run(&self, initial_state: S) -> Result<S, String> {
        let mut current_state = initial_state;
        let mut current_node = self.entry_point.clone();

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

    /// Pregel-inspired execution model for StateGraph.
    /// Runs all currently active nodes concurrently (super-steps),
    /// merging their outputs via the reducer at the end of each super-step.
    pub async fn pregel_run(&self, initial_state: S) -> Result<S, String> {
        let mut current_state = initial_state;
        let mut active_nodes = vec![self.entry_point.clone()];

        let mut iterations = 0;
        let max_iterations = 100;

        if let Some(obs) = &self.observer {
            obs.on_graph_start("pregel_start", &current_state);
        }

        while !active_nodes.is_empty() {
            if iterations >= max_iterations {
                return Err("Max iterations reached".to_string());
            }
            iterations += 1;

            let mut next_nodes = vec![];
            let mut tasks = vec![];

            // Run active nodes concurrently
            for node in active_nodes {
                if node == END {
                    continue;
                }

                if let Some(obs) = &self.observer {
                    obs.on_node_start(&node, &current_state);
                }

                let node_fn = self
                    .nodes
                    .get(&node)
                    .ok_or_else(|| format!("Node not found: {}", node))?;

                let state_clone = current_state.clone();
                let node_clone = node.clone();
                let node_fn_clone = node_fn.clone();

                tasks.push(async move {
                    let update = node_fn_clone(state_clone).await?;
                    Ok::<(String, S), String>((node_clone, update))
                });
            }

            let results = futures::future::join_all(tasks).await;

            // Reduce all updates from the super-step and determine the next nodes
            for res in results {
                let (node, update) = res?;

                if let Some(obs) = &self.observer {
                    obs.on_node_end(&node, &current_state, &update);
                }

                self.reducer.reduce(&mut current_state, update);

                if let Some(cond_fn) = self.conditional_edges.get(&node) {
                    next_nodes.push(cond_fn(&current_state));
                } else if let Some(next_node) = self.edges.get(&node) {
                    next_nodes.push(next_node.clone());
                } else {
                    next_nodes.push(END.to_string());
                }
            }

            active_nodes = next_nodes.into_iter().filter(|n| n != END).collect();
            active_nodes.sort();
            active_nodes.dedup();
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
    #[derive(Clone, Default, Debug)]
    pub struct TypedAgentState {
        pub messages: Vec<String>,
        pub has_tool_calls: bool,
    }

    impl AgentState for TypedAgentState {
        fn has_tool_calls(&self) -> bool {
            self.has_tool_calls
        }
    }

    pub struct TypedReducer;

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

        let compiled = graph.compile().unwrap();
        let final_state = compiled.run(initial_state).await.unwrap();

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
    async fn test_linear_graph() {
        let mut graph = StateGraph::<TypedAgentState>::new(Arc::new(TypedReducer));

        graph.add_node("node1", |_state| async move {
            Ok(TypedAgentState {
                messages: vec!["Node 1 executed".to_string()],
                has_tool_calls: false,
            })
        });

        graph.add_node("node2", |_state| async move {
            Ok(TypedAgentState {
                messages: vec!["Node 2 executed".to_string()],
                has_tool_calls: false,
            })
        });

        graph.add_edge("node1", "node2");
        graph.set_entry_point("node1");

        let initial_state = TypedAgentState {
            messages: vec![],
            has_tool_calls: false,
        };

        let compiled = graph.compile().unwrap();
        let final_state = compiled.run(initial_state).await.unwrap();

        assert_eq!(final_state.messages.len(), 2);
        assert_eq!(final_state.messages[0], "Node 1 executed");
        assert_eq!(final_state.messages[1], "Node 2 executed");
    }

    #[tokio::test]
    async fn test_reducer_merge() {
        let mut graph = StateGraph::<Value>::new(Arc::new(DefaultReducer));

        graph.add_node("init", |_state| async move {
            Ok(serde_json::json!({
                "key1": "value1",
                "arr": ["item1"]
            }))
        });

        graph.add_node("merge", |_state| async move {
            Ok(serde_json::json!({
                "key2": "value2",
                "arr": ["item2"]
            }))
        });

        graph.add_edge("init", "merge");
        graph.set_entry_point("init");

        let initial_state = serde_json::json!({});
        let compiled = graph.compile().unwrap();
        let final_state = compiled.run(initial_state).await.unwrap();

        assert_eq!(final_state["key1"], "value1");
        assert_eq!(final_state["key2"], "value2");
        assert_eq!(final_state["arr"][0], "item1");
        assert_eq!(final_state["arr"][1], "item2");
    }

    #[tokio::test]
    async fn test_conditional_edges() {
        let mut graph = StateGraph::<TypedAgentState>::new(Arc::new(TypedReducer));

        graph.add_node("router", |state| async move {
            Ok(TypedAgentState {
                messages: vec!["Router executed".to_string()],
                has_tool_calls: state.has_tool_calls, // preserve input flag
            })
        });

        graph.add_node("path_a", |_state| async move {
            Ok(TypedAgentState {
                messages: vec!["Path A executed".to_string()],
                has_tool_calls: false,
            })
        });

        graph.add_node("path_b", |_state| async move {
            Ok(TypedAgentState {
                messages: vec!["Path B executed".to_string()],
                has_tool_calls: false,
            })
        });

        graph.add_conditional_edges("router", |state| {
            if state.has_tool_calls {
                "path_a".to_string()
            } else {
                "path_b".to_string()
            }
        });

        graph.set_entry_point("router");

        let compiled = graph.compile().unwrap();

        // Run Path A
        let initial_state_a = TypedAgentState {
            messages: vec![],
            has_tool_calls: true, // Should route to path_a
        };
        let final_state_a = compiled.run(initial_state_a).await.unwrap();
        assert_eq!(final_state_a.messages.len(), 2);
        assert_eq!(final_state_a.messages[1], "Path A executed");

        // Run Path B
        let initial_state_b = TypedAgentState {
            messages: vec![],
            has_tool_calls: false, // Should route to path_b
        };
        let final_state_b = compiled.run(initial_state_b).await.unwrap();
        assert_eq!(final_state_b.messages.len(), 2);
        assert_eq!(final_state_b.messages[1], "Path B executed");
    }

    #[tokio::test]
    async fn test_langgraph_error_handling() {
        let mut graph = StateGraph::<TypedAgentState>::new(Arc::new(TypedReducer));

        graph.add_node("failing_node", |_state| async move {
            Err("Simulated node failure".to_string())
        });

        graph.set_entry_point("failing_node");

        let initial_state = TypedAgentState {
            messages: vec![],
            has_tool_calls: false,
        };

        let compiled = graph.compile().unwrap();
        let result = compiled.run(initial_state).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Simulated node failure");
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

        let compiled = graph.compile().unwrap();
        let _ = compiled.run(initial_state).await.unwrap();

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
