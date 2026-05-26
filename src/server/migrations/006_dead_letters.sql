-- Migration: 006_dead_letters.sql

CREATE TABLE IF NOT EXISTS department_dead_letters (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    event_type TEXT NOT NULL,
    department TEXT NOT NULL,
    payload TEXT NOT NULL,
    error_message TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
