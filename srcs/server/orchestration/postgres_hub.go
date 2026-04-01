package orchestration

import (
	"context"
	"encoding/json"
	"fmt"

	"github.com/onehumancorp/mono/srcs/server/db"
)

// PgHubRepository implements HubRepository backed by PostgreSQL and SQLite.
type PgHubRepository struct {
	pool db.Provider
}

// NewPgHubRepository creates a Database-backed hub repository.
func NewPgHubRepository(pool db.Provider) *PgHubRepository {
	return &PgHubRepository{pool: pool}
}

func (r *PgHubRepository) RegisterAgent(ctx context.Context, agent Agent) error {
	ctx, span := db.Tracer().Start(ctx, "PgHubRepository.RegisterAgent")
	defer span.End()

	_, err := r.pool.Exec(ctx, `
		INSERT INTO agents (id, name, role, organization_id, status, provider_type, region)
		VALUES ($1, $2, $3, $4, $5, $6, $7)
		ON CONFLICT (id) DO UPDATE SET
			name=EXCLUDED.name, role=EXCLUDED.role, organization_id=EXCLUDED.organization_id,
			status=EXCLUDED.status, provider_type=EXCLUDED.provider_type, region=EXCLUDED.region`,
		agent.ID, agent.Name, agent.Role, agent.OrganizationID,
		string(agent.Status), agent.ProviderType, agent.Region,
	)
	if err != nil {
		return fmt.Errorf("db: register agent: %w", err)
	}
	return nil
}

func (r *PgHubRepository) GetAgent(ctx context.Context, id string) (Agent, bool, error) {
	ctx, span := db.Tracer().Start(ctx, "PgHubRepository.GetAgent")
	defer span.End()

	var a Agent
	var status string
	err := r.pool.QueryRow(ctx, `
		SELECT id, name, role, organization_id, status, provider_type, region
		FROM agents WHERE id = $1`, id).Scan(
		&a.ID, &a.Name, &a.Role, &a.OrganizationID, &status, &a.ProviderType, &a.Region,
	)
	if err != nil {
		if err.Error() == "no rows in result set" || err.Error() == "sql: no rows in result set" {
			return Agent{}, false, nil
		}
		return Agent{}, false, fmt.Errorf("db: get agent: %w", err)
	}
	a.Status = Status(status)
	return a, true, nil
}

func (r *PgHubRepository) ListAgents(ctx context.Context) ([]Agent, error) {
	ctx, span := db.Tracer().Start(ctx, "PgHubRepository.ListAgents")
	defer span.End()

	rows, err := r.pool.Query(ctx, `
		SELECT id, name, role, organization_id, status, provider_type, region
		FROM agents ORDER BY id`)
	if err != nil {
		return nil, fmt.Errorf("db: list agents: %w", err)
	}
	defer rows.Close()

	var agents []Agent
	for rows.Next() {
		var a Agent
		var status string
		if err := rows.Scan(&a.ID, &a.Name, &a.Role, &a.OrganizationID, &status, &a.ProviderType, &a.Region); err != nil {
			return nil, fmt.Errorf("db: scan agent: %w", err)
		}
		a.Status = Status(status)
		agents = append(agents, a)
	}
	return agents, nil
}

func (r *PgHubRepository) UpdateAgentStatus(ctx context.Context, id string, status Status) error {
	ctx, span := db.Tracer().Start(ctx, "PgHubRepository.UpdateAgentStatus")
	defer span.End()

	_, err := r.pool.Exec(ctx, "UPDATE agents SET status = $2 WHERE id = $1", id, string(status))
	if err != nil {
		return fmt.Errorf("db: update agent status: %w", err)
	}
	return nil
}

func (r *PgHubRepository) RemoveAgent(ctx context.Context, id string) error {
	ctx, span := db.Tracer().Start(ctx, "PgHubRepository.RemoveAgent")
	defer span.End()

	tx, err := r.pool.Begin(ctx)
	if err != nil {
		return fmt.Errorf("db: begin remove agent: %w", err)
	}
	defer func() { _ = tx.Rollback(ctx) }()

	if _, err := tx.Exec(ctx, "DELETE FROM agent_inbox WHERE agent_id = $1", id); err != nil {
		return fmt.Errorf("db: clear inbox: %w", err)
	}
	if _, err := tx.Exec(ctx, "DELETE FROM agents WHERE id = $1", id); err != nil {
		return fmt.Errorf("db: delete agent: %w", err)
	}
	return tx.Commit(ctx)
}

