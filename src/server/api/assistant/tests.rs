use crate::api::assistant::workspaces::Workspace;
use crate::api::assistant::tasks::Task;
use crate::api::assistant::messages::Message;
use crate::api::assistant::artifacts::Artifact;
use crate::api::assistant::approvals::Approval;
use chrono::Utc;

#[test]
fn test_assistant_workspace_model() {
    let ws = Workspace {
        id: "ws-1".to_string(),
        name: "Main Workspace".to_string(),
        default_work_directory: Some("/home/user/work".to_string()),
        default_model: Some("gpt-4".to_string()),
    };
    assert_eq!(ws.id, "ws-1");
    assert_eq!(ws.name, "Main Workspace");
}

#[test]
fn test_assistant_task_model() {
    let task = Task {
        id: "task-123".to_string(),
        workspace_id: "ws-1".to_string(),
        title: "Market Research".to_string(),
        prompt: "Analyze competitors".to_string(),
        status: "pending".to_string(),
        mode: Some("agent".to_string()),
        model: Some("gpt-4o".to_string()),
        provider: Some("openai".to_string()),
        permission_profile: Some("Guarded".to_string()),
        current_step: Some("Initializing".to_string()),
        archived: false,
        created_at: Some(Utc::now()),
        updated_at: Some(Utc::now()),
    };
    assert_eq!(task.id, "task-123");
    assert_eq!(task.status, "pending");
}

#[test]
fn test_assistant_message_model() {
    let msg = Message {
        id: "msg-1".to_string(),
        task_id: "task-123".to_string(),
        role: "assistant".to_string(),
        content: "I am starting the research.".to_string(),
        attachments: serde_json::json!([]),
        tool_calls: serde_json::json!([{"tool": "web_search", "args": {"q": "bakery competitors"}}]),
        created_at: Some(Utc::now()),
    };
    assert_eq!(msg.role, "assistant");
    assert!(msg.content.contains("starting"));
}

#[test]
fn test_assistant_artifact_model() {
    let art = Artifact {
        id: "art-456".to_string(),
        task_id: "task-123".to_string(),
        type_name: "CSV".to_string(),
        filename: "competitors.csv".to_string(),
        path: Some("/tmp/competitors.csv".to_string()),
        mime_type: Some("text/csv".to_string()),
        size: Some(1024),
        preview: Some("name,location\nBakery A,NY".to_string()),
        created_at: Some(Utc::now()),
    };
    assert_eq!(art.filename, "competitors.csv");
    assert_eq!(art.type_name, "CSV");
}

#[test]
fn test_assistant_approval_model() {
    let app = Approval {
        id: "app-789".to_string(),
        task_id: "task-123".to_string(),
        tool_name: "hybrid_fs_write".to_string(),
        args: Some(serde_json::json!({"path": "report.md", "content": "..."})),
        status: Some("pending".to_string()),
        risk_level: Some("high".to_string()),
        created_at: Some(Utc::now()),
        updated_at: Some(Utc::now()),
    };
    assert_eq!(app.tool_name, "hybrid_fs_write");
    assert_eq!(app.status, Some("pending".to_string()));
}
