package tasks

import (
    "database/sql"
    "time"
)

type SwarmTask struct {
    ID              string         `db:"id"`
    MissionID       string         `db:"mission_id"`
    ParentPlanID    sql.NullString `db:"parent_plan_id"`
    Dependencies    string         `db:"dependencies"`
    Title           string         `db:"title"`
    Status          string         `db:"status"`
    AssignedAgentID sql.NullString `db:"assigned_agent_id"`
    Payload         sql.NullString `db:"payload"`
    LockedUntil     sql.NullTime   `db:"locked_until"`
    CreatedAt       time.Time      `db:"created_at"`
}

type StateMachineTransition struct {
    ID         string         `db:"id"`
    EntityID   string         `db:"entity_id"`
    EntityType string         `db:"entity_type"`
    FromState  string         `db:"from_state"`
    ToState    string         `db:"to_state"`
    AgentID    sql.NullString `db:"agent_id"`
    Reason     sql.NullString `db:"reason"`
    OccurredAt time.Time      `db:"occurred_at"`
}
