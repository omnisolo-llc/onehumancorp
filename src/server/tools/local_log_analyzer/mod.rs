use std::env;
use std::fs;
use std::path::PathBuf;
use chrono::{DateTime, Utc, TimeZone, Duration};
use std::io::{BufRead, BufReader};
use serde_json::json;

pub struct LogAnalyzerTool {
    pub log_dir: PathBuf,
}

impl LogAnalyzerTool {
    pub fn new() -> Self {
        let base_dir = env::var("OHC_LOG_DIR").unwrap_or_else(|_| "logs".to_string());
        Self {
            log_dir: PathBuf::from(base_dir),
        }
    }

    pub fn execute(&self, level: &str, minutes: i32) -> Result<String, String> {
        let log_file = self.log_dir.join("agent_harness.log");
        if !log_file.exists() {
            return Err("Log file not found".to_string());
        }

        // Optimize memory footprint by keeping only the most recent N matches
        const MAX_LINES: usize = 50;
        let mut matched_lines: std::collections::VecDeque<String> = std::collections::VecDeque::with_capacity(MAX_LINES);
        let mut total_matches = 0;

        let file = fs::File::open(&log_file).map_err(|e| e.to_string())?;
        let reader = BufReader::new(file);

        let now = Utc::now();
        let cutoff = now - Duration::minutes(minutes as i64);

        for line_result in reader.lines() {
            if let Ok(line) = line_result {
                if !line.contains(level) {
                    continue;
                }

                let mut should_include = true;

                // Typical log line starts with a timestamp: 2023-10-27T10:00:00Z INFO ...
                // Try to parse the first 20 chars
                if line.len() >= 20 {
                    let ts_str = &line[0..20];
                    if let Ok(parsed_time) = DateTime::parse_from_rfc3339(ts_str).map(|d| d.with_timezone(&Utc)) {
                        if parsed_time < cutoff {
                            should_include = false;
                        }
                    }
                }

                if should_include {
                     total_matches += 1;
                     if matched_lines.len() == MAX_LINES {
                         matched_lines.pop_front();
                     }
                     matched_lines.push_back(line);
                }
            }
        }

        let mut result = format!("Found {} logs with level {} in the last {} minutes.\n", total_matches, level, minutes);
        for l in matched_lines.iter().rev() {
            result.push_str(&format!("{}\n", l));
        }

        Ok(result)
    }
}
