use h3o::{CellIndex, LatLng, Resolution};
use std::str::FromStr;

pub fn calculate_geohash(lat: f64, lng: f64, resolution: u8) -> Result<String, String> {
    let latlng = LatLng::new(lat, lng).map_err(|e| e.to_string())?;
    let res = Resolution::try_from(resolution).map_err(|e| e.to_string())?;
    let cell = CellIndex::from_latlng(latlng, res);
    Ok(cell.to_string())
}

pub fn are_neighbors(geohash1: &str, geohash2: &str) -> bool {
    let cell1 = match CellIndex::from_str(geohash1) {
        Ok(c) => c,
        Err(_) => return false,
    };
    let cell2 = match CellIndex::from_str(geohash2) {
        Ok(c) => c,
        Err(_) => return false,
    };

    cell1.is_neighbor_with(cell2) || cell1 == cell2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_geohash() {
        let lat = 37.7749;
        let lng = -122.4194;
        let res = 8;
        let hash = calculate_geohash(lat, lng, res).unwrap();
        assert!(!hash.is_empty());
    }

    #[test]
    fn test_are_neighbors() {
        let lat1 = 37.7749;
        let lng1 = -122.4194;
        let lat2 = 37.7750;
        let lng2 = -122.4195;
        let res = 8;

        let hash1 = calculate_geohash(lat1, lng1, res).unwrap();
        let hash2 = calculate_geohash(lat2, lng2, res).unwrap();

        assert!(are_neighbors(&hash1, &hash2));
    }

    #[test]
    fn test_not_neighbors() {
        let hash1 = calculate_geohash(37.7749, -122.4194, 8).unwrap();
        let hash2 = calculate_geohash(40.7128, -74.0060, 8).unwrap();

        assert!(!are_neighbors(&hash1, &hash2));
    }
}
