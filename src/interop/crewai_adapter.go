package interop

type CrewAIAdapter struct{}

func (a *CrewAIAdapter) TranslateState(state map[string]interface{}) (map[string]interface{}, error) {
	return map[string]interface{}{"type": "LangGraphState", "original": state}, nil
}
