package interop

import "testing"

func TestSemanticKernelAdapter(t *testing.T) {
	adapter := &SemanticKernelAdapter{}
	res, _ := adapter.TranslateState(map[string]interface{}{"msg": "hello"})
	if res["type"] != "MCPJSONRPC" {
		t.Fail()
	}
}
