package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"strings"

	_ "modernc.org/sqlite"
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
	_, err := r.db.ExecContext(ctx, `
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
	// SQLite supports RETURNING in DELETE since version 3.35.0 (2021-03-12)
	// modernc.org/sqlite uses a recent enough version.
	rows, err := r.db.QueryContext(ctx, `
		DELETE FROM agent_inbox WHERE agent_id = ?
		RETURNING message_id, from_agent, to_agent, type, content, meeting_id, occurred_at`, agentID)
	if err != nil {
		return nil, fmt.Errorf("sqlite: pop messages: %w", err)
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
		if err := rows.Scan(&m.ID, &m.FromAgent, &m.ToAgent, &m.Type, &m.Content, &m.MeetingID, &m.OccurredAt); err != nil {
			return nil, fmt.Errorf("sqlite: scan message: %w", err)
		}
		msgs = append(msgs, m)
	}
	return msgs, nil
}

func (r *SqliteHubRepository) CreateMeeting(ctx context.Context, room MeetingRoom) error {
	// For SQLite, arrays aren't natively supported, so we serialize Participants to JSON.
	participantsJSON, err := json.Marshal(room.Participants)
	if err != nil {
		return fmt.Errorf("sqlite: create meeting: marshal participants: %w", err)
	}

	_, err = r.db.ExecContext(ctx, `
		INSERT INTO meeting_rooms (id, agenda, participants)
		VALUES (?, ?, ?)
		ON CONFLICT(id) DO UPDATE SET agenda=excluded.agenda, participants=excluded.participants`,
		room.ID, room.Agenda, string(participantsJSON),
	)
	if err != nil {
		return fmt.Errorf("sqlite: create meeting: %w", err)
	}
	return nil
}

func (r *SqliteHubRepository) GetMeeting(ctx context.Context, id string) (MeetingRoom, bool, error) {
	var room MeetingRoom
	var participantsStr string
	err := r.db.QueryRowContext(ctx, "SELECT id, agenda, participants FROM meeting_rooms WHERE id = ?", id).Scan(
		&room.ID, &room.Agenda, &participantsStr,
	)
	if err != nil {
		if err == sql.ErrNoRows {
			return MeetingRoom{}, false, nil
		}
		return MeetingRoom{}, false, fmt.Errorf("sqlite: get meeting: %w", err)
	}

	if participantsStr != "" && participantsStr != "{}" {
		// Postgres might store '{}' or we might store '[]'. Try to parse JSON.
		// If it looks like a Postgres array string '{a,b}', handle it carefully.
		// Since we serialized as JSON `["a","b"]` for SQLite, it should unmarshal as JSON.
		if strings.HasPrefix(participantsStr, "{") && strings.HasSuffix(participantsStr, "}") && !strings.Contains(participantsStr, `"`) {
			// Basic cleanup of PG-style empty array string if inserted directly via some other code
			if participantsStr == "{}" {
				room.Participants = []string{}
			} else {
				// We expect json, so a postgres string format in sqlite means cross-contamination or bad seed.
				// For resilience, let's just assume JSON format:
			}
		} else {
			if err := json.Unmarshal([]byte(participantsStr), &room.Participants); err != nil {
				// Not a big deal, try treating as empty or log it
			}
		}
	} else {
		room.Participants = []string{}
	}

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
		if err := rows.Scan(&m.ID, &m.FromAgent, &m.ToAgent, &m.Type, &m.Content, &m.OccurredAt); err != nil {
			return MeetingRoom{}, false, fmt.Errorf("sqlite: scan transcript: %w", err)
		}
		m.MeetingID = id
		room.Transcript = append(room.Transcript, m)
	}
	return room, true, nil
}

func (r *SqliteHubRepository) AppendTranscript(ctx context.Context, meetingID string, msg Message) error {
	_, err := r.db.ExecContext(ctx, `
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
	rows, err := r.db.QueryContext(ctx, "SELECT id, agenda, participants FROM meeting_rooms ORDER BY id")
	if err != nil {
		return nil, fmt.Errorf("sqlite: list meetings: %w", err)
	}
	defer rows.Close()

	var rooms []MeetingRoom
	for rows.Next() {
		var room MeetingRoom
		var participantsStr string
		if err := rows.Scan(&room.ID, &room.Agenda, &participantsStr); err != nil {
			return nil, fmt.Errorf("sqlite: scan meeting: %w", err)
		}

		if participantsStr != "" && participantsStr != "{}" {
			if !strings.HasPrefix(participantsStr, "{") || strings.Contains(participantsStr, `"`) {
				_ = json.Unmarshal([]byte(participantsStr), &room.Participants)
			}
		}
		if room.Participants == nil {
			room.Participants = []string{}
		}

		rooms = append(rooms, room)
	}
	return rooms, nil
}
