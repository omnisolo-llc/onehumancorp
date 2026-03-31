package agents

import (
	"time"
)

// Status indicates the current operational phase of an AI agent within the workforce.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type Status string

const (
	// StatusIdle represents the IDLE lifecycle phase of a tracked entity within the event-driven state machine.
	StatusIdle Status = "IDLE"
	// StatusActive represents the ACTIVE lifecycle phase of a tracked entity within the event-driven state machine.
	StatusActive Status = "ACTIVE"
	// StatusInMeeting represents the INMEETING lifecycle phase of a tracked entity within the event-driven state machine.
	StatusInMeeting Status = "IN_MEETING"
	// StatusBlocked represents the BLOCKED lifecycle phase of a tracked entity within the event-driven state machine.
	StatusBlocked Status = "BLOCKED"
	// StatusWaitingForTools represents the WAITINGFORTOOLS lifecycle phase of a tracked entity within the event-driven state machine.
	StatusWaitingForTools Status = "WAITING_FOR_TOOLS"
)

// Event type constants for the asynchronous pub/sub agent interaction protocol.
const (
	EventTask           = "task"
	EventStatus         = "status"
	EventHandoff        = "handoff"
	EventCodeReviewed   = "CodeReviewed"
	EventTestsFailed    = "TestsFailed"
	EventTestsPassed    = "TestsPassed"
	EventSpecApproved   = "SpecApproved"
	EventBlockerRaised  = "BlockerRaised"
	EventBlockerCleared = "BlockerCleared"
	EventPRCreated      = "PRCreated"
	EventPRMerged       = "PRMerged"
	EventDesignReviewed = "DesignReviewed"
	EventApprovalNeeded = "ApprovalNeeded"
)

// Capabilities available to agents, dynamically extended via the K8s Kubelet MCP bundles and standard registry.
var AvailableMCPBundles = []string{
	"github",
}

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
