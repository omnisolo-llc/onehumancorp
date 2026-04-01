package orchestration

import (
	"context"
	"fmt"
	"log/slog"
	"time"

	"github.com/jackc/pgx/v5/pgxpool"
)

// PgHubRepository implements HubRepository backed by PostgreSQL.
type PgHubRepository struct {
	pool *pgxpool.Pool
}

// withPgRetry executes a database operation with exponential backoff for transient errors.
func withPgRetry(ctx context.Context, op func() error) error {
	var err error
	maxRetries := 3
	retryInterval := 100 * time.Millisecond

	for i := 0; i < maxRetries; i++ {
		err = op()
		if err == nil {
			return nil
		}

		select {
		case <-ctx.Done():
			return ctx.Err()
		default:
		}

		slog.Warn("pghub: operation failed, retrying", "attempt", i+1, "error", err)
		time.Sleep(retryInterval * time.Duration(1<<i))
	}
	return err
}

// NewPgHubRepository creates a Postgres-backed hub repository.
func NewPgHubRepository(pool *pgxpool.Pool) *PgHubRepository {
	return &PgHubRepository{pool: pool}
}

func (r *PgHubRepository) RegisterAgent(ctx context.Context, agent Agent) error {
	return withPgRetry(ctx, func() error {
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
			return fmt.Errorf("pg: register agent: %w", err)
		}
		return nil
	})
}

func (r *PgHubRepository) GetAgent(ctx context.Context, id string) (Agent, bool, error) {
	var a Agent
	var status string

	err := withPgRetry(ctx, func() error {
		err := r.pool.QueryRow(ctx, `
			SELECT id, name, role, organization_id, status, provider_type, region
			FROM agents WHERE id = $1`, id).Scan(
			&a.ID, &a.Name, &a.Role, &a.OrganizationID, &status, &a.ProviderType, &a.Region,
		)
		if err != nil {
			if err.Error() == "no rows in result set" {
				return nil
			}
			return err
		}
		return nil
	})

	if err != nil {
		return Agent{}, false, fmt.Errorf("pg: get agent: %w", err)
	}
	if a.ID == "" {
		return Agent{}, false, nil
	}
	a.Status = Status(status)
	return a, true, nil
}

func (r *PgHubRepository) ListAgents(ctx context.Context) ([]Agent, error) {
	var agents []Agent
	err := withPgRetry(ctx, func() error {
		agents = nil
		rows, err := r.pool.Query(ctx, `
			SELECT id, name, role, organization_id, status, provider_type, region
			FROM agents ORDER BY id`)
		if err != nil {
			return err
		}
		defer rows.Close()

		for rows.Next() {
			var a Agent
			var status string
			if err := rows.Scan(&a.ID, &a.Name, &a.Role, &a.OrganizationID, &status, &a.ProviderType, &a.Region); err != nil {
				return err
			}
			a.Status = Status(status)
			agents = append(agents, a)
		}
		return nil
	})
	if err != nil {
		return nil, fmt.Errorf("pg: list agents: %w", err)
	}
	return agents, nil
}

func (r *PgHubRepository) UpdateAgentStatus(ctx context.Context, id string, status Status) error {
	return withPgRetry(ctx, func() error {
		_, err := r.pool.Exec(ctx, "UPDATE agents SET status = $2 WHERE id = $1", id, string(status))
		if err != nil {
			return fmt.Errorf("pg: update agent status: %w", err)
		}
		return nil
	})
}

func (r *PgHubRepository) RemoveAgent(ctx context.Context, id string) error {
	return withPgRetry(ctx, func() error {
		tx, err := r.pool.Begin(ctx)
		if err != nil {
			return fmt.Errorf("pg: begin remove agent: %w", err)
		}
		defer func() { _ = tx.Rollback(ctx) }()

		if _, err := tx.Exec(ctx, "DELETE FROM agent_inbox WHERE agent_id = $1", id); err != nil {
			return fmt.Errorf("pg: clear inbox: %w", err)
		}
		if _, err := tx.Exec(ctx, "DELETE FROM agents WHERE id = $1", id); err != nil {
			return fmt.Errorf("pg: delete agent: %w", err)
		}
		return tx.Commit(ctx)
	})
}

func (r *PgHubRepository) PushMessage(ctx context.Context, toAgent string, msg Message) error {
	return withPgRetry(ctx, func() error {
		_, err := r.pool.Exec(ctx, `
			INSERT INTO agent_inbox (agent_id, message_id, from_agent, to_agent, type, content, meeting_id, occurred_at)
			VALUES ($1, $2, $3, $4, $5, $6, $7, $8)`,
			toAgent, msg.ID, msg.FromAgent, msg.ToAgent, msg.Type, msg.Content, msg.MeetingID, msg.OccurredAt,
		)
		if err != nil {
			return fmt.Errorf("pg: push message: %w", err)
		}
		return nil
	})
}

