use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleDefinition {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub context: String,
    #[serde(default)]
    pub tools: Vec<String>,
    #[serde(default)]
    pub reports_to: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillBlueprint {
    #[serde(default)]
    pub domain: String,
    #[serde(default)]
    pub roles: Vec<RoleDefinition>,
}

pub fn parse_blueprint(data: &[u8], is_yaml: bool) -> Result<SkillBlueprint, String> {
    let bp: SkillBlueprint = if is_yaml {
        serde_yaml::from_slice(data).map_err(|e| format!("failed to unmarshal blueprint: {}", e))?
    } else {
        serde_json::from_slice(data).map_err(|e| format!("failed to unmarshal blueprint: {}", e))?
    };

    bp.validate()?;

    Ok(bp)
}

impl SkillBlueprint {
    pub fn validate(&self) -> Result<(), String> {
        if self.domain.trim().is_empty() {
            return Err("domain is required".to_string());
        }

        if self.roles.is_empty() {
            return Err("at least one role is required".to_string());
        }

        let mut roles_map = HashMap::new();
        for role in &self.roles {
            if role.id.trim().is_empty() {
                return Err("role id is required".to_string());
            }
            if role.context.trim().is_empty() {
                return Err(format!("context is required for role: {}", role.id));
            }
            if roles_map.contains_key(&role.id) {
                return Err(format!("duplicate role id: {}", role.id));
            }
            roles_map.insert(role.id.clone(), role.clone());
        }

        // Validate reports_to targets exist
        for role in &self.roles {
            if !role.reports_to.is_empty() {
                if !roles_map.contains_key(&role.reports_to) {
                    return Err(format!("role {} reports to unknown role: {}", role.id, role.reports_to));
                }
            }
        }

        // DAG Check (Cycle detection)
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();

        fn check_cycle(
            node_id: &str,
            roles_map: &HashMap<String, RoleDefinition>,
            visited: &mut HashSet<String>,
            rec_stack: &mut HashSet<String>,
        ) -> Result<(), String> {
            visited.insert(node_id.to_string());
            rec_stack.insert(node_id.to_string());

            if let Some(role) = roles_map.get(node_id) {
                let reports_to = &role.reports_to;
                if !reports_to.is_empty() {
                    if !visited.contains(reports_to) {
                        check_cycle(reports_to, roles_map, visited, rec_stack)?;
                    } else if rec_stack.contains(reports_to) {
                        return Err(format!("circular reporting loop detected involving role: {}", node_id));
                    }
                }
            }

            rec_stack.remove(node_id);
            Ok(())
        }

        for role in &self.roles {
            if !visited.contains(&role.id) {
                check_cycle(&role.id, &roles_map, &mut visited, &mut rec_stack)?;
            }
        }

        Ok(())
    }

    pub fn namespace_roles(&mut self, namespace: &str) {
        let prefix = format!("{}/", namespace);
        for role in &mut self.roles {
            role.id = format!("{}{}", prefix, role.id);
            if !role.reports_to.is_empty() {
                role.reports_to = format!("{}{}", prefix, role.reports_to);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_blueprint_yaml_success() {
        let yaml_data = r#"
domain: "Legal Consulting"
roles:
  - id: "senior_partner"
    title: "Senior Partner Agent"
    context: "You oversee high-level legal strategy and client acquisition."
    tools: ["mcp://tools/lexis-nexis", "mcp://tools/docusign"]
  - id: "associate"
    title: "Associate Agent"
    context: "You perform case law research and draft legal briefs."
    reports_to: "senior_partner"
"#;

        let bp = parse_blueprint(yaml_data.as_bytes(), true).unwrap();
        assert_eq!(bp.domain, "Legal Consulting");
        assert_eq!(bp.roles.len(), 2);
        assert_eq!(bp.roles[1].id, "associate");
        assert_eq!(bp.roles[1].reports_to, "senior_partner");
    }

    #[test]
    fn test_parse_blueprint_json_success() {
        let json_data = r#"{
		"domain": "Sales",
		"roles": [
			{
				"id": "manager",
				"title": "Sales Manager",
				"context": "Manage team."
			},
			{
				"id": "rep",
				"title": "Sales Rep",
				"context": "Sell things.",
				"reports_to": "manager"
			}
		]
	}"#;

        let bp = parse_blueprint(json_data.as_bytes(), false).unwrap();
        assert_eq!(bp.domain, "Sales");
        assert_eq!(bp.roles.len(), 2);
    }

    #[test]
    fn test_blueprint_validation_missing_fields() {
        let tests = vec![
            (
                "missing domain",
                r#"
roles:
  - id: "a"
    context: "context"
"#,
                "domain is required",
            ),
            (
                "missing roles",
                r#"
domain: "Test"
"#,
                "at least one role is required",
            ),
            (
                "missing role id",
                r#"
domain: "Test"
roles:
  - context: "context"
"#,
                "role id is required",
            ),
            (
                "missing role context",
                r#"
domain: "Test"
roles:
  - id: "a"
"#,
                "context is required for role: a",
            ),
            (
                "duplicate role id",
                r#"
domain: "Test"
roles:
  - id: "a"
    context: "context 1"
  - id: "a"
    context: "context 2"
"#,
                "duplicate role id: a",
            ),
            (
                "reports to unknown role",
                r#"
domain: "Test"
roles:
  - id: "a"
    context: "context 1"
    reports_to: "b"
"#,
                "role a reports to unknown role: b",
            ),
        ];

        for (name, yaml_data, expected_err) in tests {
            let res = parse_blueprint(yaml_data.as_bytes(), true);
            assert!(res.is_err(), "Expected error for test case: {}", name);
            let err = res.unwrap_err();
            assert!(err.contains(expected_err), "Expected error to contain '{}', got: {}", expected_err, err);
        }
    }

    #[test]
    fn test_blueprint_dag_cycle_detection() {
        let yaml_data = r#"
domain: "Cyclic Domain"
roles:
  - id: "a"
    context: "context a"
    reports_to: "b"
  - id: "b"
    context: "context b"
    reports_to: "c"
  - id: "c"
    context: "context c"
    reports_to: "a"
"#;

        let res = parse_blueprint(yaml_data.as_bytes(), true);
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert!(err.contains("circular reporting loop detected"), "Expected circular reporting loop detected, got: {}", err);
    }

    #[test]
    fn test_namespace_roles() {
        let mut bp = SkillBlueprint {
            domain: "Test".to_string(),
            roles: vec![
                RoleDefinition { id: "a".to_string(), title: "".to_string(), context: "context".to_string(), tools: vec![], reports_to: "".to_string() },
                RoleDefinition { id: "b".to_string(), title: "".to_string(), context: "context".to_string(), tools: vec![], reports_to: "a".to_string() },
            ],
        };

        bp.namespace_roles("test_v1");

        assert_eq!(bp.roles[0].id, "test_v1/a");
        assert_eq!(bp.roles[1].id, "test_v1/b");
        assert_eq!(bp.roles[1].reports_to, "test_v1/a");
    }
}