func (r *PgHubRepository) PushMessage(ctx context.Context, toAgent string, msg Message) error {
	ctx, span := db.Tracer().Start(ctx, "PgHubRepository.PushMessage")
	defer span.End()

	_, err := r.pool.Exec(ctx, `
		INSERT INTO agent_inbox (agent_id, message_id, from_agent, to_agent, type, content, meeting_id, occurred_at)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8)`,
		toAgent, msg.ID, msg.FromAgent, msg.ToAgent, msg.Type, msg.Content, msg.MeetingID, msg.OccurredAt,
	)
	if err != nil {
		return fmt.Errorf("db: push message: %w", err)
	}
	return nil
}

// PopMessages atomically retrieves and removes all pending messages.
// Uses DELETE ... RETURNING for consume-once semantics on Postgres,
// but falls back to a transaction for SQLite.
func (r *PgHubRepository) PopMessages(ctx context.Context, agentID string) ([]Message, error) {
	ctx, span := db.Tracer().Start(ctx, "PgHubRepository.PopMessages")
	defer span.End()

	_, isSqlite := r.pool.(*db.SqliteProvider)
	if !isSqlite {
		rows, err := r.pool.Query(ctx, `
			DELETE FROM agent_inbox
			WHERE agent_id = $1
			RETURNING message_id, from_agent, to_agent, type, content, meeting_id, occurred_at`, agentID)
		if err != nil {
			return nil, fmt.Errorf("db: pop messages: %w", err)
		}
		defer rows.Close()

		var msgs []Message
		for rows.Next() {
			var m Message
			if err := rows.Scan(&m.ID, &m.FromAgent, &m.ToAgent, &m.Type, &m.Content, &m.MeetingID, &m.OccurredAt); err != nil {
				return nil, fmt.Errorf("db: scan popped message: %w", err)
			}
			msgs = append(msgs, m)
		}
		return msgs, nil
	}

	// Fallback for sqlite since it doesn't support DELETE ... RETURNING well with older drivers
	// we will do a transaction
	tx, err := r.pool.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("db: begin pop: %w", err)
	}
	defer func() { _ = tx.Rollback(ctx) }()

	rows, err := tx.Query(ctx, `
		SELECT message_id, from_agent, to_agent, type, content, meeting_id, occurred_at
		FROM agent_inbox WHERE agent_id = $1 ORDER BY seq`, agentID)
	if err != nil {
		return nil, fmt.Errorf("db: peek messages for pop: %w", err)
	}

	var msgs []Message
	for rows.Next() {
		var m Message
		if err := rows.Scan(&m.ID, &m.FromAgent, &m.ToAgent, &m.Type, &m.Content, &m.MeetingID, &m.OccurredAt); err != nil {
			rows.Close()
			return nil, fmt.Errorf("db: scan message: %w", err)
		}
		msgs = append(msgs, m)
	}
	rows.Close()

	if len(msgs) > 0 {
		_, err = tx.Exec(ctx, "DELETE FROM agent_inbox WHERE agent_id = $1", agentID)
		if err != nil {
			return nil, fmt.Errorf("db: delete popped messages: %w", err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, fmt.Errorf("db: commit pop: %w", err)
	}

	return msgs, nil
}

func (r *PgHubRepository) PeekMessages(ctx context.Context, agentID string) ([]Message, error) {
	ctx, span := db.Tracer().Start(ctx, "PgHubRepository.PeekMessages")
	defer span.End()

	rows, err := r.pool.Query(ctx, `
		SELECT message_id, from_agent, to_agent, type, content, meeting_id, occurred_at
		FROM agent_inbox WHERE agent_id = $1 ORDER BY seq`, agentID)
	if err != nil {
		return nil, fmt.Errorf("db: peek messages: %w", err)
	}
	defer rows.Close()

	var msgs []Message
	for rows.Next() {
		var m Message
		if err := rows.Scan(&m.ID, &m.FromAgent, &m.ToAgent, &m.Type, &m.Content, &m.MeetingID, &m.OccurredAt); err != nil {
			return nil, fmt.Errorf("db: scan message: %w", err)
		}
		msgs = append(msgs, m)
	}
	return msgs, nil
}

func (r *PgHubRepository) CreateMeeting(ctx context.Context, room MeetingRoom) error {
	ctx, span := db.Tracer().Start(ctx, "PgHubRepository.CreateMeeting")
	defer span.End()

	participantsJSON, _ := json.Marshal(room.Participants)
	_, err := r.pool.Exec(ctx, `
		INSERT INTO meeting_rooms (id, agenda, participants)
		VALUES ($1, $2, $3)
		ON CONFLICT (id) DO UPDATE SET agenda=EXCLUDED.agenda, participants=EXCLUDED.participants`,
		room.ID, room.Agenda, string(participantsJSON),
	)
	if err != nil {
		return fmt.Errorf("db: create meeting: %w", err)
	}
	return nil
}

func (r *PgHubRepository) GetMeeting(ctx context.Context, id string) (MeetingRoom, bool, error) {
	ctx, span := db.Tracer().Start(ctx, "PgHubRepository.GetMeeting")
	defer span.End()

	var room MeetingRoom
	var participantsJSON string
	err := r.pool.QueryRow(ctx, "SELECT id, agenda, participants FROM meeting_rooms WHERE id = $1", id).Scan(
		&room.ID, &room.Agenda, &participantsJSON,
	)
	if err != nil {
		if err.Error() == "no rows in result set" || err.Error() == "sql: no rows in result set" {
			return MeetingRoom{}, false, nil
		}
		return MeetingRoom{}, false, fmt.Errorf("db: get meeting: %w", err)
	}
	_ = json.Unmarshal([]byte(participantsJSON), &room.Participants)

	// Load transcript.
	rows, err := r.pool.Query(ctx, `
		SELECT message_id, from_agent, to_agent, type, content, occurred_at
		FROM meeting_transcripts WHERE meeting_id = $1 ORDER BY seq`, id)
	if err != nil {
		return MeetingRoom{}, false, fmt.Errorf("db: get transcript: %w", err)
	}
	defer rows.Close()

	for rows.Next() {
		var m Message
		if err := rows.Scan(&m.ID, &m.FromAgent, &m.ToAgent, &m.Type, &m.Content, &m.OccurredAt); err != nil {
			return MeetingRoom{}, false, fmt.Errorf("db: scan transcript: %w", err)
		}
		m.MeetingID = id
		room.Transcript = append(room.Transcript, m)
	}
	return room, true, nil
}

func (r *PgHubRepository) AppendTranscript(ctx context.Context, meetingID string, msg Message) error {
	ctx, span := db.Tracer().Start(ctx, "PgHubRepository.AppendTranscript")
	defer span.End()

	_, err := r.pool.Exec(ctx, `
		INSERT INTO meeting_transcripts (meeting_id, message_id, from_agent, to_agent, type, content, occurred_at)
		VALUES ($1, $2, $3, $4, $5, $6, $7)`,
		meetingID, msg.ID, msg.FromAgent, msg.ToAgent, msg.Type, msg.Content, msg.OccurredAt,
	)
	if err != nil {
		return fmt.Errorf("db: append transcript: %w", err)
	}
	return nil
}

func (r *PgHubRepository) ListMeetings(ctx context.Context) ([]MeetingRoom, error) {
	ctx, span := db.Tracer().Start(ctx, "PgHubRepository.ListMeetings")
	defer span.End()

	rows, err := r.pool.Query(ctx, "SELECT id, agenda, participants FROM meeting_rooms ORDER BY id")
	if err != nil {
		return nil, fmt.Errorf("db: list meetings: %w", err)
	}
	defer rows.Close()

	var rooms []MeetingRoom
	for rows.Next() {
		var room MeetingRoom
		var participantsJSON string
		if err := rows.Scan(&room.ID, &room.Agenda, &participantsJSON); err != nil {
			return nil, fmt.Errorf("db: scan meeting: %w", err)
		}
		_ = json.Unmarshal([]byte(participantsJSON), &room.Participants)
		rooms = append(rooms, room)
	}
	return rooms, nil
}
