import sys
import re

def modify(file_path):
    with open(file_path, "r") as f:
        content = f.read()

    # Apply changes to src/server/orchestration/state/standalone.rs

    # 1. Add dead letter recording inside transition_state_inner
    pattern = r'(sqlx::query\(\n\s*"UPDATE swarm_tasks SET status = \$1, assigned_agent_id = \$2, updated_at = \$3 WHERE id = \$4"\n\s*\)\n\s*\.bind\(to_state\)\n\s*\.bind\(agent_id\)\n\s*\.bind\(now\)\n\s*\.bind\(task_id\)\n\s*\.execute\(&mut \*tx\)\n\s*\.await\n\s*\.map_err\(\|e\| e\.to_string\(\)\)\?;)'
    replacement = r'''\1

        if to_state == "FAILED" {
            let _ = crate::telemetry::record_mission_dead_letter(&self.db.pool, &tenant_id_db, task_id).await;
        }'''
    content = re.sub(pattern, replacement, content)

    with open(file_path, "w") as f:
        f.write(content)

if __name__ == "__main__":
    modify(sys.argv[1])
