package domain

// Status indicates the current operational phase of an AI agent within the workforce.
type Status string

const (
	StatusIdle            Status = "IDLE"
	StatusActive          Status = "ACTIVE"
	StatusInMeeting       Status = "IN_MEETING"
	StatusBlocked         Status = "BLOCKED"
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
