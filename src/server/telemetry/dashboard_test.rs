use std::fs;

#[test]
fn test_hybrid_telemetry_drift() {
    let source_path = "src/server/monitoring/dashboards/hybrid-telemetry.json";
    let mirror_paths = vec![
        "deploy/grafana/dashboards/hybrid-telemetry.json",
        "deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json",
        "deploy/helm/ohc/dashboards/hybrid-telemetry.json"
    ];

    let canonical_content = fs::read_to_string(source_path).expect("Failed to read canonical hybrid-telemetry.json");

    for mirror in mirror_paths {
        let mirror_content = fs::read_to_string(mirror).unwrap_or_else(|_| "".to_string());
        assert_eq!(
            canonical_content, mirror_content,
            "Drift detected in {}. Please synchronize it with the canonical source at {}.",
            mirror, source_path
        );
    }
}
