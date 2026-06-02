use tonic::{Request, Response, Status};
use std::sync::Arc;
use crate::db::DB;
use chrono::NaiveDate;
use serde_json;

use ::server_ohc::delivery::delivery_service_server::DeliveryService;
use ::server_ohc::delivery::{
    ConfigureDeliveryZoneRequest, ConfigureDeliveryZoneResponse, DeliveryTask as ProtoDeliveryTask,
    DeliveryZone as ProtoDeliveryZone, FetchDailyItineraryRequest, FetchDailyItineraryResponse,
    RoutePlan as ProtoRoutePlan, UpdateDeliveryTaskStatusRequest, UpdateDeliveryTaskStatusResponse,
};
use crate::domain::repository::delivery_repo::DeliveryRepo;

pub struct DeliveryApi {
    pub db: Arc<DB>,
}

#[tonic::async_trait]
impl DeliveryService for DeliveryApi {
    async fn configure_delivery_zone(
        &self,
        request: Request<ConfigureDeliveryZoneRequest>,
    ) -> Result<Response<ConfigureDeliveryZoneResponse>, Status> {
        let req = request.into_inner();
        let repo = DeliveryRepo::new(self.db.clone());

        match repo
            .configure_delivery_zone(
                &req.tenant_id,
                &req.polygon,
                req.flat_fee,
                req.min_order_value,
            )
            .await
        {
            Ok(zone) => {
                let proto_zone = ProtoDeliveryZone {
                    id: zone.id,
                    tenant_id: zone.tenant_id,
                    polygon: zone.polygon,
                    flat_fee: zone.flat_fee,
                    min_order_value: zone.min_order_value,
                    created_at_unix: zone.created_at.timestamp(),
                    updated_at_unix: zone.updated_at.timestamp(),
                };
                Ok(Response::new(ConfigureDeliveryZoneResponse {
                    zone: Some(proto_zone),
                }))
            }
            Err(e) => Err(Status::internal(e)),
        }
    }

    async fn fetch_daily_itinerary(
        &self,
        request: Request<FetchDailyItineraryRequest>,
    ) -> Result<Response<FetchDailyItineraryResponse>, Status> {
        let req = request.into_inner();
        let repo = DeliveryRepo::new(self.db.clone());

        let (plan_opt, tasks) = repo
            .get_daily_itinerary(&req.tenant_id, &req.delivery_date)
            .await
            .map_err(|e| Status::internal(e))?;

        let mut proto_tasks: Vec<ProtoDeliveryTask> = tasks
            .into_iter()
            .map(|t| ProtoDeliveryTask {
                id: t.id,
                tenant_id: t.tenant_id,
                order_id: t.order_id,
                driver_id: t.driver_id.unwrap_or_default(),
                status: t.status,
                estimated_arrival_unix: t
                    .estimated_arrival
                    .map(|d| d.timestamp())
                    .unwrap_or(0),
                delivery_location: t.delivery_location,
                created_at_unix: t.created_at.timestamp(),
                updated_at_unix: t.updated_at.timestamp(),
            })
            .collect();

        let route_plan = if let Some(plan) = plan_opt {
            plan
        } else {
            let task_ids: Vec<String> = proto_tasks.iter().map(|t| t.id.clone()).collect();
            let waypoint_seq_json = serde_json::to_string(&task_ids).unwrap_or("[]".to_string());
            let naive_date = NaiveDate::parse_from_str(&req.delivery_date, "%Y-%m-%d")
                .map_err(|_| Status::invalid_argument("Invalid delivery_date format"))?;

            repo.upsert_route_plan(&req.tenant_id, naive_date, &waypoint_seq_json)
                .await
                .map_err(|e| Status::internal(e))?
        };

        let proto_route_plan = ProtoRoutePlan {
            id: route_plan.id,
            tenant_id: route_plan.tenant_id,
            delivery_date: route_plan.delivery_date.format("%Y-%m-%d").to_string(),
            waypoint_sequence: route_plan.waypoint_sequence.clone(),
            created_at_unix: route_plan.created_at.timestamp(),
            updated_at_unix: route_plan.updated_at.timestamp(),
        };

        if let Ok(seq) = serde_json::from_str::<Vec<String>>(&route_plan.waypoint_sequence) {
            proto_tasks.sort_by_key(|t| seq.iter().position(|id| id == &t.id).unwrap_or(usize::MAX));
        }

        Ok(Response::new(FetchDailyItineraryResponse {
            route_plan: Some(proto_route_plan),
            tasks: proto_tasks,
        }))
    }

    async fn update_delivery_task_status(
        &self,
        request: Request<UpdateDeliveryTaskStatusRequest>,
    ) -> Result<Response<UpdateDeliveryTaskStatusResponse>, Status> {
        let req = request.into_inner();
        let repo = DeliveryRepo::new(self.db.clone());

        match repo
            .update_delivery_task_status(&req.tenant_id, &req.task_id, &req.status)
            .await
        {
            Ok(t) => {
                let proto_task = ProtoDeliveryTask {
                    id: t.id,
                    tenant_id: t.tenant_id,
                    order_id: t.order_id,
                    driver_id: t.driver_id.unwrap_or_default(),
                    status: t.status,
                    estimated_arrival_unix: t
                        .estimated_arrival
                        .map(|d| d.timestamp())
                        .unwrap_or(0),
                    delivery_location: t.delivery_location,
                    created_at_unix: t.created_at.timestamp(),
                    updated_at_unix: t.updated_at.timestamp(),
                };
                Ok(Response::new(UpdateDeliveryTaskStatusResponse {
                    task: Some(proto_task),
                }))
            }
            Err(_) => Err(Status::not_found("Task not found")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::DB;
    use std::sync::Arc;
    use ::server_ohc::delivery::delivery_service_server::DeliveryService;

    #[tokio::test]
    async fn test_delivery_api() {
        let db = Arc::new(DB::new().await);
        let api = DeliveryApi { db };

        let tenant_id = "tenant-api-1";

        let req = ConfigureDeliveryZoneRequest {
            tenant_id: tenant_id.to_string(),
            polygon: "{}".to_string(),
            flat_fee: 15.0,
            min_order_value: 30,
        };

        let res = api.configure_delivery_zone(Request::new(req)).await.unwrap().into_inner();
        assert_eq!(res.zone.unwrap().flat_fee, 15.0);
    }
}
