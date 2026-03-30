package agents

// Status indicates the current operational phase of an AI agent within the workforce.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type Status string

const (
	// StatusIdle represents the IDLE lifecycle phase of a tracked entity within the event-driven state machine.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	StatusIdle Status = "IDLE"
	// StatusActive represents the ACTIVE lifecycle phase of a tracked entity within the event-driven state machine.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	StatusActive Status = "ACTIVE"
	// StatusInMeeting represents the INMEETING lifecycle phase of a tracked entity within the event-driven state machine.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	StatusInMeeting Status = "IN_MEETING"
	// StatusBlocked represents the BLOCKED lifecycle phase of a tracked entity within the event-driven state machine.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	StatusBlocked Status = "BLOCKED"
	// StatusWaitingForTools represents the WAITINGFORTOOLS lifecycle phase of a tracked entity within the event-driven state machine.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	StatusWaitingForTools Status = "WAITING_FOR_TOOLS"
)

// Agent represents an autonomous AI actor registered in the orchestration Hub, tracking its identity, role, and current state.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type Agent struct {
	ID             string `json:"id"`
	Name           string `json:"name"`
	Role           string `json:"role"`
	OrganizationID string `json:"organizationId"`
	Status         Status `json:"status"`
	// ProviderType identifies the external agent implementation backing this worker
	// (e.g. "claude", "gemini", "opencode").  An empty string or "builtin" means
	// the platform's own lightweight agent is used.
	ProviderType string `json:"providerType,omitempty"`
	Region       string `json:"region,omitempty"`
}
