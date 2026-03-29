import sqlite3
import datetime
import os
import json

db_path = os.path.expanduser('~/.openclaw/ohc.db')
os.makedirs(os.path.dirname(db_path), exist_ok=True)
conn = sqlite3.connect(db_path)
c = conn.cursor()

c.execute('''
    CREATE TABLE IF NOT EXISTS agent_missions (
        id TEXT PRIMARY KEY,
        role TEXT,
        task TEXT,
        status TEXT,
        assigned_to TEXT,
        created_at DATETIME,
        updated_at DATETIME
    )
''')

c.execute('''
    CREATE TABLE IF NOT EXISTS agent_status (
        agent_id TEXT PRIMARY KEY,
        role TEXT,
        status TEXT,
        last_heartbeat DATETIME
    )
''')

now = datetime.datetime.now(datetime.UTC).isoformat()

# Task format should be JSON string representing a domain.Message
task_data = json.dumps({
    "role": "system",
    "content": "Mission: Performance Tuning for 10x Velocity"
})

c.execute("INSERT INTO agent_missions (id, role, task, status, assigned_to, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?)",
          ("mission_performance_1", "performance_velocity", task_data, "pending", "performance_velocity", now, now))

c.execute("INSERT INTO agent_status (agent_id, role, status, last_heartbeat) VALUES (?, ?, ?, ?)",
          ("sre_infra_1", "sre_infra", "active", now))

conn.commit()
conn.close()
