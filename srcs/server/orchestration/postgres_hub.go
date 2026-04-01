package orchestration

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/db"
)

// PgHubRepository implements HubRepository backed by PostgreSQL or SQLite.
type PgHubRepository struct {
	db db.Provider
}

// NewPgHubRepository creates a db.Provider-backed hub repository.
func NewPgHubRepository(db db.Provider) *PgHubRepository {
	return &PgHubRepository{db: db}
}

func (r *PgHubRepository) RegisterAgent(ctx context.Context, agent Agent) error {
	var err error
	if r.db.IsPostgres() {
		_, err = r.db.ExecContext(ctx, `
			INSERT INTO agents (id, name, role, organization_id, status, provider_type, region)
			VALUES ($1, $2, $3, $4, $5, $6, $7)
			ON CONFLICT (id) DO UPDATE SET
				name=EXCLUDED.name, role=EXCLUDED.role, organization_id=EXCLUDED.organization_id,
				status=EXCLUDED.status, provider_type=EXCLUDED.provider_type, region=EXCLUDED.region`,
			agent.ID, agent.Name, agent.Role, agent.OrganizationID,
			string(agent.Status), agent.ProviderType, agent.Region,
		)
	} else {
		_, err = r.db.ExecContext(ctx, `
			INSERT INTO agents (id, name, role, organization_id, status, provider_type, region)
			VALUES (?, ?, ?, ?, ?, ?, ?)
			ON CONFLICT (id) DO UPDATE SET
				name=excluded.name, role=excluded.role, organization_id=excluded.organization_id,
				status=excluded.status, provider_type=excluded.provider_type, region=excluded.region`,
			agent.ID, agent.Name, agent.Role, agent.OrganizationID,
			string(agent.Status), agent.ProviderType, agent.Region,
		)
	}
	if err != nil {
		return fmt.Errorf("db: register agent: %w", err)
	}
	return nil
}

func (r *PgHubRepository) GetAgent(ctx context.Context, id string) (Agent, bool, error) {
	var a Agent
	var status string
	var err error
	if r.db.IsPostgres() {
		err = r.db.QueryRowContext(ctx, `
			SELECT id, name, role, organization_id, status, provider_type, region
			FROM agents WHERE id = $1`, id).Scan(
			&a.ID, &a.Name, &a.Role, &a.OrganizationID, &status, &a.ProviderType, &a.Region,
		)
	} else {
		err = r.db.QueryRowContext(ctx, `
			SELECT id, name, role, organization_id, status, provider_type, region
			FROM agents WHERE id = ?`, id).Scan(
			&a.ID, &a.Name, &a.Role, &a.OrganizationID, &status, &a.ProviderType, &a.Region,
		)
	}
	if err != nil {
		if strings.Contains(err.Error(), "no rows in result set") {
			return Agent{}, false, nil
		}
		return Agent{}, false, fmt.Errorf("db: get agent: %w", err)
	}
	a.Status = Status(status)
	return a, true, nil
}

func (r *PgHubRepository) ListAgents(ctx context.Context) ([]Agent, error) {
	var rows interface {
		Next() bool
		Scan(...any) error
		Close() error
	}
	var err error
	if r.db.IsPostgres() {
		rows, err = r.db.QueryContext(ctx, `
			SELECT id, name, role, organization_id, status, provider_type, region
			FROM agents ORDER BY id`)
	} else {
		rows, err = r.db.QueryContext(ctx, `
			SELECT id, name, role, organization_id, status, provider_type, region
			FROM agents ORDER BY id`)
	}
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
	var err error
	if r.db.IsPostgres() {
		_, err = r.db.ExecContext(ctx, "UPDATE agents SET status = $2 WHERE id = $1", id, string(status))
	} else {
		_, err = r.db.ExecContext(ctx, "UPDATE agents SET status = ? WHERE id = ?", string(status), id)
	}
	if err != nil {
		return fmt.Errorf("db: update agent status: %w", err)
	}
	return nil
}

func (r *PgHubRepository) RemoveAgent(ctx context.Context, id string) error {
	tx, err := r.db.BeginTx(ctx, nil)
	if err != nil {
		return fmt.Errorf("db: begin remove agent: %w", err)
	}
	defer func() { _ = tx.Rollback() }()

	if r.db.IsPostgres() {
		if _, err := tx.ExecContext(ctx, "DELETE FROM agent_inbox WHERE agent_id = $1", id); err != nil {
			return fmt.Errorf("db: clear inbox pg: %w", err)
		}
		if _, err := tx.ExecContext(ctx, "DELETE FROM agents WHERE id = $1", id); err != nil {
			return fmt.Errorf("db: delete agent pg: %w", err)
		}
	} else {
		if _, err := tx.ExecContext(ctx, "DELETE FROM agent_inbox WHERE agent_id = ?", id); err != nil {
			return fmt.Errorf("db: clear inbox sqlite: %w", err)
		}
		if _, err := tx.ExecContext(ctx, "DELETE FROM agents WHERE id = ?", id); err != nil {
			return fmt.Errorf("db: delete agent sqlite: %w", err)
		}
	}
	return tx.Commit()
}

