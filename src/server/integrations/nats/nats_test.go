package nats

import (
	"testing"
	"github.com/onehumancorp/mono/src/server/integrations"
)

func TestNatsIntegration_Metadata(t *testing.T) {
	integration := &NatsIntegration{}
	meta := integration.Metadata()

	if meta.GetId() != "nats" {
		t.Errorf("Expected ID 'nats', got '%s'", meta.GetId())
	}
	if meta.GetCategory() != string(integrations.CategoryEventMesh) {
		t.Errorf("Expected Category '%s', got '%s'", integrations.CategoryEventMesh, meta.GetCategory())
	}
}
