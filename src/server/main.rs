use tonic::{transport::Server, Request, Response, Status};
use tokio_stream::Stream;
use tokio_stream::StreamExt;
use chrono::Utc;
use std::pin::Pin;

mod db;
mod auth;
mod hub;
mod minimax;
mod billing;
mod ultraplan;
mod autodream;
mod tasks;
mod settings;
mod scheduler;
mod msgbus;
mod pipeline;
mod oidc;
mod sip;
mod seeder;
mod orchestrator;
mod spawner;
mod queue;
mod agents;
mod domain;
pub mod pricing;
pub mod analytics;
pub mod telemetry;
pub mod chaos;
pub mod integrations;
pub mod utils;
pub mod storage;
pub mod benchmarks;
pub mod config;
pub mod http;
pub mod services {
    pub mod wizard;
    pub mod billing {
        pub mod auditor;
    }
    pub mod growth;
    pub mod onboarding;
    pub mod sync;
    pub mod chat;
    pub mod b2b;
    pub mod integration;
    pub mod ops;
    pub mod mcp;
    pub mod org;
    pub mod scheduler;
    pub mod agent;
    pub mod autodream;
}

use ohc::orchestration::*;
use ohc::orchestration::hub_service_server::{HubService, HubServiceServer};

pub mod ohc {
    pub mod orchestration {
        tonic::include_proto!("ohc.orchestration");
    }
    pub mod agent {
        pub mod service {
            tonic::include_proto!("ohc.agent.service");
        }
    }
    pub mod organization {
        tonic::include_proto!("ohc.organization");
    }
    pub mod common {
        tonic::include_proto!("ohc.common");
    }
}

pub struct MyHubService {
    pool: sqlx::PgPool,
}

#[tonic::async_trait]
impl HubService for MyHubService {
    async fn register_agent(
        &self,
        request: Request<RegisterAgentRequest>,
    ) -> Result<Response<RegisterAgentResponse>, Status> {
        let req = request.into_inner();
        let agent = req.agent.ok_or_else(|| Status::invalid_argument("agent missing"))?;
        
        println!("Registered agent: {:?}", agent);

        Ok(Response::new(RegisterAgentResponse {
            success: true,
            agent_id: agent.id,
        }))
    }

    async fn unregister_agent(
        &self,
        _request: Request<UnregisterAgentRequest>,
    ) -> Result<Response<UnregisterAgentResponse>, Status> {
        Ok(Response::new(UnregisterAgentResponse { success: true }))
    }

    async fn report_status(
        &self,
        _request: Request<AgentStatusReport>,
    ) -> Result<Response<EmptyResponse>, Status> {
        Ok(Response::new(EmptyResponse {}))
    }

    async fn push_task(
        &self,
        _request: Request<PushTaskRequest>,
    ) -> Result<Response<PushTaskResponse>, Status> {
        Ok(Response::new(PushTaskResponse {
            success: true,
            task_id: "task_123".to_string(),
        }))
    }

    type PollTasksStream = Pin<Box<dyn Stream<Item = Result<TaskProto, Status>> + Send + 'static>>;

    async fn poll_tasks(
        &self,
        _request: Request<PollTasksRequest>,
    ) -> Result<Response<Self::PollTasksStream>, Status> {
        let tasks = vec![TaskProto {
            id: "task_123".to_string(),
            payload: "Do some work".to_string(),
            priority: 1,
            assigned_to: "agent_1".to_string(),
        }];
        
        let stream = tokio_stream::iter(tasks).map(Ok);
        Ok(Response::new(Box::pin(stream) as Self::PollTasksStream))
    }

    async fn complete_task(
        &self,
        _request: Request<CompleteTaskRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        Ok(Response::new(EmptyResponse {}))
    }

    async fn create_meeting(
        &self,
        _request: Request<CreateMeetingRequest>,
    ) -> Result<Response<CreateMeetingResponse>, Status> {
        Ok(Response::new(CreateMeetingResponse {
            meeting_id: "meeting_123".to_string(),
        }))
    }

