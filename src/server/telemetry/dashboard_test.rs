use std::fs;

#[test]
fn test_hybrid_telemetry_drift() {
    let mut root = std::env::current_dir().unwrap();
    // If we are inside src/server/telemetry, go up 3 levels
    while !root.join("src").exists() && root.parent().is_some() {
        root = root.parent().unwrap().to_path_buf();
    }

    let source_path = root.join("src/server/monitoring/dashboards/hybrid-telemetry.json");
    let mirror_paths = vec![
        root.join("deploy/grafana/dashboards/hybrid-telemetry.json"),
        root.join("deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json"),
        root.join("deploy/helm/ohc/dashboards/hybrid-telemetry.json")
    ];

    let canonical_content = fs::read_to_string(&source_path).expect(&format!("Failed to read canonical {:?}", source_path));

    for mirror in mirror_paths {
        let mirror_content = fs::read_to_string(&mirror).unwrap_or_else(|_| "".to_string());
        assert_eq!(
            canonical_content, mirror_content,
            "Drift detected in {:?}. Please synchronize it with the canonical source.",
            mirror
        );
    }
}
