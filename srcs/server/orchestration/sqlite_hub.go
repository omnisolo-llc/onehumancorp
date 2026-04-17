package orchestration

import (
	"context"
	"encoding/json"
	"fmt"

	"github.com/onehumancorp/mono/srcs/server/db"
)

// SqliteHubRepository implements HubRepository backed by SQLite.
type SqliteHubRepository struct {
	pool  db.Provider
	orgID string
}

// NewSqliteHubRepository creates a SQLite-backed hub repository.
func NewSqliteHubRepository(pool db.Provider, orgID string) *SqliteHubRepository {
	return &SqliteHubRepository{pool: pool, orgID: orgID}
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
	query := "SELECT id, name, role, organization_id, status, provider_type, region FROM agents WHERE id = ?"
	var args []any = []any{id}
	if r.orgID != "" {
		query += " AND organization_id = ?"
		args = append(args, r.orgID)
	}
	err := r.pool.QueryRow(ctx, query, args...).Scan(
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
	query := "SELECT id, name, role, organization_id, status, provider_type, region FROM agents"
	var args []any
	if r.orgID != "" {
		query += " WHERE organization_id = ?"
		args = append(args, r.orgID)
	}
	query += " ORDER BY id"

	rows, err := r.pool.Query(ctx, query, args...)
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
	query := "UPDATE agents SET status = ? WHERE id = ?"
	var args []any = []any{string(status), id}
	if r.orgID != "" {
		query += " AND organization_id = ?"
		args = append(args, r.orgID)
	}
	_, err := r.pool.Exec(ctx, query, args...)
	if err != nil {
		return fmt.Errorf("sqlite: update agent status: %w", err)
	}
	return nil
}

func (r *SqliteHubRepository) RemoveAgent(ctx context.Context, id string) error {
	// Prevent unauthorized deletion if scoped
	if r.orgID != "" {
		var count int
		err := r.pool.QueryRow(ctx, "SELECT COUNT(*) FROM agents WHERE id = ? AND organization_id = ?", id, r.orgID).Scan(&count)
		if err != nil || count == 0 {
			return fmt.Errorf("sqlite: unauthorized or missing agent")
		}
	}

	tx, err := r.pool.Begin(ctx)
	if err != nil {
		return fmt.Errorf("sqlite: begin remove agent: %w", err)
	}
	defer func() { _ = tx.Rollback(ctx) }()

	clearQuery := "DELETE FROM agent_inbox WHERE agent_id = ?"
	delQuery := "DELETE FROM agents WHERE id = ?"
	var args []any = []any{id}

	if r.orgID != "" {
		clearQuery += " AND organization_id = ?"
		delQuery += " AND organization_id = ?"
		args = append(args, r.orgID)
	}

	if _, err := tx.Exec(ctx, clearQuery, args...); err != nil {
		return fmt.Errorf("sqlite: clear inbox: %w", err)
	}
	if _, err := tx.Exec(ctx, delQuery, args...); err != nil {
		return fmt.Errorf("sqlite: delete agent: %w", err)
	}
	return tx.Commit(ctx)
}

func (r *SqliteHubRepository) AppendEvent(ctx context.Context, event HubEvent) error {
	payload := json.RawMessage(event.Payload)
	if len(payload) == 0 {
		payload = json.RawMessage("{}")
	}

	_, err := r.pool.Exec(ctx, `
		INSERT INTO hub_events (type, payload, occurred_at)
		VALUES (?, ?, ?)
	`, event.Type, string(payload), event.OccurredAt.UTC())
	if err != nil {
		return fmt.Errorf("sqlite: append event: %w", err)
	}
	return nil
}

func (r *SqliteHubRepository) PushMessage(ctx context.Context, toAgent string, msg Message) error {
	if r.orgID != "" {
		var count int
		err := r.pool.QueryRow(ctx, "SELECT COUNT(*) FROM agents WHERE id = ? AND organization_id = ?", toAgent, r.orgID).Scan(&count)
		if err != nil || count == 0 {
			return fmt.Errorf("sqlite: unauthorized or missing agent for push")
		}
	}

	_, err := r.pool.Exec(ctx, `
		INSERT INTO agent_inbox (agent_id, message_id, from_agent, to_agent, type, content, meeting_id, occurred_at, organization_id)
		VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)`,
		toAgent, msg.ID, msg.FromAgent, msg.ToAgent, msg.Type, msg.Content, msg.MeetingID, msg.OccurredAt, r.orgID,
	)
	if err != nil {
		return fmt.Errorf("sqlite: push message: %w", err)
	}
	return nil
}

// PopMessages atomically retrieves and removes all pending messages.
func (r *SqliteHubRepository) PopMessages(ctx context.Context, agentID string) ([]Message, error) {
	if r.orgID != "" {
		var count int
		err := r.pool.QueryRow(ctx, "SELECT COUNT(*) FROM agents WHERE id = ? AND organization_id = ?", agentID, r.orgID).Scan(&count)
		if err != nil || count == 0 {
			return nil, fmt.Errorf("sqlite: unauthorized or missing agent for pop")
		}
	}

	tx, err := r.pool.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("sqlite: begin pop: %w", err)
	}
	defer func() { _ = tx.Rollback(ctx) }()

	query := `SELECT message_id, from_agent, to_agent, type, content, meeting_id, occurred_at FROM agent_inbox WHERE agent_id = ?`
	args := []any{agentID}
	if r.orgID != "" {
		query += ` AND organization_id = ?`
		args = append(args, r.orgID)
	}
	query += ` ORDER BY seq`

	rows, err := tx.Query(ctx, query, args...)
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
		delQuery := "DELETE FROM agent_inbox WHERE agent_id = ?"
		delArgs := []any{agentID}
		if r.orgID != "" {
			delQuery += " AND organization_id = ?"
			delArgs = append(delArgs, r.orgID)
		}
		_, err = tx.Exec(ctx, delQuery, delArgs...)
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
	if r.orgID != "" {
		var count int
		err := r.pool.QueryRow(ctx, "SELECT COUNT(*) FROM agents WHERE id = ? AND organization_id = ?", agentID, r.orgID).Scan(&count)
		if err != nil || count == 0 {
			return nil, fmt.Errorf("sqlite: unauthorized or missing agent for peek")
		}
	}

	query := `SELECT message_id, from_agent, to_agent, type, content, meeting_id, occurred_at FROM agent_inbox WHERE agent_id = ?`
	args := []any{agentID}
	if r.orgID != "" {
		query += ` AND organization_id = ?`
		args = append(args, r.orgID)
	}
	query += ` ORDER BY seq`

	rows, err := r.pool.Query(ctx, query, args...)
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
		INSERT INTO meeting_rooms (id, agenda, participants, organization_id)
		VALUES (?, ?, ?, ?)
		ON CONFLICT (id) DO UPDATE SET agenda=EXCLUDED.agenda, participants=EXCLUDED.participants, organization_id=EXCLUDED.organization_id`,
		room.ID, room.Agenda, string(participantsJSON), r.orgID,
	)
	if err != nil {
		return fmt.Errorf("sqlite: create meeting: %w", err)
	}
	return nil
}

func (r *SqliteHubRepository) GetMeeting(ctx context.Context, id string) (MeetingRoom, bool, error) {
	var room MeetingRoom
	var participantsJSON string
	query := "SELECT id, agenda, participants FROM meeting_rooms WHERE id = ?"
	args := []any{id}
	if r.orgID != "" {
		query += " AND organization_id = ?"
		args = append(args, r.orgID)
	}
	err := r.pool.QueryRow(ctx, query, args...).Scan(
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
	if r.orgID != "" {
		var count int
		err := r.pool.QueryRow(ctx, "SELECT COUNT(*) FROM meeting_rooms WHERE id = ? AND organization_id = ?", meetingID, r.orgID).Scan(&count)
		if err != nil || count == 0 {
			return fmt.Errorf("sqlite: unauthorized or missing meeting room for transcript")
		}
	}
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
	query := "SELECT id, agenda, participants FROM meeting_rooms"
	var args []any
	if r.orgID != "" {
		query += " WHERE organization_id = ?"
		args = append(args, r.orgID)
	}
	query += " ORDER BY id"

	rows, err := r.pool.Query(ctx, query, args...)
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
