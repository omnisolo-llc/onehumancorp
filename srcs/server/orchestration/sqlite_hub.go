package orchestration

import (
	"context"
	"encoding/json"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

// SqliteHubRepository implements HubRepository backed by SQLite.
type SqliteHubRepository struct {
	pool db.Provider
}

// NewSqliteHubRepository creates a SQLite-backed hub repository.
func NewSqliteHubRepository(pool db.Provider) *SqliteHubRepository {
	return &SqliteHubRepository{pool: pool}
}

func (r *SqliteHubRepository) RegisterAgent(ctx context.Context, agent Agent) error {
	_, err := r.pool.Exec(ctx, `
		INSERT INTO agents (id, name, role, organization_id, status, provider_type, region)
		VALUES (?, ?, ?, ?, ?, ?, ?)
		ON CONFLICT (id) DO UPDATE SET
			name=EXCLUDED.name, role=EXCLUDED.role, organization_id=EXCLUDED.organization_id,
			status=EXCLUDED.status, provider_type=EXCLUDED.provider_type, region=EXCLUDED.region`,
		agent.ID, agent.Name, agent.Role, agent.OrganizationID,
		string(agent.Status), agent.ProviderType, agent.Region,
	)
	if err != nil {
		return fmt.Errorf("sqlite: register agent: %w", err)
	}
	return nil
}

func (r *SqliteHubRepository) GetAgent(ctx context.Context, id string) (Agent, bool, error) {
	var a Agent
	var status string
	err := r.pool.QueryRow(ctx, `
		SELECT id, name, role, organization_id, status, provider_type, region
		FROM agents WHERE id = ?`, id).Scan(
		&a.ID, &a.Name, &a.Role, &a.OrganizationID, &status, &a.ProviderType, &a.Region,
	)
	if err != nil {
		if err.Error() == "sql: no rows in result set" {
			return Agent{}, false, nil
		}
		return Agent{}, false, fmt.Errorf("sqlite: get agent: %w", err)
	}
	a.Status = Status(status)
	return a, true, nil
}

func (r *SqliteHubRepository) ListAgents(ctx context.Context) ([]Agent, error) {
	rows, err := r.pool.Query(ctx, `
		SELECT id, name, role, organization_id, status, provider_type, region
		FROM agents ORDER BY id`)
	if err != nil {
		return nil, fmt.Errorf("sqlite: list agents: %w", err)
	}
	defer rows.Close()

	var agents []Agent
	for rows.Next() {
		var a Agent
		var status string
		if err := rows.Scan(&a.ID, &a.Name, &a.Role, &a.OrganizationID, &status, &a.ProviderType, &a.Region); err != nil {
			return nil, fmt.Errorf("sqlite: scan agent: %w", err)
		}
		a.Status = Status(status)
		agents = append(agents, a)
	}
	return agents, nil
}

func (r *SqliteHubRepository) UpdateAgentStatus(ctx context.Context, id string, status Status) error {
	_, err := r.pool.Exec(ctx, "UPDATE agents SET status = ? WHERE id = ?", string(status), id)
	if err != nil {
		return fmt.Errorf("sqlite: update agent status: %w", err)
	}
	return nil
}

func (r *SqliteHubRepository) RemoveAgent(ctx context.Context, id string) error {
	tx, err := r.pool.Begin(ctx)
	if err != nil {
		return fmt.Errorf("sqlite: begin remove agent: %w", err)
	}
	defer func() { _ = tx.Rollback(ctx) }()

	if _, err := tx.Exec(ctx, "DELETE FROM agent_inbox WHERE agent_id = ?", id); err != nil {
		return fmt.Errorf("sqlite: clear inbox: %w", err)
	}
	if _, err := tx.Exec(ctx, "DELETE FROM agents WHERE id = ?", id); err != nil {
		return fmt.Errorf("sqlite: delete agent: %w", err)
	}
	return tx.Commit(ctx)
}

func (r *SqliteHubRepository) PushMessage(ctx context.Context, toAgent string, msg Message) error {
	_, err := r.pool.Exec(ctx, `
		INSERT INTO agent_inbox (agent_id, message_id, from_agent, to_agent, type, content, meeting_id, occurred_at)
		VALUES (?, ?, ?, ?, ?, ?, ?, ?)`,
		toAgent, msg.ID, msg.FromAgent, msg.ToAgent, msg.Type, msg.Content, msg.MeetingID, msg.OccurredAt,
	)
	if err != nil {
		return fmt.Errorf("sqlite: push message: %w", err)
	}
	return nil
}

// PopMessages atomically retrieves and removes all pending messages.
func (r *SqliteHubRepository) PopMessages(ctx context.Context, agentID string) ([]Message, error) {
	tx, err := r.pool.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("sqlite: begin pop: %w", err)
	}
	defer func() { _ = tx.Rollback(ctx) }()

	rows, err := tx.Query(ctx, `
		SELECT message_id, from_agent, to_agent, type, content, meeting_id, occurred_at
		FROM agent_inbox WHERE agent_id = ? ORDER BY seq`, agentID)
	if err != nil {
		return nil, fmt.Errorf("sqlite: peek messages for pop: %w", err)
	}

	var msgs []Message
	for rows.Next() {
		var m Message
		if err := rows.Scan(&m.ID, &m.FromAgent, &m.ToAgent, &m.Type, &m.Content, &m.MeetingID, &m.OccurredAt); err != nil {
			rows.Close()
			return nil, fmt.Errorf("sqlite: scan message: %w", err)
		}
		msgs = append(msgs, m)
	}
	rows.Close()

	if len(msgs) > 0 {
		_, err = tx.Exec(ctx, "DELETE FROM agent_inbox WHERE agent_id = ?", agentID)
		if err != nil {
			return nil, fmt.Errorf("sqlite: delete popped messages: %w", err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, fmt.Errorf("sqlite: commit pop: %w", err)
	}

	return msgs, nil
}

func (r *SqliteHubRepository) PeekMessages(ctx context.Context, agentID string) ([]Message, error) {
	rows, err := r.pool.Query(ctx, `
		SELECT message_id, from_agent, to_agent, type, content, meeting_id, occurred_at
		FROM agent_inbox WHERE agent_id = ? ORDER BY seq`, agentID)
	if err != nil {
		return nil, fmt.Errorf("sqlite: peek messages: %w", err)
	}
	defer rows.Close()

	var msgs []Message
	for rows.Next() {
		var m Message
		if err := rows.Scan(&m.ID, &m.FromAgent, &m.ToAgent, &m.Type, &m.Content, &m.MeetingID, &m.OccurredAt); err != nil {
			return nil, fmt.Errorf("sqlite: scan message: %w", err)
		}
		msgs = append(msgs, m)
	}
	return msgs, nil
}

func (r *SqliteHubRepository) CreateMeeting(ctx context.Context, room MeetingRoom) error {
	participantsJSON, _ := json.Marshal(room.Participants)
	_, err := r.pool.Exec(ctx, `
		INSERT INTO meeting_rooms (id, agenda, participants)
		VALUES (?, ?, ?)
		ON CONFLICT (id) DO UPDATE SET agenda=EXCLUDED.agenda, participants=EXCLUDED.participants`,
		room.ID, room.Agenda, string(participantsJSON),
	)
	if err != nil {
		return fmt.Errorf("sqlite: create meeting: %w", err)
	}
	return nil
}

func (r *SqliteHubRepository) GetMeeting(ctx context.Context, id string) (MeetingRoom, bool, error) {
	var room MeetingRoom
	var participantsJSON string
	err := r.pool.QueryRow(ctx, "SELECT id, agenda, participants FROM meeting_rooms WHERE id = ?", id).Scan(
		&room.ID, &room.Agenda, &participantsJSON,
	)
	if err != nil {
		if err.Error() == "sql: no rows in result set" {
			return MeetingRoom{}, false, nil
		}
		return MeetingRoom{}, false, fmt.Errorf("sqlite: get meeting: %w", err)
	}
	_ = json.Unmarshal([]byte(participantsJSON), &room.Participants)

	// Load transcript.
	rows, err := r.pool.Query(ctx, `
		SELECT message_id, from_agent, to_agent, type, content, occurred_at
		FROM meeting_transcripts WHERE meeting_id = ? ORDER BY seq`, id)
	if err != nil {
		return MeetingRoom{}, false, fmt.Errorf("sqlite: get transcript: %w", err)
	}
	defer rows.Close()

	for rows.Next() {
		var m Message
		if err := rows.Scan(&m.ID, &m.FromAgent, &m.ToAgent, &m.Type, &m.Content, &m.OccurredAt); err != nil {
			return MeetingRoom{}, false, fmt.Errorf("sqlite: scan transcript: %w", err)
		}
		m.MeetingID = id
		room.Transcript = append(room.Transcript, m)
	}
	return room, true, nil
}

func (r *SqliteHubRepository) AppendTranscript(ctx context.Context, meetingID string, msg Message) error {
	_, err := r.pool.Exec(ctx, `
		INSERT INTO meeting_transcripts (meeting_id, message_id, from_agent, to_agent, type, content, occurred_at)
		VALUES (?, ?, ?, ?, ?, ?, ?)`,
		meetingID, msg.ID, msg.FromAgent, msg.ToAgent, msg.Type, msg.Content, msg.OccurredAt,
	)
	if err != nil {
		return fmt.Errorf("sqlite: append transcript: %w", err)
	}
	return nil
}

func (r *SqliteHubRepository) ListMeetings(ctx context.Context) ([]MeetingRoom, error) {
	rows, err := r.pool.Query(ctx, "SELECT id, agenda, participants FROM meeting_rooms ORDER BY id")
	if err != nil {
		return nil, fmt.Errorf("sqlite: list meetings: %w", err)
	}
	defer rows.Close()

	var rooms []MeetingRoom
	for rows.Next() {
		var room MeetingRoom
		var participantsJSON string
		if err := rows.Scan(&room.ID, &room.Agenda, &participantsJSON); err != nil {
			return nil, fmt.Errorf("sqlite: scan meeting: %w", err)
		}
		_ = json.Unmarshal([]byte(participantsJSON), &room.Participants)
		rooms = append(rooms, room)
	}
	return rooms, nil
}

func (r *SqliteHubRepository) ClaimTask(ctx context.Context, taskID, agentID string) (bool, error) {
	tx, err := r.pool.Begin(ctx)
	if err != nil {
		return false, fmt.Errorf("sqlite: begin claim task: %w", err)
	}
	defer func() { _ = tx.Rollback(ctx) }()

	var status string
	var lockedUntil *time.Time
	err = tx.QueryRow(ctx, "SELECT status, locked_until FROM swarm_tasks WHERE id = ? LIMIT 1", taskID).Scan(&status, &lockedUntil)
	if err != nil {
		if err.Error() == "sql: no rows in result set" {
			return false, nil
		}
		return false, fmt.Errorf("sqlite: query task: %w", err)
	}

	if status != "PENDING" && status != "FAILED" && (lockedUntil == nil || lockedUntil.After(time.Now())) {
		return false, nil
	}

	newLock := time.Now().Add(30 * time.Second)
	_, err = tx.Exec(ctx, "UPDATE swarm_tasks SET status = 'IN_PROGRESS', assigned_agent_id = ?, locked_until = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?", agentID, newLock, taskID)
	if err != nil {
		return false, fmt.Errorf("sqlite: update task: %w", err)
	}

	if err := tx.Commit(ctx); err != nil {
		return false, fmt.Errorf("sqlite: commit task claim: %w", err)
	}

	return true, nil
}
