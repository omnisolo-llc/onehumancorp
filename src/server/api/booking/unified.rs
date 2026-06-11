use axum::{
    extract::State,
    response::IntoResponse,
    Json,
};
use sqlx::Row;
use ::server_ohc::app::{
    GetResourcesRequest, GetResourcesResponse, BookingResource,
    GetServicesRequest, GetServicesResponse, ServiceDefinition, ServiceResourceRequirement,
    CreateUnifiedBookingRequest, CreateUnifiedBookingResponse, UnifiedBooking,
};

pub async fn get_resources(
    State(pool): State<sqlx::PgPool>,
    Json(payload): Json<GetResourcesRequest>,
) -> impl IntoResponse {
    let tenant_id = payload.tenant_id;

    let resources = sqlx::query(
        r#"
        SELECT id, name, resource_type, availability_schedule
        FROM booking_resources
        WHERE tenant_id = $1
        "#,
    )
    .bind(&tenant_id)
    .fetch_all(&pool)
    .await;

    match resources {
        Ok(rows) => {
            let resources = rows.into_iter().map(|row| BookingResource {
                id: row.get("id"),
                name: row.get("name"),
                resource_type: row.get("resource_type"),
                availability_schedule: row.get::<serde_json::Value, _>("availability_schedule").to_string(),
            }).collect();

            Json(GetResourcesResponse { resources }).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to fetch resources: {}", e);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_services(
    State(pool): State<sqlx::PgPool>,
    Json(payload): Json<GetServicesRequest>,
) -> impl IntoResponse {
    let tenant_id = payload.tenant_id;

    // Fetch services and their resource requirements
    let services = sqlx::query(
        r#"
        SELECT s.id, s.title, s.description, s.price_cents
        FROM services s
        WHERE s.tenant_id = $1
        "#,
    )
    .bind(&tenant_id)
    .fetch_all(&pool)
    .await;

    match services {
        Ok(rows) => {
            let mut service_definitions = Vec::new();
            for row in rows {
                let service_id: String = row.get("id");

                let reqs = sqlx::query(
                    r#"
                    SELECT resource_type, quantity
                    FROM service_resource_requirements
                    WHERE service_id = $1
                    "#,
                )
                .bind(&service_id)
                .fetch_all(&pool)
                .await
                .unwrap_or_default();

                let resource_requirements = reqs.into_iter().map(|r| ServiceResourceRequirement {
                    resource_type: r.get("resource_type"),
                    quantity: r.get::<i32, _>("quantity") as i32,
                }).collect();

                service_definitions.push(ServiceDefinition {
                    id: service_id,
                    title: row.get("title"),
                    description: row.try_get("description").unwrap_or_default(),
                    price_cents: row.get("price_cents"),
                    resource_requirements,
                });
            }

            Json(GetServicesResponse { services: service_definitions }).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to fetch services: {}", e);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn create_unified_booking(
    State(pool): State<sqlx::PgPool>,
    Json(payload): Json<CreateUnifiedBookingRequest>,
) -> impl IntoResponse {
    let tenant_id = payload.tenant_id;
    let service_id = payload.service_id;
    let start_time = payload.start_time;
    let end_time = payload.end_time;
    let customer_id = payload.customer_id;

    // Use a transaction for atomic booking creation and resource reservation
    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            tracing::error!("Failed to start transaction: {}", e);
            return Json(CreateUnifiedBookingResponse {
                success: false,
                booking: None,
                error: "Internal server error".to_string(),
            }).into_response();
        }
    };

    // 1. Determine resource requirements
    let reqs = match sqlx::query(
        r#"
        SELECT resource_type, quantity
        FROM service_resource_requirements
        WHERE service_id = $1
        "#
    )
    .bind(&service_id)
    .fetch_all(&mut *tx)
    .await {
        Ok(rows) => rows,
        Err(e) => {
            tracing::error!("Failed to fetch resource requirements: {}", e);
            return Json(CreateUnifiedBookingResponse {
                success: false,
                booking: None,
                error: "Failed to fetch resource requirements".to_string(),
            }).into_response();
        }
    };

    let mut locked_resource_ids = Vec::new();

    for req in reqs {
        let r_type: String = req.get("resource_type");
        let qty: i32 = req.get("quantity");

        // 2. Find and lock available resources
        // We use SKIP LOCKED to safely find resources not concurrently booked
        let available_resources = match sqlx::query(
            r#"
            SELECT r.id
            FROM booking_resources r
            WHERE r.tenant_id = $1 AND r.resource_type = $2
            AND r.id NOT IN (
                SELECT brr.resource_id
                FROM booking_resource_reservations brr
                JOIN bookings b ON brr.booking_id = b.id
                WHERE b.tenant_id = $1
                AND (
                    (b.start_time < $4::timestamptz AND b.end_time > $3::timestamptz)
                )
                AND b.status IN ('pending', 'confirmed')
            )
            LIMIT $5
            FOR UPDATE SKIP LOCKED
            "#
        )
        .bind(&tenant_id)
        .bind(&r_type)
        .bind(&start_time)
        .bind(&end_time)
        .bind(qty)
        .fetch_all(&mut *tx)
        .await {
            Ok(rows) => rows,
            Err(e) => {
                tracing::error!("Failed to find available resources: {}", e);
                return Json(CreateUnifiedBookingResponse {
                    success: false,
                    booking: None,
                    error: "Failed to find available resources".to_string(),
                }).into_response();
            }
        };

        if available_resources.len() < qty as usize {
            // Not enough resources available
            return Json(CreateUnifiedBookingResponse {
                success: false,
                booking: None,
                error: format!("Not enough resources available for type {}", r_type),
            }).into_response();
        }

        for row in available_resources {
            locked_resource_ids.push(row.get::<String, _>("id"));
        }
    }

    // 3. Create the booking
    let booking_id = uuid::Uuid::new_v4().to_string();

    if let Err(e) = sqlx::query(
        r#"
        INSERT INTO bookings (id, tenant_id, customer_id, product_id, start_time, end_time, status)
        VALUES ($1, $2, $3, $4, $5::timestamptz, $6::timestamptz, 'confirmed')
        "#
    )
    .bind(&booking_id)
    .bind(&tenant_id)
    .bind(&customer_id)
    .bind(&service_id) // using service_id as product_id for now
    .bind(&start_time)
    .bind(&end_time)
    .execute(&mut *tx)
    .await {
        tracing::error!("Failed to create booking: {}", e);
        return Json(CreateUnifiedBookingResponse {
            success: false,
            booking: None,
            error: "Failed to create booking".to_string(),
        }).into_response();
    }

    // 4. Create resource reservations
    for res_id in &locked_resource_ids {
        let res_res_id = uuid::Uuid::new_v4().to_string();
        if let Err(e) = sqlx::query(
            r#"
            INSERT INTO booking_resource_reservations (id, tenant_id, booking_id, resource_id)
            VALUES ($1, $2, $3, $4)
            "#
        )
        .bind(&res_res_id)
        .bind(&tenant_id)
        .bind(&booking_id)
        .bind(res_id)
        .execute(&mut *tx)
        .await {
            tracing::error!("Failed to reserve resource: {}", e);
            return Json(CreateUnifiedBookingResponse {
                success: false,
                booking: None,
                error: "Failed to reserve resource".to_string(),
            }).into_response();
        }
    }

    // Commit transaction
    if let Err(e) = tx.commit().await {
        tracing::error!("Failed to commit transaction: {}", e);
        return Json(CreateUnifiedBookingResponse {
            success: false,
            booking: None,
            error: "Failed to complete booking process".to_string(),
        }).into_response();
    }

    let booking = UnifiedBooking {
        id: booking_id,
        customer_id,
        service_id,
        start_time,
        end_time,
        status: "confirmed".to_string(),
        locked_resource_ids,
    };

    Json(CreateUnifiedBookingResponse {
        success: true,
        booking: Some(booking),
        error: String::new(),
    }).into_response()
}
