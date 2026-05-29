use tonic::{Request, Response, Status};
use ::server_ohc::orchestration::*;
use ::server_ohc::orchestration::scheduler_service_server::SchedulerService;
use std::sync::Arc;
use crate::hub::Hub;
use ::server_lib::scheduler::{Task, Schedule, ScheduleType, TaskStatus};
use chrono::{Utc, TimeZone};

pub struct MySchedulerService {
    hub: Arc<Hub>,
}

impl MySchedulerService {
    pub fn new(hub: Arc<Hub>) -> Self {
        MySchedulerService { hub }
    }
}

#[tonic::async_trait]
impl SchedulerService for MySchedulerService {
    async fn get_scheduled_tasks(
        &self,
        request: Request<EmptyRequest>,
    ) -> Result<Response<ScheduledTasksResponse>, Status> {
        let spiffe_id_str = ::server_auth::extract_spiffe_id_from_metadata(request.metadata()).map_err(|e| Status::unauthenticated(e))?;
        let (tenant_id, _) = ::server_auth::parse_spiffe_id(&spiffe_id_str)?;
        let org_id = if tenant_id.is_empty() { "system".to_string() } else { tenant_id };


        let tasks = self.hub.scheduler().list_for_org(&org_id);
        let proto_tasks = tasks.into_iter().map(|t| convert_to_proto(t)).collect();
        Ok(Response::new(ScheduledTasksResponse { tasks: proto_tasks }))
    }

    async fn create_scheduled_task(
        &self,
        request: Request<CreateScheduledTaskRequest>,
    ) -> Result<Response<ProtoTask>, Status> {
        let spiffe_id_str = ::server_auth::extract_spiffe_id_from_metadata(request.metadata()).map_err(|e| Status::unauthenticated(e))?;
        let (tenant_id, _) = ::server_auth::parse_spiffe_id(&spiffe_id_str)?;
        let org_id = if tenant_id.is_empty() { "system".to_string() } else { tenant_id };


        let req = request.into_inner();
        let schedule = req.schedule.ok_or_else(|| Status::invalid_argument("schedule is required"))?;
        
        let task = Task {
            id: format!("task-{}", Utc::now().timestamp()),
            organization_id: org_id,
            agent_id: req.agent_id,
            name: req.name,
            schedule: convert_from_proto_schedule(schedule),
            status: TaskStatus::Pending,
            created_at: Utc::now(),
            last_run_at: None,
            next_run_at: Some(Utc::now()),
            payload: serde_json::from_str(&req.payload).unwrap_or_default(),
        };

        self.hub.scheduler().create(task.clone())
            .map_err(|e| Status::internal(e))?;

        Ok(Response::new(convert_to_proto(task)))
    }

    async fn cancel_scheduled_task(
        &self,
        request: Request<CancelScheduledTaskRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        let spiffe_id_str = ::server_auth::extract_spiffe_id_from_metadata(request.metadata()).map_err(|e| Status::unauthenticated(e))?;
        let (tenant_id, _) = ::server_auth::parse_spiffe_id(&spiffe_id_str)?;
        let org_id = if tenant_id.is_empty() { "system".to_string() } else { tenant_id };


        let req = request.into_inner();
        self.hub.scheduler().cancel(&org_id, &req.id)
            .map_err(|e| Status::not_found(e))?;
            
        Ok(Response::new(EmptyResponse {}))
    }
}

fn convert_to_proto(t: Task) -> ProtoTask {
    ProtoTask {
        id: t.id,
        organization_id: t.organization_id,
        agent_id: t.agent_id,
        name: t.name,
        schedule: Some(ProtoSchedule {
            r#type: format!("{:?}", t.schedule.r#type),
            at_unix: t.schedule.at.map(|dt| dt.timestamp()).unwrap_or(0),
            interval_s: t.schedule.interval_s.unwrap_or(0),
            expression: t.schedule.expression.unwrap_or_default(),
        }),
        status: format!("{:?}", t.status),
        created_at_unix: t.created_at.timestamp(),
        last_run_at_unix: t.last_run_at.map(|dt| dt.timestamp()).unwrap_or(0),
        next_run_at_unix: t.next_run_at.map(|dt| dt.timestamp()).unwrap_or(0),
        payload: serde_json::to_string(&t.payload).unwrap_or_default(),
    }
}

fn convert_from_proto_schedule(s: ProtoSchedule) -> Schedule {
    let r#type = match s.r#type.as_str() {
        "Once" => ScheduleType::Once,
        "Interval" => ScheduleType::Interval,
        "Cron" => ScheduleType::Cron,
        _ => ScheduleType::Once,
    };
    
    Schedule {
        r#type,
        at: if s.at_unix > 0 { Some(Utc.timestamp_opt(s.at_unix, 0).unwrap()) } else { None },
        interval_s: if s.interval_s > 0 { Some(s.interval_s) } else { None },
        expression: if !s.expression.is_empty() { Some(s.expression) } else { None },
    }
}
