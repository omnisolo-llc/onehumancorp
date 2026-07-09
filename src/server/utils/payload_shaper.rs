use serde_json::Value;

#[derive(Debug, PartialEq)]
pub enum FieldNode {
    Leaf,
    Nested(std::collections::HashMap<String, FieldNode>),
}

pub fn parse_fields(fields: &str) -> std::collections::HashMap<String, FieldNode> {
    let mut root = std::collections::HashMap::new();
    let mut current_key = String::new();
    let mut chars = fields.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            ',' | ' ' => {
                if !current_key.is_empty() {
                    root.insert(current_key.clone(), FieldNode::Leaf);
                    current_key.clear();
                }
            }
            '(' => {
                // Find matching parenthesis
                let mut nested_str = String::new();
                let mut depth = 1;
                for nc in chars.by_ref() {
                    if nc == '(' {
                        depth += 1;
                    } else if nc == ')' {
                        depth -= 1;
                        if depth == 0 {
                            break;
                        }
                    }
                    nested_str.push(nc);
                }
                let nested_node = FieldNode::Nested(parse_fields(&nested_str));
                if !current_key.is_empty() {
                    root.insert(current_key.clone(), nested_node);
                    current_key.clear();
                }
            }
            _ => {
                current_key.push(c);
            }
        }
    }
    if !current_key.is_empty() {
        root.insert(current_key, FieldNode::Leaf);
    }
    root
}

pub fn shape_value(val: Value, tree: &std::collections::HashMap<String, FieldNode>) -> Value {
    if tree.is_empty() {
        return val; // If no specific fields requested for this level, return all (or should it be none? Usually returning all is safer if they didn't specify nested, meaning they want the whole nested object).
    }

    match val {
        Value::Object(map) => {
            let mut new_map = serde_json::Map::new();
            for (k, v) in map.into_iter() {
                if let Some(node) = tree.get(&k) {
                    match node {
                        FieldNode::Leaf => {
                            new_map.insert(k, v);
                        }
                        FieldNode::Nested(nested_tree) => {
                            new_map.insert(k, shape_value(v, nested_tree));
                        }
                    }
                }
            }
            Value::Object(new_map)
        }
        Value::Array(arr) => {
            let new_arr = arr.into_iter().map(|item| shape_value(item, tree)).collect();
            Value::Array(new_arr)
        }
        _ => val,
    }
}

pub fn shape_payload(payload: Value, fields: Option<&str>) -> Value {
    let Some(fields_str) = fields else {
        return payload;
    };
    if fields_str.trim().is_empty() {
        return payload;
    }
    let tree = parse_fields(fields_str);
    shape_value(payload, &tree)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_fields() {
        let tree = parse_fields("id,name,items(id,title)");
        assert!(tree.contains_key("id"));
        assert!(tree.contains_key("name"));
        assert!(tree.contains_key("items"));
        if let FieldNode::Nested(nested) = tree.get("items").unwrap() {
            assert!(nested.contains_key("id"));
            assert!(nested.contains_key("title"));
        } else {
            panic!("Expected nested node");
        }
    }

    #[test]
    fn test_shape_payload() {
        let payload = json!({
            "metrics": {"total": 100, "active": 50},
            "orders": [
                {"id": "1", "status": "pending", "amount": 10},
                {"id": "2", "status": "completed", "amount": 20}
            ],
            "useless": "drop me"
        });

        let shaped = shape_payload(payload, Some("metrics(total),orders(id)"));
        assert_eq!(shaped, json!({
            "metrics": {"total": 100},
            "orders": [
                {"id": "1"},
                {"id": "2"}
            ]
        }));
    }
}
