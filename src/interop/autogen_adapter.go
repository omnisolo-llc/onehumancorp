package interop

type AutoGenAdapter struct{}

func (a *AutoGenAdapter) TranslateState(state map[string]interface{}) (map[string]interface{}, error) {
	return map[string]interface{}{"type": "AgentMessage", "original": state}, nil
}
