import re

with open("srcs/server/orchestration/tasks.go", "r") as f:
    content = f.read()

replacement = """type SharedTask struct {
	ID              string     `json:"id"`
	OrganizationID  string     `json:"organization_id"`
	ParentPlanID    string     `json:"parent_plan_id"`
	Dependencies    []string   `json:"dependencies"`
	Title           string     `json:"title"`
	Description     string     `json:"description,omitempty"`
	AssignedAgentID string     `json:"assigned_agent_id,omitempty"`
	Status          string     `json:"status"` // PENDING, IN_PROGRESS, COMPLETED, FAILED, BLOCKED
	Priority        string     `json:"priority"`
	Payload         string     `json:"payload"`
	LockedUntil     *time.Time `json:"locked_until,omitempty"`
	CreatedAt       time.Time  `json:"created_at"`
	UpdatedAt       time.Time  `json:"updated_at"`
}"""

content = re.sub(r"type SharedTask struct \{.*?\n\}", replacement, content, flags=re.DOTALL)

# Since we use *time.Time instead of sql.NullTime now, we might need to adjust some queries.
# Let's check how LockedUntil is used.
with open("srcs/server/orchestration/tasks.go", "w") as f:
    f.write(content)