    async fn join_meeting(
        &self,
        _request: Request<JoinMeetingRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        Ok(Response::new(EmptyResponse {}))
    }

    async fn leave_meeting(
        &self,
        _request: Request<LeaveMeetingRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        Ok(Response::new(EmptyResponse {}))
    }

    async fn send_message(
        &self,
        _request: Request<SendMessageRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        Ok(Response::new(EmptyResponse {}))
    }

    type StreamMessagesStream = Pin<Box<dyn Stream<Item = Result<MessageProto, Status>> + Send + 'static>>;

    async fn stream_messages(
        &self,
        _request: Request<StreamMessagesRequest>,
    ) -> Result<Response<Self::StreamMessagesStream>, Status> {
        let msgs = vec![MessageProto {
            id: "msg_123".to_string(),
            meeting_id: "meeting_123".to_string(),
            from_agent: "agent_2".to_string(),
            content: "Hello!".to_string(),
            timestamp_unix: Utc::now().timestamp(),
        }];
        
        let stream = tokio_stream::iter(msgs).map(Ok);
        Ok(Response::new(Box::pin(stream) as Self::StreamMessagesStream))
    }

    async fn get_inbox(
        &self,
        _request: Request<GetInboxRequest>,
    ) -> Result<Response<GetInboxResponse>, Status> {
        Ok(Response::new(GetInboxResponse { messages: vec![] }))
    }

    async fn send_direct_message(
        &self,
        _request: Request<SendMessageRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        Ok(Response::new(EmptyResponse {}))
    }

    async fn update_capabilities(
        &self,
        _request: Request<UpdateCapabilitiesRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        Ok(Response::new(EmptyResponse {}))
    }

    type DiscoverAgentsStream = Pin<Box<dyn Stream<Item = Result<Agent, Status>> + Send + 'static>>;

    async fn discover_agents(
        &self,
        _request: Request<DiscoverAgentsRequest>,
    ) -> Result<Response<Self::DiscoverAgentsStream>, Status> {
        let agents = vec![Agent {
            id: "agent_2".to_string(),
            name: "Other Agent".to_string(),
            role: "Helper".to_string(),
            organization_id: "org_1".to_string(),
            status: "Running".to_string(),
            provider_type: "Mock".to_string(),
        }];
        let stream = tokio_stream::iter(agents).map(Ok);
        Ok(Response::new(Box::pin(stream) as Self::DiscoverAgentsStream))
    }

    type StreamMeshEventsStream = Pin<Box<dyn Stream<Item = Result<MeshEventProto, Status>> + Send + 'static>>;

    async fn stream_mesh_events(
        &self,
        _request: Request<EventStreamRequest>,
    ) -> Result<Response<Self::StreamMeshEventsStream>, Status> {
        let events = vec![MeshEventProto {
            id: "evt_1".to_string(),
            topic: "general".to_string(),
            payload: "mesh online".to_string().into_bytes(),
            published_at: Utc::now().timestamp(),
        }];
        let stream = tokio_stream::iter(events).map(Ok);
        Ok(Response::new(Box::pin(stream) as Self::StreamMeshEventsStream))
    }

    async fn publish_mesh_event(
        &self,
        _request: Request<PublishEventRequest>,
    ) -> Result<Response<PublishEventResponse>, Status> {
        Ok(Response::new(PublishEventResponse {
            success: true,
            event_id: "evt_1".to_string(),
        }))
    }

    async fn publish_teammate_mesh_event(
        &self,
        _request: Request<PublishTeammateEventRequest>,
    ) -> Result<Response<PublishEventResponse>, Status> {
        Ok(Response::new(PublishEventResponse {
            success: true,
            event_id: "teammate_evt_1".to_string(),
        }))
    }

    type StreamTeammateMeshStream = Pin<Box<dyn Stream<Item = Result<TeammateMeshEventProto, Status>> + Send + 'static>>;

