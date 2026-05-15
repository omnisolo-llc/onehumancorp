use tonic::{Request, Response, Status};
use tokio_stream::{Stream, StreamExt};
use std::pin::Pin;
use std::sync::Arc;
use crate::MyHubService;
use ::server_ohc::orchestration::*;
use ::server_ohc::orchestration::hub_service_server::HubService;
use crate::hub::Hub;
use chrono::Utc;

impl MyHubService {
    pub async fn impl_delegate_task(
        &self,
        request: Request<DelegateTaskRequest>,
    ) -> Result<Response<DelegateTaskResponse>, Status> {
        let req = request.into_inner();
        if let Some(task) = req.task {
            match self.hub.clone().delegate_task(req.from_agent_id, req.to_agent_id, task) {
                Ok(_) => Ok(Response::new(DelegateTaskResponse { success: true })),
                Err(e) => Err(Status::internal(e)),
            }
        } else {
            Err(Status::invalid_argument("task is required"))
        }
    }
    pub async fn impl_create_task(
        &self,
        request: Request<CreateTaskRequest>,
    ) -> Result<Response<SharedTask>, Status> {
        let req = request.into_inner();
        let task = self.hub.task_manager().create_task(
            "default_org".to_string(),
            req.mission_id,
            req.title,
            req.description,
            req.priority,
        ).map_err(|e| Status::internal(e))?;

        Ok(Response::new(SharedTask {
            id: task.id,
            organization_id: task.organization_id,
            parent_plan_id: task.parent_plan_id,
            dependencies: task.dependencies,
            title: task.title,
            description: task.description.unwrap_or_default(),
            status: task.status,
            assigned_agent_id: task.assigned_agent_id.unwrap_or_default(),
            priority: task.priority,
            payload: task.payload,
            locked_until_unix: task.locked_until.map(|t| t.timestamp()).unwrap_or(0),
            created_at_unix: task.created_at.timestamp(),
            updated_at_unix: task.updated_at.timestamp(),
            action_risk: match task.action_risk {
                Some(crate::tasks::ActionRisk::Low) => 1,
                Some(crate::tasks::ActionRisk::High) => 2,
                _ => 0,
            },
            approval_status: task.approval_status.unwrap_or_default(),
            proposed_content: task.proposed_content.unwrap_or_default(),
        }))
    }
    pub async fn impl_poll_tasks(
        &self,
        request: Request<PollTasksRequest>,
    ) -> Result<Response<<MyHubService as HubService>::PollTasksStream>, Status> {
        let req = request.into_inner();
        let tasks = self.hub.task_manager().poll_tasks(&req.agent_id, req.limit as usize);

        let mapped_tasks: Vec<Result<SharedTask, Status>> = tasks.into_iter().map(|task| {
            Ok(SharedTask {
                id: task.id,
                organization_id: task.organization_id,
                parent_plan_id: task.parent_plan_id,
                dependencies: task.dependencies,
                title: task.title,
                description: task.description.unwrap_or_default(),
                status: task.status,
                assigned_agent_id: task.assigned_agent_id.unwrap_or_default(),
                priority: task.priority,
                payload: task.payload,
                locked_until_unix: task.locked_until.map(|t| t.timestamp()).unwrap_or(0),
                created_at_unix: task.created_at.timestamp(),
                updated_at_unix: task.updated_at.timestamp(),
                action_risk: match task.action_risk {
                    Some(crate::tasks::ActionRisk::Low) => 1,
                    Some(crate::tasks::ActionRisk::High) => 2,
                    _ => 0,
                },
                approval_status: task.approval_status.unwrap_or_default(),
                proposed_content: task.proposed_content.unwrap_or_default(),
            })
        }).collect();

        let stream = tokio_stream::iter(mapped_tasks);
        Ok(Response::new(Box::pin(stream) as <MyHubService as HubService>::PollTasksStream))
    }
    pub async fn impl_update_task_status(
        &self,
        request: Request<UpdateTaskStatusRequest>,
    ) -> Result<Response<UpdateTaskStatusResponse>, Status> {
        let req = request.into_inner();

        match req.status.as_str() {
            "REVIEW" => {
                self.hub.task_manager().review_task(&req.task_id, &req.agent_id)
                    .map_err(|e| Status::internal(e))?;
            }
            "COMPLETED" => {
                self.hub.task_manager().complete_task(&req.task_id, &req.agent_id, req.result)
                    .map_err(|e| Status::internal(e))?;
            }
            _ => {
                self.hub.task_manager().update_task_status(&req.task_id, req.status)
                    .map_err(|e| Status::internal(e))?;
            }
        }

        Ok(Response::new(UpdateTaskStatusResponse { success: true }))
    }
    pub async fn impl_approve_task(
        &self,
        request: Request<ApproveTaskRequest>,
    ) -> Result<Response<ApproveTaskResponse>, Status> {
        let org_id = request.extensions().get::<::server_common::Claims>()
            .ok_or_else(|| Status::unauthenticated("Missing claims"))?
            .organization_id.as_ref()
            .ok_or_else(|| Status::unauthenticated("Missing org_id"))?
            .clone();

        let req = request.into_inner();
        self.hub.task_manager().approve_task(&req.task_id, req.is_approved, &org_id).await
            .map_err(|e| Status::internal(e))?;

        Ok(Response::new(ApproveTaskResponse {
            success: true,
        }))
    }
    pub async fn impl_get_pending_approvals(
        &self,
        request: Request<GetPendingApprovalsRequest>,
    ) -> Result<Response<GetPendingApprovalsResponse>, Status> {
        let req = request.into_inner();
        let tasks = self.hub.task_manager().get_pending_approvals(&req.organization_id);

        let mapped_tasks: Vec<SharedTask> = tasks.into_iter().map(|task| {
            SharedTask {
                id: task.id,
                organization_id: task.organization_id,
                parent_plan_id: task.parent_plan_id,
                dependencies: task.dependencies,
                title: task.title,
                description: task.description.unwrap_or_default(),
                status: task.status,
                assigned_agent_id: task.assigned_agent_id.unwrap_or_default(),
                priority: task.priority,
                payload: task.payload,
                locked_until_unix: task.locked_until.map(|t| t.timestamp()).unwrap_or(0),
                created_at_unix: task.created_at.timestamp(),
                updated_at_unix: task.updated_at.timestamp(),
                action_risk: match task.action_risk {
                    Some(crate::tasks::ActionRisk::Low) => 1,
                    Some(crate::tasks::ActionRisk::High) => 2,
                    _ => 0,
                },
                approval_status: task.approval_status.unwrap_or_default(),
                proposed_content: task.proposed_content.unwrap_or_default(),
            }
        }).collect();

        Ok(Response::new(GetPendingApprovalsResponse {
            tasks: mapped_tasks,
        }))
    }
    pub async fn impl_decompose_task(
        &self,
        request: Request<DecomposeTaskRequest>,
    ) -> Result<Response<DecomposeTaskResponse>, Status> {
        let req = request.into_inner();

        for st in req.sub_tasks {
            let mut filtered_deps = Vec::new();
            for dep in st.dependencies {
                if dep != req.task_id {
                    filtered_deps.push(dep);
                }
            }

            self.hub.task_manager().create_task_with_plan(
                req.organization_id.clone(),
                String::new(),
                req.task_id.clone(),
                filtered_deps,
                st.title,
                st.description,
                st.priority,
            ).map_err(|e| Status::internal(e))?;
        }

        Ok(Response::new(DecomposeTaskResponse { success: true }))
    }
    pub async fn impl_delegate_sub_task(
        &self,
        request: Request<SubTask>,
    ) -> Result<Response<DelegateTaskResponse>, Status> {
        let req = request.into_inner();

        if req.task_id.is_empty() || req.target_role.is_empty() {
            return Err(Status::invalid_argument("task_id and target_role are required"));
        }

        if self.hub.get_agent(&req.from_agent_id).is_none() {
            return Err(Status::invalid_argument("sender agent is not registered"));
        }

        // Quota Enforcement
        if self.hub.get_agents_count() >= 10 {
            return Err(Status::resource_exhausted("VRAM quota limit exceeded, cannot spawn sub-agent"));
        }

        let now_nano = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
        let sub_agent_id = format!("sub-agent-{}-{}", req.target_role, now_nano);

        let sub_agent = Agent {
            id: sub_agent_id.clone(),
            name: format!("Specialized {} Agent", req.target_role),
            role: req.target_role.clone(),
            organization_id: "dynamic-delegation".to_string(),
            status: "IDLE".to_string(),
            provider_type: "builtin".to_string(),
        };

        self.hub.register_agent(sub_agent);

        // Prompt injection checks
        if req.instruction.contains("SYSTEM:") || req.instruction.contains("\n\n") {
            return Err(Status::invalid_argument("instruction contains forbidden prompt injection sequences"));
        }
        if req.parent_thread_id.contains("SYSTEM:") || req.parent_thread_id.contains("\n\n") {
            return Err(Status::invalid_argument("parent_thread_id contains forbidden prompt injection sequences"));
        }

        // Delegate to K8s Operator
        let pod_id = crate::orchestration::hierarchical::K8sOperatorDelegator::spawn_sub_agent_pod(
            &req.target_role,
            &req.instruction,
            &req.parent_thread_id,
        ).await.map_err(|e| Status::internal(e))?;
        tracing::info!("Spawned K8s Pod {} for Hierarchical Task Delegation", pod_id);

        let msg_id = format!("msg-{}-{}", req.task_id, now_nano);
        let msg = Message {
            id: msg_id,
            from_agent: req.from_agent_id,
            to_agent: sub_agent_id,
             r#type: "TaskDelegation".to_string(),
            content: format!("Execute Task: {}\nContext: {}\nK8sPod: {}", req.instruction, req.parent_thread_id, pod_id),
            occurred_at_unix: Utc::now().timestamp(),
            meeting_id: String::new(),
        };

        match self.hub.clone().publish(msg) {
            Ok(_) => Ok(Response::new(DelegateTaskResponse { success: true })),
            Err(e) => Err(Status::internal(e)),
        }
    }
}
