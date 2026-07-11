#![allow(clippy::too_many_arguments, clippy::collapsible_if, clippy::useless_vec)]
/// Master Catalog B.2. Tools
use ohc_builtin_agent_core::types::ToolError;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::RwLock;

pub mod runner;
pub mod network_policy;
pub mod bash;
pub mod python;
pub mod read;
pub mod write;
pub mod edit;
pub mod glob;
pub mod grep;
pub mod webfetch;
pub mod websearch;
pub mod sendmessage;

pub mod toolsearch;
pub mod task;
pub mod booking;
pub mod agent_tool;
pub mod sleep;
pub mod marketing;
pub mod finance;
pub mod local_fs_sync;
pub mod ollama;
pub mod subagent;
pub mod head;
pub mod superpowers_tool;
pub mod tail;
pub mod find;
pub mod hybrid_blob;
pub mod restic;
pub mod anthropic_memory;
pub mod repo_map;
pub mod lazy_load;
pub mod screenshot;
pub mod generative_visibility;
pub mod magentic;
pub mod recall;
pub mod mcp_dynamic;
pub mod skill;
pub mod create_skill;
pub mod pydantic;
pub mod llm_judge;
pub mod marketplace;
pub mod marketplace_tool;
pub mod expert_team_tool;
pub mod workflow;
pub mod checkout;
pub mod quote;


#[async_trait::async_trait]
impl ToolExecutor for ohc_builtin_agent_core::code_native::CodeNativeAdapter {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        self.execute_adapter(args).await
    }
}


/// A tool definition and executor — mirrors Go builtin.Tool.
pub struct Tool {
    pub name: String,
    pub description: String,
    pub is_read_only: bool,
    pub parameters: Value,
    pub execute: Arc<dyn ToolExecutor>,
}

impl std::fmt::Debug for Tool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tool({})", self.name)
    }
}

impl Clone for Tool {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            description: self.description.clone(),
            is_read_only: self.is_read_only,
            parameters: self.parameters.clone(),
            execute: self.execute.clone(),
        }
    }
}

#[async_trait::async_trait]
pub trait ToolExecutor: Send + Sync {
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, ToolError>;
}

/// Shared todo list state.
///
/// Shared task store state.
pub type SharedTaskStore = Arc<RwLock<task::TaskStore>>;

/// Shared mailbox for SendMessage.
pub type SharedMailbox = Arc<RwLock<sendmessage::Mailbox>>;

/// Build the default set of all tools.
pub fn all_tools(
    agent_llm: Option<std::sync::Arc<dyn ohc_builtin_agent_llm::LlmClient>>,
    llm: Option<std::sync::Arc<dyn ohc_builtin_agent_core::expert_team::ExpertTeamLlmClient>>,
    native_env: Option<Arc<tokio::sync::RwLock<ohc_builtin_agent_core::code_native::RichExecutionEnvironment>>>,

    task_store: SharedTaskStore,
    mailbox: SharedMailbox,
    working_dir: Option<std::path::PathBuf>,
    memory_accessor: Option<Arc<dyn anthropic_memory::MemoryAccessor>>,
    observation_store: Arc<dashmap::DashMap<String, String>>,
) -> Vec<Tool> {
    let runner = Arc::new(runner::SandboxedCommandRunner::new(working_dir.clone()));
    let booking_store = Arc::new(RwLock::new(booking::BookingStore::default()));
    let mut tools = vec![
        repo_map::repomap_tool(working_dir.clone().unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("/")))),
        bash::bash_tool(working_dir.clone(), runner.clone()),
        python::python_tool(working_dir.clone(), runner.clone()),
        read::read_tool(working_dir.clone()),
        head::head_tool(working_dir.clone()),
        tail::tail_tool(working_dir.clone()),
            find::find_tool(working_dir.clone()),
        write::write_tool(working_dir.clone(), runner.clone()),
        edit::edit_tool(working_dir.clone(), runner.clone()),
        glob::glob_tool(working_dir.clone()),
        grep::grep_tool(working_dir.clone()),
        webfetch::webfetch_tool(),
        websearch::websearch_tool(),
        booking::booking_get_services_tool(booking_store.clone()),
        booking::booking_upsert_service_tool(booking_store.clone()),
        booking::booking_list_appointments_tool(booking_store.clone()),
        booking::booking_create_appointment_tool(booking_store.clone()),
        booking::booking_negotiate_time_tool(booking_store.clone()),
        booking::booking_reschedule_tool(booking_store.clone()),
        sendmessage::sendmessage_tool(mailbox.clone()),
        toolsearch::toolsearch_tool(),
        task::task_create_tool(task_store.clone()),
        task::task_get_tool(task_store.clone()),
        task::task_list_tool(task_store.clone()),
        task::task_update_tool(task_store.clone()),
        agent_tool::agent_stop_tool(),
        agent_tool::agent_status_tool(),
        sleep::sleep_tool(),
        marketing::qr_generate_tool(),
        finance::finance_report_tool(),
        local_fs_sync::local_fs_sync_tool(working_dir.clone()),
        ollama::ollama_tool(),
        subagent::subagent_tool(runner.clone(), llm.clone()),
        workflow::workflow_tool(runner.clone()),
        hybrid_blob::hybrid_blob_tool(),
        screenshot::screenshot_tool(working_dir.clone(), runner.clone()),
        generative_visibility::generative_visibility_tool(),
        magentic::magentic_tool(task_store.clone()),
        recall::recall_observation_tool(observation_store),
        mcp_dynamic::mcp_discover_tool(std::env::var("MCP_GATEWAY_URL").unwrap_or_else(|_| "http://localhost:8080".to_string())),
        mcp_dynamic::mcp_invoke_tool(std::env::var("MCP_GATEWAY_URL").unwrap_or_else(|_| "http://localhost:8080".to_string())),
        restic::restic_tool(runner.clone()),
        checkout::conversational_checkout_tool(),
        quote::generate_quote_tool(),
        aider_pair_programming::aider_pair_programming_tool(),
        superpowers_tool::superpowers_skill_tool(),
];

    if let Some(llm) = agent_llm {
        tools.push(llm_judge::llm_judge_tool(llm, "gemini-2.5-pro".to_string()));
    }


    if let Some(env) = native_env {
        tools.push(native_state::native_memory_stash_tool(env));
    }

    if let Some(accessor) = memory_accessor {
        tools.push(anthropic_memory::topic_retrieve_tool(accessor.clone()));
        tools.push(anthropic_memory::transcript_search_tool(accessor.clone()));
        tools.push(anthropic_memory::cross_session_search_tool(accessor.clone()));
        tools.push(anthropic_memory::topic_write_tool(accessor));
    }

    tools
}

pub mod native_state;
mod aider_pair_programming;
pub use aider_pair_programming::aider_pair_programming_tool;

#[cfg(test)]
mod marketing_test;

#[cfg(test)]
mod finance_test;

#[cfg(test)]
mod glob_test;
pub mod agent_protocol;
#[cfg(test)]
mod agent_protocol_test;
