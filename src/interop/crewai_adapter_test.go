package interop

import "testing"

func TestCrewAIAdapter(t *testing.T) {
	adapter := &CrewAIAdapter{}
	res, _ := adapter.TranslateState(map[string]interface{}{"msg": "hello"})
	if res["type"] != "LangGraphState" {
		t.Fail()
	}
}
