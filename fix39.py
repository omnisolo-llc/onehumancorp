import os

with open('srcs/handoff/types.go', 'r') as f:
    content = f.read()

# I forgot to add `json` tags to `Agent`! So when `json.Marshal(snapshot)` runs,
# the fields are capitalized (e.g. `ProviderType`), but the test expects `providerType`.
# Let's fix the JSON tags for `Agent` and `Status`? No `Status` is just a string.
# Also MeetingRoom.

agent_struct = """type Agent struct {
	ID             string `json:"id"`
	Name           string `json:"name"`
	Role           string `json:"role"`
	OrganizationID string `json:"organizationId"`
	Status         Status `json:"status"`
	ProviderType   string `json:"providerType"`
}"""

content = content.replace("""type Agent struct {
	ID             string
	Name           string
	Role           string
	OrganizationID string
	Status         Status
	ProviderType   string
}""", agent_struct)

meeting_struct = """type MeetingRoom struct {
	ID           string    `json:"id"`
	Agenda       string    `json:"agenda"`
	Participants []string  `json:"participants"`
	Transcript   []Message `json:"transcript"`
}"""

content = content.replace("""type MeetingRoom struct {
	ID           string
	Agenda       string
	Participants []string
	Transcript   []Message `json:"transcript"`
}""", meeting_struct)

with open('srcs/handoff/types.go', 'w') as f:
    f.write(content)
