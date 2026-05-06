package interop

import "testing"

func TestOpenClawAdapter(t *testing.T) {
	adapter := &OpenClawAdapter{}
	res, _ := adapter.TranslateState(map[string]interface{}{"msg": "hello"})
	if res["type"] != "PatchPayload" {
		t.Fail()
	}
}
