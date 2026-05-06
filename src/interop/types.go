package interop

type UniversalAgent interface {
	TranslateState(state map[string]interface{}) (map[string]interface{}, error)
}
