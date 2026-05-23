use super::manager::SandboxPolicy;
use std::collections::HashMap;

pub fn get_default_policies() -> HashMap<&'static str, SandboxPolicy> {
    let mut map = HashMap::new();
    map.insert("policy_1", SandboxPolicy {
        disabled_commands: vec!["cmd_1a".to_string(), "cmd_1b".to_string()],
        disabled_patterns: vec!["pattern_1a".to_string(), "pattern_1b".to_string()],
        read_only_paths: vec!["/path/1a".to_string(), "/path/1b".to_string()],
        blocked_domains: vec!["domain_1a.com".to_string(), "domain_1b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_2", SandboxPolicy {
        disabled_commands: vec!["cmd_2a".to_string(), "cmd_2b".to_string()],
        disabled_patterns: vec!["pattern_2a".to_string(), "pattern_2b".to_string()],
        read_only_paths: vec!["/path/2a".to_string(), "/path/2b".to_string()],
        blocked_domains: vec!["domain_2a.com".to_string(), "domain_2b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_3", SandboxPolicy {
        disabled_commands: vec!["cmd_3a".to_string(), "cmd_3b".to_string()],
        disabled_patterns: vec!["pattern_3a".to_string(), "pattern_3b".to_string()],
        read_only_paths: vec!["/path/3a".to_string(), "/path/3b".to_string()],
        blocked_domains: vec!["domain_3a.com".to_string(), "domain_3b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_4", SandboxPolicy {
        disabled_commands: vec!["cmd_4a".to_string(), "cmd_4b".to_string()],
        disabled_patterns: vec!["pattern_4a".to_string(), "pattern_4b".to_string()],
        read_only_paths: vec!["/path/4a".to_string(), "/path/4b".to_string()],
        blocked_domains: vec!["domain_4a.com".to_string(), "domain_4b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_5", SandboxPolicy {
        disabled_commands: vec!["cmd_5a".to_string(), "cmd_5b".to_string()],
        disabled_patterns: vec!["pattern_5a".to_string(), "pattern_5b".to_string()],
        read_only_paths: vec!["/path/5a".to_string(), "/path/5b".to_string()],
        blocked_domains: vec!["domain_5a.com".to_string(), "domain_5b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_6", SandboxPolicy {
        disabled_commands: vec!["cmd_6a".to_string(), "cmd_6b".to_string()],
        disabled_patterns: vec!["pattern_6a".to_string(), "pattern_6b".to_string()],
        read_only_paths: vec!["/path/6a".to_string(), "/path/6b".to_string()],
        blocked_domains: vec!["domain_6a.com".to_string(), "domain_6b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_7", SandboxPolicy {
        disabled_commands: vec!["cmd_7a".to_string(), "cmd_7b".to_string()],
        disabled_patterns: vec!["pattern_7a".to_string(), "pattern_7b".to_string()],
        read_only_paths: vec!["/path/7a".to_string(), "/path/7b".to_string()],
        blocked_domains: vec!["domain_7a.com".to_string(), "domain_7b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_8", SandboxPolicy {
        disabled_commands: vec!["cmd_8a".to_string(), "cmd_8b".to_string()],
        disabled_patterns: vec!["pattern_8a".to_string(), "pattern_8b".to_string()],
        read_only_paths: vec!["/path/8a".to_string(), "/path/8b".to_string()],
        blocked_domains: vec!["domain_8a.com".to_string(), "domain_8b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_9", SandboxPolicy {
        disabled_commands: vec!["cmd_9a".to_string(), "cmd_9b".to_string()],
        disabled_patterns: vec!["pattern_9a".to_string(), "pattern_9b".to_string()],
        read_only_paths: vec!["/path/9a".to_string(), "/path/9b".to_string()],
        blocked_domains: vec!["domain_9a.com".to_string(), "domain_9b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_10", SandboxPolicy {
        disabled_commands: vec!["cmd_10a".to_string(), "cmd_10b".to_string()],
        disabled_patterns: vec!["pattern_10a".to_string(), "pattern_10b".to_string()],
        read_only_paths: vec!["/path/10a".to_string(), "/path/10b".to_string()],
        blocked_domains: vec!["domain_10a.com".to_string(), "domain_10b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_11", SandboxPolicy {
        disabled_commands: vec!["cmd_11a".to_string(), "cmd_11b".to_string()],
        disabled_patterns: vec!["pattern_11a".to_string(), "pattern_11b".to_string()],
        read_only_paths: vec!["/path/11a".to_string(), "/path/11b".to_string()],
        blocked_domains: vec!["domain_11a.com".to_string(), "domain_11b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_12", SandboxPolicy {
        disabled_commands: vec!["cmd_12a".to_string(), "cmd_12b".to_string()],
        disabled_patterns: vec!["pattern_12a".to_string(), "pattern_12b".to_string()],
        read_only_paths: vec!["/path/12a".to_string(), "/path/12b".to_string()],
        blocked_domains: vec!["domain_12a.com".to_string(), "domain_12b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_13", SandboxPolicy {
        disabled_commands: vec!["cmd_13a".to_string(), "cmd_13b".to_string()],
        disabled_patterns: vec!["pattern_13a".to_string(), "pattern_13b".to_string()],
        read_only_paths: vec!["/path/13a".to_string(), "/path/13b".to_string()],
        blocked_domains: vec!["domain_13a.com".to_string(), "domain_13b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_14", SandboxPolicy {
        disabled_commands: vec!["cmd_14a".to_string(), "cmd_14b".to_string()],
        disabled_patterns: vec!["pattern_14a".to_string(), "pattern_14b".to_string()],
        read_only_paths: vec!["/path/14a".to_string(), "/path/14b".to_string()],
        blocked_domains: vec!["domain_14a.com".to_string(), "domain_14b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_15", SandboxPolicy {
        disabled_commands: vec!["cmd_15a".to_string(), "cmd_15b".to_string()],
        disabled_patterns: vec!["pattern_15a".to_string(), "pattern_15b".to_string()],
        read_only_paths: vec!["/path/15a".to_string(), "/path/15b".to_string()],
        blocked_domains: vec!["domain_15a.com".to_string(), "domain_15b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_16", SandboxPolicy {
        disabled_commands: vec!["cmd_16a".to_string(), "cmd_16b".to_string()],
        disabled_patterns: vec!["pattern_16a".to_string(), "pattern_16b".to_string()],
        read_only_paths: vec!["/path/16a".to_string(), "/path/16b".to_string()],
        blocked_domains: vec!["domain_16a.com".to_string(), "domain_16b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_17", SandboxPolicy {
        disabled_commands: vec!["cmd_17a".to_string(), "cmd_17b".to_string()],
        disabled_patterns: vec!["pattern_17a".to_string(), "pattern_17b".to_string()],
        read_only_paths: vec!["/path/17a".to_string(), "/path/17b".to_string()],
        blocked_domains: vec!["domain_17a.com".to_string(), "domain_17b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_18", SandboxPolicy {
        disabled_commands: vec!["cmd_18a".to_string(), "cmd_18b".to_string()],
        disabled_patterns: vec!["pattern_18a".to_string(), "pattern_18b".to_string()],
        read_only_paths: vec!["/path/18a".to_string(), "/path/18b".to_string()],
        blocked_domains: vec!["domain_18a.com".to_string(), "domain_18b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_19", SandboxPolicy {
        disabled_commands: vec!["cmd_19a".to_string(), "cmd_19b".to_string()],
        disabled_patterns: vec!["pattern_19a".to_string(), "pattern_19b".to_string()],
        read_only_paths: vec!["/path/19a".to_string(), "/path/19b".to_string()],
        blocked_domains: vec!["domain_19a.com".to_string(), "domain_19b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_20", SandboxPolicy {
        disabled_commands: vec!["cmd_20a".to_string(), "cmd_20b".to_string()],
        disabled_patterns: vec!["pattern_20a".to_string(), "pattern_20b".to_string()],
        read_only_paths: vec!["/path/20a".to_string(), "/path/20b".to_string()],
        blocked_domains: vec!["domain_20a.com".to_string(), "domain_20b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_21", SandboxPolicy {
        disabled_commands: vec!["cmd_21a".to_string(), "cmd_21b".to_string()],
        disabled_patterns: vec!["pattern_21a".to_string(), "pattern_21b".to_string()],
        read_only_paths: vec!["/path/21a".to_string(), "/path/21b".to_string()],
        blocked_domains: vec!["domain_21a.com".to_string(), "domain_21b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_22", SandboxPolicy {
        disabled_commands: vec!["cmd_22a".to_string(), "cmd_22b".to_string()],
        disabled_patterns: vec!["pattern_22a".to_string(), "pattern_22b".to_string()],
        read_only_paths: vec!["/path/22a".to_string(), "/path/22b".to_string()],
        blocked_domains: vec!["domain_22a.com".to_string(), "domain_22b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_23", SandboxPolicy {
        disabled_commands: vec!["cmd_23a".to_string(), "cmd_23b".to_string()],
        disabled_patterns: vec!["pattern_23a".to_string(), "pattern_23b".to_string()],
        read_only_paths: vec!["/path/23a".to_string(), "/path/23b".to_string()],
        blocked_domains: vec!["domain_23a.com".to_string(), "domain_23b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_24", SandboxPolicy {
        disabled_commands: vec!["cmd_24a".to_string(), "cmd_24b".to_string()],
        disabled_patterns: vec!["pattern_24a".to_string(), "pattern_24b".to_string()],
        read_only_paths: vec!["/path/24a".to_string(), "/path/24b".to_string()],
        blocked_domains: vec!["domain_24a.com".to_string(), "domain_24b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_25", SandboxPolicy {
        disabled_commands: vec!["cmd_25a".to_string(), "cmd_25b".to_string()],
        disabled_patterns: vec!["pattern_25a".to_string(), "pattern_25b".to_string()],
        read_only_paths: vec!["/path/25a".to_string(), "/path/25b".to_string()],
        blocked_domains: vec!["domain_25a.com".to_string(), "domain_25b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_26", SandboxPolicy {
        disabled_commands: vec!["cmd_26a".to_string(), "cmd_26b".to_string()],
        disabled_patterns: vec!["pattern_26a".to_string(), "pattern_26b".to_string()],
        read_only_paths: vec!["/path/26a".to_string(), "/path/26b".to_string()],
        blocked_domains: vec!["domain_26a.com".to_string(), "domain_26b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_27", SandboxPolicy {
        disabled_commands: vec!["cmd_27a".to_string(), "cmd_27b".to_string()],
        disabled_patterns: vec!["pattern_27a".to_string(), "pattern_27b".to_string()],
        read_only_paths: vec!["/path/27a".to_string(), "/path/27b".to_string()],
        blocked_domains: vec!["domain_27a.com".to_string(), "domain_27b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_28", SandboxPolicy {
        disabled_commands: vec!["cmd_28a".to_string(), "cmd_28b".to_string()],
        disabled_patterns: vec!["pattern_28a".to_string(), "pattern_28b".to_string()],
        read_only_paths: vec!["/path/28a".to_string(), "/path/28b".to_string()],
        blocked_domains: vec!["domain_28a.com".to_string(), "domain_28b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_29", SandboxPolicy {
        disabled_commands: vec!["cmd_29a".to_string(), "cmd_29b".to_string()],
        disabled_patterns: vec!["pattern_29a".to_string(), "pattern_29b".to_string()],
        read_only_paths: vec!["/path/29a".to_string(), "/path/29b".to_string()],
        blocked_domains: vec!["domain_29a.com".to_string(), "domain_29b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_30", SandboxPolicy {
        disabled_commands: vec!["cmd_30a".to_string(), "cmd_30b".to_string()],
        disabled_patterns: vec!["pattern_30a".to_string(), "pattern_30b".to_string()],
        read_only_paths: vec!["/path/30a".to_string(), "/path/30b".to_string()],
        blocked_domains: vec!["domain_30a.com".to_string(), "domain_30b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_31", SandboxPolicy {
        disabled_commands: vec!["cmd_31a".to_string(), "cmd_31b".to_string()],
        disabled_patterns: vec!["pattern_31a".to_string(), "pattern_31b".to_string()],
        read_only_paths: vec!["/path/31a".to_string(), "/path/31b".to_string()],
        blocked_domains: vec!["domain_31a.com".to_string(), "domain_31b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_32", SandboxPolicy {
        disabled_commands: vec!["cmd_32a".to_string(), "cmd_32b".to_string()],
        disabled_patterns: vec!["pattern_32a".to_string(), "pattern_32b".to_string()],
        read_only_paths: vec!["/path/32a".to_string(), "/path/32b".to_string()],
        blocked_domains: vec!["domain_32a.com".to_string(), "domain_32b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_33", SandboxPolicy {
        disabled_commands: vec!["cmd_33a".to_string(), "cmd_33b".to_string()],
        disabled_patterns: vec!["pattern_33a".to_string(), "pattern_33b".to_string()],
        read_only_paths: vec!["/path/33a".to_string(), "/path/33b".to_string()],
        blocked_domains: vec!["domain_33a.com".to_string(), "domain_33b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_34", SandboxPolicy {
        disabled_commands: vec!["cmd_34a".to_string(), "cmd_34b".to_string()],
        disabled_patterns: vec!["pattern_34a".to_string(), "pattern_34b".to_string()],
        read_only_paths: vec!["/path/34a".to_string(), "/path/34b".to_string()],
        blocked_domains: vec!["domain_34a.com".to_string(), "domain_34b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_35", SandboxPolicy {
        disabled_commands: vec!["cmd_35a".to_string(), "cmd_35b".to_string()],
        disabled_patterns: vec!["pattern_35a".to_string(), "pattern_35b".to_string()],
        read_only_paths: vec!["/path/35a".to_string(), "/path/35b".to_string()],
        blocked_domains: vec!["domain_35a.com".to_string(), "domain_35b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_36", SandboxPolicy {
        disabled_commands: vec!["cmd_36a".to_string(), "cmd_36b".to_string()],
        disabled_patterns: vec!["pattern_36a".to_string(), "pattern_36b".to_string()],
        read_only_paths: vec!["/path/36a".to_string(), "/path/36b".to_string()],
        blocked_domains: vec!["domain_36a.com".to_string(), "domain_36b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_37", SandboxPolicy {
        disabled_commands: vec!["cmd_37a".to_string(), "cmd_37b".to_string()],
        disabled_patterns: vec!["pattern_37a".to_string(), "pattern_37b".to_string()],
        read_only_paths: vec!["/path/37a".to_string(), "/path/37b".to_string()],
        blocked_domains: vec!["domain_37a.com".to_string(), "domain_37b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_38", SandboxPolicy {
        disabled_commands: vec!["cmd_38a".to_string(), "cmd_38b".to_string()],
        disabled_patterns: vec!["pattern_38a".to_string(), "pattern_38b".to_string()],
        read_only_paths: vec!["/path/38a".to_string(), "/path/38b".to_string()],
        blocked_domains: vec!["domain_38a.com".to_string(), "domain_38b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_39", SandboxPolicy {
        disabled_commands: vec!["cmd_39a".to_string(), "cmd_39b".to_string()],
        disabled_patterns: vec!["pattern_39a".to_string(), "pattern_39b".to_string()],
        read_only_paths: vec!["/path/39a".to_string(), "/path/39b".to_string()],
        blocked_domains: vec!["domain_39a.com".to_string(), "domain_39b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_40", SandboxPolicy {
        disabled_commands: vec!["cmd_40a".to_string(), "cmd_40b".to_string()],
        disabled_patterns: vec!["pattern_40a".to_string(), "pattern_40b".to_string()],
        read_only_paths: vec!["/path/40a".to_string(), "/path/40b".to_string()],
        blocked_domains: vec!["domain_40a.com".to_string(), "domain_40b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_41", SandboxPolicy {
        disabled_commands: vec!["cmd_41a".to_string(), "cmd_41b".to_string()],
        disabled_patterns: vec!["pattern_41a".to_string(), "pattern_41b".to_string()],
        read_only_paths: vec!["/path/41a".to_string(), "/path/41b".to_string()],
        blocked_domains: vec!["domain_41a.com".to_string(), "domain_41b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_42", SandboxPolicy {
        disabled_commands: vec!["cmd_42a".to_string(), "cmd_42b".to_string()],
        disabled_patterns: vec!["pattern_42a".to_string(), "pattern_42b".to_string()],
        read_only_paths: vec!["/path/42a".to_string(), "/path/42b".to_string()],
        blocked_domains: vec!["domain_42a.com".to_string(), "domain_42b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_43", SandboxPolicy {
        disabled_commands: vec!["cmd_43a".to_string(), "cmd_43b".to_string()],
        disabled_patterns: vec!["pattern_43a".to_string(), "pattern_43b".to_string()],
        read_only_paths: vec!["/path/43a".to_string(), "/path/43b".to_string()],
        blocked_domains: vec!["domain_43a.com".to_string(), "domain_43b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_44", SandboxPolicy {
        disabled_commands: vec!["cmd_44a".to_string(), "cmd_44b".to_string()],
        disabled_patterns: vec!["pattern_44a".to_string(), "pattern_44b".to_string()],
        read_only_paths: vec!["/path/44a".to_string(), "/path/44b".to_string()],
        blocked_domains: vec!["domain_44a.com".to_string(), "domain_44b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_45", SandboxPolicy {
        disabled_commands: vec!["cmd_45a".to_string(), "cmd_45b".to_string()],
        disabled_patterns: vec!["pattern_45a".to_string(), "pattern_45b".to_string()],
        read_only_paths: vec!["/path/45a".to_string(), "/path/45b".to_string()],
        blocked_domains: vec!["domain_45a.com".to_string(), "domain_45b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_46", SandboxPolicy {
        disabled_commands: vec!["cmd_46a".to_string(), "cmd_46b".to_string()],
        disabled_patterns: vec!["pattern_46a".to_string(), "pattern_46b".to_string()],
        read_only_paths: vec!["/path/46a".to_string(), "/path/46b".to_string()],
        blocked_domains: vec!["domain_46a.com".to_string(), "domain_46b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_47", SandboxPolicy {
        disabled_commands: vec!["cmd_47a".to_string(), "cmd_47b".to_string()],
        disabled_patterns: vec!["pattern_47a".to_string(), "pattern_47b".to_string()],
        read_only_paths: vec!["/path/47a".to_string(), "/path/47b".to_string()],
        blocked_domains: vec!["domain_47a.com".to_string(), "domain_47b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_48", SandboxPolicy {
        disabled_commands: vec!["cmd_48a".to_string(), "cmd_48b".to_string()],
        disabled_patterns: vec!["pattern_48a".to_string(), "pattern_48b".to_string()],
        read_only_paths: vec!["/path/48a".to_string(), "/path/48b".to_string()],
        blocked_domains: vec!["domain_48a.com".to_string(), "domain_48b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_49", SandboxPolicy {
        disabled_commands: vec!["cmd_49a".to_string(), "cmd_49b".to_string()],
        disabled_patterns: vec!["pattern_49a".to_string(), "pattern_49b".to_string()],
        read_only_paths: vec!["/path/49a".to_string(), "/path/49b".to_string()],
        blocked_domains: vec!["domain_49a.com".to_string(), "domain_49b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_50", SandboxPolicy {
        disabled_commands: vec!["cmd_50a".to_string(), "cmd_50b".to_string()],
        disabled_patterns: vec!["pattern_50a".to_string(), "pattern_50b".to_string()],
        read_only_paths: vec!["/path/50a".to_string(), "/path/50b".to_string()],
        blocked_domains: vec!["domain_50a.com".to_string(), "domain_50b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_51", SandboxPolicy {
        disabled_commands: vec!["cmd_51a".to_string(), "cmd_51b".to_string()],
        disabled_patterns: vec!["pattern_51a".to_string(), "pattern_51b".to_string()],
        read_only_paths: vec!["/path/51a".to_string(), "/path/51b".to_string()],
        blocked_domains: vec!["domain_51a.com".to_string(), "domain_51b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_52", SandboxPolicy {
        disabled_commands: vec!["cmd_52a".to_string(), "cmd_52b".to_string()],
        disabled_patterns: vec!["pattern_52a".to_string(), "pattern_52b".to_string()],
        read_only_paths: vec!["/path/52a".to_string(), "/path/52b".to_string()],
        blocked_domains: vec!["domain_52a.com".to_string(), "domain_52b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_53", SandboxPolicy {
        disabled_commands: vec!["cmd_53a".to_string(), "cmd_53b".to_string()],
        disabled_patterns: vec!["pattern_53a".to_string(), "pattern_53b".to_string()],
        read_only_paths: vec!["/path/53a".to_string(), "/path/53b".to_string()],
        blocked_domains: vec!["domain_53a.com".to_string(), "domain_53b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_54", SandboxPolicy {
        disabled_commands: vec!["cmd_54a".to_string(), "cmd_54b".to_string()],
        disabled_patterns: vec!["pattern_54a".to_string(), "pattern_54b".to_string()],
        read_only_paths: vec!["/path/54a".to_string(), "/path/54b".to_string()],
        blocked_domains: vec!["domain_54a.com".to_string(), "domain_54b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_55", SandboxPolicy {
        disabled_commands: vec!["cmd_55a".to_string(), "cmd_55b".to_string()],
        disabled_patterns: vec!["pattern_55a".to_string(), "pattern_55b".to_string()],
        read_only_paths: vec!["/path/55a".to_string(), "/path/55b".to_string()],
        blocked_domains: vec!["domain_55a.com".to_string(), "domain_55b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_56", SandboxPolicy {
        disabled_commands: vec!["cmd_56a".to_string(), "cmd_56b".to_string()],
        disabled_patterns: vec!["pattern_56a".to_string(), "pattern_56b".to_string()],
        read_only_paths: vec!["/path/56a".to_string(), "/path/56b".to_string()],
        blocked_domains: vec!["domain_56a.com".to_string(), "domain_56b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_57", SandboxPolicy {
        disabled_commands: vec!["cmd_57a".to_string(), "cmd_57b".to_string()],
        disabled_patterns: vec!["pattern_57a".to_string(), "pattern_57b".to_string()],
        read_only_paths: vec!["/path/57a".to_string(), "/path/57b".to_string()],
        blocked_domains: vec!["domain_57a.com".to_string(), "domain_57b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_58", SandboxPolicy {
        disabled_commands: vec!["cmd_58a".to_string(), "cmd_58b".to_string()],
        disabled_patterns: vec!["pattern_58a".to_string(), "pattern_58b".to_string()],
        read_only_paths: vec!["/path/58a".to_string(), "/path/58b".to_string()],
        blocked_domains: vec!["domain_58a.com".to_string(), "domain_58b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_59", SandboxPolicy {
        disabled_commands: vec!["cmd_59a".to_string(), "cmd_59b".to_string()],
        disabled_patterns: vec!["pattern_59a".to_string(), "pattern_59b".to_string()],
        read_only_paths: vec!["/path/59a".to_string(), "/path/59b".to_string()],
        blocked_domains: vec!["domain_59a.com".to_string(), "domain_59b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_60", SandboxPolicy {
        disabled_commands: vec!["cmd_60a".to_string(), "cmd_60b".to_string()],
        disabled_patterns: vec!["pattern_60a".to_string(), "pattern_60b".to_string()],
        read_only_paths: vec!["/path/60a".to_string(), "/path/60b".to_string()],
        blocked_domains: vec!["domain_60a.com".to_string(), "domain_60b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_61", SandboxPolicy {
        disabled_commands: vec!["cmd_61a".to_string(), "cmd_61b".to_string()],
        disabled_patterns: vec!["pattern_61a".to_string(), "pattern_61b".to_string()],
        read_only_paths: vec!["/path/61a".to_string(), "/path/61b".to_string()],
        blocked_domains: vec!["domain_61a.com".to_string(), "domain_61b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_62", SandboxPolicy {
        disabled_commands: vec!["cmd_62a".to_string(), "cmd_62b".to_string()],
        disabled_patterns: vec!["pattern_62a".to_string(), "pattern_62b".to_string()],
        read_only_paths: vec!["/path/62a".to_string(), "/path/62b".to_string()],
        blocked_domains: vec!["domain_62a.com".to_string(), "domain_62b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_63", SandboxPolicy {
        disabled_commands: vec!["cmd_63a".to_string(), "cmd_63b".to_string()],
        disabled_patterns: vec!["pattern_63a".to_string(), "pattern_63b".to_string()],
        read_only_paths: vec!["/path/63a".to_string(), "/path/63b".to_string()],
        blocked_domains: vec!["domain_63a.com".to_string(), "domain_63b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_64", SandboxPolicy {
        disabled_commands: vec!["cmd_64a".to_string(), "cmd_64b".to_string()],
        disabled_patterns: vec!["pattern_64a".to_string(), "pattern_64b".to_string()],
        read_only_paths: vec!["/path/64a".to_string(), "/path/64b".to_string()],
        blocked_domains: vec!["domain_64a.com".to_string(), "domain_64b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_65", SandboxPolicy {
        disabled_commands: vec!["cmd_65a".to_string(), "cmd_65b".to_string()],
        disabled_patterns: vec!["pattern_65a".to_string(), "pattern_65b".to_string()],
        read_only_paths: vec!["/path/65a".to_string(), "/path/65b".to_string()],
        blocked_domains: vec!["domain_65a.com".to_string(), "domain_65b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_66", SandboxPolicy {
        disabled_commands: vec!["cmd_66a".to_string(), "cmd_66b".to_string()],
        disabled_patterns: vec!["pattern_66a".to_string(), "pattern_66b".to_string()],
        read_only_paths: vec!["/path/66a".to_string(), "/path/66b".to_string()],
        blocked_domains: vec!["domain_66a.com".to_string(), "domain_66b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_67", SandboxPolicy {
        disabled_commands: vec!["cmd_67a".to_string(), "cmd_67b".to_string()],
        disabled_patterns: vec!["pattern_67a".to_string(), "pattern_67b".to_string()],
        read_only_paths: vec!["/path/67a".to_string(), "/path/67b".to_string()],
        blocked_domains: vec!["domain_67a.com".to_string(), "domain_67b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_68", SandboxPolicy {
        disabled_commands: vec!["cmd_68a".to_string(), "cmd_68b".to_string()],
        disabled_patterns: vec!["pattern_68a".to_string(), "pattern_68b".to_string()],
        read_only_paths: vec!["/path/68a".to_string(), "/path/68b".to_string()],
        blocked_domains: vec!["domain_68a.com".to_string(), "domain_68b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_69", SandboxPolicy {
        disabled_commands: vec!["cmd_69a".to_string(), "cmd_69b".to_string()],
        disabled_patterns: vec!["pattern_69a".to_string(), "pattern_69b".to_string()],
        read_only_paths: vec!["/path/69a".to_string(), "/path/69b".to_string()],
        blocked_domains: vec!["domain_69a.com".to_string(), "domain_69b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_70", SandboxPolicy {
        disabled_commands: vec!["cmd_70a".to_string(), "cmd_70b".to_string()],
        disabled_patterns: vec!["pattern_70a".to_string(), "pattern_70b".to_string()],
        read_only_paths: vec!["/path/70a".to_string(), "/path/70b".to_string()],
        blocked_domains: vec!["domain_70a.com".to_string(), "domain_70b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_71", SandboxPolicy {
        disabled_commands: vec!["cmd_71a".to_string(), "cmd_71b".to_string()],
        disabled_patterns: vec!["pattern_71a".to_string(), "pattern_71b".to_string()],
        read_only_paths: vec!["/path/71a".to_string(), "/path/71b".to_string()],
        blocked_domains: vec!["domain_71a.com".to_string(), "domain_71b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_72", SandboxPolicy {
        disabled_commands: vec!["cmd_72a".to_string(), "cmd_72b".to_string()],
        disabled_patterns: vec!["pattern_72a".to_string(), "pattern_72b".to_string()],
        read_only_paths: vec!["/path/72a".to_string(), "/path/72b".to_string()],
        blocked_domains: vec!["domain_72a.com".to_string(), "domain_72b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_73", SandboxPolicy {
        disabled_commands: vec!["cmd_73a".to_string(), "cmd_73b".to_string()],
        disabled_patterns: vec!["pattern_73a".to_string(), "pattern_73b".to_string()],
        read_only_paths: vec!["/path/73a".to_string(), "/path/73b".to_string()],
        blocked_domains: vec!["domain_73a.com".to_string(), "domain_73b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_74", SandboxPolicy {
        disabled_commands: vec!["cmd_74a".to_string(), "cmd_74b".to_string()],
        disabled_patterns: vec!["pattern_74a".to_string(), "pattern_74b".to_string()],
        read_only_paths: vec!["/path/74a".to_string(), "/path/74b".to_string()],
        blocked_domains: vec!["domain_74a.com".to_string(), "domain_74b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_75", SandboxPolicy {
        disabled_commands: vec!["cmd_75a".to_string(), "cmd_75b".to_string()],
        disabled_patterns: vec!["pattern_75a".to_string(), "pattern_75b".to_string()],
        read_only_paths: vec!["/path/75a".to_string(), "/path/75b".to_string()],
        blocked_domains: vec!["domain_75a.com".to_string(), "domain_75b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_76", SandboxPolicy {
        disabled_commands: vec!["cmd_76a".to_string(), "cmd_76b".to_string()],
        disabled_patterns: vec!["pattern_76a".to_string(), "pattern_76b".to_string()],
        read_only_paths: vec!["/path/76a".to_string(), "/path/76b".to_string()],
        blocked_domains: vec!["domain_76a.com".to_string(), "domain_76b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_77", SandboxPolicy {
        disabled_commands: vec!["cmd_77a".to_string(), "cmd_77b".to_string()],
        disabled_patterns: vec!["pattern_77a".to_string(), "pattern_77b".to_string()],
        read_only_paths: vec!["/path/77a".to_string(), "/path/77b".to_string()],
        blocked_domains: vec!["domain_77a.com".to_string(), "domain_77b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_78", SandboxPolicy {
        disabled_commands: vec!["cmd_78a".to_string(), "cmd_78b".to_string()],
        disabled_patterns: vec!["pattern_78a".to_string(), "pattern_78b".to_string()],
        read_only_paths: vec!["/path/78a".to_string(), "/path/78b".to_string()],
        blocked_domains: vec!["domain_78a.com".to_string(), "domain_78b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_79", SandboxPolicy {
        disabled_commands: vec!["cmd_79a".to_string(), "cmd_79b".to_string()],
        disabled_patterns: vec!["pattern_79a".to_string(), "pattern_79b".to_string()],
        read_only_paths: vec!["/path/79a".to_string(), "/path/79b".to_string()],
        blocked_domains: vec!["domain_79a.com".to_string(), "domain_79b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_80", SandboxPolicy {
        disabled_commands: vec!["cmd_80a".to_string(), "cmd_80b".to_string()],
        disabled_patterns: vec!["pattern_80a".to_string(), "pattern_80b".to_string()],
        read_only_paths: vec!["/path/80a".to_string(), "/path/80b".to_string()],
        blocked_domains: vec!["domain_80a.com".to_string(), "domain_80b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_81", SandboxPolicy {
        disabled_commands: vec!["cmd_81a".to_string(), "cmd_81b".to_string()],
        disabled_patterns: vec!["pattern_81a".to_string(), "pattern_81b".to_string()],
        read_only_paths: vec!["/path/81a".to_string(), "/path/81b".to_string()],
        blocked_domains: vec!["domain_81a.com".to_string(), "domain_81b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_82", SandboxPolicy {
        disabled_commands: vec!["cmd_82a".to_string(), "cmd_82b".to_string()],
        disabled_patterns: vec!["pattern_82a".to_string(), "pattern_82b".to_string()],
        read_only_paths: vec!["/path/82a".to_string(), "/path/82b".to_string()],
        blocked_domains: vec!["domain_82a.com".to_string(), "domain_82b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_83", SandboxPolicy {
        disabled_commands: vec!["cmd_83a".to_string(), "cmd_83b".to_string()],
        disabled_patterns: vec!["pattern_83a".to_string(), "pattern_83b".to_string()],
        read_only_paths: vec!["/path/83a".to_string(), "/path/83b".to_string()],
        blocked_domains: vec!["domain_83a.com".to_string(), "domain_83b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_84", SandboxPolicy {
        disabled_commands: vec!["cmd_84a".to_string(), "cmd_84b".to_string()],
        disabled_patterns: vec!["pattern_84a".to_string(), "pattern_84b".to_string()],
        read_only_paths: vec!["/path/84a".to_string(), "/path/84b".to_string()],
        blocked_domains: vec!["domain_84a.com".to_string(), "domain_84b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_85", SandboxPolicy {
        disabled_commands: vec!["cmd_85a".to_string(), "cmd_85b".to_string()],
        disabled_patterns: vec!["pattern_85a".to_string(), "pattern_85b".to_string()],
        read_only_paths: vec!["/path/85a".to_string(), "/path/85b".to_string()],
        blocked_domains: vec!["domain_85a.com".to_string(), "domain_85b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_86", SandboxPolicy {
        disabled_commands: vec!["cmd_86a".to_string(), "cmd_86b".to_string()],
        disabled_patterns: vec!["pattern_86a".to_string(), "pattern_86b".to_string()],
        read_only_paths: vec!["/path/86a".to_string(), "/path/86b".to_string()],
        blocked_domains: vec!["domain_86a.com".to_string(), "domain_86b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_87", SandboxPolicy {
        disabled_commands: vec!["cmd_87a".to_string(), "cmd_87b".to_string()],
        disabled_patterns: vec!["pattern_87a".to_string(), "pattern_87b".to_string()],
        read_only_paths: vec!["/path/87a".to_string(), "/path/87b".to_string()],
        blocked_domains: vec!["domain_87a.com".to_string(), "domain_87b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_88", SandboxPolicy {
        disabled_commands: vec!["cmd_88a".to_string(), "cmd_88b".to_string()],
        disabled_patterns: vec!["pattern_88a".to_string(), "pattern_88b".to_string()],
        read_only_paths: vec!["/path/88a".to_string(), "/path/88b".to_string()],
        blocked_domains: vec!["domain_88a.com".to_string(), "domain_88b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_89", SandboxPolicy {
        disabled_commands: vec!["cmd_89a".to_string(), "cmd_89b".to_string()],
        disabled_patterns: vec!["pattern_89a".to_string(), "pattern_89b".to_string()],
        read_only_paths: vec!["/path/89a".to_string(), "/path/89b".to_string()],
        blocked_domains: vec!["domain_89a.com".to_string(), "domain_89b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_90", SandboxPolicy {
        disabled_commands: vec!["cmd_90a".to_string(), "cmd_90b".to_string()],
        disabled_patterns: vec!["pattern_90a".to_string(), "pattern_90b".to_string()],
        read_only_paths: vec!["/path/90a".to_string(), "/path/90b".to_string()],
        blocked_domains: vec!["domain_90a.com".to_string(), "domain_90b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_91", SandboxPolicy {
        disabled_commands: vec!["cmd_91a".to_string(), "cmd_91b".to_string()],
        disabled_patterns: vec!["pattern_91a".to_string(), "pattern_91b".to_string()],
        read_only_paths: vec!["/path/91a".to_string(), "/path/91b".to_string()],
        blocked_domains: vec!["domain_91a.com".to_string(), "domain_91b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_92", SandboxPolicy {
        disabled_commands: vec!["cmd_92a".to_string(), "cmd_92b".to_string()],
        disabled_patterns: vec!["pattern_92a".to_string(), "pattern_92b".to_string()],
        read_only_paths: vec!["/path/92a".to_string(), "/path/92b".to_string()],
        blocked_domains: vec!["domain_92a.com".to_string(), "domain_92b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_93", SandboxPolicy {
        disabled_commands: vec!["cmd_93a".to_string(), "cmd_93b".to_string()],
        disabled_patterns: vec!["pattern_93a".to_string(), "pattern_93b".to_string()],
        read_only_paths: vec!["/path/93a".to_string(), "/path/93b".to_string()],
        blocked_domains: vec!["domain_93a.com".to_string(), "domain_93b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_94", SandboxPolicy {
        disabled_commands: vec!["cmd_94a".to_string(), "cmd_94b".to_string()],
        disabled_patterns: vec!["pattern_94a".to_string(), "pattern_94b".to_string()],
        read_only_paths: vec!["/path/94a".to_string(), "/path/94b".to_string()],
        blocked_domains: vec!["domain_94a.com".to_string(), "domain_94b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_95", SandboxPolicy {
        disabled_commands: vec!["cmd_95a".to_string(), "cmd_95b".to_string()],
        disabled_patterns: vec!["pattern_95a".to_string(), "pattern_95b".to_string()],
        read_only_paths: vec!["/path/95a".to_string(), "/path/95b".to_string()],
        blocked_domains: vec!["domain_95a.com".to_string(), "domain_95b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_96", SandboxPolicy {
        disabled_commands: vec!["cmd_96a".to_string(), "cmd_96b".to_string()],
        disabled_patterns: vec!["pattern_96a".to_string(), "pattern_96b".to_string()],
        read_only_paths: vec!["/path/96a".to_string(), "/path/96b".to_string()],
        blocked_domains: vec!["domain_96a.com".to_string(), "domain_96b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_97", SandboxPolicy {
        disabled_commands: vec!["cmd_97a".to_string(), "cmd_97b".to_string()],
        disabled_patterns: vec!["pattern_97a".to_string(), "pattern_97b".to_string()],
        read_only_paths: vec!["/path/97a".to_string(), "/path/97b".to_string()],
        blocked_domains: vec!["domain_97a.com".to_string(), "domain_97b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_98", SandboxPolicy {
        disabled_commands: vec!["cmd_98a".to_string(), "cmd_98b".to_string()],
        disabled_patterns: vec!["pattern_98a".to_string(), "pattern_98b".to_string()],
        read_only_paths: vec!["/path/98a".to_string(), "/path/98b".to_string()],
        blocked_domains: vec!["domain_98a.com".to_string(), "domain_98b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_99", SandboxPolicy {
        disabled_commands: vec!["cmd_99a".to_string(), "cmd_99b".to_string()],
        disabled_patterns: vec!["pattern_99a".to_string(), "pattern_99b".to_string()],
        read_only_paths: vec!["/path/99a".to_string(), "/path/99b".to_string()],
        blocked_domains: vec!["domain_99a.com".to_string(), "domain_99b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_100", SandboxPolicy {
        disabled_commands: vec!["cmd_100a".to_string(), "cmd_100b".to_string()],
        disabled_patterns: vec!["pattern_100a".to_string(), "pattern_100b".to_string()],
        read_only_paths: vec!["/path/100a".to_string(), "/path/100b".to_string()],
        blocked_domains: vec!["domain_100a.com".to_string(), "domain_100b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_101", SandboxPolicy {
        disabled_commands: vec!["cmd_101a".to_string(), "cmd_101b".to_string()],
        disabled_patterns: vec!["pattern_101a".to_string(), "pattern_101b".to_string()],
        read_only_paths: vec!["/path/101a".to_string(), "/path/101b".to_string()],
        blocked_domains: vec!["domain_101a.com".to_string(), "domain_101b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_102", SandboxPolicy {
        disabled_commands: vec!["cmd_102a".to_string(), "cmd_102b".to_string()],
        disabled_patterns: vec!["pattern_102a".to_string(), "pattern_102b".to_string()],
        read_only_paths: vec!["/path/102a".to_string(), "/path/102b".to_string()],
        blocked_domains: vec!["domain_102a.com".to_string(), "domain_102b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_103", SandboxPolicy {
        disabled_commands: vec!["cmd_103a".to_string(), "cmd_103b".to_string()],
        disabled_patterns: vec!["pattern_103a".to_string(), "pattern_103b".to_string()],
        read_only_paths: vec!["/path/103a".to_string(), "/path/103b".to_string()],
        blocked_domains: vec!["domain_103a.com".to_string(), "domain_103b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_104", SandboxPolicy {
        disabled_commands: vec!["cmd_104a".to_string(), "cmd_104b".to_string()],
        disabled_patterns: vec!["pattern_104a".to_string(), "pattern_104b".to_string()],
        read_only_paths: vec!["/path/104a".to_string(), "/path/104b".to_string()],
        blocked_domains: vec!["domain_104a.com".to_string(), "domain_104b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_105", SandboxPolicy {
        disabled_commands: vec!["cmd_105a".to_string(), "cmd_105b".to_string()],
        disabled_patterns: vec!["pattern_105a".to_string(), "pattern_105b".to_string()],
        read_only_paths: vec!["/path/105a".to_string(), "/path/105b".to_string()],
        blocked_domains: vec!["domain_105a.com".to_string(), "domain_105b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_106", SandboxPolicy {
        disabled_commands: vec!["cmd_106a".to_string(), "cmd_106b".to_string()],
        disabled_patterns: vec!["pattern_106a".to_string(), "pattern_106b".to_string()],
        read_only_paths: vec!["/path/106a".to_string(), "/path/106b".to_string()],
        blocked_domains: vec!["domain_106a.com".to_string(), "domain_106b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_107", SandboxPolicy {
        disabled_commands: vec!["cmd_107a".to_string(), "cmd_107b".to_string()],
        disabled_patterns: vec!["pattern_107a".to_string(), "pattern_107b".to_string()],
        read_only_paths: vec!["/path/107a".to_string(), "/path/107b".to_string()],
        blocked_domains: vec!["domain_107a.com".to_string(), "domain_107b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_108", SandboxPolicy {
        disabled_commands: vec!["cmd_108a".to_string(), "cmd_108b".to_string()],
        disabled_patterns: vec!["pattern_108a".to_string(), "pattern_108b".to_string()],
        read_only_paths: vec!["/path/108a".to_string(), "/path/108b".to_string()],
        blocked_domains: vec!["domain_108a.com".to_string(), "domain_108b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_109", SandboxPolicy {
        disabled_commands: vec!["cmd_109a".to_string(), "cmd_109b".to_string()],
        disabled_patterns: vec!["pattern_109a".to_string(), "pattern_109b".to_string()],
        read_only_paths: vec!["/path/109a".to_string(), "/path/109b".to_string()],
        blocked_domains: vec!["domain_109a.com".to_string(), "domain_109b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_110", SandboxPolicy {
        disabled_commands: vec!["cmd_110a".to_string(), "cmd_110b".to_string()],
        disabled_patterns: vec!["pattern_110a".to_string(), "pattern_110b".to_string()],
        read_only_paths: vec!["/path/110a".to_string(), "/path/110b".to_string()],
        blocked_domains: vec!["domain_110a.com".to_string(), "domain_110b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_111", SandboxPolicy {
        disabled_commands: vec!["cmd_111a".to_string(), "cmd_111b".to_string()],
        disabled_patterns: vec!["pattern_111a".to_string(), "pattern_111b".to_string()],
        read_only_paths: vec!["/path/111a".to_string(), "/path/111b".to_string()],
        blocked_domains: vec!["domain_111a.com".to_string(), "domain_111b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_112", SandboxPolicy {
        disabled_commands: vec!["cmd_112a".to_string(), "cmd_112b".to_string()],
        disabled_patterns: vec!["pattern_112a".to_string(), "pattern_112b".to_string()],
        read_only_paths: vec!["/path/112a".to_string(), "/path/112b".to_string()],
        blocked_domains: vec!["domain_112a.com".to_string(), "domain_112b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_113", SandboxPolicy {
        disabled_commands: vec!["cmd_113a".to_string(), "cmd_113b".to_string()],
        disabled_patterns: vec!["pattern_113a".to_string(), "pattern_113b".to_string()],
        read_only_paths: vec!["/path/113a".to_string(), "/path/113b".to_string()],
        blocked_domains: vec!["domain_113a.com".to_string(), "domain_113b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_114", SandboxPolicy {
        disabled_commands: vec!["cmd_114a".to_string(), "cmd_114b".to_string()],
        disabled_patterns: vec!["pattern_114a".to_string(), "pattern_114b".to_string()],
        read_only_paths: vec!["/path/114a".to_string(), "/path/114b".to_string()],
        blocked_domains: vec!["domain_114a.com".to_string(), "domain_114b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_115", SandboxPolicy {
        disabled_commands: vec!["cmd_115a".to_string(), "cmd_115b".to_string()],
        disabled_patterns: vec!["pattern_115a".to_string(), "pattern_115b".to_string()],
        read_only_paths: vec!["/path/115a".to_string(), "/path/115b".to_string()],
        blocked_domains: vec!["domain_115a.com".to_string(), "domain_115b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_116", SandboxPolicy {
        disabled_commands: vec!["cmd_116a".to_string(), "cmd_116b".to_string()],
        disabled_patterns: vec!["pattern_116a".to_string(), "pattern_116b".to_string()],
        read_only_paths: vec!["/path/116a".to_string(), "/path/116b".to_string()],
        blocked_domains: vec!["domain_116a.com".to_string(), "domain_116b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_117", SandboxPolicy {
        disabled_commands: vec!["cmd_117a".to_string(), "cmd_117b".to_string()],
        disabled_patterns: vec!["pattern_117a".to_string(), "pattern_117b".to_string()],
        read_only_paths: vec!["/path/117a".to_string(), "/path/117b".to_string()],
        blocked_domains: vec!["domain_117a.com".to_string(), "domain_117b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_118", SandboxPolicy {
        disabled_commands: vec!["cmd_118a".to_string(), "cmd_118b".to_string()],
        disabled_patterns: vec!["pattern_118a".to_string(), "pattern_118b".to_string()],
        read_only_paths: vec!["/path/118a".to_string(), "/path/118b".to_string()],
        blocked_domains: vec!["domain_118a.com".to_string(), "domain_118b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_119", SandboxPolicy {
        disabled_commands: vec!["cmd_119a".to_string(), "cmd_119b".to_string()],
        disabled_patterns: vec!["pattern_119a".to_string(), "pattern_119b".to_string()],
        read_only_paths: vec!["/path/119a".to_string(), "/path/119b".to_string()],
        blocked_domains: vec!["domain_119a.com".to_string(), "domain_119b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_120", SandboxPolicy {
        disabled_commands: vec!["cmd_120a".to_string(), "cmd_120b".to_string()],
        disabled_patterns: vec!["pattern_120a".to_string(), "pattern_120b".to_string()],
        read_only_paths: vec!["/path/120a".to_string(), "/path/120b".to_string()],
        blocked_domains: vec!["domain_120a.com".to_string(), "domain_120b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_121", SandboxPolicy {
        disabled_commands: vec!["cmd_121a".to_string(), "cmd_121b".to_string()],
        disabled_patterns: vec!["pattern_121a".to_string(), "pattern_121b".to_string()],
        read_only_paths: vec!["/path/121a".to_string(), "/path/121b".to_string()],
        blocked_domains: vec!["domain_121a.com".to_string(), "domain_121b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_122", SandboxPolicy {
        disabled_commands: vec!["cmd_122a".to_string(), "cmd_122b".to_string()],
        disabled_patterns: vec!["pattern_122a".to_string(), "pattern_122b".to_string()],
        read_only_paths: vec!["/path/122a".to_string(), "/path/122b".to_string()],
        blocked_domains: vec!["domain_122a.com".to_string(), "domain_122b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_123", SandboxPolicy {
        disabled_commands: vec!["cmd_123a".to_string(), "cmd_123b".to_string()],
        disabled_patterns: vec!["pattern_123a".to_string(), "pattern_123b".to_string()],
        read_only_paths: vec!["/path/123a".to_string(), "/path/123b".to_string()],
        blocked_domains: vec!["domain_123a.com".to_string(), "domain_123b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_124", SandboxPolicy {
        disabled_commands: vec!["cmd_124a".to_string(), "cmd_124b".to_string()],
        disabled_patterns: vec!["pattern_124a".to_string(), "pattern_124b".to_string()],
        read_only_paths: vec!["/path/124a".to_string(), "/path/124b".to_string()],
        blocked_domains: vec!["domain_124a.com".to_string(), "domain_124b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_125", SandboxPolicy {
        disabled_commands: vec!["cmd_125a".to_string(), "cmd_125b".to_string()],
        disabled_patterns: vec!["pattern_125a".to_string(), "pattern_125b".to_string()],
        read_only_paths: vec!["/path/125a".to_string(), "/path/125b".to_string()],
        blocked_domains: vec!["domain_125a.com".to_string(), "domain_125b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_126", SandboxPolicy {
        disabled_commands: vec!["cmd_126a".to_string(), "cmd_126b".to_string()],
        disabled_patterns: vec!["pattern_126a".to_string(), "pattern_126b".to_string()],
        read_only_paths: vec!["/path/126a".to_string(), "/path/126b".to_string()],
        blocked_domains: vec!["domain_126a.com".to_string(), "domain_126b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_127", SandboxPolicy {
        disabled_commands: vec!["cmd_127a".to_string(), "cmd_127b".to_string()],
        disabled_patterns: vec!["pattern_127a".to_string(), "pattern_127b".to_string()],
        read_only_paths: vec!["/path/127a".to_string(), "/path/127b".to_string()],
        blocked_domains: vec!["domain_127a.com".to_string(), "domain_127b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_128", SandboxPolicy {
        disabled_commands: vec!["cmd_128a".to_string(), "cmd_128b".to_string()],
        disabled_patterns: vec!["pattern_128a".to_string(), "pattern_128b".to_string()],
        read_only_paths: vec!["/path/128a".to_string(), "/path/128b".to_string()],
        blocked_domains: vec!["domain_128a.com".to_string(), "domain_128b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_129", SandboxPolicy {
        disabled_commands: vec!["cmd_129a".to_string(), "cmd_129b".to_string()],
        disabled_patterns: vec!["pattern_129a".to_string(), "pattern_129b".to_string()],
        read_only_paths: vec!["/path/129a".to_string(), "/path/129b".to_string()],
        blocked_domains: vec!["domain_129a.com".to_string(), "domain_129b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_130", SandboxPolicy {
        disabled_commands: vec!["cmd_130a".to_string(), "cmd_130b".to_string()],
        disabled_patterns: vec!["pattern_130a".to_string(), "pattern_130b".to_string()],
        read_only_paths: vec!["/path/130a".to_string(), "/path/130b".to_string()],
        blocked_domains: vec!["domain_130a.com".to_string(), "domain_130b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_131", SandboxPolicy {
        disabled_commands: vec!["cmd_131a".to_string(), "cmd_131b".to_string()],
        disabled_patterns: vec!["pattern_131a".to_string(), "pattern_131b".to_string()],
        read_only_paths: vec!["/path/131a".to_string(), "/path/131b".to_string()],
        blocked_domains: vec!["domain_131a.com".to_string(), "domain_131b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_132", SandboxPolicy {
        disabled_commands: vec!["cmd_132a".to_string(), "cmd_132b".to_string()],
        disabled_patterns: vec!["pattern_132a".to_string(), "pattern_132b".to_string()],
        read_only_paths: vec!["/path/132a".to_string(), "/path/132b".to_string()],
        blocked_domains: vec!["domain_132a.com".to_string(), "domain_132b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_133", SandboxPolicy {
        disabled_commands: vec!["cmd_133a".to_string(), "cmd_133b".to_string()],
        disabled_patterns: vec!["pattern_133a".to_string(), "pattern_133b".to_string()],
        read_only_paths: vec!["/path/133a".to_string(), "/path/133b".to_string()],
        blocked_domains: vec!["domain_133a.com".to_string(), "domain_133b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_134", SandboxPolicy {
        disabled_commands: vec!["cmd_134a".to_string(), "cmd_134b".to_string()],
        disabled_patterns: vec!["pattern_134a".to_string(), "pattern_134b".to_string()],
        read_only_paths: vec!["/path/134a".to_string(), "/path/134b".to_string()],
        blocked_domains: vec!["domain_134a.com".to_string(), "domain_134b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_135", SandboxPolicy {
        disabled_commands: vec!["cmd_135a".to_string(), "cmd_135b".to_string()],
        disabled_patterns: vec!["pattern_135a".to_string(), "pattern_135b".to_string()],
        read_only_paths: vec!["/path/135a".to_string(), "/path/135b".to_string()],
        blocked_domains: vec!["domain_135a.com".to_string(), "domain_135b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_136", SandboxPolicy {
        disabled_commands: vec!["cmd_136a".to_string(), "cmd_136b".to_string()],
        disabled_patterns: vec!["pattern_136a".to_string(), "pattern_136b".to_string()],
        read_only_paths: vec!["/path/136a".to_string(), "/path/136b".to_string()],
        blocked_domains: vec!["domain_136a.com".to_string(), "domain_136b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_137", SandboxPolicy {
        disabled_commands: vec!["cmd_137a".to_string(), "cmd_137b".to_string()],
        disabled_patterns: vec!["pattern_137a".to_string(), "pattern_137b".to_string()],
        read_only_paths: vec!["/path/137a".to_string(), "/path/137b".to_string()],
        blocked_domains: vec!["domain_137a.com".to_string(), "domain_137b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_138", SandboxPolicy {
        disabled_commands: vec!["cmd_138a".to_string(), "cmd_138b".to_string()],
        disabled_patterns: vec!["pattern_138a".to_string(), "pattern_138b".to_string()],
        read_only_paths: vec!["/path/138a".to_string(), "/path/138b".to_string()],
        blocked_domains: vec!["domain_138a.com".to_string(), "domain_138b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_139", SandboxPolicy {
        disabled_commands: vec!["cmd_139a".to_string(), "cmd_139b".to_string()],
        disabled_patterns: vec!["pattern_139a".to_string(), "pattern_139b".to_string()],
        read_only_paths: vec!["/path/139a".to_string(), "/path/139b".to_string()],
        blocked_domains: vec!["domain_139a.com".to_string(), "domain_139b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_140", SandboxPolicy {
        disabled_commands: vec!["cmd_140a".to_string(), "cmd_140b".to_string()],
        disabled_patterns: vec!["pattern_140a".to_string(), "pattern_140b".to_string()],
        read_only_paths: vec!["/path/140a".to_string(), "/path/140b".to_string()],
        blocked_domains: vec!["domain_140a.com".to_string(), "domain_140b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_141", SandboxPolicy {
        disabled_commands: vec!["cmd_141a".to_string(), "cmd_141b".to_string()],
        disabled_patterns: vec!["pattern_141a".to_string(), "pattern_141b".to_string()],
        read_only_paths: vec!["/path/141a".to_string(), "/path/141b".to_string()],
        blocked_domains: vec!["domain_141a.com".to_string(), "domain_141b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_142", SandboxPolicy {
        disabled_commands: vec!["cmd_142a".to_string(), "cmd_142b".to_string()],
        disabled_patterns: vec!["pattern_142a".to_string(), "pattern_142b".to_string()],
        read_only_paths: vec!["/path/142a".to_string(), "/path/142b".to_string()],
        blocked_domains: vec!["domain_142a.com".to_string(), "domain_142b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_143", SandboxPolicy {
        disabled_commands: vec!["cmd_143a".to_string(), "cmd_143b".to_string()],
        disabled_patterns: vec!["pattern_143a".to_string(), "pattern_143b".to_string()],
        read_only_paths: vec!["/path/143a".to_string(), "/path/143b".to_string()],
        blocked_domains: vec!["domain_143a.com".to_string(), "domain_143b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_144", SandboxPolicy {
        disabled_commands: vec!["cmd_144a".to_string(), "cmd_144b".to_string()],
        disabled_patterns: vec!["pattern_144a".to_string(), "pattern_144b".to_string()],
        read_only_paths: vec!["/path/144a".to_string(), "/path/144b".to_string()],
        blocked_domains: vec!["domain_144a.com".to_string(), "domain_144b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_145", SandboxPolicy {
        disabled_commands: vec!["cmd_145a".to_string(), "cmd_145b".to_string()],
        disabled_patterns: vec!["pattern_145a".to_string(), "pattern_145b".to_string()],
        read_only_paths: vec!["/path/145a".to_string(), "/path/145b".to_string()],
        blocked_domains: vec!["domain_145a.com".to_string(), "domain_145b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_146", SandboxPolicy {
        disabled_commands: vec!["cmd_146a".to_string(), "cmd_146b".to_string()],
        disabled_patterns: vec!["pattern_146a".to_string(), "pattern_146b".to_string()],
        read_only_paths: vec!["/path/146a".to_string(), "/path/146b".to_string()],
        blocked_domains: vec!["domain_146a.com".to_string(), "domain_146b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_147", SandboxPolicy {
        disabled_commands: vec!["cmd_147a".to_string(), "cmd_147b".to_string()],
        disabled_patterns: vec!["pattern_147a".to_string(), "pattern_147b".to_string()],
        read_only_paths: vec!["/path/147a".to_string(), "/path/147b".to_string()],
        blocked_domains: vec!["domain_147a.com".to_string(), "domain_147b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_148", SandboxPolicy {
        disabled_commands: vec!["cmd_148a".to_string(), "cmd_148b".to_string()],
        disabled_patterns: vec!["pattern_148a".to_string(), "pattern_148b".to_string()],
        read_only_paths: vec!["/path/148a".to_string(), "/path/148b".to_string()],
        blocked_domains: vec!["domain_148a.com".to_string(), "domain_148b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_149", SandboxPolicy {
        disabled_commands: vec!["cmd_149a".to_string(), "cmd_149b".to_string()],
        disabled_patterns: vec!["pattern_149a".to_string(), "pattern_149b".to_string()],
        read_only_paths: vec!["/path/149a".to_string(), "/path/149b".to_string()],
        blocked_domains: vec!["domain_149a.com".to_string(), "domain_149b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_150", SandboxPolicy {
        disabled_commands: vec!["cmd_150a".to_string(), "cmd_150b".to_string()],
        disabled_patterns: vec!["pattern_150a".to_string(), "pattern_150b".to_string()],
        read_only_paths: vec!["/path/150a".to_string(), "/path/150b".to_string()],
        blocked_domains: vec!["domain_150a.com".to_string(), "domain_150b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_151", SandboxPolicy {
        disabled_commands: vec!["cmd_151a".to_string(), "cmd_151b".to_string()],
        disabled_patterns: vec!["pattern_151a".to_string(), "pattern_151b".to_string()],
        read_only_paths: vec!["/path/151a".to_string(), "/path/151b".to_string()],
        blocked_domains: vec!["domain_151a.com".to_string(), "domain_151b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_152", SandboxPolicy {
        disabled_commands: vec!["cmd_152a".to_string(), "cmd_152b".to_string()],
        disabled_patterns: vec!["pattern_152a".to_string(), "pattern_152b".to_string()],
        read_only_paths: vec!["/path/152a".to_string(), "/path/152b".to_string()],
        blocked_domains: vec!["domain_152a.com".to_string(), "domain_152b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_153", SandboxPolicy {
        disabled_commands: vec!["cmd_153a".to_string(), "cmd_153b".to_string()],
        disabled_patterns: vec!["pattern_153a".to_string(), "pattern_153b".to_string()],
        read_only_paths: vec!["/path/153a".to_string(), "/path/153b".to_string()],
        blocked_domains: vec!["domain_153a.com".to_string(), "domain_153b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_154", SandboxPolicy {
        disabled_commands: vec!["cmd_154a".to_string(), "cmd_154b".to_string()],
        disabled_patterns: vec!["pattern_154a".to_string(), "pattern_154b".to_string()],
        read_only_paths: vec!["/path/154a".to_string(), "/path/154b".to_string()],
        blocked_domains: vec!["domain_154a.com".to_string(), "domain_154b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_155", SandboxPolicy {
        disabled_commands: vec!["cmd_155a".to_string(), "cmd_155b".to_string()],
        disabled_patterns: vec!["pattern_155a".to_string(), "pattern_155b".to_string()],
        read_only_paths: vec!["/path/155a".to_string(), "/path/155b".to_string()],
        blocked_domains: vec!["domain_155a.com".to_string(), "domain_155b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_156", SandboxPolicy {
        disabled_commands: vec!["cmd_156a".to_string(), "cmd_156b".to_string()],
        disabled_patterns: vec!["pattern_156a".to_string(), "pattern_156b".to_string()],
        read_only_paths: vec!["/path/156a".to_string(), "/path/156b".to_string()],
        blocked_domains: vec!["domain_156a.com".to_string(), "domain_156b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_157", SandboxPolicy {
        disabled_commands: vec!["cmd_157a".to_string(), "cmd_157b".to_string()],
        disabled_patterns: vec!["pattern_157a".to_string(), "pattern_157b".to_string()],
        read_only_paths: vec!["/path/157a".to_string(), "/path/157b".to_string()],
        blocked_domains: vec!["domain_157a.com".to_string(), "domain_157b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_158", SandboxPolicy {
        disabled_commands: vec!["cmd_158a".to_string(), "cmd_158b".to_string()],
        disabled_patterns: vec!["pattern_158a".to_string(), "pattern_158b".to_string()],
        read_only_paths: vec!["/path/158a".to_string(), "/path/158b".to_string()],
        blocked_domains: vec!["domain_158a.com".to_string(), "domain_158b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_159", SandboxPolicy {
        disabled_commands: vec!["cmd_159a".to_string(), "cmd_159b".to_string()],
        disabled_patterns: vec!["pattern_159a".to_string(), "pattern_159b".to_string()],
        read_only_paths: vec!["/path/159a".to_string(), "/path/159b".to_string()],
        blocked_domains: vec!["domain_159a.com".to_string(), "domain_159b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_160", SandboxPolicy {
        disabled_commands: vec!["cmd_160a".to_string(), "cmd_160b".to_string()],
        disabled_patterns: vec!["pattern_160a".to_string(), "pattern_160b".to_string()],
        read_only_paths: vec!["/path/160a".to_string(), "/path/160b".to_string()],
        blocked_domains: vec!["domain_160a.com".to_string(), "domain_160b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_161", SandboxPolicy {
        disabled_commands: vec!["cmd_161a".to_string(), "cmd_161b".to_string()],
        disabled_patterns: vec!["pattern_161a".to_string(), "pattern_161b".to_string()],
        read_only_paths: vec!["/path/161a".to_string(), "/path/161b".to_string()],
        blocked_domains: vec!["domain_161a.com".to_string(), "domain_161b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_162", SandboxPolicy {
        disabled_commands: vec!["cmd_162a".to_string(), "cmd_162b".to_string()],
        disabled_patterns: vec!["pattern_162a".to_string(), "pattern_162b".to_string()],
        read_only_paths: vec!["/path/162a".to_string(), "/path/162b".to_string()],
        blocked_domains: vec!["domain_162a.com".to_string(), "domain_162b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_163", SandboxPolicy {
        disabled_commands: vec!["cmd_163a".to_string(), "cmd_163b".to_string()],
        disabled_patterns: vec!["pattern_163a".to_string(), "pattern_163b".to_string()],
        read_only_paths: vec!["/path/163a".to_string(), "/path/163b".to_string()],
        blocked_domains: vec!["domain_163a.com".to_string(), "domain_163b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_164", SandboxPolicy {
        disabled_commands: vec!["cmd_164a".to_string(), "cmd_164b".to_string()],
        disabled_patterns: vec!["pattern_164a".to_string(), "pattern_164b".to_string()],
        read_only_paths: vec!["/path/164a".to_string(), "/path/164b".to_string()],
        blocked_domains: vec!["domain_164a.com".to_string(), "domain_164b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_165", SandboxPolicy {
        disabled_commands: vec!["cmd_165a".to_string(), "cmd_165b".to_string()],
        disabled_patterns: vec!["pattern_165a".to_string(), "pattern_165b".to_string()],
        read_only_paths: vec!["/path/165a".to_string(), "/path/165b".to_string()],
        blocked_domains: vec!["domain_165a.com".to_string(), "domain_165b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_166", SandboxPolicy {
        disabled_commands: vec!["cmd_166a".to_string(), "cmd_166b".to_string()],
        disabled_patterns: vec!["pattern_166a".to_string(), "pattern_166b".to_string()],
        read_only_paths: vec!["/path/166a".to_string(), "/path/166b".to_string()],
        blocked_domains: vec!["domain_166a.com".to_string(), "domain_166b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_167", SandboxPolicy {
        disabled_commands: vec!["cmd_167a".to_string(), "cmd_167b".to_string()],
        disabled_patterns: vec!["pattern_167a".to_string(), "pattern_167b".to_string()],
        read_only_paths: vec!["/path/167a".to_string(), "/path/167b".to_string()],
        blocked_domains: vec!["domain_167a.com".to_string(), "domain_167b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_168", SandboxPolicy {
        disabled_commands: vec!["cmd_168a".to_string(), "cmd_168b".to_string()],
        disabled_patterns: vec!["pattern_168a".to_string(), "pattern_168b".to_string()],
        read_only_paths: vec!["/path/168a".to_string(), "/path/168b".to_string()],
        blocked_domains: vec!["domain_168a.com".to_string(), "domain_168b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_169", SandboxPolicy {
        disabled_commands: vec!["cmd_169a".to_string(), "cmd_169b".to_string()],
        disabled_patterns: vec!["pattern_169a".to_string(), "pattern_169b".to_string()],
        read_only_paths: vec!["/path/169a".to_string(), "/path/169b".to_string()],
        blocked_domains: vec!["domain_169a.com".to_string(), "domain_169b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_170", SandboxPolicy {
        disabled_commands: vec!["cmd_170a".to_string(), "cmd_170b".to_string()],
        disabled_patterns: vec!["pattern_170a".to_string(), "pattern_170b".to_string()],
        read_only_paths: vec!["/path/170a".to_string(), "/path/170b".to_string()],
        blocked_domains: vec!["domain_170a.com".to_string(), "domain_170b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_171", SandboxPolicy {
        disabled_commands: vec!["cmd_171a".to_string(), "cmd_171b".to_string()],
        disabled_patterns: vec!["pattern_171a".to_string(), "pattern_171b".to_string()],
        read_only_paths: vec!["/path/171a".to_string(), "/path/171b".to_string()],
        blocked_domains: vec!["domain_171a.com".to_string(), "domain_171b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_172", SandboxPolicy {
        disabled_commands: vec!["cmd_172a".to_string(), "cmd_172b".to_string()],
        disabled_patterns: vec!["pattern_172a".to_string(), "pattern_172b".to_string()],
        read_only_paths: vec!["/path/172a".to_string(), "/path/172b".to_string()],
        blocked_domains: vec!["domain_172a.com".to_string(), "domain_172b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_173", SandboxPolicy {
        disabled_commands: vec!["cmd_173a".to_string(), "cmd_173b".to_string()],
        disabled_patterns: vec!["pattern_173a".to_string(), "pattern_173b".to_string()],
        read_only_paths: vec!["/path/173a".to_string(), "/path/173b".to_string()],
        blocked_domains: vec!["domain_173a.com".to_string(), "domain_173b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_174", SandboxPolicy {
        disabled_commands: vec!["cmd_174a".to_string(), "cmd_174b".to_string()],
        disabled_patterns: vec!["pattern_174a".to_string(), "pattern_174b".to_string()],
        read_only_paths: vec!["/path/174a".to_string(), "/path/174b".to_string()],
        blocked_domains: vec!["domain_174a.com".to_string(), "domain_174b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_175", SandboxPolicy {
        disabled_commands: vec!["cmd_175a".to_string(), "cmd_175b".to_string()],
        disabled_patterns: vec!["pattern_175a".to_string(), "pattern_175b".to_string()],
        read_only_paths: vec!["/path/175a".to_string(), "/path/175b".to_string()],
        blocked_domains: vec!["domain_175a.com".to_string(), "domain_175b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_176", SandboxPolicy {
        disabled_commands: vec!["cmd_176a".to_string(), "cmd_176b".to_string()],
        disabled_patterns: vec!["pattern_176a".to_string(), "pattern_176b".to_string()],
        read_only_paths: vec!["/path/176a".to_string(), "/path/176b".to_string()],
        blocked_domains: vec!["domain_176a.com".to_string(), "domain_176b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_177", SandboxPolicy {
        disabled_commands: vec!["cmd_177a".to_string(), "cmd_177b".to_string()],
        disabled_patterns: vec!["pattern_177a".to_string(), "pattern_177b".to_string()],
        read_only_paths: vec!["/path/177a".to_string(), "/path/177b".to_string()],
        blocked_domains: vec!["domain_177a.com".to_string(), "domain_177b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_178", SandboxPolicy {
        disabled_commands: vec!["cmd_178a".to_string(), "cmd_178b".to_string()],
        disabled_patterns: vec!["pattern_178a".to_string(), "pattern_178b".to_string()],
        read_only_paths: vec!["/path/178a".to_string(), "/path/178b".to_string()],
        blocked_domains: vec!["domain_178a.com".to_string(), "domain_178b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_179", SandboxPolicy {
        disabled_commands: vec!["cmd_179a".to_string(), "cmd_179b".to_string()],
        disabled_patterns: vec!["pattern_179a".to_string(), "pattern_179b".to_string()],
        read_only_paths: vec!["/path/179a".to_string(), "/path/179b".to_string()],
        blocked_domains: vec!["domain_179a.com".to_string(), "domain_179b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_180", SandboxPolicy {
        disabled_commands: vec!["cmd_180a".to_string(), "cmd_180b".to_string()],
        disabled_patterns: vec!["pattern_180a".to_string(), "pattern_180b".to_string()],
        read_only_paths: vec!["/path/180a".to_string(), "/path/180b".to_string()],
        blocked_domains: vec!["domain_180a.com".to_string(), "domain_180b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_181", SandboxPolicy {
        disabled_commands: vec!["cmd_181a".to_string(), "cmd_181b".to_string()],
        disabled_patterns: vec!["pattern_181a".to_string(), "pattern_181b".to_string()],
        read_only_paths: vec!["/path/181a".to_string(), "/path/181b".to_string()],
        blocked_domains: vec!["domain_181a.com".to_string(), "domain_181b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_182", SandboxPolicy {
        disabled_commands: vec!["cmd_182a".to_string(), "cmd_182b".to_string()],
        disabled_patterns: vec!["pattern_182a".to_string(), "pattern_182b".to_string()],
        read_only_paths: vec!["/path/182a".to_string(), "/path/182b".to_string()],
        blocked_domains: vec!["domain_182a.com".to_string(), "domain_182b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_183", SandboxPolicy {
        disabled_commands: vec!["cmd_183a".to_string(), "cmd_183b".to_string()],
        disabled_patterns: vec!["pattern_183a".to_string(), "pattern_183b".to_string()],
        read_only_paths: vec!["/path/183a".to_string(), "/path/183b".to_string()],
        blocked_domains: vec!["domain_183a.com".to_string(), "domain_183b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_184", SandboxPolicy {
        disabled_commands: vec!["cmd_184a".to_string(), "cmd_184b".to_string()],
        disabled_patterns: vec!["pattern_184a".to_string(), "pattern_184b".to_string()],
        read_only_paths: vec!["/path/184a".to_string(), "/path/184b".to_string()],
        blocked_domains: vec!["domain_184a.com".to_string(), "domain_184b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_185", SandboxPolicy {
        disabled_commands: vec!["cmd_185a".to_string(), "cmd_185b".to_string()],
        disabled_patterns: vec!["pattern_185a".to_string(), "pattern_185b".to_string()],
        read_only_paths: vec!["/path/185a".to_string(), "/path/185b".to_string()],
        blocked_domains: vec!["domain_185a.com".to_string(), "domain_185b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_186", SandboxPolicy {
        disabled_commands: vec!["cmd_186a".to_string(), "cmd_186b".to_string()],
        disabled_patterns: vec!["pattern_186a".to_string(), "pattern_186b".to_string()],
        read_only_paths: vec!["/path/186a".to_string(), "/path/186b".to_string()],
        blocked_domains: vec!["domain_186a.com".to_string(), "domain_186b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_187", SandboxPolicy {
        disabled_commands: vec!["cmd_187a".to_string(), "cmd_187b".to_string()],
        disabled_patterns: vec!["pattern_187a".to_string(), "pattern_187b".to_string()],
        read_only_paths: vec!["/path/187a".to_string(), "/path/187b".to_string()],
        blocked_domains: vec!["domain_187a.com".to_string(), "domain_187b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_188", SandboxPolicy {
        disabled_commands: vec!["cmd_188a".to_string(), "cmd_188b".to_string()],
        disabled_patterns: vec!["pattern_188a".to_string(), "pattern_188b".to_string()],
        read_only_paths: vec!["/path/188a".to_string(), "/path/188b".to_string()],
        blocked_domains: vec!["domain_188a.com".to_string(), "domain_188b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_189", SandboxPolicy {
        disabled_commands: vec!["cmd_189a".to_string(), "cmd_189b".to_string()],
        disabled_patterns: vec!["pattern_189a".to_string(), "pattern_189b".to_string()],
        read_only_paths: vec!["/path/189a".to_string(), "/path/189b".to_string()],
        blocked_domains: vec!["domain_189a.com".to_string(), "domain_189b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_190", SandboxPolicy {
        disabled_commands: vec!["cmd_190a".to_string(), "cmd_190b".to_string()],
        disabled_patterns: vec!["pattern_190a".to_string(), "pattern_190b".to_string()],
        read_only_paths: vec!["/path/190a".to_string(), "/path/190b".to_string()],
        blocked_domains: vec!["domain_190a.com".to_string(), "domain_190b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_191", SandboxPolicy {
        disabled_commands: vec!["cmd_191a".to_string(), "cmd_191b".to_string()],
        disabled_patterns: vec!["pattern_191a".to_string(), "pattern_191b".to_string()],
        read_only_paths: vec!["/path/191a".to_string(), "/path/191b".to_string()],
        blocked_domains: vec!["domain_191a.com".to_string(), "domain_191b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_192", SandboxPolicy {
        disabled_commands: vec!["cmd_192a".to_string(), "cmd_192b".to_string()],
        disabled_patterns: vec!["pattern_192a".to_string(), "pattern_192b".to_string()],
        read_only_paths: vec!["/path/192a".to_string(), "/path/192b".to_string()],
        blocked_domains: vec!["domain_192a.com".to_string(), "domain_192b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_193", SandboxPolicy {
        disabled_commands: vec!["cmd_193a".to_string(), "cmd_193b".to_string()],
        disabled_patterns: vec!["pattern_193a".to_string(), "pattern_193b".to_string()],
        read_only_paths: vec!["/path/193a".to_string(), "/path/193b".to_string()],
        blocked_domains: vec!["domain_193a.com".to_string(), "domain_193b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_194", SandboxPolicy {
        disabled_commands: vec!["cmd_194a".to_string(), "cmd_194b".to_string()],
        disabled_patterns: vec!["pattern_194a".to_string(), "pattern_194b".to_string()],
        read_only_paths: vec!["/path/194a".to_string(), "/path/194b".to_string()],
        blocked_domains: vec!["domain_194a.com".to_string(), "domain_194b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_195", SandboxPolicy {
        disabled_commands: vec!["cmd_195a".to_string(), "cmd_195b".to_string()],
        disabled_patterns: vec!["pattern_195a".to_string(), "pattern_195b".to_string()],
        read_only_paths: vec!["/path/195a".to_string(), "/path/195b".to_string()],
        blocked_domains: vec!["domain_195a.com".to_string(), "domain_195b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_196", SandboxPolicy {
        disabled_commands: vec!["cmd_196a".to_string(), "cmd_196b".to_string()],
        disabled_patterns: vec!["pattern_196a".to_string(), "pattern_196b".to_string()],
        read_only_paths: vec!["/path/196a".to_string(), "/path/196b".to_string()],
        blocked_domains: vec!["domain_196a.com".to_string(), "domain_196b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_197", SandboxPolicy {
        disabled_commands: vec!["cmd_197a".to_string(), "cmd_197b".to_string()],
        disabled_patterns: vec!["pattern_197a".to_string(), "pattern_197b".to_string()],
        read_only_paths: vec!["/path/197a".to_string(), "/path/197b".to_string()],
        blocked_domains: vec!["domain_197a.com".to_string(), "domain_197b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_198", SandboxPolicy {
        disabled_commands: vec!["cmd_198a".to_string(), "cmd_198b".to_string()],
        disabled_patterns: vec!["pattern_198a".to_string(), "pattern_198b".to_string()],
        read_only_paths: vec!["/path/198a".to_string(), "/path/198b".to_string()],
        blocked_domains: vec!["domain_198a.com".to_string(), "domain_198b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_199", SandboxPolicy {
        disabled_commands: vec!["cmd_199a".to_string(), "cmd_199b".to_string()],
        disabled_patterns: vec!["pattern_199a".to_string(), "pattern_199b".to_string()],
        read_only_paths: vec!["/path/199a".to_string(), "/path/199b".to_string()],
        blocked_domains: vec!["domain_199a.com".to_string(), "domain_199b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map.insert("policy_200", SandboxPolicy {
        disabled_commands: vec!["cmd_200a".to_string(), "cmd_200b".to_string()],
        disabled_patterns: vec!["pattern_200a".to_string(), "pattern_200b".to_string()],
        read_only_paths: vec!["/path/200a".to_string(), "/path/200b".to_string()],
        blocked_domains: vec!["domain_200a.com".to_string(), "domain_200b.com".to_string()],
        dangerously_disable_sandbox: false,
    });
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ui_e2e_mock_1() {
        let map = get_default_policies();
        assert!(map.contains_key("policy_1"));
    }
    #[test]
    fn test_ui_e2e_mock_2() {
        let map = get_default_policies();
        assert!(map.contains_key("policy_2"));
    }
    #[test]
    fn test_ui_e2e_mock_3() {
        let map = get_default_policies();
        assert!(map.contains_key("policy_3"));
    }
    #[test]
    fn test_ui_e2e_mock_4() {
        let map = get_default_policies();
        assert!(map.contains_key("policy_4"));
    }
    #[test]
    fn test_ui_e2e_mock_5() {
        let map = get_default_policies();
        assert!(map.contains_key("policy_5"));
    }
}
