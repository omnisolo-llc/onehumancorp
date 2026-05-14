use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct DailyCoRoomPayload {
    pub booking_id: String,
}

#[derive(Debug, Serialize)]
pub struct DailyCoRoomResponse {
    pub status: String,
    pub room_url: String,
}

pub async fn handle_dailyco_room(
    Json(payload): Json<DailyCoRoomPayload>,
) -> Json<DailyCoRoomResponse> {
    tracing::info!("Received Daily.co room creation request: {:?}", payload);

    // In a real implementation, we would call the Daily.co API to generate a room
    // and store it along with the booking.

    Json(DailyCoRoomResponse {
        status: "ok".to_string(),
        room_url: format!("https://ohc-mock.daily.co/room_{}", payload.booking_id),
    })
}
