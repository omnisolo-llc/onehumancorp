package interop

type OpenClawAdapter struct{}

func (a *OpenClawAdapter) TranslateState(state map[string]interface{}) (map[string]interface{}, error) {
	return map[string]interface{}{"type": "PatchPayload", "original": state}, nil
}
