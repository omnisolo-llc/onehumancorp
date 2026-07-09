use std::fs;

#[test]
fn test_hybrid_telemetry_drift() {
    let mut root = std::env::current_dir().unwrap();
    // If we are inside src/server/telemetry, go up 3 levels
    while !root.join("src").exists() && root.parent().is_some() {
        root = root.parent().unwrap().to_path_buf();
    }

    let dashboards_dir = root.join("src/server/monitoring/dashboards");
    let entries = fs::read_dir(&dashboards_dir).expect("Failed to read dashboards directory");

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
            let filename = path.file_name().unwrap().to_str().unwrap();

            let canonical_content = fs::read_to_string(&path).unwrap_or_else(|_| panic!("Failed to read canonical {:?}", path));

            let mirror_paths = vec![
                root.join(format!("deploy/grafana/dashboards/{}", filename)),
                root.join(format!("deploy/docker/grafana/provisioning/dashboards/{}", filename)),
                root.join(format!("deploy/helm/ohc/dashboards/{}", filename)),
            ];

            for mirror in mirror_paths {
                let mirror_content = fs::read_to_string(&mirror).unwrap_or_else(|_| "".to_string());
                assert_eq!(
                    canonical_content, mirror_content,
                    "Drift detected in {:?}. Please synchronize it with the canonical source {:?}",
                    mirror, path
                );
            }
        }
    }
}
