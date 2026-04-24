package nats

import (
	"context"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestNatsIntegration_Metadata(t *testing.T) {
	n := &NatsIntegration{}
	meta := n.Metadata()

	assert.NotNil(t, meta)
	assert.Equal(t, "nats", meta.GetId())
	assert.Equal(t, "NATS Event Mesh", meta.GetName())
	assert.Equal(t, "Event Bus", meta.GetCategory())
	assert.Equal(t, "nats", meta.GetType())
}

func TestNatsIntegration_WizardSteps(t *testing.T) {
	n := &NatsIntegration{}
	steps := n.WizardSteps()

	assert.NotNil(t, steps)
	assert.Len(t, steps, 1)
	assert.Equal(t, "Connect to NATS", steps[0].GetTitle())
	assert.Len(t, steps[0].GetFields(), 1)
	assert.Equal(t, "url", steps[0].GetFields()[0].GetKey())
}

// Since connecting to NATS requires a running NATS server, we can only do
// a negative test here in a simple unit test.
func TestNatsIntegration_Connect_Fail(t *testing.T) {
	n := &NatsIntegration{}
	err := n.Connect("nats://invalid:4222")
	assert.Error(t, err)
}

func TestNatsIntegration_Publish_NotConnected(t *testing.T) {
	n := &NatsIntegration{}
	err := n.Publish(context.Background(), "test", []byte("data"))
	assert.Error(t, err)
	assert.Contains(t, err.Error(), "not connected")
}

func TestNatsIntegration_Subscribe_NotConnected(t *testing.T) {
	n := &NatsIntegration{}
	_, err := n.Subscribe(context.Background(), "test", func(msg []byte) {})
	assert.Error(t, err)
	assert.Contains(t, err.Error(), "not connected")
}

func TestNatsIntegration_Close(t *testing.T) {
	n := &NatsIntegration{}
	// Should not panic when nc is nil
	n.Close()
}
