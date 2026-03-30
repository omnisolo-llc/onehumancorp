import sqlite3
import os
import sys

def update_db():
    db_path = ".agents-tasks/swarm.db"
    if not os.path.exists(db_path):
        print(f"Warning: DB {db_path} not found. Skipping DB updates to prevent polluting git history with binary files.")
        return True

    try:
        conn = sqlite3.connect(db_path)
        cursor = conn.cursor()

        timestamp = os.popen('date +%s').read().strip()

        cursor.execute("INSERT INTO swarm_memory (id, type, content) VALUES (?, 'capability_manifest', 'Blueprint updated with a modular, plugin-based capability system. Transitioned from static Skill Blueprints to a Capability Plugin Mesh.');", (f'arch_roadmap_mesh_{timestamp}',))
        cursor.execute("INSERT INTO swarm_memory (id, type, content) VALUES (?, 'plugin_state', 'Next-generation OHC design system tokens: backdrop-filter: blur(15px) saturate(180%); background: rgba(255, 255, 255, 0.05); border: 1px solid rgba(255, 255, 255, 0.1); font-family: Outfit, Inter, sans-serif.');", (f'design_tokens_mesh_{timestamp}',))
        cursor.execute("INSERT INTO agent_missions (id, status, payload) VALUES (?, 'pending', '{\"title\": \"Apply Design Tokens\", \"description\": \"Update the OHC Next.js dashboard with Glassmorphism tokens.\", \"target\": \"ui_dev\"}');", (f'mission_ui_mesh_{timestamp}',))
        cursor.execute("INSERT INTO agent_missions (id, status, payload) VALUES (?, 'pending', '{\"title\": \"Visual Prototyping\", \"description\": \"Generate high-fidelity mockups of the new Capability Dashboard and plugin mesh integration.\", \"target\": \"visualizer\"}');", (f'mission_visual_mesh_{timestamp}',))
        cursor.execute("INSERT INTO agent_status (id, status, memory_type) VALUES (?, 'active', 'architecture_update');", (f'heartbeat_mesh_{timestamp}',))

        conn.commit()
        conn.close()
        print("Successfully updated swarm.db")
        return True
    except Exception as e:
        print(f"Error updating DB: {e}")
        return False

if __name__ == "__main__":
    if not update_db():
        sys.exit(1)
