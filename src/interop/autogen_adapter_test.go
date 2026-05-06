package interop

import "testing"

func TestAutoGenAdapter(t *testing.T) {
	adapter := &AutoGenAdapter{}
	res, _ := adapter.TranslateState(map[string]interface{}{"msg": "hello"})
	if res["type"] != "AgentMessage" {
		t.Fail()
	}
}