func (r *PgHubRepository) PushMessage(ctx context.Context, toAgent string, msg Message) error {
	var err error
	if r.db.IsPostgres() {
		_, err = r.db.ExecContext(ctx, `
			INSERT INTO agent_inbox (agent_id, message_id, from_agent, to_agent, type, content, meeting_id, occurred_at)
			VALUES ($1, $2, $3, $4, $5, $6, $7, $8)`,
			toAgent, msg.ID, msg.FromAgent, msg.ToAgent, msg.Type, msg.Content, msg.MeetingID, msg.OccurredAt,
		)
	} else {
		_, err = r.db.ExecContext(ctx, `
			INSERT INTO agent_inbox (agent_id, message_id, from_agent, to_agent, type, content, meeting_id, occurred_at)
			VALUES (?, ?, ?, ?, ?, ?, ?, ?)`,
			toAgent, msg.ID, msg.FromAgent, msg.ToAgent, msg.Type, msg.Content, msg.MeetingID, msg.OccurredAt,
		)
	}
	if err != nil {
		return fmt.Errorf("db: push message: %w", err)
	}
	return nil
}

func (r *PgHubRepository) PopMessages(ctx context.Context, agentID string) ([]Message, error) {
	if r.db.IsPostgres() {
		rows, err := r.db.QueryContext(ctx, `
			DELETE FROM agent_inbox WHERE agent_id = $1
			RETURNING message_id, from_agent, to_agent, type, content, meeting_id, occurred_at`, agentID)
		if err != nil {
			return nil, fmt.Errorf("db: pop messages pg: %w", err)
		}
		defer rows.Close()

		var msgs []Message
		for rows.Next() {
			var m Message
			if err := rows.Scan(&m.ID, &m.FromAgent, &m.ToAgent, &m.Type, &m.Content, &m.MeetingID, &m.OccurredAt); err != nil {
				return nil, fmt.Errorf("db: scan message pg: %w", err)
			}
			msgs = append(msgs, m)
		}
		return msgs, nil
	} else {
		// SQLite doesn't reliably support DELETE ... RETURNING across all versions without extra pragmas
		// Use a transaction to SELECT then DELETE
		tx, err := r.db.BeginTx(ctx, nil)
		if err != nil {
			return nil, fmt.Errorf("db: begin pop messages tx: %w", err)
		}
		defer func() { _ = tx.Rollback() }()

		rows, err := tx.QueryContext(ctx, `
			SELECT message_id, from_agent, to_agent, type, content, meeting_id, occurred_at
			FROM agent_inbox WHERE agent_id = ? ORDER BY seq`, agentID)
		if err != nil {
			return nil, fmt.Errorf("db: pop messages select sqlite: %w", err)
		}

		var msgs []Message
		for rows.Next() {
			var m Message
			if err := rows.Scan(&m.ID, &m.FromAgent, &m.ToAgent, &m.Type, &m.Content, &m.MeetingID, &m.OccurredAt); err != nil {
				rows.Close()
				return nil, fmt.Errorf("db: scan message sqlite: %w", err)
			}
			msgs = append(msgs, m)
		}
		rows.Close()

		if len(msgs) > 0 {
			if _, err := tx.ExecContext(ctx, "DELETE FROM agent_inbox WHERE agent_id = ?", agentID); err != nil {
				return nil, fmt.Errorf("db: pop messages delete sqlite: %w", err)
			}
		}

		if err := tx.Commit(); err != nil {
			return nil, fmt.Errorf("db: pop messages commit tx: %w", err)
		}

		return msgs, nil
	}
}

