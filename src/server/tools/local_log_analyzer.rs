use crate::ohc::orchestration::{McpInvokeRequest, McpInvokeResponse, McpToolProto};
use chrono::{DateTime, Utc, TimeZone};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub struct LocalLogAnalyzerServer {
    log_path: PathBuf,
}

impl Default for LocalLogAnalyzerServer {
    fn default() -> Self {
        Self {
            log_path: PathBuf::from("logs/agent_harness.log"),
        }
    }
}

impl LocalLogAnalyzerServer {
    pub fn new(log_path: PathBuf) -> Self {
        Self { log_path }
    }

    pub fn get_tools(&self) -> Vec<McpToolProto> {
        vec![McpToolProto {
            id: "local_log_analyzer".to_string(),
            name: "Local Log Analyzer".to_string(),
            description: "Analyzes local agent_harness.log. Input schema: {\"type\":\"object\",\"properties\":{\"log_level\":{\"type\":\"string\"},\"time_range_minutes\":{\"type\":\"integer\"}}}".to_string(),
            category: "observability".to_string(),
            status: "active".to_string(),
        }]
    }

    pub async fn invoke_tool(
        &self,
        req: &McpInvokeRequest,
    ) -> Result<McpInvokeResponse, tonic::Status> {
        let params: serde_json::Value = serde_json::from_str(&req.params)
            .map_err(|e| tonic::Status::invalid_argument(format!("invalid JSON params: {}", e)))?;

        if req.tool_id != "local_log_analyzer" {
            return Err(tonic::Status::unimplemented(format!(
                "tool {} not implemented in local log analyzer",
                req.tool_id
            )));
        }

        let log_level = params["log_level"].as_str().unwrap_or("ERROR");
        let time_range_minutes = params["time_range_minutes"].as_i64().unwrap_or(60);

        // Calculate the cutoff time
        let now = Utc::now();
        let cutoff = now - chrono::Duration::minutes(time_range_minutes);

        let mut matching_lines = Vec::new();

        if let Ok(file) = File::open(&self.log_path) {
            let reader = BufReader::new(file);

            for line in reader.lines().filter_map(|l| l.ok()) {
                // Extremely basic parsing: looking for level string
                if !line.contains(log_level) {
                    continue;
                }

                // Try to parse timestamp from beginning of line assuming standard formatting
                // format like: 2023-10-25T15:30:00Z
                if let Some(timestamp_str) = line.split_whitespace().next() {
                    if let Ok(timestamp) = timestamp_str.parse::<DateTime<Utc>>() {
                        if timestamp < cutoff {
                            continue; // Too old
                        }
                    }
                }

                matching_lines.push(line);
                if matching_lines.len() >= 50 {
                    break;
                }
            }
        } else {
            // Log file not found or cannot be opened
            return Ok(McpInvokeResponse {
                payload: serde_json::to_string(&serde_json::json!({
                    "error": format!("Log file not found or inaccessible at {:?}", self.log_path),
                    "summary": "No logs found"
                })).unwrap(),
            });
        }

        let summary = if matching_lines.is_empty() {
            format!("No logs found matching level {} in the last {} minutes.", log_level, time_range_minutes)
        } else {
            matching_lines.join("\n")
        };

        let resp = serde_json::json!({
            "summary": summary,
            "count": matching_lines.len()
        });

        Ok(McpInvokeResponse {
            payload: serde_json::to_string(&resp).unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_local_log_analyzer() {
        let dir = tempdir().unwrap();
        let log_path = dir.path().join("agent_harness.log");
        let mut file = File::create(&log_path).unwrap();

        let now = Utc::now();
        let old = now - chrono::Duration::minutes(120);
        let recent = now - chrono::Duration::minutes(10);

        writeln!(file, "{} INFO some info log", old.to_rfc3339()).unwrap();
        writeln!(file, "{} ERROR some old error", old.to_rfc3339()).unwrap();
        writeln!(file, "{} ERROR some recent error", recent.to_rfc3339()).unwrap();
        writeln!(file, "{} WARN some recent warning", recent.to_rfc3339()).unwrap();

        // Let's add more to test limits
        for i in 0..60 {
            writeln!(file, "{} ERROR error {}", recent.to_rfc3339(), i).unwrap();
        }

        let analyzer = LocalLogAnalyzerServer::new(log_path);

        // Test getting recent errors
        let req = McpInvokeRequest {
            tool_id: "local_log_analyzer".to_string(),
            action: "invoke".to_string(),
            params: r#"{"log_level":"ERROR","time_range_minutes":60}"#.to_string(),
            agent_id: "test".to_string(),
            spiffe_id: "test".to_string(),
        };

        let resp = analyzer.invoke_tool(&req).await.unwrap();
        let payload: serde_json::Value = serde_json::from_str(&resp.payload).unwrap();

        let count = payload["count"].as_i64().unwrap();
        assert_eq!(count, 50); // limited to top 50 lines

        let summary = payload["summary"].as_str().unwrap();
        assert!(!summary.contains("old error"));
        assert!(summary.contains("recent error"));
    }
}
