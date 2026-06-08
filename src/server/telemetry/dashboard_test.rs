use std::fs;

#[test]
fn test_hybrid_telemetry_drift() {
    // Resolve the workspace root by looking at runfiles directly, or fall back to cargo paths
    let test_src_dir = std::env::var("TEST_SRCDIR").unwrap_or_else(|_| ".".to_string());
    let workspace_name = std::env::var("TEST_WORKSPACE").unwrap_or_else(|_| "_main".to_string());

    let base_dir = if std::env::var("TEST_SRCDIR").is_ok() {
        format!("{}/{}", test_src_dir, workspace_name)
    } else if let Ok(root) = std::env::var("BUILD_WORKSPACE_DIRECTORY") {
        root
    } else {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
        let path = std::path::Path::new(&manifest_dir);
        if path.ends_with("src/server/telemetry") {
            path.parent().unwrap().parent().unwrap().parent().unwrap().to_str().unwrap().to_string()
        } else {
            ".".to_string()
        }
    };

    let source_path = format!("{}/src/server/monitoring/dashboards/hybrid-telemetry.json", base_dir);
    let mirror_paths = vec![
        format!("{}/deploy/grafana/dashboards/hybrid-telemetry.json", base_dir),
        format!("{}/deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json", base_dir),
        format!("{}/deploy/helm/ohc/dashboards/hybrid-telemetry.json", base_dir)
    ];

    let canonical_content = fs::read_to_string(&source_path).expect("Failed to read canonical hybrid-telemetry.json");

    for mirror in mirror_paths {
        let mirror_content = fs::read_to_string(&mirror).unwrap_or_else(|_| "".to_string());
        assert_eq!(
            canonical_content, mirror_content,
            "Drift detected in {}. Please synchronize it with the canonical source at {}.",
            mirror, source_path
        );
    }
}
