use std::collections::HashMap;

pub struct ValidationEndpoint;

impl ValidationEndpoint {
    pub fn validate_config(&self, config: &HashMap<String, String>) -> Result<(), String> {
        let mode = match config.get("mode") {
            Some(m) => m,
            None => return Err("mode is required".to_string()),
        };

        let db = config.get("db").map(|s| s.as_str()).unwrap_or("");
        let cache = config.get("cache").map(|s| s.as_str()).unwrap_or("");

        if mode == "cloud" {
            if db != "postgres" || cache != "redis" {
                return Err("invalid configuration for cloud mode: requires postgres and redis".to_string());
            }
        } else if mode == "standalone" {
            if db != "sqlite" || cache != "memory" {
                return Err("invalid configuration for standalone mode: requires sqlite and memory".to_string());
            }
        } else {
            return Err("unknown mode".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_validate_config() {
        let v = ValidationEndpoint;

        struct Test {
            name: &'static str,
            config: HashMap<String, String>,
            want_err: bool,
        }

        let tests = vec![
            Test {
                name: "valid cloud config",
                config: [
                    ("mode".to_string(), "cloud".to_string()),
                    ("db".to_string(), "postgres".to_string()),
                    ("cache".to_string(), "redis".to_string()),
                ].iter().cloned().collect(),
                want_err: false,
            },
            Test {
                name: "invalid cloud config db",
                config: [
                    ("mode".to_string(), "cloud".to_string()),
                    ("db".to_string(), "sqlite".to_string()),
                    ("cache".to_string(), "redis".to_string()),
                ].iter().cloned().collect(),
                want_err: true,
            },
            Test {
                name: "valid standalone config",
                config: [
                    ("mode".to_string(), "standalone".to_string()),
                    ("db".to_string(), "sqlite".to_string()),
                    ("cache".to_string(), "memory".to_string()),
                ].iter().cloned().collect(),
                want_err: false,
            },
            Test {
                name: "invalid standalone config cache",
                config: [
                    ("mode".to_string(), "standalone".to_string()),
                    ("db".to_string(), "sqlite".to_string()),
                    ("cache".to_string(), "redis".to_string()),
                ].iter().cloned().collect(),
                want_err: true,
            },
            Test {
                name: "missing mode",
                config: [
                    ("db".to_string(), "sqlite".to_string()),
                    ("cache".to_string(), "memory".to_string()),
                ].iter().cloned().collect(),
                want_err: true,
            },
            Test {
                name: "unknown mode",
                config: [
                    ("mode".to_string(), "unknown".to_string()),
                    ("db".to_string(), "sqlite".to_string()),
                    ("cache".to_string(), "memory".to_string()),
                ].iter().cloned().collect(),
                want_err: true,
            },
        ];

        for tt in tests {
            let res = v.validate_config(&tt.config);
            assert_eq!(res.is_err(), tt.want_err, "Test '{}' failed", tt.name);
        }
    }
}
