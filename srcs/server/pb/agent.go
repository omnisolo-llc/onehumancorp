package pb

// Agent represents an agent in the mesh registry.
type Agent struct {
	ID           string   `json:"agent_id"`
	Capabilities []string `json:"capabilities"`
	Status       string   `json:"status"`
}
