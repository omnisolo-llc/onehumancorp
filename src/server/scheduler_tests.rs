use super::*;

    #[test]
    fn test_create_and_poll_task() {
        let s = Scheduler::new();
        let now = Utc::now();

        let task = Task {
            id: "task1".to_string(),
            organization_id: "org1".to_string(),
            agent_id: "agent1".to_string(),
            name: "Test Task".to_string(),
            schedule: Schedule {
                r#type: ScheduleType::Once,
                at: Some(now - Duration::seconds(10)),
                interval_s: None,
                expression: None,
            },
            status: TaskStatus::Pending,
            created_at: now,
            last_run_at: None,
            next_run_at: Some(now - Duration::seconds(10)),
            payload: serde_json::json!({}),
        };

        s.create(task.clone()).unwrap();

        let due = s.poll_due();
        assert_eq!(due.len(), 1);
        assert_eq!(due[0].id, "task1");
    }

    #[test]
    fn test_mark_running_and_done() {
        let s = Scheduler::new();
        let now = Utc::now();

        let task = Task {
            id: "task2".to_string(),
            organization_id: "org1".to_string(),
            agent_id: "agent1".to_string(),
            name: "Test Task 2".to_string(),
            schedule: Schedule {
                r#type: ScheduleType::Once,
                at: Some(now),
                interval_s: None,
                expression: None,
            },
            status: TaskStatus::Pending,
            created_at: now,
            last_run_at: None,
            next_run_at: Some(now),
            payload: serde_json::json!({}),
        };

        s.create(task.clone()).unwrap();

        let running = s.mark_running("org1", "task2").unwrap();
        assert_eq!(running.status, TaskStatus::Running);

        s.mark_done("org1", "task2", true).unwrap();

        let tasks = s.list_for_org("org1");
        assert_eq!(tasks[0].status, TaskStatus::Succeeded);
    }