    async fn stream_teammate_mesh(
        &self,
        _request: Request<EventStreamRequest>,
    ) -> Result<Response<Self::StreamTeammateMeshStream>, Status> {
        let events = vec![TeammateMeshEventProto {
            id: "evt_1".to_string(),
            sender_id: "agent_1".to_string(),
            topic: "teammate".to_string(),
            payload: "hello teammate".to_string().into_bytes(),
            published_at: Utc::now().timestamp(),
        }];
        let stream = tokio_stream::iter(events).map(Ok);
        Ok(Response::new(Box::pin(stream) as Self::StreamTeammateMeshStream))
    }

    async fn subscribe_mesh_topic(
        &self,
        _request: Request<SubscribeTopicRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        Ok(Response::new(EmptyResponse {}))
    }

    async fn get_budget_alerts(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<BudgetAlertsResponse>, Status> {
        Ok(Response::new(BudgetAlertsResponse { alerts: vec![] }))
    }

    async fn create_budget_alert(
        &self,
        _request: Request<CreateBudgetAlertRequest>,
    ) -> Result<Response<BudgetAlert>, Status> {
        Ok(Response::new(BudgetAlert {
            id: "alert_1".to_string(),
            organization_id: "org_1".to_string(),
            threshold_usd: 100.0,
            notify_at_pct: 80.0,
            predictive: false,
            forecast_hours: 0,
            triggered: false,
            created_at_unix: Utc::now().timestamp(),
        }))
    }

    async fn get_agents(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<GetAgentsResponse>, Status> {
        Ok(Response::new(GetAgentsResponse { agents: vec![] }))
    }

    async fn get_meetings(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<GetMeetingsResponse>, Status> {
        Ok(Response::new(GetMeetingsResponse { meetings: vec![] }))
    }

    async fn sync_auto_dream(
        &self,
        _request: Request<SyncAutoDreamRequest>,
    ) -> Result<Response<SyncAutoDreamResponse>, Status> {
        Ok(Response::new(SyncAutoDreamResponse {
            synced_items: 0,
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = tracing_subscriber::fmt::try_init();

    let addr = "[::1]:18789".parse()?;
    
    let db = db::DB::new().await?;
    db.run_migrations().await?;
    
    let service = MyHubService { pool: db.pool };

    println!("Starting HubService on {}", addr);

    Server::builder()
        .add_service(HubServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ohc::orchestration::*;

    async fn setup_test_service() -> Option<MyHubService> {
        // Attempt to connect to a local DB for testing. If it fails, skip the test.
        std::env::set_var("DATABASE_URL", "postgres://postgres:postgres@localhost:5432/ohc");
        if let Ok(db) = db::DB::new().await {
            Some(MyHubService { pool: db.pool })
        } else {
            None
        }
    }

    #[tokio::test]
    async fn test_register_agent_valid() {
        let service = match setup_test_service().await {
            Some(s) => s,
            None => return,
        };

        let req = Request::new(RegisterAgentRequest {
            agent: Some(Agent {
                id: "agent_test".to_string(),
                name: "Test Agent".to_string(),
                role: "Tester".to_string(),
                organization_id: "org_1".to_string(),
                status: "Running".to_string(),
                provider_type: "Mock".to_string(),
            }),
        });

        let resp = service.register_agent(req).await;
        assert!(resp.is_ok());
        let inner = resp.unwrap().into_inner();
        assert!(inner.success);
        assert_eq!(inner.agent_id, "agent_test");
    }

    #[tokio::test]
    async fn test_register_agent_missing_agent() {
        let service = match setup_test_service().await {
            Some(s) => s,
            None => return,
        };

        let req = Request::new(RegisterAgentRequest { agent: None });

        let resp = service.register_agent(req).await;
        assert!(resp.is_err());
        assert_eq!(resp.unwrap_err().code(), tonic::Code::InvalidArgument);
    }

    #[tokio::test]
    async fn test_create_meeting() {
        let service = match setup_test_service().await {
            Some(s) => s,
            None => return,
        };

        let req = Request::new(CreateMeetingRequest {
            topic: "Test Meeting".to_string(),
            participants: vec!["agent_1".to_string()],
        });

        let resp = service.create_meeting(req).await;
        assert!(resp.is_ok());
        assert_eq!(resp.unwrap().into_inner().meeting_id, "meeting_123");
    }

    #[tokio::test]
    async fn test_push_task() {
        let service = match setup_test_service().await {
            Some(s) => s,
            None => return,
        };

        let req = Request::new(PushTaskRequest {
            payload: "Do some testing".to_string(),
            priority: 1,
        });

        let resp = service.push_task(req).await;
        assert!(resp.is_ok());
        assert!(resp.unwrap().into_inner().success);
    }

    #[tokio::test]
    async fn test_poll_tasks() {
        let service = match setup_test_service().await {
            Some(s) => s,
            None => return,
        };

        let req = Request::new(PollTasksRequest {
            agent_id: "agent_1".to_string(),
            max_tasks: 5,
        });

        let resp = service.poll_tasks(req).await;
        assert!(resp.is_ok());
    }

    #[tokio::test]
    async fn test_complete_task() {
        let service = match setup_test_service().await {
            Some(s) => s,
            None => return,
        };

        let req = Request::new(CompleteTaskRequest {
            task_id: "task_123".to_string(),
            result: "Done".to_string(),
            status: "SUCCESS".to_string(),
        });

        let resp = service.complete_task(req).await;
        assert!(resp.is_ok());
    }

    #[tokio::test]
    async fn test_publish_mesh_event() {
        let service = match setup_test_service().await {
            Some(s) => s,
            None => return,
        };

        let req = Request::new(PublishEventRequest {
            event: Some(MeshEventProto {
                id: "evt_1".to_string(),
                topic: "general".to_string(),
                published_at: Utc::now().timestamp(),
                payload: "hello".to_string().into(),
            }),
        });

        let resp = service.publish_mesh_event(req).await;
        assert!(resp.is_ok());
        assert!(resp.unwrap().into_inner().success);
    }

    #[tokio::test]
    async fn test_publish_teammate_mesh_event() {
        let service = match setup_test_service().await {
            Some(s) => s,
            None => return,
        };

        let req = Request::new(PublishTeammateEventRequest {
            event: Some(TeammateMeshEventProto {
                id: "evt_1".to_string(),
                sender_id: "agent_1".to_string(),
                topic: "general".to_string(),
                published_at: Utc::now().timestamp(),
                payload: "hello".to_string().into(),
            }),
        });

        let resp = service.publish_teammate_mesh_event(req).await;
        assert!(resp.is_ok());
        assert!(resp.unwrap().into_inner().success);
    }

    #[tokio::test]
    #[ignore]
    async fn test_stream_mesh_events_valid() {
        let service = match setup_test_service().await {
            Some(s) => s,
            None => return,
        };

        let req = Request::new(EventStreamRequest {
            topic: "test".to_string(),
        });

        let resp = service.stream_mesh_events(req).await;
        assert!(resp.is_ok());
    }
}

#[cfg(test)]
mod benchmark_tests {
    use super::*;

    #[tokio::test]
    async fn test_hybrid_latency_benchmarks() {
        // Extract connection string dynamically
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());

        if let Ok(pool) = sqlx::PgPool::connect(&db_url).await {
            // Run Cloud Postgres Benchmark
            let pg_results = crate::domain::benchmarks::hybrid_latency::run_hybrid_latency_benchmark_pg(pool.clone()).await;
            for res in &pg_results {
                println!("[{}] {} - p50: {:.2}ms, p95: {:.2}ms, p99: {:.2}ms, avg: {:.2}ms",
                    res.mode, res.name, res.p50_ms, res.p95_ms, res.p99_ms, res.avg_ms);
            }

            // Run API Latency Parallel execution benchmark
            let api_results = crate::domain::benchmarks::api_latency::run_api_response_benchmark("Cloud", pool).await;
            println!("[{}] {} - p50: {:.2}ms, p95: {:.2}ms, p99: {:.2}ms, avg: {:.2}ms",
                api_results.mode, api_results.name, api_results.p50_ms, api_results.p95_ms, api_results.p99_ms, api_results.avg_ms);
        }

        // Run Standalone SQLite Benchmark
        if let Ok(pool) = sqlx::SqlitePool::connect("sqlite::memory:").await {
            let _ = sqlx::query("CREATE TABLE IF NOT EXISTS tasks (id TEXT PRIMARY KEY, status TEXT)").execute(&pool).await;
            let _ = sqlx::query("CREATE TABLE IF NOT EXISTS sub_agent_queue (id TEXT PRIMARY KEY, organization_id TEXT, parent_task_id TEXT, payload TEXT, status TEXT, scheduled_at TEXT, created_at TEXT, updated_at TEXT)").execute(&pool).await;
            let sq_results = crate::domain::benchmarks::hybrid_latency::run_hybrid_latency_benchmark_sqlite(pool).await;
            for res in &sq_results {
                println!("[{}] {} - p50: {:.2}ms, p95: {:.2}ms, p99: {:.2}ms, avg: {:.2}ms",
                    res.mode, res.name, res.p50_ms, res.p95_ms, res.p99_ms, res.avg_ms);
            }
        }
    }
}


#[cfg(test)]
mod benchmark_tests {
    use super::*;
    use sqlx::SqlitePool;

    #[tokio::test]
    async fn test_hybrid_latency_benchmarks() {
        // Run Postgres (Cloud) Benchmark
        std::env::set_var("DATABASE_URL", "postgres://postgres:postgres@localhost:5432/ohc");
        if let Ok(db) = crate::db::DB::new().await {
            let _ = db.run_migrations().await; // ensure schema
            let pg_results = crate::benchmarks::hybrid_latency::run_hybrid_latency_benchmark_pg(db.pool.clone()).await;
            for res in &pg_results {
                println!("[{}] {} - p50: {:.2}ms, p95: {:.2}ms, p99: {:.2}ms, avg: {:.2}ms",
                    res.mode, res.name, res.p50_ms, res.p95_ms, res.p99_ms, res.avg_ms);
            }

            // Run API Latency Parallel execution benchmark
            let api_results = crate::benchmarks::api_latency::run_api_response_benchmark("Cloud", db.pool.clone()).await;
            println!("[{}] {} - p50: {:.2}ms, p95: {:.2}ms, p99: {:.2}ms, avg: {:.2}ms",
                api_results.mode, api_results.name, api_results.p50_ms, api_results.p95_ms, api_results.p99_ms, api_results.avg_ms);
        }

        // Run SQLite (Standalone) Benchmark
        if let Ok(pool) = SqlitePool::connect("sqlite::memory:").await {
            // Setup minimum schema for sub_agent_queue and tasks
            let _ = sqlx::query("CREATE TABLE IF NOT EXISTS tasks (id TEXT PRIMARY KEY, status TEXT)").execute(&pool).await;
            let _ = sqlx::query("CREATE TABLE IF NOT EXISTS sub_agent_queue (id TEXT PRIMARY KEY, organization_id TEXT, parent_task_id TEXT, payload TEXT, status TEXT, scheduled_at TEXT, created_at TEXT, updated_at TEXT)").execute(&pool).await;
            let sq_results = crate::benchmarks::hybrid_latency::run_hybrid_latency_benchmark_sqlite(pool).await;
            for res in &sq_results {
                println!("[{}] {} - p50: {:.2}ms, p95: {:.2}ms, p99: {:.2}ms, avg: {:.2}ms",
                    res.mode, res.name, res.p50_ms, res.p95_ms, res.p99_ms, res.avg_ms);
            }
        }
    }
}
