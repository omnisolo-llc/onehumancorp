use serde::Serialize;
use serde_json::Value;

pub trait MobileShaper {
    fn shape_for_mobile(&mut self);
}

impl MobileShaper for Value {
    fn shape_for_mobile(&mut self) {
        if let Some(obj) = self.as_object_mut() {
            // Remove heavy fields typically not needed for initial mobile dashboard view
            obj.remove("transcript");
            obj.remove("raw_metadata");
            obj.remove("debug_info");
            obj.remove("history");
            obj.remove("embeddings");

            // Recursively shape nested objects
            for (_key, value) in obj.iter_mut() {
                value.shape_for_mobile();
            }
        } else if let Some(arr) = self.as_array_mut() {
            for item in arr.iter_mut() {
                item.shape_for_mobile();
            }
        }
    }
}

pub fn is_mobile_request<T>(request: &tonic::Request<T>) -> bool {
    request.metadata().get("x-client-platform")
        .map(|v| v.to_str().unwrap_or("").to_lowercase().contains("mobile"))
        .unwrap_or(false)
}
