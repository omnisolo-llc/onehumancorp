package scheduler

import "context"

// TaskRepository defines the persistence contract for scheduled tasks.
// The in-memory Scheduler satisfies this interface by default; a
// Postgres-backed implementation enables horizontal scaling with proper
// distributed locking (e.g. SELECT ... FOR UPDATE SKIP LOCKED).
type TaskRepository interface {
	// Create adds a new task.  Returns an error if the task ID already exists.
	Create(ctx context.Context, task Task) error
	// Get returns a task by ID.
	Get(ctx context.Context, orgID, id string) (Task, error)
	// ListForOrg returns all tasks associated with an organization.
	ListForOrg(ctx context.Context, orgID string) ([]Task, error)
	// PollDue returns tasks that are ready to execute.  In a distributed
	// setting the implementation should use row-level locking to prevent
	// duplicate execution across replicas.
	PollDue(ctx context.Context) ([]Task, error)
	// UpdateStatus transitions a task to a new status and optionally
	// reschedules it (for interval tasks).
	UpdateStatus(ctx context.Context, id string, status TaskStatus, reschedule bool) error
	// Cancel marks a task as cancelled.
	Cancel(ctx context.Context, orgID, id string) error
}
