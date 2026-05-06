package interop

type SemanticKernelAdapter struct{}

func (a *SemanticKernelAdapter) TranslateState(state map[string]interface{}) (map[string]interface{}, error) {
	return map[string]interface{}{"type": "MCPJSONRPC", "original": state}, nil
}
