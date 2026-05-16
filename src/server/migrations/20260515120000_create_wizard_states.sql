
CREATE TABLE IF NOT EXISTS wizard_states (
    session_id TEXT PRIMARY KEY,
    state_json TEXT NOT NULL
);
