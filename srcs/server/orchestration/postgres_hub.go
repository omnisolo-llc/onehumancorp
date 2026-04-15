package orchestration

import (
	"context"
	"encoding/json"
	"fmt"
	"math/rand"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

// PgHubRepository implements HubRepository backed by PostgreSQL.
type PgHubRepository struct {
	pool  db.Provider
	orgID string
}

// pgWithRetry wraps a database operation with exponential backoff for transient errors
// to ensure mode parity with SQLite deployments and handle network partitions in Cloud Mode.
func pgWithRetry(ctx context.Context, op func() error) error {
	maxRetries := 5
	baseDelay := 10 * time.Millisecond
	maxDelay := 500 * time.Millisecond

	var err error
	for i := 0; i < maxRetries; i++ {
		err = op()
		if err == nil {
			return nil
		}

		// Check for common transient or concurrency errors
		// For Postgres this might be deadlocks or network issues.
		// For SQLite this might be "database is locked" or "database table is locked".
		errMsg := err.Error()
		isTransient := errMsg == "database is locked" ||
			errMsg == "database table is locked" ||
			// Postgres serialization/deadlock errors (often 40001 or 40P01)
			// Network partition/timeout
			// For simplicity we will retry on all errors that sound transient or if the operation is fundamentally retriable
			// Note: A more precise check can be done, but for Chaos Engineering resilience, a broad retry is often safer for transient network issues.
			// Let's implement backoff
			true // retry on all errors since we don't inspect PgxError codes directly here easily due to db.Provider abstraction.

		if isTransient {
			delay := baseDelay * (1 << i)
			if delay > maxDelay {
				delay = maxDelay
			}
			jitter := time.Duration(rand.Int63n(int64(delay) / 5))

			select {
			case <-time.After(delay + jitter):
			case <-ctx.Done():
				return ctx.Err()
			}
			continue
		}
		break
	}
	return err
}

// NewPgHubRepository creates a Postgres-backed hub repository.
func NewPgHubRepository(pool db.Provider, orgID string) *PgHubRepository {
	return &PgHubRepository{pool: pool, orgID: orgID}
}

func (r *PgHubRepository) RegisterAgent(ctx context.Context, agent Agent) error {
	return pgWithRetry(ctx, func() error {
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
	var queryErr error
	var isNotFound bool

	err := pgWithRetry(ctx, func() error {
		query := `SELECT id, name, role, organization_id, status, provider_type, region FROM agents WHERE id = $1`
		args := []any{id}
		if r.orgID != "" {
			query += ` AND organization_id = $2`
			args = append(args, r.orgID)
		}
		queryErr = r.pool.QueryRow(ctx, query, args...).Scan(
			&a.ID, &a.Name, &a.Role, &a.OrganizationID, &status, &a.ProviderType, &a.Region,
		)
		if queryErr != nil {
			if queryErr.Error() == "no rows in result set" {
				isNotFound = true
				return nil // Don't retry on not found
			}
			return fmt.Errorf("pg: get agent: %w", queryErr)
		}
		return nil
	})

	if err != nil {
		return Agent{}, false, err
	}
	if isNotFound {
		return Agent{}, false, nil
	}

	a.Status = Status(status)
	return a, true, nil
}

func (r *PgHubRepository) ListAgents(ctx context.Context) ([]Agent, error) {
	var agents []Agent
	err := pgWithRetry(ctx, func() error {
		agents = nil // Reset on retry
		query := "SELECT id, name, role, organization_id, status, provider_type, region FROM agents"
		var args []any
		if r.orgID != "" {
			query += " WHERE organization_id = $1"
			args = append(args, r.orgID)
		}
		query += " ORDER BY id"
		rows, err := r.pool.Query(ctx, query, args...)
		if err != nil {
			return fmt.Errorf("pg: list agents: %w", err)
		}
		defer rows.Close()

		for rows.Next() {
			var a Agent
			var status string
			if err := rows.Scan(&a.ID, &a.Name, &a.Role, &a.OrganizationID, &status, &a.ProviderType, &a.Region); err != nil {
				return fmt.Errorf("pg: scan agent: %w", err)
			}
			a.Status = Status(status)
			agents = append(agents, a)
		}
		return nil
	})

	return agents, err
}

func (r *PgHubRepository) UpdateAgentStatus(ctx context.Context, id string, status Status) error {
	return pgWithRetry(ctx, func() error {
		query := "UPDATE agents SET status = $2 WHERE id = $1"
		var args []any = []any{id, string(status)}
		if r.orgID != "" {
			query += " AND organization_id = $3"
			args = append(args, r.orgID)
		}
		_, err := r.pool.Exec(ctx, query, args...)
		if err != nil {
			return fmt.Errorf("pg: update agent status: %w", err)
		}
		return nil
	})
}

func (r *PgHubRepository) RemoveAgent(ctx context.Context, id string) error {
	// Prevent unauthorized deletion if scoped
	if r.orgID != "" {
		var count int
		err := r.pool.QueryRow(ctx, "SELECT COUNT(*) FROM agents WHERE id = $1 AND organization_id = $2", id, r.orgID).Scan(&count)
		if err != nil || count == 0 {
			return fmt.Errorf("pg: unauthorized or missing agent")
		}
	}

	return pgWithRetry(ctx, func() error {
		tx, err := r.pool.Begin(ctx)
		if err != nil {
			return fmt.Errorf("pg: begin remove agent: %w", err)
		}
		defer func() { _ = tx.Rollback(ctx) }()

		clearQuery := "DELETE FROM agent_inbox WHERE agent_id = $1"
		delQuery := "DELETE FROM agents WHERE id = $1"
		var args []any = []any{id}

		if r.orgID != "" {
			clearQuery += " AND organization_id = $2"
			delQuery += " AND organization_id = $2"
			args = append(args, r.orgID)
		}

		if _, err := tx.Exec(ctx, clearQuery, args...); err != nil {
			return fmt.Errorf("pg: clear inbox: %w", err)
		}
		if _, err := tx.Exec(ctx, delQuery, args...); err != nil {
			return fmt.Errorf("pg: delete agent: %w", err)
		}
		return tx.Commit(ctx)
	})
}

func (r *PgHubRepository) AppendEvent(ctx context.Context, event HubEvent) error {
	return pgWithRetry(ctx, func() error {
		payload := json.RawMessage(event.Payload)
		if len(payload) == 0 {
			payload = json.RawMessage("{}")
		}

		_, err := r.pool.Exec(ctx, `
			INSERT INTO hub_events (type, payload, occurred_at)
			VALUES ($1, $2::jsonb, $3)
		`, event.Type, string(payload), event.OccurredAt.UTC())
		if err != nil {
			return fmt.Errorf("pg: append event: %w", err)
		}
		return nil
	})
}

func (r *PgHubRepository) PushMessage(ctx context.Context, toAgent string, msg Message) error {
	return pgWithRetry(ctx, func() error {
		if r.orgID != "" {
			var count int
			err := r.pool.QueryRow(ctx, "SELECT COUNT(*) FROM agents WHERE id = $1 AND organization_id = $2", toAgent, r.orgID).Scan(&count)
			if err != nil || count == 0 {
				return fmt.Errorf("pg: unauthorized or missing agent for push")
			}
		}
		_, err := r.pool.Exec(ctx, `
			INSERT INTO agent_inbox (agent_id, message_id, from_agent, to_agent, type, content, meeting_id, occurred_at, organization_id)
			VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)`,
			toAgent, msg.ID, msg.FromAgent, msg.ToAgent, msg.Type, msg.Content, msg.MeetingID, msg.OccurredAt, r.orgID,
		)
		if err != nil {
			return fmt.Errorf("pg: push message: %w", err)
		}
		return nil
	})
}

// PopMessages atomically retrieves and removes all pending messages.
// Uses a transaction for consume-once semantics.
func (r *PgHubRepository) PopMessages(ctx context.Context, agentID string) ([]Message, error) {
	var msgs []Message
	err := pgWithRetry(ctx, func() error {
		msgs = nil // Reset on retry

		if r.orgID != "" {
			var count int
			err := r.pool.QueryRow(ctx, "SELECT COUNT(*) FROM agents WHERE id = $1 AND organization_id = $2", agentID, r.orgID).Scan(&count)
			if err != nil || count == 0 {
				return fmt.Errorf("pg: unauthorized or missing agent for pop")
			}
		}

		tx, err := r.pool.Begin(ctx)
		if err != nil {
			return fmt.Errorf("pg: begin pop: %w", err)
		}
		defer func() { _ = tx.Rollback(ctx) }()

		query := `SELECT message_id, from_agent, to_agent, type, content, meeting_id, occurred_at FROM agent_inbox WHERE agent_id = $1`
		args := []any{agentID}
		if r.orgID != "" {
			query += ` AND organization_id = $2`
			args = append(args, r.orgID)
		}
		query += ` ORDER BY seq`

		rows, err := tx.Query(ctx, query, args...)
		if err != nil {
			return fmt.Errorf("pg: peek messages for pop: %w", err)
		}

		for rows.Next() {
			var m Message
			if err := rows.Scan(&m.ID, &m.FromAgent, &m.ToAgent, &m.Type, &m.Content, &m.MeetingID, &m.OccurredAt); err != nil {
				rows.Close()
				return fmt.Errorf("pg: scan message: %w", err)
			}
			msgs = append(msgs, m)
		}
		rows.Close()

		if len(msgs) > 0 {
			delQuery := "DELETE FROM agent_inbox WHERE agent_id = $1"
			delArgs := []any{agentID}
			if r.orgID != "" {
				delQuery += " AND organization_id = $2"
				delArgs = append(delArgs, r.orgID)
			}
			_, err = tx.Exec(ctx, delQuery, delArgs...)
			if err != nil {
				return fmt.Errorf("pg: delete popped messages: %w", err)
			}
		}

		if err := tx.Commit(ctx); err != nil {
			return fmt.Errorf("pg: commit pop: %w", err)
		}
		return nil
	})

	return msgs, err
}

func (r *PgHubRepository) PeekMessages(ctx context.Context, agentID string) ([]Message, error) {
	var msgs []Message
	err := pgWithRetry(ctx, func() error {
		if r.orgID != "" {
			var count int
			err := r.pool.QueryRow(ctx, "SELECT COUNT(*) FROM agents WHERE id = $1 AND organization_id = $2", agentID, r.orgID).Scan(&count)
			if err != nil || count == 0 {
				return fmt.Errorf("pg: unauthorized or missing agent for peek")
			}
		}

		msgs = nil // Reset on retry

		query := `SELECT message_id, from_agent, to_agent, type, content, meeting_id, occurred_at FROM agent_inbox WHERE agent_id = $1`
		args := []any{agentID}
		if r.orgID != "" {
			query += ` AND organization_id = $2`
			args = append(args, r.orgID)
		}
		query += ` ORDER BY seq`

		rows, err := r.pool.Query(ctx, query, args...)
		if err != nil {
			return fmt.Errorf("pg: peek messages: %w", err)
		}
		defer rows.Close()

		for rows.Next() {
			var m Message
			if err := rows.Scan(&m.ID, &m.FromAgent, &m.ToAgent, &m.Type, &m.Content, &m.MeetingID, &m.OccurredAt); err != nil {
				return fmt.Errorf("pg: scan message: %w", err)
			}
			msgs = append(msgs, m)
		}
		return nil
	})
	return msgs, err
}

func (r *PgHubRepository) CreateMeeting(ctx context.Context, room MeetingRoom) error {
	return pgWithRetry(ctx, func() error {
		participantsJSON, _ := json.Marshal(room.Participants)
		_, err := r.pool.Exec(ctx, `
			INSERT INTO meeting_rooms (id, agenda, participants, organization_id)
			VALUES ($1, $2, $3, $4)
			ON CONFLICT (id) DO UPDATE SET agenda=EXCLUDED.agenda, participants=EXCLUDED.participants, organization_id=EXCLUDED.organization_id`,
			room.ID, room.Agenda, string(participantsJSON), r.orgID,
		)
		if err != nil {
			return fmt.Errorf("pg: create meeting: %w", err)
		}
		return nil
	})
}

func (r *PgHubRepository) GetMeeting(ctx context.Context, id string) (MeetingRoom, bool, error) {
	var room MeetingRoom
	var isNotFound bool

	err := pgWithRetry(ctx, func() error {
		var participantsJSON string
		query := "SELECT id, agenda, participants FROM meeting_rooms WHERE id = $1"
		args := []any{id}
		if r.orgID != "" {
			query += " AND organization_id = $2"
			args = append(args, r.orgID)
		}
		err := r.pool.QueryRow(ctx, query, args...).Scan(
			&room.ID, &room.Agenda, &participantsJSON,
		)
		if err != nil {
			if err.Error() == "no rows in result set" {
				isNotFound = true
				return nil
			}
			return fmt.Errorf("pg: get meeting: %w", err)
		}
		_ = json.Unmarshal([]byte(participantsJSON), &room.Participants)

		// Load transcript.
		room.Transcript = nil // Reset on retry
		rows, err := r.pool.Query(ctx, `
			SELECT message_id, from_agent, to_agent, type, content, occurred_at
			FROM meeting_transcripts WHERE meeting_id = $1 ORDER BY seq`, id)
		if err != nil {
			return fmt.Errorf("pg: get transcript: %w", err)
		}
		defer rows.Close()

		for rows.Next() {
			var m Message
			if err := rows.Scan(&m.ID, &m.FromAgent, &m.ToAgent, &m.Type, &m.Content, &m.OccurredAt); err != nil {
				return fmt.Errorf("pg: scan transcript: %w", err)
			}
			m.MeetingID = id
			room.Transcript = append(room.Transcript, m)
		}
		return nil
	})

	if err != nil {
		return MeetingRoom{}, false, err
	}
	if isNotFound {
		return MeetingRoom{}, false, nil
	}
	return room, true, nil
}

func (r *PgHubRepository) AppendTranscript(ctx context.Context, meetingID string, msg Message) error {
	return pgWithRetry(ctx, func() error {
		if r.orgID != "" {
			var count int
			err := r.pool.QueryRow(ctx, "SELECT COUNT(*) FROM meeting_rooms WHERE id = $1 AND organization_id = $2", meetingID, r.orgID).Scan(&count)
			if err != nil || count == 0 {
				return fmt.Errorf("pg: unauthorized or missing meeting room for transcript")
			}
		}
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
	err := pgWithRetry(ctx, func() error {
		rooms = nil // Reset on retry
		query := "SELECT id, agenda, participants FROM meeting_rooms"
		var args []any
		if r.orgID != "" {
			query += " WHERE organization_id = $1"
			args = append(args, r.orgID)
		}
		query += " ORDER BY id"
		rows, err := r.pool.Query(ctx, query, args...)
		if err != nil {
			return fmt.Errorf("pg: list meetings: %w", err)
		}
		defer rows.Close()

		for rows.Next() {
			var room MeetingRoom
			var participantsJSON string
			if err := rows.Scan(&room.ID, &room.Agenda, &participantsJSON); err != nil {
				return fmt.Errorf("pg: scan meeting: %w", err)
			}
			_ = json.Unmarshal([]byte(participantsJSON), &room.Participants)
			rooms = append(rooms, room)
		}
		return nil
	})
	return rooms, err
}
