use serde_json::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct TrackingUpdate {
    pub provider: &'static str,
    pub provider_object_id: Option<String>,
    pub tracking_number: Option<String>,
    pub carrier: Option<String>,
    pub status: String,
    pub status_at_ms: i64,
    pub is_test: Option<bool>,
    pub driver_id: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

pub fn text(value: &Value, key: &str, maximum: usize) -> Result<String, String> {
    let text = value
        .get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("missing {key}"))?;
    if text.trim().is_empty() || text.len() > maximum || text.chars().any(char::is_control) {
        return Err(format!("invalid {key}"));
    }
    Ok(text.to_owned())
}
fn optional_text(value: &Value, key: &str, maximum: usize) -> Result<Option<String>, String> {
    match value.get(key) {
        None | Some(Value::Null) => Ok(None),
        _ => text(value, key, maximum).map(Some),
    }
}
fn date(value: &Value, key: &str) -> Result<i64, String> {
    chrono::DateTime::parse_from_rfc3339(&text(value, key, 64)?)
        .map(|date| date.timestamp_millis())
        .map_err(|_| format!("invalid {key}"))
}

pub fn parse_shippo(payload: &Value) -> Result<TrackingUpdate, String> {
    let event = text(payload, "event", 64)?;
    if event != "track_updated" {
        return Err(format!("Ignoring Shippo webhook event: {event}"));
    }
    let data = payload.get("data").ok_or("missing tracking data")?;
    let tracking_number = text(data, "tracking_number", 256)?;
    let carrier = text(data, "carrier", 128)?;
    if !carrier
        .bytes()
        .all(|c| c.is_ascii_alphanumeric() || c == b'_')
    {
        return Err("invalid carrier token".into());
    }
    let status_data = data
        .get("tracking_status")
        .ok_or("missing tracking_status")?;
    let status = text(status_data, "status", 32)?;
    if !matches!(
        status.as_str(),
        "UNKNOWN" | "PRE_TRANSIT" | "TRANSIT" | "DELIVERED" | "RETURNED" | "FAILURE"
    ) {
        return Err("unsupported tracking status".into());
    }
    let status_at_ms = date(status_data, "status_date")?;
    let top_mode = payload.get("test").and_then(Value::as_bool);
    let data_mode = data.get("test").and_then(Value::as_bool);
    if top_mode.is_some() && data_mode.is_some() && top_mode != data_mode {
        return Err("conflicting tracking mode".into());
    }
    let is_test = top_mode.or(data_mode).ok_or("missing tracking mode")?;
    Ok(TrackingUpdate {
        provider: "shippo",
        provider_object_id: optional_text(data, "transaction", 128)?,
        tracking_number: Some(tracking_number),
        carrier: Some(carrier),
        status,
        status_at_ms,
        is_test: Some(is_test),
        driver_id: None,
        latitude: None,
        longitude: None,
    })
}

pub fn parse_doordash(payload: &Value) -> Result<TrackingUpdate, String> {
    let data = payload
        .get("data")
        .filter(|value| value.is_object())
        .unwrap_or(payload);
    let id = text(data, "external_delivery_id", 256)?;
    let event = payload
        .get("event_name")
        .or_else(|| payload.get("event_type"))
        .or_else(|| data.get("delivery_status"))
        .or_else(|| data.get("status"))
        .and_then(Value::as_str)
        .ok_or("missing delivery event")?;
    let status = match event {
        "DASHER_CONFIRMED" | "dasher_confirmed" => "DRIVER_CONFIRMED",
        "DASHER_CONFIRMED_PICKUP_ARRIVAL" | "dasher_enroute_to_pickup" => {
            "DRIVER_ENROUTE_TO_PICKUP"
        }
        "DASHER_PICKED_UP"
        | "DASHER_CONFIRMED_DROPOFF_ARRIVAL"
        | "dasher_enroute_to_dropoff"
        | "enroute_to_dropoff" => "TRANSIT",
        "DASHER_DROPPED_OFF" | "delivered" => "DELIVERED",
        "DELIVERY_CANCELLED" | "cancelled" | "canceled" => "CANCELLED",
        "DELIVERY_RETURN_INITIALIZED"
        | "DASHER_CONFIRMED_RETURN_ARRIVAL"
        | "dasher_enroute_to_return" => "RETURNING",
        "DELIVERY_RETURNED" | "returned" => "RETURNED",
        "DELIVERY_BATCHED" => "DRIVER_REQUESTED",
        _ => return Err("unsupported DoorDash event".into()),
    }
    .to_owned();
    let timestamp = if payload.get("created_at").is_some() {
        payload
    } else {
        data
    };
    let status_at_ms = date(timestamp, "created_at")?;
    let driver = data
        .get("dasher_id")
        .or_else(|| data.get("driver_id"))
        .or_else(|| data.get("dasher").and_then(|v| v.get("id")));
    let driver_id = match driver {
        None | Some(Value::Null) => None,
        Some(Value::String(value))
            if !value.is_empty() && value.len() <= 128 && !value.chars().any(char::is_control) =>
        {
            Some(value.clone())
        }
        Some(Value::Number(value)) if value.as_u64().is_some() => Some(value.to_string()),
        _ => return Err("invalid driver id".into()),
    };
    let (latitude, longitude) = match data
        .get("dasher_location")
        .or_else(|| data.get("driver_location"))
    {
        None | Some(Value::Null) => (None, None),
        Some(value) => {
            let latitude = value
                .get("lat")
                .or_else(|| value.get("latitude"))
                .and_then(Value::as_f64)
                .ok_or("missing latitude")?;
            let longitude = value
                .get("lng")
                .or_else(|| value.get("longitude"))
                .and_then(Value::as_f64)
                .ok_or("missing longitude")?;
            if !(-90.0..=90.0).contains(&latitude) || !(-180.0..=180.0).contains(&longitude) {
                return Err("invalid driver location".into());
            }
            (Some(latitude), Some(longitude))
        }
    };
    Ok(TrackingUpdate {
        provider: "doordash",
        provider_object_id: Some(id),
        tracking_number: None,
        carrier: None,
        status,
        status_at_ms,
        is_test: None,
        driver_id,
        latitude,
        longitude,
    })
}

pub fn transition_allowed(previous: &str, previous_ms: Option<i64>, next: &TrackingUpdate) -> bool {
    if previous_ms.is_some_and(|last| next.status_at_ms <= last)
        || matches!(
            previous.to_ascii_uppercase().as_str(),
            "DELIVERED" | "RETURNED" | "CANCELLED" | "CANCELED"
        )
    {
        return false;
    }
    match next.status.as_str() {
        "UNKNOWN" => false,
        "PRE_TRANSIT" => matches!(
            previous,
            "PENDING" | "LABEL_CREATED" | "UNKNOWN" | "PRE_TRANSIT"
        ),
        "DRIVER_REQUESTED" | "DRIVER_CONFIRMED" | "DRIVER_ENROUTE_TO_PICKUP" => {
            !matches!(previous, "TRANSIT" | "RETURNING" | "HANDED_OFF")
        }
        "TRANSIT" => previous != "RETURNING",
        "DELIVERED" | "RETURNED" | "FAILURE" | "RETURNING" | "CANCELLED" => true,
        _ => false,
    }
}
