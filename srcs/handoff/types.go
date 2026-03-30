package handoff

import "time"

// Status represents the operational state of an agent within the swarm.
type Status string

const (
	StatusIdle      Status = "IDLE"
	StatusActive    Status = "ACTIVE"
	StatusInMeeting Status = "IN_MEETING"
	StatusBlocked   Status = "BLOCKED"

	StatusWaitingForTools Status = "WAITING_FOR_TOOLS"

)

type Agent struct {
	ID             string `json:"id"`
	Name           string `json:"name"`
	Role           string `json:"role"`
	OrganizationID string `json:"organizationId"`
	Status         Status `json:"status"`
	ProviderType   string `json:"providerType"`
}




// Message represents a discrete packet of communication between agents within a meeting room, containing the content and sender identity.
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
type MeetingRoom struct {
	ID           string    `json:"id"`
	Agenda       string    `json:"agenda"`
	Participants []string  `json:"participants"`
	Transcript   []Message `json:"transcript"`
}

const (
	EventChat           = "CHAT"
	EventReasoning      = "REASONING"
	EventTask           = "TASK"
	EventHandoff        = "HANDOFF"
	EventNotification   = "NOTIFICATION"
	EventStatus         = "STATUS"
	EventCodeReviewed   = "CODE_REVIEWED"
	EventTestsFailed    = "TESTS_FAILED"
	EventCodeMerged     = "CODE_MERGED"
	EventTestsPassed    = "TESTS_PASSED"
	EventSpecApproved   = "SPEC_APPROVED"
	EventBlockerRaised  = "BLOCKER_RAISED"
	EventBlockerCleared = "BLOCKER_CLEARED"
	EventPRCreated      = "PR_CREATED"
	EventPRMerged       = "PR_MERGED"
	EventDesignReviewed = "DESIGN_REVIEWED"
	EventApprovalNeeded = "APPROVAL_NEEDED"
)
