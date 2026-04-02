package orchestration

import "context"

// HubRepository defines the persistence contract for the orchestration
// Hub's core state: agents, message inboxes, and meeting rooms.
//
// The in-memory Hub satisfies this interface by default; a Postgres-backed
// implementation enables horizontal scaling by externalising all mutable
// state into a shared database.
type HubRepository interface {
	// --- Agent registry ---

	// RegisterAgent persists an agent registration.  If the agent already
	// exists, it updates the record.
	RegisterAgent(ctx context.Context, agent Agent) error
	// GetAgent returns a single agent by ID.
	GetAgent(ctx context.Context, id string) (Agent, bool, error)
	// ListAgents returns all registered agents.
	ListAgents(ctx context.Context) ([]Agent, error)
	// UpdateAgentStatus transitions an agent to a new status.
	UpdateAgentStatus(ctx context.Context, id string, status Status) error
	// RemoveAgent deregisters an agent and clears its inbox.
	RemoveAgent(ctx context.Context, id string) error

	// --- Message inbox ---

	// PushMessage delivers a message to the specified agent's inbox.
	PushMessage(ctx context.Context, toAgent string, msg Message) error
	// PopMessages atomically retrieves and removes all pending messages
	// from an agent's inbox (consume-once semantics).
	PopMessages(ctx context.Context, agentID string) ([]Message, error)
	// PeekMessages returns pending messages without consuming them.
	PeekMessages(ctx context.Context, agentID string) ([]Message, error)

	// --- Meeting rooms ---

	// CreateMeeting persists a new meeting room.
	CreateMeeting(ctx context.Context, room MeetingRoom) error
	// GetMeeting returns a meeting room by ID.
	GetMeeting(ctx context.Context, id string) (MeetingRoom, bool, error)
	// AppendTranscript adds a message to a meeting room's transcript.
	AppendTranscript(ctx context.Context, meetingID string, msg Message) error
	// ListMeetings returns all meeting rooms.
	ListMeetings(ctx context.Context) ([]MeetingRoom, error)

	// --- Swarm tasks ---

	// ClaimTask claims a swarm task for an agent.
	ClaimTask(ctx context.Context, taskID, agentID string) error
	// CompleteTask marks a swarm task as completed.
	CompleteTask(ctx context.Context, taskID string) error
	// CreateTask creates a new swarm task.
	CreateTask(ctx context.Context, missionID, title string, payload map[string]interface{}) (string, error)
}
