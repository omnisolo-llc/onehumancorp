use super::*;
use checkpointer::{Checkpoint, CheckpointSaver, GitCheckpointer};

#[tokio::test]
async fn protocol_rejects_another_tasks_checkpoint_without_workspace_mutation() {
    let dir = tempfile::tempdir().unwrap();
    let saver = GitCheckpointer::new(dir.path().to_path_buf());
    saver
        .put_checkpoint(Checkpoint {
            thread_id: "task-a".into(),
            checkpoint_id: "cp-a".into(),
            parent_id: None,
            data: serde_json::json!({}),
            metadata: serde_json::json!({}),
            created_at: chrono::Utc::now(),
        })
        .await
        .unwrap();
    std::fs::write(dir.path().join("important.txt"), "untracked work").unwrap();
    let index_before = std::fs::read(dir.path().join(".git/index")).unwrap();
    let head_before = std::fs::read(dir.path().join(".git/HEAD")).unwrap();
    let server = AgentProtocolServer {
        runner: Runner {
            core: Core {
                agent: Agent {
                    checkpointer: Some(std::sync::Arc::new(saver)),
                },
            },
        },
    };
    let response = server
        .restore_checkpoint("task-b", r#"{"checkpoint_id":"cp-a"}"#)
        .await;
    assert!(response.get("error").is_some(), "{response}");
    assert_eq!(
        std::fs::read(dir.path().join(".git/index")).unwrap(),
        index_before
    );
    assert_eq!(
        std::fs::read(dir.path().join(".git/HEAD")).unwrap(),
        head_before
    );
    assert_eq!(
        std::fs::read_to_string(dir.path().join("important.txt")).unwrap(),
        "untracked work"
    );
    assert!(!dir.path().join(".git/refs/stash").exists());
}

#[tokio::test]
async fn protocol_pg_wrong_task_cannot_prune_another_tasks_history() {
    let pool = checkpointer::private_checkpoint_test_pool().await;
    let saver = checkpointer::PgCheckpointer::new(pool.clone());
    for (id, hour) in [("cp-a", 2), ("cp-b", 1)] {
        saver
            .put_checkpoint(Checkpoint {
                thread_id: "task-a".into(),
                checkpoint_id: id.into(),
                parent_id: None,
                data: serde_json::json!({}),
                metadata: serde_json::json!({}),
                created_at: chrono::Utc::now() - chrono::Duration::hours(hour),
            })
            .await
            .unwrap();
    }
    let server = AgentProtocolServer {
        runner: Runner {
            core: Core {
                agent: Agent {
                    checkpointer: Some(std::sync::Arc::new(saver)),
                },
            },
        },
    };
    let response = server
        .restore_checkpoint("task-b", r#"{"checkpoint_id":"cp-a"}"#)
        .await;
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM swarm_checkpoints")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(
        response.get("error").is_some(),
        "wrong task unexpectedly succeeded: {response}"
    );
    assert_eq!(
        count, 2,
        "wrong-task restore removed another task's history"
    );
}

#[tokio::test]
async fn postgres_duplicate_checkpoint_ids_only_prune_the_requested_task() {
    let pool = checkpointer::private_checkpoint_test_pool().await;
    let saver = checkpointer::PgCheckpointer::new(pool.clone());
    for thread in ["task-a", "task-b"] {
        for (id, hour) in [("shared", 2), ("later", 1)] {
            saver
                .put_checkpoint(Checkpoint {
                    thread_id: thread.into(),
                    checkpoint_id: id.into(),
                    parent_id: None,
                    data: serde_json::json!({}),
                    metadata: serde_json::json!({}),
                    created_at: chrono::Utc::now() - chrono::Duration::hours(hour),
                })
                .await
                .unwrap();
        }
    }
    assert!(
        saver
            .restore_checkpoint_for_thread("task-a", "missing")
            .await
            .is_err()
    );
    assert!(saver.restore_checkpoint("shared").await.is_err());
    assert_eq!(saver.list_checkpoints("task-a").await.unwrap().len(), 2);
    assert_eq!(saver.list_checkpoints("task-b").await.unwrap().len(), 2);
    saver
        .restore_checkpoint_for_thread("task-b", "shared")
        .await
        .unwrap();
    assert_eq!(saver.list_checkpoints("task-a").await.unwrap().len(), 2);
    let after = saver.list_checkpoints("task-b").await.unwrap();
    assert_eq!(after.len(), 1);
    assert_eq!(after[0].checkpoint_id, "shared");
}

#[tokio::test]
async fn old_adapter_without_scoped_restore_fails_closed() {
    struct LegacyAdapter;
    #[async_trait::async_trait]
    impl CheckpointSaver for LegacyAdapter {
        async fn get_checkpoint(&self, _: &str, _: &str) -> Result<Option<Checkpoint>, String> {
            Ok(None)
        }
        async fn put_checkpoint(&self, _: Checkpoint) -> Result<(), String> {
            Ok(())
        }
        async fn list_checkpoints(&self, _: &str) -> Result<Vec<Checkpoint>, String> {
            Ok(vec![])
        }
        async fn restore_checkpoint(&self, _: &str) -> Result<(), String> {
            panic!("scoped restore must never silently call the legacy unscoped operation")
        }
    }
    assert!(
        LegacyAdapter
            .restore_checkpoint_for_thread("task-a", "cp-a")
            .await
            .is_err()
    );
}

#[tokio::test]
async fn protocol_restores_a_checkpoint_for_its_own_task() {
    let dir = tempfile::tempdir().unwrap();
    let saver = GitCheckpointer::new(dir.path().to_path_buf());
    for id in ["cp-first", "cp-second"] {
        std::fs::write(dir.path().join("tracked"), id).unwrap();
        saver
            .put_checkpoint(Checkpoint {
                thread_id: "task-a".into(),
                checkpoint_id: id.into(),
                parent_id: None,
                data: serde_json::json!({}),
                metadata: serde_json::json!({}),
                created_at: chrono::Utc::now(),
            })
            .await
            .unwrap();
    }
    let server = AgentProtocolServer {
        runner: Runner {
            core: Core {
                agent: Agent {
                    checkpointer: Some(std::sync::Arc::new(saver)),
                },
            },
        },
    };
    let response = server
        .restore_checkpoint("task-a", r#"{"checkpoint_id":"cp-first"}"#)
        .await;
    assert_eq!(response["success"], true, "{response}");
    assert_eq!(
        std::fs::read_to_string(dir.path().join("tracked")).unwrap(),
        "cp-first"
    );
}
