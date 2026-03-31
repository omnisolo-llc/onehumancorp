package agents

// Status indicates the current operational phase of an AI agent within the workforce.
type Status string

const (
	StatusIdle            Status = "IDLE"
	StatusActive          Status = "ACTIVE"
	StatusInMeeting       Status = "IN_MEETING"
	StatusBlocked         Status = "BLOCKED"
	StatusWaitingForTools Status = "WAITING_FOR_TOOLS"
)

// Agent represents an autonomous AI actor registered in the orchestration Hub, tracking its identity, role, and current state.
type Agent struct {
	ID             string `json:"id"`
	Name           string `json:"name"`
	Role           string `json:"role"`
	OrganizationID string `json:"organizationId"`
	Status         Status `json:"status"`
	ProviderType   string `json:"providerType,omitempty"`
	Region         string `json:"region,omitempty"`
}
