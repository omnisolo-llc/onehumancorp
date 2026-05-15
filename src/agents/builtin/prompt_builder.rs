use crate::agent::AgentRunConfig;
use crate::tools::Tool;

pub(crate) fn build_hierarchical_system_prompt(cfg: &AgentRunConfig, tools: &[crate::tools::Tool]) -> String {
    let mut end_idx = 32768;
    if cfg.user_instructions.len() > 32768 {
        while end_idx > 0 && !cfg.user_instructions.is_char_boundary(end_idx) {
            end_idx -= 1;
        }
    } else {
        end_idx = cfg.user_instructions.len();
    }
    let user_instr = &cfg.user_instructions[..end_idx];

    let mut combined_system = String::new();

    // 1. Server-controlled System Message (Highest Priority)
    if !cfg.server_system_message.is_empty() {
        combined_system.push_str("[Server System Message]\n");
        combined_system.push_str(&cfg.server_system_message);
    }

    // 2. Tool Definitions
    if !tools.is_empty() {
        if !combined_system.is_empty() {
            combined_system.push_str("\n\n");
        }
        combined_system.push_str("[Tool Definitions]\n");
        for tool in tools {
            combined_system.push_str(&format!("Tool: {}\n", tool.name));
            combined_system.push_str(&format!("Description: {}\n", tool.description));
            combined_system.push_str(&format!("Parameters: {}\n", tool.parameters));
        }
        // Remove trailing newline
        combined_system.pop();
    }

    // 3. Developer Instructions
    if !cfg.developer_instructions.is_empty() {
        if !combined_system.is_empty() {
            combined_system.push_str("\n\n");
        }
        combined_system.push_str("[Developer Instructions]\n");
        combined_system.push_str(&cfg.developer_instructions);
    }

    // 4. User Instructions
    if !user_instr.is_empty() {
        if !combined_system.is_empty() {
            combined_system.push_str("\n\n");
        }
        combined_system.push_str("[User Instructions]\n");
        combined_system.push_str(user_instr);
    }

    combined_system
}
