pub mod tool;
pub use tool::{
    ConfigResponse, ConfigSyncPayload, ConfigSyncTool, McpConfigSyncError, PgConfigSyncTool,
    register_config_sync_schema,
};
