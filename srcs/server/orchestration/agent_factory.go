package orchestration

// AgentFactory creates a new Agent configured to use the builtin AI engine by default.
func AgentFactory(id, name, role, organizationID, region string) Agent {
	return Agent{
		ID:             id,
		Name:           name,
		Role:           role,
		OrganizationID: organizationID,
		Status:         StatusIdle,
		ProviderType:   "builtin",
		Region:         region,
	}
}