// PopMessages atomically retrieves and removes all pending messages.
// Uses DELETE ... RETURNING for consume-once semantics.
func (r *PgHubRepository) PopMessages(ctx context.Context, agentID string) ([]Message, error) {
	var msgs []Message
	err := withPgRetry(ctx, func() error {
		msgs = nil
		rows, err := r.pool.Query(ctx, `
			DELETE FROM agent_inbox WHERE agent_id = $1
			RETURNING message_id, from_agent, to_agent, type, content, meeting_id, occurred_at`, agentID)
		if err != nil {
			return err
		}
		defer rows.Close()

		for rows.Next() {
			var m Message
			if err := rows.Scan(&m.ID, &m.FromAgent, &m.ToAgent, &m.Type, &m.Content, &m.MeetingID, &m.OccurredAt); err != nil {
				return err
			}
			msgs = append(msgs, m)
		}
		return nil
	})
	if err != nil {
		return nil, fmt.Errorf("pg: pop messages: %w", err)
	}
	return msgs, nil
}

func (r *PgHubRepository) PeekMessages(ctx context.Context, agentID string) ([]Message, error) {
	var msgs []Message
	err := withPgRetry(ctx, func() error {
		msgs = nil
		rows, err := r.pool.Query(ctx, `
			SELECT message_id, from_agent, to_agent, type, content, meeting_id, occurred_at
			FROM agent_inbox WHERE agent_id = $1 ORDER BY seq`, agentID)
		if err != nil {
			return err
		}
		defer rows.Close()

		for rows.Next() {
			var m Message
			if err := rows.Scan(&m.ID, &m.FromAgent, &m.ToAgent, &m.Type, &m.Content, &m.MeetingID, &m.OccurredAt); err != nil {
				return err
			}
			msgs = append(msgs, m)
		}
		return nil
	})
	if err != nil {
		return nil, fmt.Errorf("pg: peek messages: %w", err)
	}
	return msgs, nil
}

func (r *PgHubRepository) CreateMeeting(ctx context.Context, room MeetingRoom) error {
	return withPgRetry(ctx, func() error {
		_, err := r.pool.Exec(ctx, `
			INSERT INTO meeting_rooms (id, agenda, participants)
			VALUES ($1, $2, $3)
			ON CONFLICT (id) DO UPDATE SET agenda=EXCLUDED.agenda, participants=EXCLUDED.participants`,
			room.ID, room.Agenda, room.Participants,
		)
		if err != nil {
			return fmt.Errorf("pg: create meeting: %w", err)
		}
		return nil
	})
}

func (r *PgHubRepository) GetMeeting(ctx context.Context, id string) (MeetingRoom, bool, error) {
	var room MeetingRoom

	err := withPgRetry(ctx, func() error {
		err := r.pool.QueryRow(ctx, "SELECT id, agenda, participants FROM meeting_rooms WHERE id = $1", id).Scan(
			&room.ID, &room.Agenda, &room.Participants,
		)
		if err != nil {
			if err.Error() == "no rows in result set" {
				return nil
			}
			return err
		}

		// Load transcript.
		rows, err := r.pool.Query(ctx, `
			SELECT message_id, from_agent, to_agent, type, content, occurred_at
			FROM meeting_transcripts WHERE meeting_id = $1 ORDER BY seq`, id)
		if err != nil {
			return err
		}
		defer rows.Close()

		room.Transcript = nil
		for rows.Next() {
			var m Message
			if err := rows.Scan(&m.ID, &m.FromAgent, &m.ToAgent, &m.Type, &m.Content, &m.OccurredAt); err != nil {
				return err
			}
			m.MeetingID = id
			room.Transcript = append(room.Transcript, m)
		}
		return nil
	})

	if err != nil {
		return MeetingRoom{}, false, fmt.Errorf("pg: get meeting: %w", err)
	}
	if room.ID == "" {
		return MeetingRoom{}, false, nil
	}

	return room, true, nil
}

func (r *PgHubRepository) AppendTranscript(ctx context.Context, meetingID string, msg Message) error {
	return withPgRetry(ctx, func() error {
		_, err := r.pool.Exec(ctx, `
			INSERT INTO meeting_transcripts (meeting_id, message_id, from_agent, to_agent, type, content, occurred_at)
			VALUES ($1, $2, $3, $4, $5, $6, $7)`,
			meetingID, msg.ID, msg.FromAgent, msg.ToAgent, msg.Type, msg.Content, msg.OccurredAt,
		)
		if err != nil {
			return fmt.Errorf("pg: append transcript: %w", err)
		}
		return nil
	})
}

func (r *PgHubRepository) ListMeetings(ctx context.Context) ([]MeetingRoom, error) {
	var rooms []MeetingRoom
	err := withPgRetry(ctx, func() error {
		rooms = nil
		rows, err := r.pool.Query(ctx, "SELECT id, agenda, participants FROM meeting_rooms ORDER BY id")
		if err != nil {
			return err
		}
		defer rows.Close()

		for rows.Next() {
			var room MeetingRoom
			if err := rows.Scan(&room.ID, &room.Agenda, &room.Participants); err != nil {
				return err
			}
			rooms = append(rooms, room)
		}
		return nil
	})
	if err != nil {
		return nil, fmt.Errorf("pg: list meetings: %w", err)
	}
	return rooms, nil
}
