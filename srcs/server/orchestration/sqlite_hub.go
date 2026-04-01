package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"time"
)

// SqliteHubRepository implements HubRepository backed by SQLite.
type SqliteHubRepository struct {
	db *sql.DB
}

// NewSqliteHubRepository creates a SQLite-backed hub repository.
func NewSqliteHubRepository(db *sql.DB) *SqliteHubRepository {
	return &SqliteHubRepository{db: db}
}

func (r *SqliteHubRepository) RegisterAgent(ctx context.Context, agent Agent) error {
	_, err := r.db.ExecContext(ctx, `
		INSERT INTO agents (id, name, role, organization_id, status, provider_type, region)
		VALUES (?, ?, ?, ?, ?, ?, ?)
		ON CONFLICT(id) DO UPDATE SET
			name=excluded.name, role=excluded.role, organization_id=excluded.organization_id,
			status=excluded.status, provider_type=excluded.provider_type, region=excluded.region`,
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
	err := r.db.QueryRowContext(ctx, `
		SELECT id, name, role, organization_id, status, provider_type, region
		FROM agents WHERE id = ?`, id).Scan(
		&a.ID, &a.Name, &a.Role, &a.OrganizationID, &status, &a.ProviderType, &a.Region,
	)
	if err != nil {
		if err == sql.ErrNoRows {
			return Agent{}, false, nil
		}
		return Agent{}, false, fmt.Errorf("sqlite: get agent: %w", err)
	}
	a.Status = Status(status)
	return a, true, nil
}

func (r *SqliteHubRepository) ListAgents(ctx context.Context) ([]Agent, error) {
	rows, err := r.db.QueryContext(ctx, `
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
	_, err := r.db.ExecContext(ctx, "UPDATE agents SET status = ? WHERE id = ?", string(status), id)
	if err != nil {
		return fmt.Errorf("sqlite: update agent status: %w", err)
	}
	return nil
}

func (r *SqliteHubRepository) RemoveAgent(ctx context.Context, id string) error {
	tx, err := r.db.BeginTx(ctx, nil)
	if err != nil {
		return fmt.Errorf("sqlite: begin remove agent: %w", err)
	}
	defer func() { _ = tx.Rollback() }()

	if _, err := tx.ExecContext(ctx, "DELETE FROM agent_inbox WHERE agent_id = ?", id); err != nil {
		return fmt.Errorf("sqlite: clear inbox: %w", err)
	}
	if _, err := tx.ExecContext(ctx, "DELETE FROM agents WHERE id = ?", id); err != nil {
		return fmt.Errorf("sqlite: delete agent: %w", err)
	}
	return tx.Commit()
}

func (r *SqliteHubRepository) PushMessage(ctx context.Context, toAgent string, msg Message) error {
	occurredAt := msg.OccurredAt.Format("2006-01-02 15:04:05")
	if occurredAt == "0001-01-01 00:00:00" {
		occurredAt = time.Now().UTC().Format("2006-01-02 15:04:05")
	}

	_, err := r.db.ExecContext(ctx, `
		INSERT INTO agent_inbox (agent_id, message_id, from_agent, to_agent, type, content, meeting_id, occurred_at)
		VALUES (?, ?, ?, ?, ?, ?, ?, ?)`,
		toAgent, msg.ID, msg.FromAgent, msg.ToAgent, msg.Type, msg.Content, msg.MeetingID, occurredAt,
	)
	if err != nil {
		return fmt.Errorf("sqlite: push message: %w", err)
	}
	return nil
}

// PopMessages atomically retrieves and removes all pending messages.
func (r *SqliteHubRepository) PopMessages(ctx context.Context, agentID string) ([]Message, error) {
	tx, err := r.db.BeginTx(ctx, nil)
	if err != nil {
		return nil, fmt.Errorf("sqlite: begin pop messages: %w", err)
	}
	defer func() { _ = tx.Rollback() }()

	rows, err := tx.QueryContext(ctx, `
		SELECT message_id, from_agent, to_agent, type, content, meeting_id, occurred_at
		FROM agent_inbox WHERE agent_id = ? ORDER BY seq`, agentID)
	if err != nil {
		return nil, fmt.Errorf("sqlite: query pop messages: %w", err)
	}

	var msgs []Message
	for rows.Next() {
		var m Message
		var tStr string
		if err := rows.Scan(&m.ID, &m.FromAgent, &m.ToAgent, &m.Type, &m.Content, &m.MeetingID, &tStr); err != nil {
			rows.Close()
			return nil, fmt.Errorf("sqlite: scan message: %w", err)
		}
		if t, err := time.Parse("2006-01-02 15:04:05", tStr); err == nil {
			m.OccurredAt = t
		}
		msgs = append(msgs, m)
	}
	rows.Close()

	if len(msgs) > 0 {
		if _, err := tx.ExecContext(ctx, "DELETE FROM agent_inbox WHERE agent_id = ?", agentID); err != nil {
			return nil, fmt.Errorf("sqlite: delete pop messages: %w", err)
		}
	}

	if err := tx.Commit(); err != nil {
		return nil, fmt.Errorf("sqlite: commit pop messages: %w", err)
	}

	return msgs, nil
}

func (r *SqliteHubRepository) PeekMessages(ctx context.Context, agentID string) ([]Message, error) {
	rows, err := r.db.QueryContext(ctx, `
		SELECT message_id, from_agent, to_agent, type, content, meeting_id, occurred_at
		FROM agent_inbox WHERE agent_id = ? ORDER BY seq`, agentID)
	if err != nil {
		return nil, fmt.Errorf("sqlite: peek messages: %w", err)
	}
	defer rows.Close()

	var msgs []Message
	for rows.Next() {
		var m Message
		var tStr string
		if err := rows.Scan(&m.ID, &m.FromAgent, &m.ToAgent, &m.Type, &m.Content, &m.MeetingID, &tStr); err != nil {
			return nil, fmt.Errorf("sqlite: scan message: %w", err)
		}
		if t, err := time.Parse("2006-01-02 15:04:05", tStr); err == nil {
			m.OccurredAt = t
		}
		msgs = append(msgs, m)
	}
	return msgs, nil
}

func (r *SqliteHubRepository) CreateMeeting(ctx context.Context, room MeetingRoom) error {
	pBytes, _ := json.Marshal(room.Participants)

	_, err := r.db.ExecContext(ctx, `
		INSERT INTO meeting_rooms (id, agenda, participants)
		VALUES (?, ?, ?)
		ON CONFLICT(id) DO UPDATE SET agenda=excluded.agenda, participants=excluded.participants`,
		room.ID, room.Agenda, string(pBytes),
	)
	if err != nil {
		return fmt.Errorf("sqlite: create meeting: %w", err)
	}
	return nil
}

func (r *SqliteHubRepository) GetMeeting(ctx context.Context, id string) (MeetingRoom, bool, error) {
	var room MeetingRoom
	var pStr string
	err := r.db.QueryRowContext(ctx, "SELECT id, agenda, participants FROM meeting_rooms WHERE id = ?", id).Scan(
		&room.ID, &room.Agenda, &pStr,
	)
	if err != nil {
		if err == sql.ErrNoRows {
			return MeetingRoom{}, false, nil
		}
		return MeetingRoom{}, false, fmt.Errorf("sqlite: get meeting: %w", err)
	}

	_ = json.Unmarshal([]byte(pStr), &room.Participants)

	// Load transcript.
	rows, err := r.db.QueryContext(ctx, `
		SELECT message_id, from_agent, to_agent, type, content, occurred_at
		FROM meeting_transcripts WHERE meeting_id = ? ORDER BY seq`, id)
	if err != nil {
		return MeetingRoom{}, false, fmt.Errorf("sqlite: get transcript: %w", err)
	}
	defer rows.Close()

	for rows.Next() {
		var m Message
		var tStr string
		if err := rows.Scan(&m.ID, &m.FromAgent, &m.ToAgent, &m.Type, &m.Content, &tStr); err != nil {
			return MeetingRoom{}, false, fmt.Errorf("sqlite: scan transcript: %w", err)
		}
		if t, err := time.Parse("2006-01-02 15:04:05", tStr); err == nil {
			m.OccurredAt = t
		}
		m.MeetingID = id
		room.Transcript = append(room.Transcript, m)
	}
	return room, true, nil
}

func (r *SqliteHubRepository) AppendTranscript(ctx context.Context, meetingID string, msg Message) error {
	occurredAt := msg.OccurredAt.Format("2006-01-02 15:04:05")
	if occurredAt == "0001-01-01 00:00:00" {
		occurredAt = time.Now().UTC().Format("2006-01-02 15:04:05")
	}

	_, err := r.db.ExecContext(ctx, `
		INSERT INTO meeting_transcripts (meeting_id, message_id, from_agent, to_agent, type, content, occurred_at)
		VALUES (?, ?, ?, ?, ?, ?, ?)`,
		meetingID, msg.ID, msg.FromAgent, msg.ToAgent, msg.Type, msg.Content, occurredAt,
	)
	if err != nil {
		return fmt.Errorf("sqlite: append transcript: %w", err)
	}
	return nil
}

func (r *SqliteHubRepository) ListMeetings(ctx context.Context) ([]MeetingRoom, error) {
	rows, err := r.db.QueryContext(ctx, "SELECT id, agenda, participants FROM meeting_rooms ORDER BY id")
	if err != nil {
		return nil, fmt.Errorf("sqlite: list meetings: %w", err)
	}
	defer rows.Close()

	var rooms []MeetingRoom
	for rows.Next() {
		var room MeetingRoom
		var pStr string
		if err := rows.Scan(&room.ID, &room.Agenda, &pStr); err != nil {
			return nil, fmt.Errorf("sqlite: scan meeting: %w", err)
		}
		_ = json.Unmarshal([]byte(pStr), &room.Participants)
		rooms = append(rooms, room)
	}
	return rooms, nil
}
