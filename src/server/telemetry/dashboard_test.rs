use std::fs;
use std::path::PathBuf;

fn get_workspace_dir() -> PathBuf {
    if let Ok(runfiles_dir) = std::env::var("RUNFILES_DIR") {
        let workspace = std::env::var("TEST_WORKSPACE").unwrap_or_else(|_| "mono".to_string());
        return PathBuf::from(runfiles_dir).join(workspace);
    }

    // Fallback for direct cargo test
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().parent().unwrap().to_path_buf()
}

#[test]
fn test_hybrid_telemetry_drift() {
    let workspace_dir = get_workspace_dir();
    let source_path = workspace_dir.join("src/server/monitoring/dashboards/hybrid-telemetry.json");

    let mirror_paths = vec![
        workspace_dir.join("deploy/grafana/dashboards/hybrid-telemetry.json"),
        workspace_dir.join("deploy/docker/grafana/provisioning/dashboards/hybrid-telemetry.json"),
        workspace_dir.join("deploy/helm/ohc/dashboards/hybrid-telemetry.json")
    ];

    let canonical_content = fs::read_to_string(&source_path)
        .unwrap_or_else(|_| panic!("Failed to read canonical hybrid-telemetry.json at {}", source_path.display()));

    for mirror in mirror_paths {
        let mirror_content = fs::read_to_string(&mirror).unwrap_or_else(|_| "".to_string());
        assert_eq!(
            canonical_content, mirror_content,
            "Drift detected in {}. Please synchronize it with the canonical source at {}.",
            mirror.display(), source_path.display()
        );
    }
}