func (r *PgHubRepository) PeekMessages(ctx context.Context, agentID string) ([]Message, error) {
	var rows interface {
		Next() bool
		Scan(...any) error
		Close() error
	}
	var err error

	if r.db.IsPostgres() {
		rows, err = r.db.QueryContext(ctx, `
			SELECT message_id, from_agent, to_agent, type, content, meeting_id, occurred_at
			FROM agent_inbox WHERE agent_id = $1 ORDER BY seq`, agentID)
	} else {
		rows, err = r.db.QueryContext(ctx, `
			SELECT message_id, from_agent, to_agent, type, content, meeting_id, occurred_at
			FROM agent_inbox WHERE agent_id = ? ORDER BY seq`, agentID)
	}
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
	var err error
	if r.db.IsPostgres() {
		_, err = r.db.ExecContext(ctx, `
			INSERT INTO meeting_rooms (id, agenda, participants)
			VALUES ($1, $2, $3)
			ON CONFLICT (id) DO UPDATE SET agenda=EXCLUDED.agenda, participants=EXCLUDED.participants`,
			room.ID, room.Agenda, room.Participants,
		)
	} else {
		participantsJson, _ := json.Marshal(room.Participants)
		_, err = r.db.ExecContext(ctx, `
			INSERT INTO meeting_rooms (id, agenda, participants)
			VALUES (?, ?, ?)
			ON CONFLICT (id) DO UPDATE SET agenda=excluded.agenda, participants=excluded.participants`,
			room.ID, room.Agenda, string(participantsJson),
		)
	}
	if err != nil {
		return fmt.Errorf("db: create meeting: %w", err)
	}
	return nil
}

func (r *PgHubRepository) GetMeeting(ctx context.Context, id string) (MeetingRoom, bool, error) {
	var room MeetingRoom
	var err error
	if r.db.IsPostgres() {
		err = r.db.QueryRowContext(ctx, "SELECT id, agenda, participants FROM meeting_rooms WHERE id = $1", id).Scan(
			&room.ID, &room.Agenda, &room.Participants,
		)
	} else {
		var participantsJson string
		err = r.db.QueryRowContext(ctx, "SELECT id, agenda, participants FROM meeting_rooms WHERE id = ?", id).Scan(
			&room.ID, &room.Agenda, &participantsJson,
		)
		if err == nil {
			json.Unmarshal([]byte(participantsJson), &room.Participants)
		}
	}
	if err != nil {
		if strings.Contains(err.Error(), "no rows in result set") {
			return MeetingRoom{}, false, nil
		}
		return MeetingRoom{}, false, fmt.Errorf("db: get meeting: %w", err)
	}

	// Load transcript.
	var rows interface {
		Next() bool
		Scan(...any) error
		Close() error
	}

	if r.db.IsPostgres() {
		rows, err = r.db.QueryContext(ctx, `
			SELECT message_id, from_agent, to_agent, type, content, occurred_at
			FROM meeting_transcripts WHERE meeting_id = $1 ORDER BY seq`, id)
	} else {
		rows, err = r.db.QueryContext(ctx, `
			SELECT message_id, from_agent, to_agent, type, content, occurred_at
			FROM meeting_transcripts WHERE meeting_id = ? ORDER BY seq`, id)
	}
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
	var err error
	if r.db.IsPostgres() {
		_, err = r.db.ExecContext(ctx, `
			INSERT INTO meeting_transcripts (meeting_id, message_id, from_agent, to_agent, type, content, occurred_at)
			VALUES ($1, $2, $3, $4, $5, $6, $7)`,
			meetingID, msg.ID, msg.FromAgent, msg.ToAgent, msg.Type, msg.Content, msg.OccurredAt,
		)
	} else {
		_, err = r.db.ExecContext(ctx, `
			INSERT INTO meeting_transcripts (meeting_id, message_id, from_agent, to_agent, type, content, occurred_at)
			VALUES (?, ?, ?, ?, ?, ?, ?)`,
			meetingID, msg.ID, msg.FromAgent, msg.ToAgent, msg.Type, msg.Content, msg.OccurredAt,
		)
	}
	if err != nil {
		return fmt.Errorf("db: append transcript: %w", err)
	}
	return nil
}

func (r *PgHubRepository) ListMeetings(ctx context.Context) ([]MeetingRoom, error) {
	var rows interface {
		Next() bool
		Scan(...any) error
		Close() error
	}
	var err error

	if r.db.IsPostgres() {
		rows, err = r.db.QueryContext(ctx, "SELECT id, agenda, participants FROM meeting_rooms ORDER BY id")
	} else {
		rows, err = r.db.QueryContext(ctx, "SELECT id, agenda, participants FROM meeting_rooms ORDER BY id")
	}
	if err != nil {
		return nil, fmt.Errorf("db: list meetings: %w", err)
	}
	defer rows.Close()

	var rooms []MeetingRoom
	for rows.Next() {
		var room MeetingRoom
		if r.db.IsPostgres() {
			if err := rows.Scan(&room.ID, &room.Agenda, &room.Participants); err != nil {
				return nil, fmt.Errorf("db: scan meeting pg: %w", err)
			}
		} else {
			var participantsJson string
			if err := rows.Scan(&room.ID, &room.Agenda, &participantsJson); err != nil {
				return nil, fmt.Errorf("db: scan meeting sqlite: %w", err)
			}
			json.Unmarshal([]byte(participantsJson), &room.Participants)
		}
		rooms = append(rooms, room)
	}
	return rooms, nil
}
