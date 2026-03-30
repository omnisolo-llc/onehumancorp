package core

import "time"

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

// Event type constants for the asynchronous pub/sub agent interaction protocol.
const (
	// EventTask provides domain-specific context and typed constraints for EventTask operations across the application.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	EventTask = "task"
	// EventStatus provides domain-specific context and typed constraints for EventStatus operations across the application.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	EventStatus = "status"
	// EventHandoff provides domain-specific context and typed constraints for EventHandoff operations across the application.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	EventHandoff = "handoff"
	// EventCodeReviewed provides domain-specific context and typed constraints for EventCodeReviewed operations across the application.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	EventCodeReviewed = "CodeReviewed"
	// EventTestsFailed provides domain-specific context and typed constraints for EventTestsFailed operations across the application.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	EventTestsFailed = "TestsFailed"
	// EventTestsPassed provides domain-specific context and typed constraints for EventTestsPassed operations across the application.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	EventTestsPassed = "TestsPassed"
	// EventSpecApproved provides domain-specific context and typed constraints for EventSpecApproved operations across the application.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	EventSpecApproved = "SpecApproved"
	// EventBlockerRaised provides domain-specific context and typed constraints for EventBlockerRaised operations across the application.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	EventBlockerRaised = "BlockerRaised"
	// EventBlockerCleared provides domain-specific context and typed constraints for EventBlockerCleared operations across the application.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	EventBlockerCleared = "BlockerCleared"
	// EventPRCreated provides domain-specific context and typed constraints for EventPRCreated operations across the application.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	EventPRCreated = "PRCreated"
	// EventPRMerged provides domain-specific context and typed constraints for EventPRMerged operations across the application.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	EventPRMerged = "PRMerged"
	// EventDesignReviewed provides domain-specific context and typed constraints for EventDesignReviewed operations across the application.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	EventDesignReviewed = "DesignReviewed"
	// EventApprovalNeeded provides domain-specific context and typed constraints for EventApprovalNeeded operations across the application.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	EventApprovalNeeded = "ApprovalNeeded"
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

// Message represents a discrete packet of communication between agents within a meeting room, containing the content and sender identity.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type Message struct {
	ID         string    `json:"id"`
	FromAgent  string    `json:"fromAgent"`
	ToAgent    string    `json:"toAgent"`
	Type       string    `json:"type"`
	Content    string    `json:"content"`
	MeetingID  string    `json:"meetingId,omitempty"`
	OccurredAt time.Time `json:"occurredAt"`
}

// MeetingRoom provides a thread-safe, isolated collaborative space where multiple agents can exchange messages and context.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type MeetingRoom struct {
	ID           string    `json:"id"`
	Agenda       string    `json:"agenda,omitempty"`
	Participants []string  `json:"participants"`
	Transcript   []Message `json:"transcript"`
}
