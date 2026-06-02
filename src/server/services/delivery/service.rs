use ::server_ohc::delivery::delivery_service_server::DeliveryService;
use ::server_ohc::delivery::{
    CreateDeliveryZoneRequest, CreateDeliveryZoneResponse, DeliveryTask, DeliveryZone, GetDailyItineraryRequest, GetDailyItineraryResponse, GetDeliveryZonesRequest, GetDeliveryZonesResponse, RoutePlan, UpdateTaskStatusRequest, UpdateTaskStatusResponse, Point, Polygon, Waypoint
};
use tonic::{Request, Response, Status};

pub struct MyDeliveryService {}

impl MyDeliveryService {
    pub fn new() -> Self {
        Self {}
    }
}

#[tonic::async_trait]
impl DeliveryService for MyDeliveryService {
    async fn create_delivery_zone(
        &self,
        request: Request<CreateDeliveryZoneRequest>,
    ) -> Result<Response<CreateDeliveryZoneResponse>, Status> {
        let req = request.into_inner();
        let zone = req.zone.ok_or_else(|| Status::invalid_argument("zone is missing"))?;

        let spiffe_id_str = ::server_auth::extract_spiffe_id_from_metadata(request.metadata())
            .map_err(|e| Status::unauthenticated(e))?;
        let (tenant_id, _) = ::server_auth::parse_spiffe_id(&spiffe_id_str)?;
        let mut zone = zone;
        zone.id = uuid::Uuid::new_v4().to_string();
        zone.tenant_id = tenant_id.clone();

        Ok(Response::new(CreateDeliveryZoneResponse {
            zone: Some(zone),
        }))
    }

    async fn get_delivery_zones(
        &self,
        request: Request<GetDeliveryZonesRequest>,
    ) -> Result<Response<GetDeliveryZonesResponse>, Status> {
        let _spiffe_id_str = ::server_auth::extract_spiffe_id_from_metadata(request.metadata())
            .map_err(|e| Status::unauthenticated(e))?;

        let zones = vec![];
        Ok(Response::new(GetDeliveryZonesResponse { zones }))
    }

    async fn get_daily_itinerary(
        &self,
        request: Request<GetDailyItineraryRequest>,
    ) -> Result<Response<GetDailyItineraryResponse>, Status> {
        let _spiffe_id_str = ::server_auth::extract_spiffe_id_from_metadata(request.metadata())
            .map_err(|e| Status::unauthenticated(e))?;

        Ok(Response::new(GetDailyItineraryResponse {
            route_plan: None,
            tasks: vec![],
        }))
    }

    async fn update_task_status(
        &self,
        request: Request<UpdateTaskStatusRequest>,
    ) -> Result<Response<UpdateTaskStatusResponse>, Status> {
        let _req = request.into_inner();

        Ok(Response::new(UpdateTaskStatusResponse {
            task: None,
        }))
    }
}
