use tonic::{Request, Response, Status};
use ::server_ohc::orchestration::*;
use ::server_ohc::orchestration::workforce_service_server::WorkforceService;
use std::sync::{Arc, RwLock};
use chrono::Utc;
use crate::hub::Hub;
use ::server_utils::cache::HybridCache;
use std::sync::OnceLock;
use uuid::Uuid;

static STAFF_CACHE: OnceLock<HybridCache<Vec<StaffMember>>> = OnceLock::new();
static SHIFTS_CACHE: OnceLock<HybridCache<Vec<Shift>>> = OnceLock::new();
static TASKS_CACHE: OnceLock<HybridCache<Vec<Task>>> = OnceLock::new();

pub struct MyWorkforceService {
    hub: Arc<Hub>,
    staff: RwLock<Vec<StaffMember>>,
    shifts: RwLock<Vec<Shift>>,
    tasks: RwLock<Vec<Task>>,
}

impl MyWorkforceService {
    pub fn new(hub: Arc<Hub>) -> Self {
        MyWorkforceService {
            hub,
            staff: RwLock::new(Vec::new()),
            shifts: RwLock::new(Vec::new()),
            tasks: RwLock::new(Vec::new()),
        }
    }
}

#[tonic::async_trait]
impl WorkforceService for MyWorkforceService {
    async fn get_staff(
        &self,
        request: Request<GetStaffRequest>,
    ) -> Result<Response<GetStaffResponse>, Status> {
        let req = request.into_inner();
        let cache_key = format!("staff_{}", req.tenant_id);
        let cache = STAFF_CACHE.get_or_init(|| HybridCache::new(self.hub.redis_client.clone()));

        if let Some(staff) = cache.get(&cache_key).await {
            return Ok(Response::new(GetStaffResponse { staff }));
        }

        let staff: Vec<StaffMember> = self.staff.read().unwrap()
            .iter()
            .filter(|s| s.tenant_id == req.tenant_id)
            .cloned()
            .collect();

        cache.set(&cache_key, staff.clone(), std::time::Duration::from_secs(5)).await;

        Ok(Response::new(GetStaffResponse {
            staff,
        }))
    }

    async fn clock_in(
        &self,
        request: Request<ClockInRequest>,
    ) -> Result<Response<ClockInResponse>, Status> {
        let req = request.into_inner();
        let now = prost_types::Timestamp {
            seconds: Utc::now().timestamp(),
            nanos: 0,
        };

        let shift = Shift {
            id: Uuid::new_v4().to_string(),
            tenant_id: req.tenant_id.clone(),
            staff_member_id: req.staff_member_id.clone(),
            start_time: Some(now.clone()),
            end_time: None,
            status: "ACTIVE".to_string(),
        };

        {
            let mut shifts = self.shifts.write().unwrap();
            shifts.push(shift.clone());
        }

        {
            let mut staff = self.staff.write().unwrap();
            if let Some(s) = staff.iter_mut().find(|s| s.id == req.staff_member_id && s.tenant_id == req.tenant_id) {
                s.status = "CLOCKED_IN".to_string();
                s.updated_at = Some(now);
            }
        }

        let cache = SHIFTS_CACHE.get_or_init(|| HybridCache::new(self.hub.redis_client.clone()));
        cache.invalidate(&format!("shifts_{}", req.tenant_id)).await;

        let staff_cache = STAFF_CACHE.get_or_init(|| HybridCache::new(self.hub.redis_client.clone()));
        staff_cache.invalidate(&format!("staff_{}", req.tenant_id)).await;

        Ok(Response::new(ClockInResponse { shift: Some(shift) }))
    }

    async fn clock_out(
        &self,
        request: Request<ClockOutRequest>,
    ) -> Result<Response<ClockOutResponse>, Status> {
        let req = request.into_inner();
        let now = prost_types::Timestamp {
            seconds: Utc::now().timestamp(),
            nanos: 0,
        };

        let mut updated_shift = None;
        {
            let mut shifts = self.shifts.write().unwrap();
            if let Some(s) = shifts.iter_mut().find(|s| s.staff_member_id == req.staff_member_id && s.tenant_id == req.tenant_id && s.status == "ACTIVE") {
                s.status = "COMPLETED".to_string();
                s.end_time = Some(now.clone());
                updated_shift = Some(s.clone());
            }
        }

        {
            let mut staff = self.staff.write().unwrap();
            if let Some(s) = staff.iter_mut().find(|s| s.id == req.staff_member_id && s.tenant_id == req.tenant_id) {
                s.status = "CLOCKED_OUT".to_string();
                s.updated_at = Some(now);
            }
        }

        let cache = SHIFTS_CACHE.get_or_init(|| HybridCache::new(self.hub.redis_client.clone()));
        cache.invalidate(&format!("shifts_{}", req.tenant_id)).await;

        let staff_cache = STAFF_CACHE.get_or_init(|| HybridCache::new(self.hub.redis_client.clone()));
        staff_cache.invalidate(&format!("staff_{}", req.tenant_id)).await;

        if let Some(shift) = updated_shift {
            Ok(Response::new(ClockOutResponse { shift: Some(shift) }))
        } else {
            Err(Status::not_found("No active shift found"))
        }
    }

    async fn get_tasks(
        &self,
        request: Request<GetTasksRequest>,
    ) -> Result<Response<GetTasksResponse>, Status> {
        let req = request.into_inner();
        let cache_key = format!("tasks_{}_{}", req.tenant_id, req.staff_member_id);
        let cache = TASKS_CACHE.get_or_init(|| HybridCache::new(self.hub.redis_client.clone()));

        if let Some(tasks) = cache.get(&cache_key).await {
            return Ok(Response::new(GetTasksResponse { tasks }));
        }

        let tasks: Vec<Task> = self.tasks.read().unwrap()
            .iter()
            .filter(|t| t.tenant_id == req.tenant_id && t.staff_member_id == req.staff_member_id)
            .cloned()
            .collect();

        cache.set(&cache_key, tasks.clone(), std::time::Duration::from_secs(5)).await;

        Ok(Response::new(GetTasksResponse {
            tasks,
        }))
    }

    async fn complete_task(
        &self,
        request: Request<CompleteTaskRequest>,
    ) -> Result<Response<CompleteTaskResponse>, Status> {
        let req = request.into_inner();
        let now = prost_types::Timestamp {
            seconds: Utc::now().timestamp(),
            nanos: 0,
        };

        let mut updated_task = None;
        {
            let mut tasks = self.tasks.write().unwrap();
            if let Some(t) = tasks.iter_mut().find(|t| t.id == req.task_id && t.tenant_id == req.tenant_id) {
                t.status = "COMPLETED".to_string();
                t.updated_at = Some(now.clone());
                updated_task = Some(t.clone());
            }
        }

        if let Some(task) = updated_task {
            let cache = TASKS_CACHE.get_or_init(|| HybridCache::new(self.hub.redis_client.clone()));
            cache.invalidate(&format!("tasks_{}_{}", req.tenant_id, task.staff_member_id)).await;

            // Generate a business event (e.g. order completed) in a real system here

            Ok(Response::new(CompleteTaskResponse { task: Some(task) }))
        } else {
            Err(Status::not_found("Task not found"))
        }
    }
}
